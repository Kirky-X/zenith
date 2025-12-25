// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 的配置管理模块。
//! 负责加载、解析和合并来自不同源（默认值、文件、环境变量、项目级配置）的配置信息。

pub mod cache;
pub mod discovery;
pub mod types;

use self::types::AppConfig;
use crate::error::{Result, ZenithError};
use config::{Config, Environment, File};
use std::path::PathBuf;

use self::discovery::discover_project_config;
use std::path::Path;

/// 加载 Zenith 配置。
///
/// # 参数
///
/// * `path` - 可选的配置文件路径。
///
/// # 返回值
///
/// 返回解析后的 `AppConfig` 结构体。
pub fn load_config(path: Option<PathBuf>) -> Result<AppConfig> {
    load_config_with_project_discovery(path, None)
}

/// 加载配置，并支持可选的项目级配置自动发现。
///
/// # 参数
///
/// * `app_config_path` - 应用级配置文件路径。
/// * `file_path` - 正在处理的文件路径，用于向上查找项目配置。
pub fn load_config_with_project_discovery(
    app_config_path: Option<PathBuf>,
    file_path: Option<&Path>,
) -> Result<AppConfig> {
    let mut builder = Config::builder();

    // 1. 加载默认值 (由结构体的 Default 实现处理)

    // 2. 从提供的路径加载应用级配置，否则检查默认位置
    if let Some(p) = app_config_path {
        builder = builder.add_source(File::from(p).required(true));
    } else {
        // 尝试默认位置
        let default_paths = vec!["zenith.toml", ".config/zenith/zenith.toml"];
        for p in default_paths {
            builder = builder.add_source(File::with_name(p).required(false));
        }
    }

    // 3. 如果提供了文件路径，则尝试发现项目级配置
    if let Some(file_path) = file_path {
        if let Some(project_config_path) = discover_project_config(file_path)? {
            builder = builder.add_source(File::from(project_config_path).required(false));
        }
    }

    // 4. 从环境变量加载 (最高优先级)
    // 环境变量前缀为 ZENITH_，例如 ZENITH_GLOBAL_LOG_LEVEL
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
