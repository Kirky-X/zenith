// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

pub mod assertions;
pub mod mocks;

pub use assertions::assert_command_success;

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

pub fn create_test_file(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
    let file_path = dir.join(name);
    fs::write(&file_path, content).unwrap();
    file_path
}

pub fn create_temp_dir() -> TempDir {
    TempDir::new().unwrap()
}
