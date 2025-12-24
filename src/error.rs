use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZenithError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Zenith '{name}' failed: {reason}")]
    ZenithFailed { name: String, reason: String },

    #[error("Backup failed: {0}")]
    BackupFailed(String),

    #[error("Backup not found: {0}")]
    BackupNotFound(String),

    #[error("Recovery failed: {0}")]
    RecoverFailed(String),

    #[error("Unsupported file extension: {0}")]
    UnsupportedExtension(String),

    #[error("External tool not found: {tool}")]
    ToolNotFound { tool: String },

    #[error("File too large: {size} bytes (limit: {limit} bytes)")]
    FileTooLarge { size: u64, limit: u64 },

    #[error("Path traversal attempt detected: {0}")]
    PathTraversal(PathBuf),

    #[error("Backup is disabled")]
    BackupDisabled,

    #[error("No backups available")]
    NoBackupsAvailable,

    #[error("Permission denied for file: {path} - {reason}")]
    PermissionDenied { path: PathBuf, reason: String },

    #[error("Plugin validation error for '{name}': {error}")]
    PluginValidationError { name: String, error: String },

    #[error("Plugin '{name}' is disabled")]
    PluginDisabled { name: String },

    #[error("Plugin error for '{name}': {error}")]
    PluginError { name: String, error: String },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML deserialization error: {0}")]
    TomlDeserialization(#[from] toml::de::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error),
}

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
}
