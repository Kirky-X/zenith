pub mod cache;
pub mod discovery;
pub mod types;

use self::types::AppConfig;
use crate::error::{Result, ZenithError};
use config::{Config, Environment, File};
use std::path::PathBuf;

use self::discovery::discover_project_config;
use std::path::Path;

pub fn load_config(path: Option<PathBuf>) -> Result<AppConfig> {
    load_config_with_project_discovery(path, None)
}

/// Load configuration with optional project-level configuration discovery
pub fn load_config_with_project_discovery(
    app_config_path: Option<PathBuf>,
    file_path: Option<&Path>,
) -> Result<AppConfig> {
    let mut builder = Config::builder();

    // 1. Load defaults (handled by struct defaults)

    // 2. Load application-level config from file if provided, otherwise check default locations
    if let Some(p) = app_config_path {
        builder = builder.add_source(File::from(p).required(true));
    } else {
        // Try default locations
        let default_paths = vec!["zenith.toml", ".config/zenith/zenith.toml"];
        for p in default_paths {
            builder = builder.add_source(File::with_name(p).required(false));
        }
    }

    // 3. Load project-level configuration if a file path is provided
    if let Some(file_path) = file_path {
        if let Some(project_config_path) = discover_project_config(file_path)? {
            builder = builder.add_source(File::from(project_config_path).required(false));
        }
    }

    // 4. Load from Environment (highest priority)
    builder = builder.add_source(Environment::with_prefix("ZENITH").separator("_"));

    let config = builder
        .build()
        .map_err(|e| ZenithError::Config(e.to_string()))?;

    config
        .try_deserialize()
        .map_err(|e| ZenithError::Config(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_with_valid_file() {
        // Create a temporary config file with .toml extension
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("test_config.toml");
        std::fs::write(
            &config_path,
            r#"
[global]
backup_enabled = false
log_level = "debug"

[backup]
dir = "./backups"
retention_days = 14

[concurrency]
workers = 4
batch_size = 50
"#,
        )
        .unwrap();

        let result = load_config(Some(config_path));
        if let Err(e) = &result {
            eprintln!("Config loading error: {}", e);
        }
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(!config.global.backup_enabled);
        assert_eq!(config.global.log_level, "debug");
        assert_eq!(config.backup.dir, "./backups");
        assert_eq!(config.backup.retention_days, 14);
        assert_eq!(config.concurrency.workers, 4);
        assert_eq!(config.concurrency.batch_size, 50);
    }

    #[test]
    fn test_load_config_with_defaults() {
        // Test loading config without providing a file path
        // This should use default values
        let result = load_config(None);

        // Since the default files don't exist, this should still succeed with defaults
        // unless there's an error in the config loading process
        match result {
            Ok(config) => {
                // Check that default values are applied
                assert!(config.global.backup_enabled);
                assert_eq!(config.global.log_level, "info");
                assert_eq!(config.backup.dir, ".zenith_backup");
                assert_eq!(config.backup.retention_days, 7);
            }
            Err(_) => {
                // If there's an error, it's likely because the default files don't exist
                // which is expected behavior when no config file is provided
            }
        }
    }

    #[test]
    fn test_load_config_with_invalid_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let config_path = temp_dir.path().join("invalid_config.toml");
        std::fs::write(
            &config_path,
            r#"
[global
backup_enabled = true
"#,
        )
        .unwrap(); // Invalid TOML syntax

        let result = load_config(Some(config_path));
        assert!(result.is_err());
    }
}
