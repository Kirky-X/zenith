use crate::config::types::AppConfig;
use crate::core::types::{FormatResult, ZenithConfig};
use crate::error::Result;
use crate::storage::{backup::BackupService, cache::HashCache};
use crate::utils::path::validate_path;
use crate::zeniths::registry::ZenithRegistry;
use ignore::WalkBuilder;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::fs;
use tokio::sync::Semaphore;

use sysinfo::System;

pub struct ZenithService {
    config: Arc<AppConfig>,
    registry: Arc<ZenithRegistry>,
    backup_service: Arc<BackupService>,
    cache: Arc<HashCache>, // 新增：文件状态缓存
    check_mode: bool,
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
            cache: Arc::new(HashCache::new()),
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
            } else if !path.exists() {
                // If a path was explicitly provided but doesn't exist, return an error
                return Err(crate::error::ZenithError::FileNotFound { path });
            }
        }

        // 2. 使用缓存过滤不需要处理的文件
        let files_to_process = if self.config.global.cache_enabled {
            self.filter_files_with_cache(&files).await?
        } else {
            files
        };

        // 3. 初始化备份 (仅在非检查模式且启用备份时)
        if !self.check_mode && self.config.global.backup_enabled {
            self.backup_service.init().await?;
        }

        // 4. 并发处理
        let semaphore = Arc::new(Semaphore::new(self.config.concurrency.workers));
        let mut handles = Vec::new();

        for file in files_to_process {
            let sem_clone = semaphore.clone();
            let service = self.clone();
            let root = root_path.clone();

            let handle = tokio::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                service.process_file(root, file).await
            });
            handles.push(handle);
            
            // Check memory usage periodically during batch processing
            if handles.len() % 10 == 0 { // Check every 10 tasks
                if let Err(e) = self.check_memory_usage() {
                    tracing::warn!("Memory limit approached during batch processing: {}", e);
                }
            }
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(res) = handle.await {
                results.push(res);
            }
        }

        // 5. Calculate performance metrics
        self.calculate_performance_metrics(&results);

        Ok(results)
    }

    /// 使用缓存过滤不需要处理的文件
    async fn filter_files_with_cache(&self, files: &[PathBuf]) -> Result<Vec<PathBuf>> {
        let mut files_to_process = Vec::new();
        let mut cache_updates = Vec::new();

        // 批量检查文件是否需要处理
        let needs_processing = self.cache.batch_needs_processing(files).await?;

        for (i, path) in files.iter().enumerate() {
            if needs_processing[i] {
                files_to_process.push(path.clone());
            }
        }

        // 更新缓存状态
        for path in &files_to_process {
            let state = self.cache.compute_file_state(path).await?;
            cache_updates.push((path.clone(), state));
        }

        if !cache_updates.is_empty() {
            self.cache.batch_update(cache_updates).await?;
        }

        Ok(files_to_process)
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

        // Check memory usage before processing
        if let Err(e) = self.check_memory_usage() {
            result.error = Some(format!("Memory limit exceeded: {}", e));
            return result;
        }

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

        // Check memory usage after reading file content
        if let Err(e) = self.check_memory_usage() {
            result.error = Some(format!("Memory limit exceeded: {}", e));
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
        match zenith.format(&content, &path, &zenith_config).await {
            Ok(formatted) => {
                result.formatted_size = formatted.len() as u64;
                
                // Check memory usage after formatting
                if let Err(e) = self.check_memory_usage() {
                    result.error = Some(format!("Memory limit exceeded: {}", e));
                    return result;
                }
                
                if formatted != content {
                    result.changed = true;
                    if !self.check_mode {
                        if let Err(e) = fs::write(&path, &formatted).await {
                            // 如果写入失败，尝试回退到备份
                            if let Err(rollback_err) = self.rollback_file(&root, &path).await {
                                result.error = Some(format!("Write failed: {}; Rollback failed: {}", e, rollback_err));
                            } else {
                                result.error = Some(format!("Write failed: {}; Rolled back to backup", e));
                            }
                        } else {
                            result.success = true;

                            // 更新缓存状态
                            if self.config.global.cache_enabled {
                                if let Ok(state) = self.cache.compute_file_state(&path).await {
                                    let _ = self.cache.update(path.clone(), state).await;
                                }
                            }
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
                // 如果格式化失败，尝试回退到备份
                if !self.check_mode && self.config.global.backup_enabled {
                    if let Err(rollback_err) = self.rollback_file(&root, &path).await {
                        result.error = Some(format!("Format failed: {}; Rollback failed: {}", e, rollback_err));
                    } else {
                        result.error = Some(format!("Format failed: {}; Rolled back to backup", e));
                    }
                } else {
                    result.error = Some(e.to_string());
                }
            }
        }

        result.duration_ms = start.elapsed().as_millis() as u64;
        result
    }

    /// 清空缓存
    pub async fn clear_cache(&self) -> Result<()> {
        self.cache.clear().await
    }

    /// 获取缓存统计信息
    pub async fn cache_stats(&self) -> crate::storage::CacheStats {
        self.cache.stats().await
    }

    /// Calculate performance metrics from format results
    fn calculate_performance_metrics(&self, results: &[FormatResult]) {
        // Filter out failed operations for performance metrics
        let mut successful_durations: Vec<f64> = results
            .iter()
            .filter(|result| result.success)
            .map(|result| result.duration_ms as f64)
            .collect();

        if !successful_durations.is_empty() {
            // Sort durations for percentile calculation
            successful_durations.sort_by(|a, b| a.partial_cmp(b).unwrap());
            
            // Calculate P95 and P99 percentiles
            let p95 = self.calculate_percentile(&successful_durations, 95.0);
            let p99 = self.calculate_percentile(&successful_durations, 99.0);
            
            // Calculate average
            let avg = successful_durations.iter().sum::<f64>() / successful_durations.len() as f64;
            
            // Calculate min and max
            let min = successful_durations[0] as u64;
            let max = successful_durations[successful_durations.len() - 1] as u64;
            
            // Calculate standard deviation
            let std_dev = self.calculate_standard_deviation(&successful_durations, avg);

            let metrics = crate::core::types::PerformanceMetrics {
                total_files: successful_durations.len(),
                p95_duration_ms: p95,
                p99_duration_ms: p99,
                avg_duration_ms: avg,
                min_duration_ms: min,
                max_duration_ms: max,
                std_deviation_ms: std_dev,
            };

            // Log performance metrics
            tracing::info!(
                "Performance Metrics: P95={}ms, P99={}ms, AVG={}ms, MIN={}ms, MAX={}ms, STDDEV={}ms, TotalFiles={}",
                metrics.p95_duration_ms,
                metrics.p99_duration_ms,
                metrics.avg_duration_ms,
                metrics.min_duration_ms,
                metrics.max_duration_ms,
                metrics.std_deviation_ms,
                metrics.total_files
            );
        }
    }

    /// Calculate percentile of a sorted slice of values
    fn calculate_percentile(&self, values: &[f64], percentile: f64) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let n = values.len();
        let index = (percentile / 100.0 * (n as f64 - 1.0)).round() as usize;
        values[index.min(n - 1)]
    }

    /// Calculate standard deviation
    fn calculate_standard_deviation(&self, values: &[f64], mean: f64) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        
        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / values.len() as f64;
        
        variance.sqrt()
    }

    /// Check current memory usage and enforce limits
    fn check_memory_usage(&self) -> std::result::Result<(), String> {
        let mut system = System::new_all();
        system.refresh_memory();

        // Get total memory used by the current process in bytes
        let current_process_id = sysinfo::get_current_pid().map_err(|e| e.to_string())?;
        let mut system_process = System::new();
        system_process.refresh_process(current_process_id);
        
        // If we can't get process-specific memory, fall back to system memory usage
        let memory_used_mb = if system_process.refresh_process(current_process_id) {
            let process = system_process.process(current_process_id).unwrap();
            process.memory() / (1024 * 1024) // Convert bytes to MB
        } else {
            // Fallback to system memory usage (less precise)
            let total_memory = system.total_memory();
            let available_memory = system.available_memory();
            let used_memory = total_memory - available_memory;
            used_memory / (1024 * 1024) // Convert bytes to MB
        };

        let max_memory_mb = self.config.limits.max_memory_mb;
        
        if memory_used_mb > max_memory_mb {
            return Err(format!(
                "Memory usage ({:.0}MB) exceeded limit ({}MB)",
                memory_used_mb, max_memory_mb
            ));
        }

        Ok(())
    }

    /// 回退单个文件到备份状态
    async fn rollback_file(&self, root: &PathBuf, file_path: &PathBuf) -> Result<()> {
        if !self.config.global.backup_enabled {
            return Err(crate::error::ZenithError::BackupDisabled);
        }

        // 计算相对路径
        let relative_path = pathdiff::diff_paths(file_path, root)
            .unwrap_or_else(|| file_path.file_name().map(PathBuf::from).unwrap_or_default());

        // 构建备份路径
        let backup_root = std::path::Path::new(&self.config.backup.dir).join(self.backup_service.get_session_id());
        let backup_file_path = backup_root.join(&relative_path);
        
        // 检查备份文件是否存在
        if !backup_file_path.exists() {
            return Err(crate::error::ZenithError::BackupNotFound(relative_path.display().to_string()));
        }

        // 读取备份内容
        let backup_content = tokio::fs::read(&backup_file_path).await
            .map_err(|e| crate::error::ZenithError::RecoverFailed(e.to_string()))?;

        // 验证哈希（如果存在）
        let hash_path = backup_root.join(format!("{}.blake3", relative_path.display()));
        if hash_path.exists() {
            let actual_hash = blake3::hash(&backup_content).to_hex().to_string();
            let expected_hash = tokio::fs::read_to_string(&hash_path).await
                .map_err(|e| crate::error::ZenithError::RecoverFailed(e.to_string()))?;

            if actual_hash.trim() != expected_hash.trim() {
                return Err(crate::error::ZenithError::RecoverFailed(
                    format!("Hash mismatch for file: {}", relative_path.display())
                ));
            }
        }

        // 将备份内容写回原文件
        tokio::fs::write(file_path, &backup_content).await
            .map_err(|e| crate::error::ZenithError::RecoverFailed(e.to_string()))?;

        Ok(())
    }

    /// 自动回滚到最新的备份状态
    pub async fn auto_rollback(&self) -> Result<Vec<String>> {
        if !self.config.global.backup_enabled {
            return Err(crate::error::ZenithError::BackupDisabled);
        }

        // 获取最新的备份
        let backup_service = &self.backup_service;
        let backups = backup_service.list_backups().await?;
        
        if backups.is_empty() {
            return Err(crate::error::ZenithError::NoBackupsAvailable);
        }

        // 获取最新的备份ID
        let latest_backup = backups
                .into_iter()
                .max_by_key(|(_, time, _)| *time)
                .ok_or(crate::error::ZenithError::NoBackupsAvailable)?;

        let (backup_id, _, _) = latest_backup;
        
        // 获取当前工作目录作为恢复根目录
        let current_dir = std::env::current_dir()?;
        
        // 执行恢复操作
        let _recovered_files = backup_service.recover(&backup_id, Some(current_dir.clone())).await?;
        
        // 返回恢复的文件列表
        let mut recovered_file_paths = Vec::new();
        let backup_root = std::path::Path::new(&self.config.backup.dir).join(&backup_id);
        
        // 获取备份目录中所有文件
        let walker = ignore::Walk::new(&backup_root);
        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                // 跳过哈希文件
                if entry.path().extension().is_none_or(|ext| ext != "blake3") {
                    let relative_path = pathdiff::diff_paths(entry.path(), &backup_root)
                        .unwrap_or_else(|| PathBuf::from(entry.file_name()));
                    
                    let full_path = current_dir.join(&relative_path);
                    recovered_file_paths.push(full_path.display().to_string());
                }
            }
        }

        Ok(recovered_file_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::{AppConfig, BackupConfig, ConcurrencyConfig, GlobalConfig, LimitsConfig, ZenithSettings};
    use crate::zeniths::registry::ZenithRegistry;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_rollback_functionality() {
        // 创建临时目录用于测试
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        
        // 创建一个测试文件
        std::fs::write(&test_file, "fn main() { println!(\"hello\"); }").unwrap();
        
        // 创建配置
        let mut config = AppConfig::default();
        config.global.backup_enabled = true;
        config.backup.dir = temp_dir.path().join(".zenith_backup").to_string_lossy().to_string();
        
        // 创建服务
        let registry = Arc::new(ZenithRegistry::new());
        let backup_service = Arc::new(BackupService::new(config.backup.clone()));
        let service = ZenithService::new(config, registry, backup_service, false);
        
        // 初始化备份服务
        service.backup_service.init().await.unwrap();
        
        // 备份文件
        let content = tokio::fs::read(&test_file).await.unwrap();
        service.backup_service.backup_file(temp_dir.path(), &test_file, &content).await.unwrap();
        
        // 修改文件内容
        tokio::fs::write(&test_file, "fn main() { println!(\"modified\"); }").await.unwrap();
        
        // 执行回退
        let result = service.rollback_file(&temp_dir.path().to_path_buf(), &test_file).await;
        assert!(result.is_ok());
        
        // 验证文件内容已回退
        let reverted_content = tokio::fs::read_to_string(&test_file).await.unwrap();
        assert_eq!(reverted_content, "fn main() { println!(\"hello\"); }");
    }

    #[tokio::test]
    async fn test_auto_rollback_functionality() {
        // 创建临时目录用于测试
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");
        
        // 创建一个测试文件
        std::fs::write(&test_file, "fn main() { println!(\"hello\"); }").unwrap();
        
        // 创建配置
        let mut config = AppConfig::default();
        config.global.backup_enabled = true;
        config.backup.dir = temp_dir.path().join(".zenith_backup").to_string_lossy().to_string();
        
        // 创建服务
        let registry = Arc::new(ZenithRegistry::new());
        let backup_service = Arc::new(BackupService::new(config.backup.clone()));
        let service = ZenithService::new(config, registry, backup_service, false);
        
        // 初始化备份服务
        service.backup_service.init().await.unwrap();
        
        // 备份文件
        let content = tokio::fs::read(&test_file).await.unwrap();
        service.backup_service.backup_file(temp_dir.path(), &test_file, &content).await.unwrap();
        
        // 修改文件内容
        tokio::fs::write(&test_file, "fn main() { println!(\"modified\"); }").await.unwrap();
        
        // 验证文件已被修改
        let modified_content = tokio::fs::read_to_string(&test_file).await.unwrap();
        assert_eq!(modified_content, "fn main() { println!(\"modified\"); }");
        
        // 执行自动回退
        let current_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();
        
        let result = service.auto_rollback().await;
        assert!(result.is_ok());
        
        // 验证文件内容已回退
        let reverted_content = tokio::fs::read_to_string(&test_file).await.unwrap();
        assert_eq!(reverted_content, "fn main() { println!(\"hello\"); }");
        
        // 恢复原始目录
        std::env::set_current_dir(&current_dir).unwrap();
    }
}

impl Clone for ZenithService {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            registry: self.registry.clone(),
            backup_service: self.backup_service.clone(),
            cache: self.cache.clone(),
            check_mode: self.check_mode,
        }
    }
}
