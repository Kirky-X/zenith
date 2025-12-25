//! Plugin system for external formatting tools
//!
//! This module provides functionality for loading and managing external formatting tools as plugins.
//! Plugins are defined through configuration files (JSON or TOML) that specify the command to run,
//! arguments to pass, and file extensions the plugin should handle.
//!
//! The plugin system includes:
//! - Dynamic loading of plugins from configuration files
//! - Validation of plugin commands to ensure they exist and are executable
//! - Integration with the main Zenith registry system
//! - Error handling for plugin loading and execution
use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::{Result, ZenithError};
use crate::plugins::types::PluginInfo;
use crate::utils::path::sanitize_path_for_log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use std::sync::Arc;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::{debug, error, info, warn};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;
    use tokio;

    #[tokio::test]
    async fn test_plugin_loader_creation() {
        let loader = PluginLoader::new();
        assert_eq!(loader.list_plugins().len(), 0);
    }

    #[tokio::test]
    async fn test_load_plugins_from_empty_dir() {
        let mut loader = PluginLoader::new();
        let temp_dir = TempDir::new().unwrap();

        // This should not fail even if the directory is empty
        let result = loader.load_plugins_from_dir(temp_dir.path()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_external_plugin_config_parsing() {
        let config_content = r#"{
            "name": "test-prettier",
            "command": "prettier",
            "args": ["--stdin", "--parser", "babel"],
            "extensions": ["js", "jsx"],
            "enabled": true
        }"#;

        let config: ExternalPluginConfig = serde_json::from_str(config_content).unwrap();
        assert_eq!(config.name, "test-prettier");
        assert_eq!(config.command, "prettier");
        assert_eq!(config.args, vec!["--stdin", "--parser", "babel"]);
        assert_eq!(config.extensions, vec!["js", "jsx"]);
        assert!(config.enabled);
    }

    #[tokio::test]
    async fn test_disabled_plugin_error() {
        let config_content = r#"{
            "name": "disabled-plugin",
            "command": "prettier",
            "args": [],
            "extensions": ["js"],
            "enabled": false
        }"#;

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("test_plugin.json");
        let mut file = File::create(&config_file).unwrap();
        file.write_all(config_content.as_bytes()).unwrap();

        let loader = PluginLoader::new();
        let result = loader.load_plugin_from_config(config_file).await;

        match result {
            Err(ZenithError::PluginDisabled { name }) => {
                assert_eq!(name, "disabled-plugin");
            }
            _ => panic!("Expected PluginDisabled error"),
        }
    }

    #[tokio::test]
    async fn test_external_zenith_creation() {
        let external_plugin = ExternalZenith::new(
            "test".to_string(),
            "echo".to_string(),
            vec!["--version".to_string()],
            vec!["txt".to_string()],
        );

        assert_eq!(external_plugin.name(), "test");
        assert_eq!(external_plugin.extensions(), &["txt"]);
    }
}

/// Configuration for an external plugin
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalPluginConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub extensions: Vec<String>,
    pub enabled: bool,
}

/// Configuration for a list of plugins (TOML array format)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExternalPluginConfigList {
    pub plugins: Vec<ExternalPluginConfig>,
}

/// Security configuration for plugin loading
#[derive(Debug, Clone)]
pub struct PluginSecurityConfig {
    /// List of allowed command prefixes or exact command names
    /// If empty, all commands are allowed (default behavior)
    pub allowed_commands: Vec<String>,
    /// Whether to allow absolute paths in plugin commands
    pub allow_absolute_paths: bool,
    /// Whether to allow relative paths in plugin commands
    pub allow_relative_paths: bool,
}

impl Default for PluginSecurityConfig {
    fn default() -> Self {
        Self {
            allowed_commands: Vec::new(),
            allow_absolute_paths: true,
            allow_relative_paths: false,
        }
    }
}

pub struct PluginLoader {
    loaded_plugins: HashMap<String, Arc<dyn Zenith>>,
    security_config: PluginSecurityConfig,
}

impl PluginLoader {
    pub fn new() -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            security_config: PluginSecurityConfig::default(),
        }
    }

    pub fn with_security_config(security_config: PluginSecurityConfig) -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            security_config,
        }
    }

    /// Validate that a command is allowed according to security configuration
    fn validate_command_security(&self, command: &str) -> Result<()> {
        let path = Path::new(command);

        // Check if it's an absolute path
        if path.is_absolute() && !self.security_config.allow_absolute_paths {
            return Err(ZenithError::PluginValidationError {
                name: "security".to_string(),
                error: format!("Absolute paths are not allowed: {}", command),
            });
        }

        // Check if it's a relative path
        if !path.is_absolute()
            && command.contains('/')
            && !self.security_config.allow_relative_paths
        {
            return Err(ZenithError::PluginValidationError {
                name: "security".to_string(),
                error: format!("Relative paths are not allowed: {}", command),
            });
        }

        // Check against allowed commands whitelist
        if !self.security_config.allowed_commands.is_empty() {
            let command_name = path.file_name().and_then(|n| n.to_str()).unwrap_or(command);

            let is_allowed = self
                .security_config
                .allowed_commands
                .iter()
                .any(|allowed| allowed == command_name || command.starts_with(allowed));

            if !is_allowed {
                return Err(ZenithError::PluginValidationError {
                    name: "security".to_string(),
                    error: format!(
                        "Command '{}' is not in the allowed list: {:?}",
                        command, self.security_config.allowed_commands
                    ),
                });
            }
        }

        Ok(())
    }

    /// Load plugins from a directory by scanning plugin configuration files
    pub async fn load_plugins_from_dir<P: AsRef<Path>>(&mut self, dir: P) -> Result<()> {
        let dir = dir.as_ref();

        // Check if directory exists
        if !dir.exists() {
            return Ok(());
        }

        let mut entries = fs::read_dir(dir).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            // Look for plugin configuration files (e.g., .json, .toml)
            if path
                .extension()
                .is_some_and(|ext| ext == "json" || ext == "toml")
            {
                match self.load_plugin_from_config(&path).await {
                    Ok(plugin) => {
                        self.register_plugin(plugin);
                    }
                    Err(e) => {
                        eprintln!("Failed to load plugin from {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    /// Load a single plugin from its configuration file
    /// Supports both single plugin (JSON or TOML) and plugin list (TOML array) formats
    async fn load_plugin_from_config<P: AsRef<Path>>(
        &self,
        config_path: P,
    ) -> Result<Arc<dyn Zenith>> {
        let config_path = config_path.as_ref();
        let sanitized_path = sanitize_path_for_log(config_path);
        info!("Loading plugin from: {}", sanitized_path);

        let config_content = fs::read_to_string(config_path).await?;

        // Check if this is a TOML file that might contain a list of plugins
        if config_path.extension().is_some_and(|ext| ext == "toml") {
            // Try to parse as a list of plugins first
            if let Ok(config_list) = toml::from_str::<ExternalPluginConfigList>(&config_content) {
                if !config_list.plugins.is_empty() {
                    info!(
                        "Found {} plugins in list format from: {}",
                        config_list.plugins.len(),
                        sanitized_path
                    );

                    // Load the first enabled plugin from the list
                    for config in &config_list.plugins {
                        if config.enabled {
                            debug!(
                                "Loading plugin from list: name={}, extensions={:?}",
                                config.name, config.extensions
                            );

                            self.validate_plugin_config(config).await?;

                            let external_plugin =
                                ExternalZenith::new(config.name.clone(), config.command.clone(), config.args.clone(), config.extensions.iter().map(|s| s.clone()).collect());

                            info!("Successfully loaded plugin: {}", external_plugin.name());
                            return Ok(Arc::new(external_plugin));
                        }
                    }

                    // All plugins are disabled
                    let first_disabled_name = config_list.plugins.first().map(|p| p.name.clone()).unwrap_or_else(|| "unknown".to_string());
                    return Err(ZenithError::PluginDisabled { name: first_disabled_name });
                }
            }
        }

        // Try to parse as single plugin config (JSON or TOML)
        let config: ExternalPluginConfig =
            if config_path.extension().is_some_and(|ext| ext == "json") {
                serde_json::from_str(&config_content)?
            } else {
                toml::from_str(&config_content)?
            };

        debug!(
            "Parsed plugin config: name={}, extensions={:?}",
            config.name, config.extensions
        );

        if !config.enabled {
            warn!("Plugin '{}' is disabled, skipping", config.name);
            return Err(ZenithError::PluginDisabled { name: config.name });
        }

        // Validate the plugin configuration
        self.validate_plugin_config(&config).await?;

        // Create an external plugin instance
        let external_plugin =
            ExternalZenith::new(config.name, config.command, config.args, config.extensions);

        info!("Successfully loaded plugin: {}", external_plugin.name());
        Ok(Arc::new(external_plugin))
    }

    /// Validate plugin configuration and check if the command exists and is executable
    async fn validate_plugin_config(&self, config: &ExternalPluginConfig) -> Result<()> {
        // Security validation first
        self.validate_command_security(&config.command)?;
        info!("Validating plugin '{}'", config.name);

        // Check if the command exists
        let command_path = if Path::new(&config.command).exists() {
            config.command.clone()
        } else if let Ok(output) = Command::new("which").arg(&config.command).output().await {
            if output.status.success() {
                String::from_utf8(output.stdout)?.trim().to_string()
            } else {
                return Err(ZenithError::ToolNotFound {
                    tool: config.command.clone(),
                });
            }
        } else {
            return Err(ZenithError::ToolNotFound {
                tool: config.command.clone(),
            });
        };

        debug!("Plugin '{}' command resolved", config.name);

        // Test if the command is executable by running a simple test
        // Add a simple test argument to verify the command works (e.g., --version or similar)
        // For many formatters, we can try a simple help or version flag
        let test_args = &["--help", "--version", "-h"];
        let mut test_successful = false;

        for &test_arg in test_args {
            let mut test_cmd = Command::new(&command_path);
            test_cmd.arg(test_arg);
            test_cmd.stdout(Stdio::null());
            test_cmd.stderr(Stdio::null());

            if let Ok(status) = test_cmd.status().await {
                if status.success() {
                    test_successful = true;
                    debug!(
                        "Plugin '{}' passed basic functionality test with arg: {}",
                        config.name, test_arg
                    );
                    break;
                }
            }
        }

        if !test_successful {
            warn!(
                "Plugin '{}' command exists but failed basic functionality test",
                config.name
            );
            return Err(ZenithError::PluginValidationError {
                name: config.name.clone(),
                error: "Command exists but failed basic functionality test".to_string(),
            });
        }

        info!("Plugin '{}' validation successful", config.name);
        Ok(())
    }

    /// Register a plugin manually (for testing and built-in plugins)
    pub fn register_plugin(&mut self, plugin: Arc<dyn Zenith>) {
        let plugin_name = plugin.name().to_string();
        self.loaded_plugins.insert(plugin_name, plugin);
    }

    /// Get a plugin by name
    pub fn get_plugin(&self, name: &str) -> Option<Arc<dyn Zenith>> {
        self.loaded_plugins.get(name).cloned()
    }

    /// Get information about all loaded plugins
    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        self.loaded_plugins
            .iter()
            .map(|(name, plugin)| PluginInfo {
                name: name.clone(),
                extensions: plugin.extensions().iter().map(|s| s.to_string()).collect(),
            })
            .collect()
    }
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// A mapping of common extensions to static string slices
const EXTENSION_MAP: &[(&str, &str)] = &[
    ("js", "js"),
    ("jsx", "jsx"),
    ("ts", "ts"),
    ("tsx", "tsx"),
    ("rs", "rs"),
    ("py", "py"),
    ("java", "java"),
    ("cpp", "cpp"),
    ("c", "c"),
    ("h", "h"),
    ("hpp", "hpp"),
    ("html", "html"),
    ("css", "css"),
    ("json", "json"),
    ("yaml", "yaml"),
    ("yml", "yml"),
    ("toml", "toml"),
    ("md", "md"),
    ("txt", "txt"),
    ("xml", "xml"),
    ("ini", "ini"),
    ("sh", "sh"),
    ("bash", "bash"),
    ("sql", "sql"),
    ("go", "go"),
    ("rb", "rb"),
    ("php", "php"),
    ("ts", "ts"),
];

/// Plugin implementation for external tools
pub struct ExternalZenith {
    name: String,
    command: String,
    args: Vec<String>,
    extensions: Vec<&'static str>,
}

impl ExternalZenith {
    pub fn new(
        name: String,
        command: String,
        args: Vec<String>,
        extension_strings: Vec<String>,
    ) -> Self {
        // Map the extension strings to static string slices
        let extensions: Vec<&'static str> = extension_strings
            .iter()
            .map(|ext| {
                // Look up the extension in our predefined map
                EXTENSION_MAP
                    .iter()
                    .find(|(key, _)| key == ext)
                    .map(|(_, static_ext)| *static_ext)
                    .unwrap_or("unknown") // Default to "unknown" if not found
            })
            .collect();

        Self {
            name,
            command,
            args,
            extensions,
        }
    }
}

#[async_trait::async_trait]
impl Zenith for ExternalZenith {
    fn name(&self) -> &str {
        &self.name
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }

    async fn format(
        &self,
        content: &[u8],
        _path: &std::path::Path,
        _config: &ZenithConfig,
    ) -> Result<Vec<u8>> {
        debug!(
            "Executing plugin '{}' with command: {} and args: {:?}",
            self.name, self.command, self.args
        );

        let mut cmd = Command::new(&self.command);

        // Add the configured arguments
        for arg in &self.args {
            cmd.arg(arg);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            error!("Failed to spawn plugin '{}': {}", self.name, e);
            ZenithError::PluginError {
                name: self.name.clone(),
                error: e.to_string(),
            }
        })?;

        // Write content to stdin
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content).await.map_err(|e| {
                error!("Failed to write to plugin '{}' stdin: {}", self.name, e);
                ZenithError::PluginError {
                    name: self.name.clone(),
                    error: e.to_string(),
                }
            })?;
            // Drop stdin to signal EOF
            drop(stdin);
        }

        let output = child.wait_with_output().await.map_err(|e| {
            error!("Failed to wait for plugin '{}': {}", self.name, e);
            ZenithError::PluginError {
                name: self.name.clone(),
                error: e.to_string(),
            }
        })?;

        if output.status.success() {
            debug!(
                "Plugin '{}' executed successfully, output size: {} bytes",
                self.name,
                output.stdout.len()
            );
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(
                "Plugin '{}' failed with exit code: {:?}, stderr: {}",
                self.name,
                output.status.code(),
                stderr
            );
            Err(ZenithError::PluginError {
                name: self.name.clone(),
                error: stderr.to_string(),
            })
        }
    }
}
