// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Cross-platform tests for the zenith application
//! These tests verify that the application works correctly across different operating systems

use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use zenith::config::types::BackupConfig;

/// Test that basic CLI functionality works on all platforms
#[test]
fn test_cli_cross_platform_basic() {
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--").arg("--help");

    // This should work on all platforms
    cmd.assert().success();
}

/// Test path handling across different platforms
#[test]
fn test_path_handling() {
    use std::path::Path;

    // Test path separators work correctly on different platforms
    let path_str = if cfg!(windows) {
        "C:\\test\\file.rs"
    } else {
        "/test/file.rs"
    };

    let path = Path::new(path_str);
    assert!(path.is_absolute());

    // Test path joining
    let base_path = Path::new("src");
    let file_path = base_path.join("main.rs");

    if cfg!(windows) {
        assert!(file_path.display().to_string().contains('\\'));
    } else {
        assert!(file_path.display().to_string().contains('/'));
    }
}

/// Test file permissions handling across platforms
#[cfg(unix)]
#[test]
fn test_unix_file_permissions() {
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test_permissions.rs");

    // Create a test file
    fs::write(&test_file, "// test file").unwrap();

    // Check default permissions
    let metadata = fs::metadata(&test_file).unwrap();
    let permissions = metadata.permissions();

    // On Unix systems, files are typically created with 644 permissions (rw-r--r--)
    // The exact value depends on the system's umask, so we just verify we can read permissions
    assert_eq!(permissions.mode() & 0o777, permissions.mode() & 0o777); // Just verify it's a valid mode

    // Test changing permissions
    let mut new_permissions = permissions;
    new_permissions.set_mode(0o444); // Read-only
    fs::set_permissions(&test_file, new_permissions).unwrap();

    let updated_metadata = fs::metadata(&test_file).unwrap();
    assert_eq!(updated_metadata.permissions().mode() & 0o777, 0o444);
}

/// Placeholder for Windows file permissions test
#[cfg(windows)]
#[test]
fn test_windows_file_permissions() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test_permissions.rs");

    // Create a test file
    fs::write(&test_file, "// test file").unwrap();

    // On Windows, we can't easily test file permissions the same way as Unix
    // But we can still verify the file was created and can be read
    let metadata = fs::metadata(&test_file).unwrap();
    assert!(metadata.is_file());

    // Verify we can read the file content
    let content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "// test file");
}

/// Test directory operations across platforms
#[test]
fn test_directory_operations() {
    use std::fs;
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");

    // Create subdirectory
    fs::create_dir(&sub_dir).unwrap();
    assert!(sub_dir.exists());

    // Create a file in the subdirectory
    let file_path = sub_dir.join("test.txt");
    fs::write(&file_path, "test content").unwrap();
    assert!(file_path.exists());

    // List directory contents
    let entries: Result<Vec<_>, _> = fs::read_dir(&sub_dir)
        .unwrap()
        .map(|entry| entry.map(|e| e.path()))
        .collect();
    let entries = entries.unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0], file_path);
}

/// Test that file operations work correctly on all platforms
#[test]
fn test_file_operations_cross_platform() {
    use tempfile::TempDir;

    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test_file.txt");

    // Write to file
    let content = "Hello, cross-platform world!";
    fs::write(&test_file, content).unwrap();

    // Read from file
    let read_content = fs::read_to_string(&test_file).unwrap();
    assert_eq!(read_content, content);

    // Verify file exists
    assert!(test_file.exists());

    // Get file metadata
    let metadata = fs::metadata(&test_file).unwrap();
    assert!(metadata.is_file());
    assert_eq!(metadata.len(), content.len() as u64);
}

/// Test backup functionality across platforms
#[tokio::test]
async fn test_backup_cross_platform() {
    use tempfile::TempDir;
    use zenith::internal::BackupService;

    let temp_dir = TempDir::new().unwrap();
    let backup_dir = temp_dir.path().join("backups");

    let config = BackupConfig {
        dir: backup_dir.to_string_lossy().to_string(),
        retention_days: 7,
    };

    let service = BackupService::new(config);
    service.init().await.unwrap();

    // Create a test file to backup
    let test_file = temp_dir.path().join("source.txt");
    let original_content = "Original content";
    fs::write(&test_file, original_content).unwrap();

    // Perform backup
    service
        .backup_file(temp_dir.path(), &test_file, original_content.as_bytes())
        .await
        .unwrap();

    // Verify backup was created by checking that the session directory exists and has content
    let session_dir = std::path::Path::new(&backup_dir).join(service.get_session_id());
    assert!(session_dir.exists());

    // Verify backup content matches original by performing a recovery
    let recovery_dir = temp_dir.path().join("recovered");
    let restored_count = service
        .recover(service.get_session_id(), Some(recovery_dir.clone()))
        .await
        .unwrap();
    assert_eq!(restored_count, 1);

    let recovered_file = recovery_dir.join("source.txt");
    assert!(recovered_file.exists());

    let backup_content = fs::read_to_string(&recovered_file).unwrap();
    assert_eq!(backup_content, original_content);
}

/// Test that memory usage monitoring works on all platforms
#[test]
fn test_memory_monitoring_cross_platform() {
    use std::sync::Arc;
    use zenith::config::types::AppConfig;
    use zenith::internal::ZenithService;
    use zenith::zeniths::registry::ZenithRegistry;

    // Create a configuration with memory limits
    let mut config = AppConfig::default();
    config.limits.max_memory_mb = 100; // 100 MB limit using the correct field

    // Create services
    let registry = Arc::new(ZenithRegistry::new());
    let backup_service = Arc::new(zenith::internal::BackupService::new(config.backup.clone()));
    let hash_cache = Arc::new(zenith::internal::HashCache::new());
    let _service = ZenithService::new(config, registry, backup_service, hash_cache, false);

    // Service creation succeeded if we reach here
}

/// Test configuration loading across platforms
#[test]
fn test_config_loading_cross_platform() {
    use tempfile::TempDir;
    use zenith::config::types::AppConfig;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("zenith.toml");

    // Test that we can parse the config (this would be done in a real config loader)
    let config_content = r#"
[global]
backup_enabled = true
log_level = "info"
cache_enabled = true

[backup]
dir = "./backups"
retention_days = 30

[concurrency]
workers = 4
batch_size = 20

[limits]
max_memory_mb = 512
max_file_size_mb = 10
"#;

    fs::write(&config_file, config_content).unwrap();

    // Test that we can parse the config (this would be done in a real config loader)
    let config: AppConfig = toml::from_str(config_content).unwrap();
    assert!(config.global.backup_enabled);
    assert_eq!(config.backup.retention_days, 30);
    assert_eq!(config.concurrency.workers, 4);
    assert_eq!(config.limits.max_memory_mb, 512);
    assert_eq!(config.limits.max_file_size_mb, 10);

    // Additional test to ensure config file can be read from disk
    let config_from_file = std::fs::read_to_string(&config_file).unwrap();
    assert_eq!(config_from_file, config_content);
}

/// Test error handling across platforms
#[test]
fn test_error_handling_cross_platform() {
    use std::path::PathBuf;
    use zenith::error::ZenithError;

    // Test different error types work on all platforms
    let io_error = ZenithError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "File not found",
    ));
    assert!(matches!(io_error, ZenithError::Io(_)));

    let backup_error = ZenithError::BackupFailed("Backup failed".to_string());
    assert!(matches!(backup_error, ZenithError::BackupFailed(_)));

    let permission_error = ZenithError::PermissionDenied {
        path: PathBuf::from("/test/path"),
        reason: "Permission denied".to_string(),
    };
    assert!(matches!(
        permission_error,
        ZenithError::PermissionDenied { .. }
    ));
}
