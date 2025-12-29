// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! 配置类型定义模块。
//! 包含 Zenith 应用的所有配置结构体及其默认值实现。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Zenith 应用的主配置结构体。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// 全局配置项。
    #[serde(default)]
    pub global: GlobalConfig,
    /// 各个格式化工具 (Zenith) 的特定设置。
    #[serde(default)]
    pub zeniths: HashMap<String, ZenithSettings>,
    /// 备份相关配置。
    #[serde(default)]
    pub backup: BackupConfig,
    /// 并发与性能相关配置。
    #[serde(default)]
    pub concurrency: ConcurrencyConfig,
    /// 资源限制相关配置。
    #[serde(default)]
    pub limits: LimitsConfig,
    /// MCP 服务相关配置。
    #[serde(default)]
    pub mcp: McpConfig,
    /// 安全相关配置。
    #[serde(default)]
    pub security: SecurityConfig,
}

/// 全局通用配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// 是否启用自动备份。
    #[serde(default = "default_true")]
    pub backup_enabled: bool,
    /// 日志级别。
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// 是否递归处理目录。
    #[serde(default = "default_true")]
    pub recursive: bool,
    /// 是否启用缓存优化。
    #[serde(default = "default_true")]
    pub cache_enabled: bool,
    /// 配置文件和插件的存放目录。
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

/// 单个格式化工具的设置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenithSettings {
    /// 是否启用该格式化工具。
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 该工具的自定义配置文件路径。
    pub config_path: Option<String>,
    /// 是否使用默认规则。
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

/// 备份功能配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// 备份文件存放目录。
    #[serde(default = "default_backup_dir")]
    pub dir: String,
    /// 备份保留天数。
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

/// 并发执行配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcurrencyConfig {
    /// 并行工作的线程数。
    #[serde(default = "default_workers")]
    pub workers: usize,
    /// 批量处理的文件数量。
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

/// 资源与文件限制配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsConfig {
    /// 允许处理的最大文件大小 (MB)。
    #[serde(default = "default_max_file_size_mb")]
    pub max_file_size_mb: u64,
    /// 允许使用的最大内存 (MB)。
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

/// MCP (Model Context Protocol) 服务配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// 是否启用 MCP 服务。
    #[serde(default = "default_mcp_enabled")]
    pub enabled: bool,
    /// 服务监听的主机名或 IP。
    #[serde(default = "default_mcp_host")]
    pub host: String,
    /// 服务监听的端口号。
    #[serde(default = "default_mcp_port")]
    pub port: u16,
    /// 是否启用身份验证。
    #[serde(default = "default_mcp_auth_enabled")]
    pub auth_enabled: bool,
    /// 主 API 密钥。
    #[serde(default)]
    pub api_key: Option<String>,
    /// 允许的跨域来源 (CORS)。
    #[serde(default = "default_mcp_allowed_origins")]
    pub allowed_origins: Vec<String>,
    /// 用户列表及其角色。
    #[serde(default)]
    pub users: Vec<McpUser>,
}

/// 插件安全配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 允许的插件命令白名单。
    #[serde(default)]
    pub allowed_plugin_commands: Vec<String>,
    /// 是否允许插件使用绝对路径。
    #[serde(default = "default_allow_absolute_paths")]
    pub allow_absolute_paths: bool,
    /// 是否允许插件使用相对路径。
    #[serde(default = "default_allow_relative_paths")]
    pub allow_relative_paths: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            allowed_plugin_commands: Vec::new(),
            allow_absolute_paths: default_allow_absolute_paths(),
            allow_relative_paths: default_allow_relative_paths(),
        }
    }
}

/// MCP 用户信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpUser {
    /// 用户 API 密钥。
    pub api_key: String,
    /// 用户角色（例如 admin, user）。
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

/// 传递给各个格式化工具的配置。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZenithConfig {
    /// 自定义配置文件路径。
    pub custom_config_path: Option<PathBuf>,
    /// 是否使用默认规则。
    pub use_default_rules: bool,
    /// 特定于某个格式化工具的 JSON 配置。
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

/// 格式化操作的结果。
#[derive(Debug, Clone, Serialize, Default)]
pub struct FormatResult {
    /// 文件路径。
    pub file_path: PathBuf,
    /// 是否执行成功。
    pub success: bool,
    /// 内容是否发生了改变。
    pub changed: bool,
    /// 原始文件大小 (字节)。
    pub original_size: u64,
    /// 格式化后的文件大小 (字节)。
    pub formatted_size: u64,
    /// 执行耗时 (毫秒)。
    pub duration_ms: u64,
    /// 错误信息（如果失败）。
    pub error: Option<String>,
}

/// 性能指标统计。
#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    /// 处理的文件总数。
    pub total_files: usize,
    /// 95% 分位数耗时 (ms)。
    pub p95_duration_ms: f64,
    /// 99% 分位数耗时 (ms)。
    pub p99_duration_ms: f64,
    /// 平均耗时 (ms)。
    pub avg_duration_ms: f64,
    /// 最小耗时 (ms)。
    pub min_duration_ms: u64,
    /// 最大耗时 (ms)。
    pub max_duration_ms: u64,
    /// 标准差 (ms)。
    pub std_deviation_ms: f64,
}

// 默认值助手函数
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
    100 // 根据 PRD，默认限制为 100MB
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

fn default_allow_absolute_paths() -> bool {
    true
}

fn default_allow_relative_paths() -> bool {
    false
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
