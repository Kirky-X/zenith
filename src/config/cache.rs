// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! 配置缓存模块。
//! 用于缓存项目级的配置，以避免频繁的文件系统查找。

use crate::config::{load_config_with_project_discovery, types::AppConfig};
use crate::error::{Result, ZenithError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// 项目级配置缓存。
pub struct ConfigCache {
    /// 缓存映射：目录路径 -> 该目录对应的项目配置。
    cache: HashMap<PathBuf, AppConfig>,
}

impl ConfigCache {
    /// 创建一个新的配置缓存。
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// 获取指定文件路径的配置，如果需要则执行项目级自动发现。
    ///
    /// # 参数
    ///
    /// * `app_config` - 全局应用配置，作为基础配置。
    /// * `file_path` - 需要获取配置的文件路径。
    pub fn get_config_for_file(
        &mut self,
        app_config: &AppConfig,
        file_path: &Path,
    ) -> Result<AppConfig> {
        // 通过查找项目配置文件来确定该文件所属的项目目录
        let project_dir = self.find_project_directory(file_path)?;

        // 检查是否已有缓存
        if let Some(cached_config) = self.cache.get(&project_dir) {
            // 与应用级配置合并，确保应用级设置得以保留
            return Ok(self.merge_configs(app_config, cached_config));
        }

        // 执行项目级配置自动发现并加载
        let project_config = load_config_with_project_discovery(None, Some(file_path))?;

        // 存入缓存
        self.cache.insert(project_dir, project_config.clone());

        // 与应用级配置合并
        Ok(self.merge_configs(app_config, &project_config))
    }

    /// 将应用级配置与项目级配置合并（项目级配置优先级更高）。
    fn merge_configs(&self, app_config: &AppConfig, project_config: &AppConfig) -> AppConfig {
        // Create a new config with app-level settings as base and project settings overriding them
        AppConfig {
            global: if project_config.global.log_level != app_config.global.log_level
                || project_config.global.backup_enabled != app_config.global.backup_enabled
                || project_config.global.recursive != app_config.global.recursive
                || project_config.global.cache_enabled != app_config.global.cache_enabled
                || project_config.global.config_dir != app_config.global.config_dir
            {
                project_config.global.clone()
            } else {
                app_config.global.clone()
            },
            zeniths: if !project_config.zeniths.is_empty() {
                // If project has specific zenith settings, use them; otherwise use app settings
                project_config.zeniths.clone()
            } else {
                app_config.zeniths.clone()
            },
            backup: if project_config.backup.dir != app_config.backup.dir
                || project_config.backup.retention_days != app_config.backup.retention_days
            {
                project_config.backup.clone()
            } else {
                app_config.backup.clone()
            },
            concurrency: if project_config.concurrency.workers != app_config.concurrency.workers
                || project_config.concurrency.batch_size != app_config.concurrency.batch_size
            {
                project_config.concurrency.clone()
            } else {
                app_config.concurrency.clone()
            },
            limits: if project_config.limits.max_file_size_mb != app_config.limits.max_file_size_mb
                || project_config.limits.max_memory_mb != app_config.limits.max_memory_mb
            {
                project_config.limits.clone()
            } else {
                app_config.limits.clone()
            },
            mcp: if project_config.mcp.enabled != app_config.mcp.enabled
                || project_config.mcp.host != app_config.mcp.host
                || project_config.mcp.port != app_config.mcp.port
                || project_config.mcp.auth_enabled != app_config.mcp.auth_enabled
                || project_config.mcp.users.len() != app_config.mcp.users.len()
            {
                project_config.mcp.clone()
            } else {
                app_config.mcp.clone()
            },
            security: if !project_config.security.allowed_plugin_commands.is_empty()
                || project_config.security.allow_absolute_paths
                    != app_config.security.allow_absolute_paths
                || project_config.security.allow_relative_paths
                    != app_config.security.allow_relative_paths
            {
                project_config.security.clone()
            } else {
                app_config.security.clone()
            },
        }
    }

    /// Find the project directory for a given file by looking for configuration files
    pub fn find_project_directory(&self, file_path: &Path) -> Result<PathBuf> {
        let mut current_dir = file_path
            .parent()
            .ok_or_else(|| ZenithError::Config("Invalid file path".to_string()))?
            .to_path_buf();

        // Common project markers to identify project boundaries
        let project_markers = [
            ".git",
            "Cargo.toml",
            "package.json",
            "pom.xml",
            "build.gradle",
            "CMakeLists.txt",
            "Makefile",
            ".svn",
            ".hg",
            ".project",
            ".vscode",
            ".idea",
            "requirements.txt",
            "setup.py",
            "pyproject.toml",
            "Gemfile",
            "composer.json",
            "mix.exs",
            "build.sbt",
            "go.mod",
            ".zenith.toml",
            "zenith.toml",
            ".prettierrc",
            ".eslintrc",
            ".stylelintrc",
            ".clang-format",
            ".rustfmt.toml",
            ".editorconfig",
        ];

        // Traverse up the directory tree looking for project markers
        loop {
            // Check if any project marker exists in the current directory
            for marker in &project_markers {
                let marker_path = current_dir.join(marker);
                if marker_path.exists() {
                    return Ok(current_dir);
                }
            }

            // Move to parent directory
            match current_dir.parent() {
                Some(parent) => current_dir = parent.to_path_buf(),
                None => break, // Reached root directory
            }
        }

        // If no project marker is found, return the directory of the file
        Ok(file_path
            .parent()
            .unwrap_or(&PathBuf::from("."))
            .to_path_buf())
    }
}

impl Default for ConfigCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_config_cache_basic() {
        let mut cache = ConfigCache::new();
        let app_config = AppConfig::default();

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test").unwrap();

        let config = cache.get_config_for_file(&app_config, &test_file).unwrap();
        assert_eq!(config.global.log_level, "info"); // Default value
    }

    #[test]
    fn test_find_project_directory() {
        let cache = ConfigCache::new();

        let temp_dir = TempDir::new().unwrap();
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();

        let test_file = src_dir.join("main.rs");
        fs::write(&test_file, "fn main() {}").unwrap();

        let project_dir = cache.find_project_directory(&test_file).unwrap();
        assert_eq!(project_dir, temp_dir.path());
    }

    #[test]
    fn test_find_project_directory_no_marker() {
        let cache = ConfigCache::new();

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test").unwrap();

        let project_dir = cache.find_project_directory(&test_file).unwrap();
        assert_eq!(project_dir, temp_dir.path());
    }
}
