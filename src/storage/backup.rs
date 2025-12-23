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

        // 写入内容
        fs::write(&target_path, content)
            .await
            .map_err(|e| ZenithError::BackupFailed(e.to_string()))?;

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

                    fs::copy(&path, &restore_target).await?;
                    restored_count += 1;
                }
            }
        }

        Ok(restored_count)
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
}
