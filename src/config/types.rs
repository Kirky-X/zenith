use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            backup_enabled: true,
            log_level: "info".into(),
            recursive: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZenithSettings {
    #[serde(default = "default_true")]
    pub enabled: bool,
    pub config_path: Option<String>,
    #[serde(default = "default_true")]
    pub use_default: bool,
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
}

impl Default for LimitsConfig {
    fn default() -> Self {
        Self {
            max_file_size_mb: default_max_file_size_mb(),
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
