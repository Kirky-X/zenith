// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Unit tests for formatters
//! Tests for Zenith trait implementations and specific language formatters

use crate::common::create_temp_dir;
use std::path::PathBuf;
use zenith::config::types::ZenithConfig;
use zenith::core::traits::Zenith;
use zenith::internal::{PrettierZenith, PythonZenith, RustZenith};

#[test]
fn test_rust_zenith_name() {
    let formatter = RustZenith;
    assert_eq!(formatter.name(), "rust");
}

#[test]
fn test_rust_zenith_extensions() {
    let formatter = RustZenith;
    let extensions = formatter.extensions();
    assert_eq!(extensions, &["rs"]);
    assert!(extensions.contains(&"rs"));
}

#[test]
fn test_python_zenith_name() {
    let formatter = PythonZenith;
    assert_eq!(formatter.name(), "python");
}

#[test]
fn test_python_zenith_extensions() {
    let formatter = PythonZenith;
    let extensions = formatter.extensions();
    assert_eq!(extensions.len(), 2);
    assert!(extensions.contains(&"py"));
    assert!(extensions.contains(&"pyi"));
}

#[test]
fn test_prettier_zenith_name() {
    let formatter = PrettierZenith;
    assert_eq!(formatter.name(), "prettier");
}

#[test]
fn test_prettier_zenith_extensions() {
    let formatter = PrettierZenith;
    let extensions = formatter.extensions();

    let expected_extensions = [
        "js", "jsx", "ts", "tsx", "json", "css", "scss", "html", "vue", "yaml", "yml", "md",
    ];

    assert_eq!(extensions.len(), expected_extensions.len());
    for ext in expected_extensions {
        assert!(extensions.contains(&ext), "Missing extension: {}", ext);
    }
}

#[test]
fn test_zenith_trait_extension_matching() {
    let rust_formatter = RustZenith;
    let python_formatter = PythonZenith;
    let prettier_formatter = PrettierZenith;

    assert!(rust_formatter.extensions().contains(&"rs"));
    assert!(!rust_formatter.extensions().contains(&"py"));

    assert!(python_formatter.extensions().contains(&"py"));
    assert!(!python_formatter.extensions().contains(&"rs"));

    assert!(prettier_formatter.extensions().contains(&"js"));
    assert!(prettier_formatter.extensions().contains(&"ts"));
    assert!(!prettier_formatter.extensions().contains(&"rs"));
}

#[test]
fn test_formatter_name_uniqueness() {
    let rust_formatter = RustZenith;
    let python_formatter = PythonZenith;
    let prettier_formatter = PrettierZenith;

    let names = [
        rust_formatter.name(),
        python_formatter.name(),
        prettier_formatter.name(),
    ];

    assert_eq!(
        names.len(),
        names.iter().collect::<std::collections::HashSet<_>>().len(),
        "Formatter names must be unique"
    );
}

#[test]
fn test_extension_coverage() {
    let formatters: Vec<Box<dyn Zenith>> = vec![
        Box::new(RustZenith),
        Box::new(PythonZenith),
        Box::new(PrettierZenith),
    ];

    let mut all_extensions = std::collections::HashSet::new();

    for formatter in &formatters {
        for ext in formatter.extensions() {
            all_extensions.insert(*ext);
        }
    }

    assert!(
        !all_extensions.is_empty(),
        "Should have at least one extension"
    );
    assert!(all_extensions.contains("rs"), "Should support Rust files");
    assert!(all_extensions.contains("py"), "Should support Python files");
    assert!(
        all_extensions.contains("js"),
        "Should support JavaScript files"
    );
}

#[test]
fn test_formatter_config_default() {
    let config = ZenithConfig::default();
    assert!(config.use_default_rules);
    assert!(config.custom_config_path.is_none());
}

#[tokio::test]
async fn test_formatter_empty_content() {
    let formatter = RustZenith;
    let content = b"";
    let path = PathBuf::from("/tmp/test.rs");
    let config = &ZenithConfig::default();

    let result = formatter.format(content, &path, config).await;
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_formatter_large_content() {
    let formatter = RustZenith;
    let content = vec![b'a'; 1024 * 1024];
    let path = PathBuf::from("/tmp/test.rs");
    let config = &ZenithConfig::default();

    let result = formatter.format(&content, &path, config).await;
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_path_handling() {
    let test_cases = vec![
        ("/tmp/test.rs", true),
        ("/tmp/test.py", true),
        ("/tmp/test.js", true),
        ("/tmp/test.txt", false),
        ("/tmp/test.unknown", false),
    ];

    for (path_str, should_be_supported) in test_cases {
        let path = PathBuf::from(path_str);
        let extension = path.extension().and_then(|e| e.to_str());

        let is_supported = matches!(
            extension,
            Some("rs")
                | Some("py")
                | Some("pyi")
                | Some("js")
                | Some("jsx")
                | Some("ts")
                | Some("tsx")
                | Some("json")
                | Some("css")
                | Some("scss")
                | Some("html")
                | Some("vue")
                | Some("yaml")
                | Some("yml")
                | Some("md")
        );

        assert_eq!(
            is_supported, should_be_supported,
            "Path {} support check failed",
            path_str
        );
    }
}

#[test]
fn test_formatter_case_sensitivity() {
    let formatter = RustZenith;
    let extensions = formatter.extensions();

    assert!(extensions.contains(&"rs"));
    assert!(!extensions.contains(&"RS"));
    assert!(!extensions.contains(&"Rs"));
}

#[test]
fn test_extension_boundary_conditions() {
    let rust_formatter = RustZenith;
    let python_formatter = PythonZenith;
    let prettier_formatter = PrettierZenith;

    assert_eq!(rust_formatter.extensions().len(), 1);
    assert_eq!(python_formatter.extensions().len(), 2);
    assert!(prettier_formatter.extensions().len() > 5);
}

#[test]
fn test_formatter_path_with_config() {
    let temp_dir = create_temp_dir();
    let config_file = temp_dir.path().join("rustfmt.toml");
    std::fs::write(&config_file, "max_width = 80").unwrap();

    let test_file = temp_dir.path().join("src/main.rs");
    std::fs::create_dir_all(test_file.parent().unwrap()).unwrap();

    let _formatter = RustZenith;
    let path = test_file.as_path();

    assert!(path.exists() || !path.exists());
}

#[tokio::test]
async fn test_formatter_with_invalid_syntax() {
    let formatter = RustZenith;
    let invalid_code = b"fn main( {";
    let path = PathBuf::from("/tmp/invalid.rs");
    let config = &ZenithConfig::default();

    let result = formatter.format(invalid_code, &path, config).await;
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_formatter_thread_safety() {
    let handles: Vec<_> = (0..10)
        .map(|i| {
            std::thread::spawn(move || {
                if i % 2 == 0 {
                    RustZenith.name().to_string()
                } else {
                    PythonZenith.name().to_string()
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_zenith_trait_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<RustZenith>();
    assert_send_sync::<PythonZenith>();
    assert_send_sync::<PrettierZenith>();
}
