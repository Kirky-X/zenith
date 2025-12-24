use crate::config::types::BackupConfig;
use crate::error::{Result, ZenithError};
use chrono::Utc;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::fs;

pub struct BackupService {
    config: BackupConfig,
    session_id: String,
}

impl BackupService {
    pub fn new(config: BackupConfig) -> Self {
        let session_id = format!("backup_{}", Utc::now().format("%Y%m%d_%H%M%S"));
        Self { config, session_id }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub async fn init(&self) -> Result<()> {
        let path = Path::new(&self.config.dir).join(&self.session_id);
        if !path.exists() {
            fs::create_dir_all(&path).await?;
        }
        Ok(())
    }

    /// 备份单个文件，保持相对路径结构并保存哈希校验和
    pub async fn backup_file(
        &self,
        root_path: &Path,
        file_path: &Path,
        content: &[u8],
    ) -> Result<()> {
        let backup_root = Path::new(&self.config.dir).join(&self.session_id);

        // 计算相对路径以保持目录结构
        let relative_path = pathdiff::diff_paths(file_path, root_path)
            .unwrap_or_else(|| file_path.file_name().map(PathBuf::from).unwrap_or_default());

        let target_path = backup_root.join(&relative_path);
        let hash_path = backup_root.join(format!("{}.blake3", relative_path.display()));

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // 检查目标文件写入权限
        self.check_file_permissions(&target_path, "write").await?;

        // 写入内容
        fs::write(&target_path, content)
            .await
            .map_err(|e| ZenithError::BackupFailed(e.to_string()))?;

        // 检查哈希文件写入权限
        self.check_file_permissions(&hash_path, "write").await?;

        // 写入哈希
        let hash = blake3::hash(content);
        fs::write(&hash_path, hash.to_hex().as_str())
            .await
            .map_err(|e| ZenithError::BackupFailed(e.to_string()))?;

        Ok(())
    }

    /// 列出所有备份
    pub async fn list_backups(&self) -> Result<Vec<(String, SystemTime, u64)>> {
        let mut backups = Vec::new();
        let dir = Path::new(&self.config.dir);

        if !dir.exists() {
            return Ok(backups);
        }

        let mut entries = fs::read_dir(dir).await?;
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            if metadata.is_dir() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with("backup_") {
                    let created = metadata.created().unwrap_or(SystemTime::now());
                    // 计算大小（简单递归）
                    let size = fs_extra::dir::get_size(entry.path()).unwrap_or(0);
                    backups.push((name, created, size));
                }
            }
        }

        // 按时间倒序排序
        backups.sort_by(|a, b| b.1.cmp(&a.1));
        Ok(backups)
    }

    /// 恢复指定备份
    pub async fn recover(&self, backup_id: &str, target_dir: Option<PathBuf>) -> Result<usize> {
        let backup_path = Path::new(&self.config.dir).join(backup_id);
        if !backup_path.exists() {
            return Err(ZenithError::BackupNotFound(backup_id.into()));
        }

        let target_root = target_dir.unwrap_or_else(|| std::env::current_dir().unwrap());
        let mut restored_count = 0;

        // 遍历备份目录并恢复
        let mut stack = vec![backup_path.clone()];
        while let Some(curr) = stack.pop() {
            let mut entries = fs::read_dir(&curr).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().map(|e| e == "blake3").unwrap_or(false) {
                    // 跳过哈希文件
                    continue;
                } else {
                    // 计算相对于备份根目录的路径
                    let rel_path = path
                        .strip_prefix(&backup_path)
                        .map_err(|_| ZenithError::RecoverFailed("Invalid path structure".into()))?;

                    // 验证哈希（如果存在）
                    let hash_path = backup_path.join(format!("{}.blake3", rel_path.display()));
                    if hash_path.exists() {
                        let content = fs::read(&path).await?;
                        let actual_hash = blake3::hash(&content).to_hex().to_string();
                        let expected_hash = fs::read_to_string(&hash_path).await?;

                        if actual_hash != expected_hash.trim() {
                            return Err(ZenithError::RecoverFailed(format!(
                                "Hash mismatch for file: {}",
                                rel_path.display()
                            )));
                        }
                    }

                    let restore_target = target_root.join(rel_path);

                    if let Some(parent) = restore_target.parent() {
                        fs::create_dir_all(parent).await?;
                    }

                    // 检查恢复目标文件的写入权限
                    self.check_file_permissions(&restore_target, "write")
                        .await?;

                    fs::copy(&path, &restore_target).await?;
                    restored_count += 1;
                }
            }
        }

        Ok(restored_count)
    }

    /// 检查文件权限
    async fn check_file_permissions(&self, path: &Path, operation: &str) -> Result<()> {
        use tokio::fs::metadata;

        // Get file metadata
        let metadata = match metadata(path).await {
            Ok(metadata) => metadata,
            Err(e) => {
                // If the file doesn't exist, check the parent directory permissions
                if let Some(parent) = path.parent() {
                    let parent_metadata =
                        metadata(parent)
                            .await
                            .map_err(|_| ZenithError::PermissionDenied {
                                path: path.to_path_buf(),
                                reason: format!("Cannot access parent directory: {}", e),
                            })?;

                    if !parent_metadata.permissions().readonly() {
                        // Parent directory is writable, so we can create the file
                        return Ok(());
                    } else {
                        return Err(ZenithError::PermissionDenied {
                            path: path.to_path_buf(),
                            reason: "Parent directory is read-only".to_string(),
                        });
                    }
                } else {
                    return Err(ZenithError::PermissionDenied {
                        path: path.to_path_buf(),
                        reason: format!("Cannot access file: {}", e),
                    });
                }
            }
        };

        // Check if the file is read-only when we need to write to it
        if operation == "write" && metadata.permissions().readonly() {
            return Err(ZenithError::PermissionDenied {
                path: path.to_path_buf(),
                reason: "File is read-only".to_string(),
            });
        }

        Ok(())
    }

    /// 清理过期备份
    pub async fn clean_backups(&self, retention_days: u32) -> Result<usize> {
        let backups = self.list_backups().await?;
        let now = SystemTime::now();
        let retention_duration =
            std::time::Duration::from_secs((retention_days as u64) * 24 * 3600);

        let mut deleted_count = 0;

        for (name, created, _) in backups {
            if let Ok(age) = now.duration_since(created) {
                if age > retention_duration {
                    let path = Path::new(&self.config.dir).join(name);
                    fs::remove_dir_all(path).await?;
                    deleted_count += 1;
                }
            }
        }

        Ok(deleted_count)
    }

    /// 恢复最新备份
    pub async fn recover_latest(&self) -> Result<Vec<PathBuf>> {
        let backups = self.list_backups().await?;
        if backups.is_empty() {
            return Err(ZenithError::BackupNotFound("No backups available".into()));
        }

        // Get the most recent backup (first in the list since it's sorted by time)
        let latest_backup = &backups[0];
        let backup_id = &latest_backup.0;

        // Get the current directory as the target
        let current_dir = std::env::current_dir()?;

        // Use the recover method to restore the backup
        let _restored_count = self.recover(backup_id, Some(current_dir)).await?;

        // Since the recover method returns the count, we'll return an empty vector
        // To return the actual paths, we need to track them during recovery
        // For now, let's implement a method that returns the paths that were recovered

        // Get all files in the backup to return their paths
        let backup_path = Path::new(&self.config.dir).join(backup_id);
        let mut recovered_files = Vec::new();

        if backup_path.exists() {
            // Walk through the backup directory to get the files that would be restored
            let mut stack = vec![backup_path.clone()];
            while let Some(curr) = stack.pop() {
                let mut entries = fs::read_dir(&curr).await?;
                while let Some(entry) = entries.next_entry().await? {
                    let path = entry.path();
                    if path.is_dir() {
                        stack.push(path);
                    } else if path.extension().map(|e| e == "blake3").unwrap_or(false) {
                        // Skip hash files
                        continue;
                    } else {
                        // Calculate the relative path from backup root
                        let rel_path = path.strip_prefix(&backup_path).map_err(|_| {
                            ZenithError::RecoverFailed("Invalid path structure".into())
                        })?;

                        // The restored path would be current_dir + rel_path
                        let restored_path = std::env::current_dir()?.join(rel_path);
                        recovered_files.push(restored_path);
                    }
                }
            }
        }

        Ok(recovered_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_backup_permission_checks() {
        // Skip this test on non-Unix systems
        #[cfg(unix)]
        {
            // Create a temporary directory for testing
            let temp_dir = TempDir::new().unwrap();
            let backup_dir = temp_dir.path().join("backups");

            // Create backup config
            let config = BackupConfig {
                dir: backup_dir.to_string_lossy().to_string(),
                retention_days: 7,
            };

            // Create backup service
            let service = BackupService::new(config);

            // Initialize backup service
            service.init().await.unwrap();

            // Create a test file in the temp directory
            let test_file = temp_dir.path().join("readonly.txt");
            std::fs::write(&test_file, "test content").unwrap();

            // Make the file read-only
            let mut perms = std::fs::metadata(&test_file).unwrap().permissions();
            perms.set_readonly(true);
            std::fs::set_permissions(&test_file, perms).unwrap();

            // Test write permission check on read-only file (should fail)
            let result = service.check_file_permissions(&test_file, "write").await;
            assert!(result.is_err());
            match result.unwrap_err() {
                ZenithError::PermissionDenied { path, reason } => {
                    assert_eq!(path, test_file);
                    assert!(reason.contains("read-only"));
                }
                _ => panic!("Expected PermissionDenied error"),
            }
        }

        // For non-Unix systems, we'll just have a placeholder test
        #[cfg(not(unix))]
        {
            use crate::error::ZenithError;
            // Create a temporary directory for testing
            let temp_dir = TempDir::new().unwrap();
            let backup_dir = temp_dir.path().join("backups");

            // Create backup config
            let config = BackupConfig {
                dir: backup_dir.to_string_lossy().to_string(),
                retention_days: 7,
            };

            // Create backup service
            let service = BackupService::new(config);

            // Create a test file
            let test_file = temp_dir.path().join("test.txt");
            std::fs::write(&test_file, "test content").unwrap();

            // Test permission checks (these will pass on non-Unix systems)
            let result = service.check_file_permissions(&test_file, "write").await;
            assert!(result.is_ok());
        }
    }
}
