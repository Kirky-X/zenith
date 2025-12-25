// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 库的错误处理模块。
//! 定义了整个项目中使用的 `ZenithError` 枚举和 `Result` 类型。

use std::path::PathBuf;
use thiserror::Error;

/// Zenith 项目中所有可能的错误类型。
#[derive(Error, Debug)]
pub enum ZenithError {
    /// 配置相关错误。
    #[error("Configuration error: {0}")]
    Config(String),

    /// 文件未找到。
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    /// I/O 错误。
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Zenith 格式化工具执行失败。
    #[error("Zenith '{name}' failed: {reason}")]
    ZenithFailed { name: String, reason: String },

    /// 备份失败。
    #[error("Backup failed: {0}")]
    BackupFailed(String),

    /// 备份文件未找到。
    #[error("Backup not found: {0}")]
    BackupNotFound(String),

    /// 恢复失败。
    #[error("Recovery failed: {0}")]
    RecoverFailed(String),

    /// 不支持的文件扩展名。
    #[error("Unsupported file extension: {0}")]
    UnsupportedExtension(String),

    /// 外部工具未找到。
    #[error("External tool not found: {tool}")]
    ToolNotFound { tool: String },

    /// 文件过大，超过限制。
    #[error("File too large: {size} bytes (limit: {limit} bytes)")]
    FileTooLarge { size: u64, limit: u64 },

    /// 检测到路径穿越尝试（安全检查）。
    #[error("Path traversal attempt detected: {0}")]
    PathTraversal(PathBuf),

    /// 备份功能已禁用。
    #[error("Backup is disabled")]
    BackupDisabled,

    /// 无可用备份。
    #[error("No backups available")]
    NoBackupsAvailable,

    /// 文件权限不足。
    #[error("Permission denied for file: {path} - {reason}")]
    PermissionDenied { path: PathBuf, reason: String },

    /// 插件验证失败。
    #[error("Plugin validation error for '{name}': {error}")]
    PluginValidationError { name: String, error: String },

    /// 插件已禁用。
    #[error("Plugin '{name}' is disabled")]
    PluginDisabled { name: String },

    /// 插件运行时错误。
    #[error("Plugin error for '{name}': {error}")]
    PluginError { name: String, error: String },

    /// JSON 序列化/反序列化错误。
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// TOML 反序列化错误。
    #[error("TOML deserialization error: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    /// UTF-8 转换错误。
    #[error("UTF-8 conversion error: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error),

    /// 版本不兼容。
    #[error("Version incompatible: {tool} requires {required}, but found {actual}")]
    VersionIncompatible {
        tool: String,
        required: String,
        actual: String,
    },
}

/// Zenith 库通用的 `Result` 类型。
pub type Result<T> = std::result::Result<T, ZenithError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_config_error() {
        let error = ZenithError::Config("Invalid configuration".to_string());
        assert!(format!("{}", error).contains("Configuration error"));
    }

    #[test]
    fn test_file_not_found_error() {
        let path = PathBuf::from("/nonexistent/file");
        let error = ZenithError::FileNotFound { path: path.clone() };
        assert!(format!("{}", error).contains("File not found"));
    }

    #[test]
    fn test_io_error() {
        let io_error = io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied");
        let error = ZenithError::Io(io_error);
        assert!(format!("{}", error).contains("IO error"));
    }

    #[test]
    fn test_zenith_failed_error() {
        let error = ZenithError::ZenithFailed {
            name: "RustFormatter".to_string(),
            reason: "Tool not found".to_string(),
        };
        assert!(format!("{}", error).contains("Zenith 'RustFormatter' failed"));
    }

    #[test]
    fn test_backup_failed_error() {
        let error = ZenithError::BackupFailed("Backup directory not writable".to_string());
        assert!(format!("{}", error).contains("Backup failed"));
    }

    #[test]
    fn test_unsupported_extension_error() {
        let error = ZenithError::UnsupportedExtension("xyz".to_string());
        assert!(format!("{}", error).contains("Unsupported file extension"));
    }

    #[test]
    fn test_tool_not_found_error() {
        let error = ZenithError::ToolNotFound {
            tool: "rustfmt".to_string(),
        };
        assert!(format!("{}", error).contains("External tool not found"));
    }

    #[test]
    fn test_file_too_large_error() {
        let error = ZenithError::FileTooLarge {
            size: 1000,
            limit: 500,
        };
        assert!(format!("{}", error).contains("File too large"));
    }

    #[test]
    fn test_path_traversal_error() {
        let path = PathBuf::from("../etc/passwd");
        let error = ZenithError::PathTraversal(path);
        assert!(format!("{}", error).contains("Path traversal attempt detected"));
    }

    #[test]
    fn test_version_incompatible_error() {
        let error = ZenithError::VersionIncompatible {
            tool: "rustfmt".to_string(),
            required: ">= 2.0.0".to_string(),
            actual: "1.5.0".to_string(),
        };
        assert!(format!("{}", error).contains("Version incompatible"));
        assert!(format!("{}", error).contains("rustfmt"));
        assert!(format!("{}", error).contains(">= 2.0.0"));
        assert!(format!("{}", error).contains("1.5.0"));
    }
}
