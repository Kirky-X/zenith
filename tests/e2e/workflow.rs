// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! End-to-end tests for the zenith application
//! These tests verify complete workflows from user perspective

use crate::common::{assert_command_success, create_temp_dir, create_test_file};
use assert_cmd::cargo;
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;

/// End-to-end test: Complete formatting workflow
#[test]
fn test_complete_formatting_workflow() {
    let temp_dir = create_temp_dir();

    // Create a project structure
    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Create multiple files
    create_test_file(&src_dir, "main.rs", r#"fn main(){println!("Hello");}"#);
    create_test_file(&src_dir, "lib.rs", r#"pub fn test(){}"#);

    // Format the project
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive");
    assert_command_success(format_cmd.assert());

    // Verify formatting
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("format")
        .arg(temp_dir.path())
        .arg("--recursive")
        .arg("--check");
    assert_command_success(check_cmd.assert());
}

/// End-to-end test: Format with backup and recovery
#[test]
fn test_backup_recovery_workflow() {
    let temp_dir = create_temp_dir();
    let test_file = temp_dir.path().join("test.rs");

    // Create and format a file
    create_test_file(
        temp_dir.path(),
        "test.rs",
        r#"fn main(){println!("Original");}"#,
    );

    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd.arg("format").arg(&test_file);
    assert_command_success(format_cmd.assert());

    // List backups
    let mut list_cmd = Command::new(cargo::cargo_bin!("zenith"));
    list_cmd.arg("list-backups");
    assert_command_success(list_cmd.assert());
}

/// End-to-end test: Configuration-based formatting
#[test]
fn test_config_based_workflow() {
    let temp_dir = create_temp_dir();
    let config_file = temp_dir.path().join("zenith.toml");

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

    // Format with config
    let mut format_cmd = Command::new(cargo::cargo_bin!("zenith"));
    format_cmd
        .arg("--config")
        .arg(&config_file)
        .arg("format")
        .arg(temp_dir.path());
    assert_command_success(format_cmd.assert());

    // Verify
    let mut check_cmd = Command::new(cargo::cargo_bin!("zenith"));
    check_cmd
        .arg("--config")
        .arg(&config_file)
        .arg("format")
        .arg(temp_dir.path())
        .arg("--check");
    assert_command_success(check_cmd.assert());
}
