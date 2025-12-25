//! Unit tests for core components
//! Tests for path validation, registry, and other core functionality

use crate::common::mocks::MockZenith;
use std::fs;
use tempfile::TempDir;
use walkdir::WalkDir;
use zenith::core::traits::Zenith;
use zenith::error::ZenithError;
use zenith::utils::path::{is_hidden, validate_path};
use zenith::zeniths::registry::ZenithRegistry;

#[test]
fn test_validate_path_normal() {
    let path = std::path::Path::new("/home/user/project/src/main.rs");
    assert!(validate_path(path).is_ok());
}

#[test]
fn test_validate_path_with_parent_dir() {
    let path = std::path::Path::new("/home/user/project/../etc/passwd");
    let result = validate_path(path);
    assert!(result.is_err());
    match result.unwrap_err() {
        ZenithError::PathTraversal(_) => {}
        _ => panic!("Expected PathTraversal error"),
    }
}

#[test]
fn test_validate_path_with_multiple_parent_dirs() {
    let path = std::path::Path::new("../../../etc/passwd");
    let result = validate_path(path);
    assert!(result.is_err());
}

#[test]
fn test_validate_path_relative_safe() {
    let path = std::path::Path::new("src/main.rs");
    assert!(validate_path(path).is_ok());
}

#[test]
fn test_is_hidden_dot_files() {
    let temp_dir = TempDir::new().unwrap();

    let hidden_file = temp_dir.path().join(".hidden");
    fs::write(&hidden_file, "content").unwrap();

    let entry = WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == ".hidden")
        .unwrap();
    assert!(is_hidden(&entry));
}

#[test]
fn test_is_hidden_normal_files() {
    let temp_dir = TempDir::new().unwrap();

    let normal_file = temp_dir.path().join("normal.txt");
    fs::write(&normal_file, "content").unwrap();

    let entry = WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == "normal.txt")
        .unwrap();
    assert!(!is_hidden(&entry));
}

#[test]
fn test_is_hidden_dot_and_dotdot() {
    let temp_dir = TempDir::new().unwrap();

    let entries: Vec<_> = WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.depth() == 1)
        .collect();

    for entry in entries {
        let name = entry.file_name();
        if name == "." || name == ".." {
            assert!(!is_hidden(&entry));
        }
    }
}

#[test]
fn test_registry_register_and_get() {
    let registry = ZenithRegistry::new();
    let formatter = std::sync::Arc::new(MockZenith::new("rust", &["rs", "rust"]));

    registry.register(formatter);

    let retrieved = registry.get_by_extension("rs");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name(), "rust");
}

#[test]
fn test_registry_multiple_extensions() {
    let registry = ZenithRegistry::new();
    let formatter = std::sync::Arc::new(MockZenith::new("rust", &["rs", "rust"]));

    registry.register(formatter);

    assert!(registry.get_by_extension("rs").is_some());
    assert!(registry.get_by_extension("rust").is_some());
}

#[test]
fn test_registry_get_nonexistent() {
    let registry = ZenithRegistry::new();

    let retrieved = registry.get_by_extension("xyz");
    assert!(retrieved.is_none());
}

#[test]
fn test_registry_list_all() {
    let registry = ZenithRegistry::new();

    let rust_formatter = std::sync::Arc::new(MockZenith::new("rust", &["rs"]));
    let js_formatter = std::sync::Arc::new(MockZenith::new("js", &["js", "jsx", "ts", "tsx"]));

    registry.register(rust_formatter);
    registry.register(js_formatter);

    let all = registry.list_all();
    assert_eq!(all.len(), 2);
}

#[test]
fn test_registry_extension_override() {
    let registry = ZenithRegistry::new();

    let formatter1 = std::sync::Arc::new(MockZenith::new("prettier", &["js"]));
    let formatter2 = std::sync::Arc::new(MockZenith::new("eslint", &["js"]));

    registry.register(formatter1);
    registry.register(formatter2);

    let retrieved = registry.get_by_extension("js");
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name(), "eslint");
}

#[test]
fn test_registry_default() {
    let registry = ZenithRegistry::default();
    assert_eq!(registry.list_all().len(), 0);
}

#[test]
fn test_path_traversal_detection() {
    let test_cases = vec![
        ("../etc/passwd", true),
        ("./normal/path", false),
        ("/absolute/path", false),
        ("../../secret", true),
        ("normal/../file", true),
        ("./normal/file.rs", false),
    ];

    for (path_str, should_fail) in test_cases {
        let path = std::path::Path::new(path_str);
        let result = validate_path(path);

        if should_fail {
            assert!(result.is_err(), "Path {} should fail validation", path_str);
        } else {
            assert!(result.is_ok(), "Path {} should pass validation", path_str);
        }
    }
}

#[test]
fn test_is_hidden_various_patterns() {
    let temp_dir = TempDir::new().unwrap();

    let test_cases = vec![
        (".hidden", true),
        ("normal", false),
        (".gitignore", true),
        (".env", true),
        ("README.md", false),
        ("src", false),
        (".vscode", true),
    ];

    for (name, should_be_hidden) in test_cases {
        let file_path = temp_dir.path().join(name);
        fs::write(&file_path, "content").unwrap();

        let entry = WalkDir::new(temp_dir.path())
            .into_iter()
            .filter_map(|e| e.ok())
            .find(|e| e.file_name() == name)
            .unwrap();
        assert_eq!(
            is_hidden(&entry),
            should_be_hidden,
            "File {} visibility check failed",
            name
        );
    }
}

#[tokio::test]
async fn test_zenith_format_basic() {
    let formatter = crate::common::mocks::MockFormatter::new("test", &["txt"]);
    let content = b"test content";
    let path = std::path::Path::new("/tmp/test.txt");
    let config = &zenith::config::types::ZenithConfig::default();

    let result = formatter.format(content, path, config).await.unwrap();
    assert_eq!(result, content);
}

#[tokio::test]
async fn test_zenith_validate_default() {
    let formatter = MockZenith::new("test", &["txt"]);
    let content = b"test content";

    let result = formatter.validate(content).await.unwrap();
    assert!(result);
}
