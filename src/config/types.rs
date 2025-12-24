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
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            backup_enabled: true,
            log_level: "info".into(),
            recursive: true,
            cache_enabled: true,
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
    100  // 100MB default limit as per PRD
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
