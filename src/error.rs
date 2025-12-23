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
}

pub type Result<T> = std::result::Result<T, ZenithError>;
