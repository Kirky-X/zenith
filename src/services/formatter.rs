use crate::config::types::AppConfig;
use crate::core::types::{FormatResult, ZenithConfig};
use crate::error::Result;
use crate::storage::backup::BackupService;
use crate::utils::path::{is_hidden, validate_path};
use crate::zeniths::registry::ZenithRegistry;
use ignore::WalkBuilder;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::sync::Semaphore;

pub struct ZenithService {
    config: Arc<AppConfig>,
    registry: Arc<ZenithRegistry>,
    backup_service: Arc<BackupService>,
    check_mode: bool, // 新增：检查模式标志
}

impl ZenithService {
    pub fn new(
        config: AppConfig,
        registry: Arc<ZenithRegistry>,
        backup_service: Arc<BackupService>,
        check_mode: bool,
    ) -> Self {
        Self {
            config: Arc::new(config),
            registry,
            backup_service,
            check_mode,
        }
    }

    pub async fn format_paths(&self, paths: Vec<PathBuf>) -> Result<Vec<FormatResult>> {
        let mut files = Vec::new();
        let root_path = std::env::current_dir()?; // 用于计算相对路径

        // 1. 扫描文件
        for path in paths {
            validate_path(&path)?; // 安全检查

            if path.is_file() {
                files.push(path);
            } else if path.is_dir() && self.config.global.recursive {
                // 使用 ignore::WalkBuilder 支持 .gitignore
                let walker = WalkBuilder::new(&path)
                    .hidden(true) // 跳过隐藏文件
                    .git_ignore(true) // 遵循 .gitignore
                    .build();

                for entry in walker.filter_map(|e| e.ok()) {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        files.push(entry.path().to_path_buf());
                    }
                }
            }
        }

        // 2. 初始化备份 (仅在非检查模式且启用备份时)
        if !self.check_mode && self.config.global.backup_enabled {
            self.backup_service.init().await?;
        }

        // 3. 并发处理
        let semaphore = Arc::new(Semaphore::new(self.config.concurrency.workers));
        let mut handles = Vec::new();

        for file in files {
            let sem_clone = semaphore.clone();
            let service = self.clone();
            let root = root_path.clone();

            let handle = tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                service.process_file(root, file).await
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(res) = handle.await {
                results.push(res);
            }
        }

        Ok(results)
    }

    async fn process_file(&self, root: PathBuf, path: PathBuf) -> FormatResult {
        let start = Instant::now();
        let mut result = FormatResult {
            file_path: path.clone(),
            success: false,
            changed: false,
            original_size: 0,
            formatted_size: 0,
            duration_ms: 0,
            error: None,
        };

        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e,
            None => {
                result.error = Some("No extension".into());
                return result;
            }
        };

        let zenith = match self.registry.get_by_extension(ext) {
            Some(z) => z,
            None => {
                // 忽略不支持的文件，不报错
                result.error = Some(format!("Skipped: .{} not supported", ext));
                return result;
            }
        };

        let content = match fs::read(&path).await {
            Ok(c) => c,
            Err(e) => {
                result.error = Some(e.to_string());
                return result;
            }
        };
        result.original_size = content.len() as u64;

        let limit = self.config.limits.max_file_size_mb * 1024 * 1024;
        if result.original_size > limit {
            result.error = Some(format!(
                "File too large (> {}MB)",
                self.config.limits.max_file_size_mb
            ));
            return result;
        }

        // 备份 (仅在非检查模式)
        if !self.check_mode && self.config.global.backup_enabled {
            if let Err(e) = self
                .backup_service
                .backup_file(&root, &path, &content)
                .await
            {
                result.error = Some(format!("Backup failed: {}", e));
                return result;
            }
        }

        // 格式化
        let zenith_config = ZenithConfig::default();
        match zenith.format(&content, &zenith_config).await {
            Ok(formatted) => {
                result.formatted_size = formatted.len() as u64;
                if formatted != content {
                    result.changed = true;
                    if !self.check_mode {
                        if let Err(e) = fs::write(&path, &formatted).await {
                            result.error = Some(format!("Write failed: {}", e));
                        } else {
                            result.success = true;
                        }
                    } else {
                        // 检查模式下，内容不同即视为成功检测到变化
                        result.success = true;
                    }
                } else {
                    result.success = true;
                    result.changed = false;
                }
            }
            Err(e) => {
                result.error = Some(e.to_string());
            }
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }
}

impl Clone for ZenithService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            registry: self.registry.clone(),
            backup_service: self.backup_service.clone(),
            check_mode: self.check_mode,
        }
    }
}
