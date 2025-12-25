use crate::error::Result;
use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct FileState {
    pub hash: Hash,
    pub modified: SystemTime,
    pub size: u64,
}

impl FileState {
    pub fn new(hash: Hash, modified: SystemTime, size: u64) -> Self {
        Self {
            hash,
            modified,
            size,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedFileState {
    hash: String,
    modified_secs: u64,
    modified_nanos: u32,
    size: u64,
}

impl SerializedFileState {
    pub fn from_file_state(state: &FileState) -> Self {
        let duration = state
            .modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Self {
            hash: format!("{}", state.hash),
            modified_secs: duration.as_secs(),
            modified_nanos: duration.subsec_nanos(),
            size: state.size,
        }
    }

    pub fn to_file_state(&self) -> Result<FileState> {
        let hash = self.hash.parse().map_err(|e| {
            let io_error = std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Invalid hash format: {}", e),
            );
            crate::error::ZenithError::from(serde_json::Error::io(io_error))
        })?;
        let modified = SystemTime::UNIX_EPOCH
            + std::time::Duration::new(self.modified_secs, self.modified_nanos);
        Ok(FileState::new(hash, modified, self.size))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedCache {
    version: u32,
    entries: Vec<(String, SerializedFileState)>,
}

impl SerializedCache {
    pub fn version() -> u32 {
        1
    }
}

#[derive(Debug)]
pub struct HashCache {
    cache: Arc<RwLock<HashMap<PathBuf, FileState>>>,
    cache_dir: Option<PathBuf>,
}

impl HashCache {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: None,
        }
    }

    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_dir).ok();
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: Some(cache_dir),
        }
    }

    pub fn cache_dir(&self) -> Option<&Path> {
        self.cache_dir.as_deref()
    }

    /// 将 PathBuf 序列化为字符串用于存储
    fn serialize_path(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }

    /// 将存储的字符串反序列化为 PathBuf
    fn deserialize_path(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    /// 将缓存保存到磁盘
    pub async fn save(&self) -> Result<()> {
        let cache_dir = if let Some(dir) = &self.cache_dir {
            dir
        } else {
            return Ok(());
        };

        let cache_file = cache_dir.join("file_cache.json");
        let cache = self.cache.read().await;

        let entries: Vec<(String, SerializedFileState)> = cache
            .iter()
            .map(|(path, state)| {
                (
                    Self::serialize_path(path),
                    SerializedFileState::from_file_state(state),
                )
            })
            .collect();

        let serialized = SerializedCache {
            version: SerializedCache::version(),
            entries,
        };

        let json = serde_json::to_string_pretty(&serialized)
            .map_err(crate::error::ZenithError::Serialization)?;

        let file = File::create(&cache_file).await?;
        let mut writer = BufWriter::new(file);
        writer.write_all(json.as_bytes()).await?;
        writer.flush().await?;

        Ok(())
    }

    /// 从磁盘加载缓存
    pub async fn load(&mut self) -> Result<()> {
        let cache_dir = if let Some(dir) = &self.cache_dir {
            dir
        } else {
            return Ok(());
        };

        let cache_file = cache_dir.join("file_cache.json");
        if !cache_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&cache_file).await?;
        let serialized: SerializedCache =
            serde_json::from_str(&content).map_err(crate::error::ZenithError::Serialization)?;

        if serialized.version != SerializedCache::version() {
            return Ok(());
        }

        let mut cache = self.cache.write().await;
        for (path_str, state) in serialized.entries {
            let path = Self::deserialize_path(&path_str);
            let file_state = state.to_file_state()?;
            cache.insert(path, file_state);
        }

        Ok(())
    }

    /// 异步保存缓存到磁盘
    pub fn save_background(&self) {
        let cache_dir = if let Some(dir) = &self.cache_dir {
            dir.clone()
        } else {
            return;
        };

        let cache_arc = Arc::clone(&self.cache);
        tokio::spawn(async move {
            let cache = cache_arc.read().await;
            let entries: Vec<(String, SerializedFileState)> = cache
                .iter()
                .map(|(path, state)| {
                    (
                        HashCache::serialize_path(path),
                        SerializedFileState::from_file_state(state),
                    )
                })
                .collect();

            drop(cache);

            let serialized = SerializedCache {
                version: SerializedCache::version(),
                entries,
            };

            if let Ok(json) = serde_json::to_string_pretty(&serialized) {
                let cache_file = cache_dir.join("file_cache.json");
                if let Ok(file) = File::create(&cache_file).await {
                    let mut writer = BufWriter::new(file);
                    if writer.write_all(json.as_bytes()).await.is_ok() {
                        let _ = writer.flush().await;
                    }
                }
            }
        });
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

    #[tokio::test]
    async fn test_cache_save_and_load() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let cache = HashCache::with_cache_dir(cache_dir.clone());
        let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        let path = temp_file.path().to_path_buf();

        fs::write(&path, b"test content for persistence")
            .await
            .unwrap();
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        cache.save().await.unwrap();

        let mut new_cache = HashCache::with_cache_dir(cache_dir);
        new_cache.load().await.unwrap();

        let stats = new_cache.stats().await;
        assert_eq!(stats.entries, 1);

        assert!(!new_cache.needs_processing(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_cache_load_nonexistent_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let mut cache = HashCache::with_cache_dir(cache_dir);
        cache.load().await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 0);
    }

    #[tokio::test]
    async fn test_cache_persistence_multiple_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        let cache = HashCache::with_cache_dir(cache_dir.clone());

        for i in 0..3 {
            let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
            let path = temp_file.path().to_path_buf();
            let content = format!("test content {}", i);

            fs::write(&path, content.as_bytes()).await.unwrap();
            let state = cache.compute_file_state(&path).await.unwrap();
            cache.update(path.clone(), state).await.unwrap();
        }

        cache.save().await.unwrap();

        let mut new_cache = HashCache::with_cache_dir(cache_dir);
        new_cache.load().await.unwrap();

        let stats = new_cache.stats().await;
        assert_eq!(stats.entries, 3);
    }

    #[tokio::test]
    async fn test_cache_with_cache_dir() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        let cache = HashCache::with_cache_dir(cache_dir.clone());

        assert!(cache_dir.exists());
        assert_eq!(cache.cache_dir(), Some(cache_dir.as_path()));
    }

    #[tokio::test]
    async fn test_cache_without_cache_dir() {
        let cache = HashCache::new();
        assert_eq!(cache.cache_dir(), None);
    }

    #[tokio::test]
    async fn test_serialized_file_state_roundtrip() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, b"test content").await.unwrap();

        let cache = HashCache::new();
        let original_state = cache.compute_file_state(path).await.unwrap();

        let serialized = SerializedFileState::from_file_state(&original_state);
        let deserialized = serialized.to_file_state().unwrap();

        assert_eq!(original_state.hash, deserialized.hash);
        assert_eq!(original_state.size, deserialized.size);
    }
}
