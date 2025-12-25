use crate::error::{Result, ZenithError};
use crate::utils::directory::traverse_upwards;
use std::path::{Path, PathBuf};

const PROJECT_CONFIG_FILES: &[&str] = &[
    ".zenith.toml",
    "zenith.toml",
    ".zenith.yaml",
    "zenith.yaml",
    ".zenith.json",
    "zenith.json",
    ".editorconfig",
    ".prettierrc",
    ".prettierrc.json",
    ".prettierrc.yaml",
    ".prettierrc.yml",
    ".prettierrc.js",
    ".eslintrc",
    ".eslintrc.json",
    ".eslintrc.yaml",
    ".eslintrc.yml",
    ".stylelintrc",
    ".stylelintrc.json",
    ".stylelintrc.yaml",
    ".stylelintrc.yml",
    ".clang-format",
    ".clang-tidy",
    ".rustfmt.toml",
    "_clang-format",
    "_clang-tidy",
    ".gitattributes",
];

fn get_formatter_config_files(formatter_name: &str) -> &'static [&'static str] {
    match formatter_name {
        "rust" => &[".rustfmt.toml", "rustfmt.toml"],
        "javascript" | "typescript" | "json" | "html" | "css" | "less" | "scss" | "graphql" => &[
            ".prettierrc",
            ".prettierrc.json",
            ".prettierrc.yaml",
            ".prettierrc.yml",
            ".prettierrc.js",
        ],
        "python" => &[
            ".black",
            "pyproject.toml",
            "setup.cfg",
            ".flake8",
            "pycodestyle.cfg",
        ],
        "java" => &[".google-java-format", "google-java-format.properties"],
        "c" | "cpp" | "c++" => &[
            ".clang-format",
            "_clang-format",
            ".clang-tidy",
            "_clang-tidy",
        ],
        "shell" | "bash" => &[".shellcheckrc", "shell.nix"],
        "go" => &[
            ".golangci.yml",
            ".golangci.yaml",
            "golangci.yml",
            "golangci.yaml",
        ],
        "docker" => &[".dockerignore", ".dive.yaml"],
        "markdown" => &[
            ".markdownlint.json",
            ".markdownlint.yaml",
            ".markdownlint.yml",
            ".prettierrc",
            ".prettierrc.json",
            ".prettierrc.yaml",
            ".prettierrc.yml",
            ".prettierrc.js",
        ],
        "yaml" => &[
            ".yamllint",
            ".yamllint.yml",
            ".yamllint.yaml",
            ".prettierrc",
            ".prettierrc.json",
            ".prettierrc.yaml",
            ".prettierrc.yml",
            ".prettierrc.js",
        ],
        "toml" => &[".taplo.toml", "taplo.toml"],
        _ => &[],
    }
}

pub fn discover_project_config(file_path: &Path) -> Result<Option<PathBuf>> {
    let config_files = PROJECT_CONFIG_FILES.to_vec();
    let start_dir = file_path
        .parent()
        .ok_or_else(|| ZenithError::Config("Invalid file path".to_string()))?;

    traverse_upwards(start_dir, |dir| {
        for config_file in &config_files {
            let config_path = dir.join(config_file);
            if config_path.exists() {
                return Some(config_path);
            }
        }
        None
    })
}

pub fn discover_formatter_config(
    file_path: &Path,
    formatter_name: &str,
) -> Result<Option<PathBuf>> {
    let config_files = get_formatter_config_files(formatter_name);
    if config_files.is_empty() {
        return Ok(None);
    }

    let start_dir = file_path
        .parent()
        .ok_or_else(|| ZenithError::Config("Invalid file path".to_string()))?;

    traverse_upwards(start_dir, |dir| {
        for config_file in config_files {
            let config_path = dir.join(config_file);
            if config_path.exists() {
                return Some(config_path);
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discover_project_config_no_config() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test content").unwrap();

        let result = discover_project_config(&test_file).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_discover_project_config_with_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join(".zenith.toml");
        fs::write(&config_file, "test = true").unwrap();

        let test_file = temp_dir.path().join("subdir").join("test.txt");
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(&test_file, "test content").unwrap();

        let result = discover_project_config(&test_file).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_file);
    }

    #[test]
    fn test_discover_formatter_config_rust() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join(".rustfmt.toml");
        fs::write(&config_file, "[rustfmt]\nmax_width = 80").unwrap();

        let test_file = temp_dir.path().join("src").join("main.rs");
        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::write(&test_file, "fn main() {}").unwrap();

        let result = discover_formatter_config(&test_file, "rust").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_file);
    }

    #[test]
    fn test_discover_formatter_config_javascript() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join(".prettierrc.json");
        fs::write(&config_file, r#"{"semi": false, "trailingComma": "es5"}"#).unwrap();

        let test_file = temp_dir.path().join("src").join("index.js");
        fs::create_dir(temp_dir.path().join("src")).unwrap();
        fs::write(&test_file, "function test() {}").unwrap();

        let result = discover_formatter_config(&test_file, "javascript").unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), config_file);
    }
}
