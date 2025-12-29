// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Performance tests for the zenith application
//! These tests measure performance characteristics

use crate::common::{assert_command_success, create_temp_dir, create_test_file};
use assert_cmd::cargo;
use assert_cmd::prelude::*;
use std::fs;
use std::process::Command;
use std::time::Instant;

/// Performance test: Large file formatting
#[test]
fn test_large_file_formatting_performance() {
    let temp_dir = create_temp_dir();

    // Using a smaller file to avoid hanging with external formatters
    let large_content = "fn main() { println!(\"test\"); }\n".repeat(100);
    create_test_file(temp_dir.path(), "large.rs", &large_content);

    let start = Instant::now();
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("format").arg(temp_dir.path().join("large.rs"));
    assert_command_success(cmd.assert());
    let duration = start.elapsed();

    println!("Large file formatting took: {:?}", duration);
    assert!(
        duration.as_secs() < 5,
        "Formatting took too long: {:?}",
        duration
    );
}

/// Performance test: Multiple files formatting
#[test]
fn test_multiple_files_formatting_performance() {
    let temp_dir = create_temp_dir();

    let src_dir = temp_dir.path().join("src");
    fs::create_dir_all(&src_dir).unwrap();

    for i in 0..100 {
        let content = format!("fn test_{}() {{ println!(\"{}\"); }}", i, i);
        create_test_file(&src_dir, &format!("test_{}.rs", i), &content);
    }

    let start = Instant::now();
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("format").arg(&src_dir).arg("--recursive");
    assert_command_success(cmd.assert());
    let duration = start.elapsed();

    println!("Multiple files formatting took: {:?}", duration);
    assert!(
        duration.as_secs() < 30,
        "Multiple files formatting took too long: {:?}",
        duration
    );
}

/// Performance test: Config loading
#[test]
fn test_config_loading_performance() {
    let temp_dir = create_temp_dir();

    let config_content = r#"{
        "formatter": {
            "rust": {
                "max_width": 100
            }
        }
    }"#;
    let config_path = temp_dir.path().join("zenith.json");
    create_test_file(temp_dir.path(), "zenith.json", config_content);

    let rust_code = r#"fn main(){println!("Hello, World!");}"#;
    create_test_file(temp_dir.path(), "test.rs", rust_code);

    let start = Instant::now();
    let mut cmd = Command::new(cargo::cargo_bin!("zenith"));
    cmd.arg("--config")
        .arg(&config_path)
        .arg("format")
        .arg(temp_dir.path().join("test.rs"));
    assert_command_success(cmd.assert());
    let duration = start.elapsed();

    println!("Config loading took: {:?}", duration);
    assert!(
        duration.as_millis() < 1500,
        "Config loading took too long: {:?}",
        duration
    );
}
