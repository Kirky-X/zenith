use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenithConfig {
    pub custom_config_path: Option<PathBuf>,
    pub use_default_rules: bool,
    pub zenith_specific: serde_json::Value,
}

impl Default for ZenithConfig {
    fn default() -> Self {
        Self {
            custom_config_path: None,
            use_default_rules: true,
            zenith_specific: serde_json::Value::Null,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FormatResult {
    pub file_path: PathBuf,
    pub success: bool,
    pub changed: bool,
    pub original_size: u64,
    pub formatted_size: u64,
    pub duration_ms: u64,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    pub total_files: usize,
    pub p95_duration_ms: f64,
    pub p99_duration_ms: f64,
    pub avg_duration_ms: f64,
    pub min_duration_ms: u64,
    pub max_duration_ms: u64,
    pub std_deviation_ms: f64,
}
