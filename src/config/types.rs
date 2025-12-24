use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub zeniths: HashMap<String, ZenithSettings>,
    #[serde(default)]
    pub backup: BackupConfig,
    #[serde(default)]
    pub concurrency: ConcurrencyConfig,
    #[serde(default)]
    pub limits: LimitsConfig,
    #[serde(default)]
    pub mcp: McpConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_true")]
    pub backup_enabled: bool,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_true")]
    pub recursive: bool,
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
    #[serde(default = "default_config_dir")]
    pub config_dir: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            backup_enabled: true,
            log_level: "info".into(),
            recursive: true,
            cache_enabled: true,
            config_dir: default_config_dir(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenithSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub config_path: Option<String>,
    #[serde(default = "default_true")]
    pub use_default: bool,
}

impl Default for ZenithSettings {
    fn default() -> Self {
        Self {
            enabled: default_true(),
            config_path: None,
            use_default: default_true(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    #[serde(default = "default_backup_dir")]
    pub dir: String,
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            dir: default_backup_dir(),
            retention_days: default_retention_days(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    #[serde(default = "default_workers")]
    pub workers: usize,
    #[serde(default = "default_batch_size")]
    pub batch_size: usize,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            workers: default_workers(),
            batch_size: default_batch_size(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,
    #[serde(default = "default_max_memory_mb")]
    pub max_memory_mb: u64,
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: default_max_file_size_mb(),
            max_memory_mb: default_max_memory_mb(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    #[serde(default = "default_mcp_enabled")]
    pub enabled: bool,
    #[serde(default = "default_mcp_host")]
    pub host: String,
    #[serde(default = "default_mcp_port")]
    pub port: u16,
    #[serde(default = "default_mcp_auth_enabled")]
    pub auth_enabled: bool,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default = "default_mcp_allowed_origins")]
    pub allowed_origins: Vec<String>,
    #[serde(default)]
    pub users: Vec<McpUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpUser {
    pub api_key: String,
    #[serde(default = "default_mcp_user_role")]
    pub role: String,
}

fn default_mcp_user_role() -> String {
    "user".into()
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            enabled: default_mcp_enabled(),
            host: default_mcp_host(),
            port: default_mcp_port(),
            auth_enabled: default_mcp_auth_enabled(),
            api_key: None,
            allowed_origins: default_mcp_allowed_origins(),
            users: vec![],
        }
    }
}

// Default value helpers
fn default_true() -> bool {
    true
}
fn default_log_level() -> String {
    "info".into()
}
fn default_backup_dir() -> String {
    ".zenith_backup".into()
}
fn default_retention_days() -> u32 {
    7
}
fn default_workers() -> usize {
    num_cpus::get()
}
fn default_batch_size() -> usize {
    100
}
fn default_max_file_size_mb() -> u64 {
    10
}

fn default_max_memory_mb() -> u64 {
    100 // 100MB default limit as per PRD
}

fn default_config_dir() -> String {
    ".zenith".into()
}

fn default_mcp_enabled() -> bool {
    false
}

fn default_mcp_host() -> String {
    "127.0.0.1".into()
}

fn default_mcp_port() -> u16 {
    8080
}

fn default_mcp_auth_enabled() -> bool {
    true
}

fn default_mcp_allowed_origins() -> Vec<String> {
    vec!["*".to_string()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_config_defaults() {
        let config = GlobalConfig::default();
        assert!(config.backup_enabled);
        assert_eq!(config.log_level, "info");
        assert!(config.recursive);
        assert!(config.cache_enabled);
    }

    #[test]
    fn test_backup_config_defaults() {
        let config = BackupConfig::default();
        assert_eq!(config.dir, ".zenith_backup");
        assert_eq!(config.retention_days, 7);
    }

    #[test]
    fn test_concurrency_config_defaults() {
        let config = ConcurrencyConfig::default();
        assert_eq!(config.workers, num_cpus::get());
        assert_eq!(config.batch_size, 100);
    }

    #[test]
    fn test_limits_config_defaults() {
        let config = LimitsConfig::default();
        assert_eq!(config.max_file_size_mb, 10);
        assert_eq!(config.max_memory_mb, 100);
    }

    #[test]
    fn test_zenith_settings_defaults() {
        let config = ZenithSettings::default();
        assert!(config.enabled);
        assert!(config.use_default);
        assert_eq!(config.config_path, None);
    }
}
