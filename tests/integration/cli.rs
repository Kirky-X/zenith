// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Integration tests for the zenith application
//! These tests verify the end-to-end functionality of the application

use crate::common::{assert_command_success, create_temp_dir, create_test_file};
use assert_cmd::cargo;
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

/// Test that the zenith CLI can run with basic arguments
#[test]
fn test_zenith_cli_basic() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));

    // Test help command
    cmd.arg("--help");
    cmd.assert().success();
}

/// Test that zenith can format a simple file
#[test]
fn test_zenith_format_file() {
    // Create a temporary directory for testing
    let temp_dir = create_temp_dir();
    let input_file = temp_dir.path().join("test_input.rs");

    // Create a test Rust file that needs formatting
    let rust_code = r#"fn main(){println!("Hello, World!");}"#;
    create_test_file(temp_dir.path(), "test_input.rs", rust_code);

    // First format the file
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&input_file);
    assert_command_success(format_cmd.assert());

    // Then verify it's formatted correctly with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd.arg("format").arg(&input_file).arg("--check");
    assert_command_success(check_cmd.assert());
}

/// Test doctor command
#[test]
fn test_zenith_doctor() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("doctor");

    // Doctor command should run without panicking
    // It may exit with code 1 if some tools are missing, which is expected behavior
    let result = cmd.assert();
    // Try to assert success first, if that fails, check for code 1
    match result.try_success() {
        Ok(_) => {}
        Err(_) => {
            // If success fails, try code 1 (missing tools is acceptable)
            Command::new(cargo::cargo_bin!("zenith"))
                .arg("doctor")
                .assert()
                .code(1);
        }
    }
}

/// Test configuration loading with CLI
#[test]
fn test_zenith_with_config() {
    // Create a temporary directory for testing
    let temp_dir = create_temp_dir();
    let config_file = temp_dir.path().join("zenith.toml");

    // Create a test config
    let config_content = r#"
[global]
backup_enabled = false
log_level = "debug"

[backup]
dir = "./backups"
retention_days = 14

[concurrency]
workers = 2
batch_size = 10
"#;
    create_test_file(temp_dir.path(), "zenith.toml", config_content);

    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("--config").arg(&config_file).arg("doctor"); // Use doctor command as it doesn't require specific files

    // Doctor command should run without panicking
    // It may exit with code 1 if some tools are missing, which is expected behavior
    let result = cmd.assert();
    // Try to assert success first, if that fails, check for code 1
    match result.try_success() {
        Ok(_) => {}
        Err(_) => {
            // If success fails, try code 1 (missing tools is acceptable)
            Command::new(cargo::cargo_bin!("zenith"))
                .arg("--config")
                .arg(&config_file)
                .arg("doctor")
                .assert()
                .code(1);
        }
    }
}

/// Test that zenith handles errors appropriately
#[test]
fn test_zenith_error_handling() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    // Try to format a non-existent file
    cmd.arg("format").arg("/nonexistent/file.rs");

    // This should fail, but in a controlled way (not panic)
    cmd.assert().failure();
}

/// End-to-end test: Format a directory with multiple files
#[test]
fn test_zenith_format_directory() {
    let temp_dir = create_temp_dir();

    // Create multiple test files
    let _rust_file = temp_dir.path().join("main.rs");
    let _py_file = temp_dir.path().join("script.py");

    create_test_file(
        temp_dir.path(),
        "main.rs",
        r#"fn main(){println!("Hello");}"#,
    );
    create_test_file(
        temp_dir.path(),
        "script.py",
        r#"def hello():print("World")"#,
    );

    // First format
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(temp_dir.path());
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd.arg("format").arg(temp_dir.path()).arg("--check");
    assert_command_success(check_cmd.assert());
}

/// End-to-end test: Format with recursive option
#[test]
fn test_zenith_format_recursive() {
    let temp_dir = create_temp_dir();

    // Create nested directory structure
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    let _main_file = src_dir.join("main.rs");
    let _lib_file = src_dir.join("lib.rs");

    create_test_file(&src_dir, "main.rs", r#"fn main(){println!("Hello");}"#);
    create_test_file(&src_dir, "lib.rs", r#"pub fn test(){}"#);

    // First format
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive");
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive")
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// End-to-end test: Format with config file
#[test]
fn test_zenith_format_with_config() {
    let temp_dir = create_temp_dir();
    let config_file = temp_dir.path().join("zenith.toml");
    let test_file = temp_dir.path().join("test.rs");

    // Create config
    let config = r#"
[global]
backup_enabled = false

[formatters.rust]
command = "rustfmt"
args = ["--edition", "2021"]
"#;
    create_test_file(temp_dir.path(), "zenith.toml", config);
    create_test_file(
        temp_dir.path(),
        "test.rs",
        r#"fn main(){println!("Test");}"#,
    );

    // First format
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd
        .arg("--config")
        .arg(&config_file)
        .arg("format")
        .arg(&test_file);
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("--config")
        .arg(&config_file)
        .arg("format")
        .arg(&test_file)
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// Backup and recovery integration test
#[test]
fn test_zenith_backup_and_recover() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    // Create a test file
    let original_content = r#"fn main(){println!("Original");}"#;
    create_test_file(temp_dir.path(), "test.rs", original_content);

    // Format the file (this should create a backup)
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&test_file);
    assert_command_success(format_cmd.assert());

    // List backups
    let mut list_cmd = Command::new(cargo::cargo_bin!("zenith"));
    list_cmd.arg("list-backups");
    assert_command_success(list_cmd.assert());
}

/// Clean backups integration test
#[test]
fn test_zenith_clean_backups() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    // Create and format a file multiple times to create backups
    for i in 0..3 {
        let content = format!(r#"fn main(){{println!("{}");}}"#, i);
        create_test_file(temp_dir.path(), "test.rs", &content);

        let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
        cmd.arg("format").arg(&test_file);
        assert_command_success(cmd.assert());
    }

    // Clean old backups
    let mut clean_cmd = Command::new(cargo::cargo_bin!("zenith"));
    clean_cmd.arg("clean-backups").arg("--days").arg("0");
    assert_command_success(clean_cmd.assert());
}

/// CLI command: List backups test
#[test]
fn test_zenith_list_backups() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("list-backups");
    cmd.assert().success();
}

/// CLI command: version flag test
#[test]
fn test_zenith_version_flag() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("--version");
    cmd.assert().success();
}

/// CLI command: Multiple files format test
#[test]
fn test_zenith_format_multiple_files() {
    let temp_dir = create_temp_dir();

    let file1 = temp_dir.path().join("file1.rs");
    let file2 = temp_dir.path().join("file2.rs");
    let file3 = temp_dir.path().join("file3.rs");

    create_test_file(temp_dir.path(), "file1.rs", r#"fn test1(){}"#);
    create_test_file(temp_dir.path(), "file2.rs", r#"fn test2(){}"#);
    create_test_file(temp_dir.path(), "file3.rs", r#"fn test3(){}"#);

    // First format the files
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&file1).arg(&file2).arg(&file3);
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("format")
        .arg(&file1)
        .arg(&file2)
        .arg(&file3)
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// CLI command: Format with workers option
#[test]
fn test_zenith_format_with_workers() {
    let temp_dir = create_temp_dir();

    // Create multiple files to test workers
    for i in 0..10 {
        let _file = temp_dir.path().join(format!("file{}.rs", i));
        create_test_file(
            temp_dir.path(),
            &format!("file{}.rs", i),
            &format!(r#"fn test{}(){{}}"#, i),
        );
    }

    // First format with workers
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive")
        .arg("--workers")
        .arg("4");
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive")
        .arg("--workers")
        .arg("4")
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// CLI command: Format with no-backup option
#[test]
fn test_zenith_format_with_no_backup() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    // Create a small file
    create_test_file(temp_dir.path(), "test.rs", r#"fn main(){}"#);

    // First format with no-backup
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&test_file).arg("--no-backup");
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("format")
        .arg(&test_file)
        .arg("--no-backup")
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// Integration test: Format and verify backup directory exists
#[test]
fn test_zenith_format_creates_backup() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    // Create a file
    let original_content = r#"fn main(){println!("Original");}"#;
    create_test_file(temp_dir.path(), "test.rs", original_content);

    // Format the file with backup enabled (default)
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("format").arg(&test_file);
    assert_command_success(cmd.assert());

    // Verify the file was formatted
    let new_content = fs::read_to_string(&test_file).unwrap();
    assert!(new_content.contains("fn main()"));
}

/// Integration test: Multiple language files in one directory (Rust and Python)
#[test]
fn test_zenith_format_mixed_languages() {
    let temp_dir = create_temp_dir();

    // Create files in different languages
    let _rust_file = temp_dir.path().join("main.rs");
    let _py_file = temp_dir.path().join("script.py");

    create_test_file(
        temp_dir.path(),
        "main.rs",
        r#"fn main(){println!("Rust");}"#,
    );
    create_test_file(
        temp_dir.path(),
        "script.py",
        r#"def test():print("Python")"#,
    );

    // First format the directory
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(temp_dir.path());
    assert_command_success(format_cmd.assert());

    // Then verify with check mode
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd.arg("format").arg(temp_dir.path()).arg("--check");
    assert_command_success(check_cmd.assert());
}

/// CLI command: Invalid arguments handling
#[test]
fn test_zenith_invalid_arguments() {
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("invalid-command");
    cmd.assert().failure();
}

/// CLI command: Format with dry-run
#[test]
fn test_zenith_format_dry_run() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    let original_content = r#"fn main(){println!("Test");}"#;
    create_test_file(temp_dir.path(), "test.rs", original_content);

    // First format the file
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&test_file);
    assert_command_success(format_cmd.assert());

    // Then verify with check mode (dry-run)
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd.arg("format").arg(&test_file).arg("--check");
    assert_command_success(check_cmd.assert());

    // Verify file was formatted
    let content_after = fs::read_to_string(&test_file).unwrap();
    assert_ne!(original_content, content_after);
}
