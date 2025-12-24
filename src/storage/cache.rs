use crate::error::Result;
use blake3::Hash;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct FileState {
    pub hash: Hash,
    pub modified: SystemTime,
    pub size: u64,
}

#[derive(Debug)]
pub struct HashCache {
    cache: Arc<RwLock<HashMap<PathBuf, FileState>>>,
}

impl HashCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 计算文件的哈希值和状态信息
    pub async fn compute_file_state(&self, path: &Path) -> Result<FileState> {
        use tokio::fs;

        let metadata = fs::metadata(path).await?;
        let content = fs::read(path).await?;
        let hash = blake3::hash(&content);

        Ok(FileState {
            hash,
            modified: metadata.modified()?,
            size: metadata.len(),
        })
    }

    /// 检查文件是否需要处理（哈希值或修改时间发生变化）
    pub async fn needs_processing(&self, path: &Path) -> Result<bool> {
        let current_state = self.compute_file_state(path).await?;
        let cache = self.cache.read().await;

        match cache.get(path) {
            Some(cached_state) => {
                // 检查哈希值、修改时间和文件大小
                Ok(cached_state.hash != current_state.hash
                    || cached_state.modified != current_state.modified
                    || cached_state.size != current_state.size)
            }
            None => Ok(true), // 文件不在缓存中，需要处理
        }
    }

    /// 更新文件的缓存状态
    pub async fn update(&self, path: PathBuf, state: FileState) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.insert(path, state);
        Ok(())
    }

    /// 从缓存中移除文件
    pub async fn remove(&self, path: &Path) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(path);
        Ok(())
    }

    /// 清空缓存
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    /// 获取缓存统计信息
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        CacheStats {
            entries: cache.len(),
        }
    }

    /// 批量检查文件是否需要处理
    pub async fn batch_needs_processing(&self, paths: &[PathBuf]) -> Result<Vec<bool>> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            results.push(self.needs_processing(path).await?);
        }

        Ok(results)
    }

    /// 批量更新缓存状态
    pub async fn batch_update(&self, updates: Vec<(PathBuf, FileState)>) -> Result<()> {
        let mut cache = self.cache.write().await;

        for (path, state) in updates {
            cache.insert(path, state);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
}

impl Default for HashCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use tokio::fs;

    #[tokio::test]
    async fn test_file_state_computation() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 写入测试内容
        fs::write(path, b"test content").await.unwrap();

        let state = cache.compute_file_state(path).await.unwrap();
        assert_eq!(state.size, 12); // "test content" 的长度
        assert!(state.hash != blake3::hash(b""));
    }

    #[tokio::test]
    async fn test_needs_processing_new_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();

        // 新文件应该需要处理
        assert!(cache.needs_processing(path).await.unwrap());
    }

    #[tokio::test]
    async fn test_needs_processing_unchanged_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        fs::write(&path, b"test content").await.unwrap();

        // 计算并缓存文件状态
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        // 文件未改变，不需要处理
        assert!(!cache.needs_processing(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_needs_processing_modified_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // 写入初始内容并缓存
        fs::write(&path, b"test content").await.unwrap();
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        // 修改文件内容
        fs::write(&path, b"modified content").await.unwrap();

        // 文件已改变，需要处理
        assert!(cache.needs_processing(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        fs::write(&path, b"test content").await.unwrap();
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path, state).await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 1);
    }
}
