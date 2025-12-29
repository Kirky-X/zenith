// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::error::Result;
use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::fs::{self, File};
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::RwLock;

/// Represents the state of a file including content hash and metadata
#[derive(Debug, Clone)]
pub struct FileState {
    pub hash: Hash,
    pub modified: SystemTime,
    pub size: u64,
    /// Config hash for config-aware caching
    pub config_hash: Option<Hash>,
    /// Timestamp when this entry was added to cache
    pub cached_at: SystemTime,
}

impl FileState {
    pub fn new(hash: Hash, modified: SystemTime, size: u64) -> Self {
        Self {
            hash,
            modified,
            size,
            config_hash: None,
            cached_at: SystemTime::now(),
        }
    }

    pub fn with_config(hash: Hash, modified: SystemTime, size: u64, config: &ZenithConfig) -> Self {
        let config_str = serde_json::to_string(config).unwrap_or_default();
        let config_hash = blake3::hash(config_str.as_bytes());
        Self {
            hash,
            modified,
            size,
            config_hash: Some(config_hash),
            cached_at: SystemTime::now(),
        }
    }

    /// Check if this cache entry is expired
    pub fn is_expired(&self, max_age: Duration) -> bool {
        if let Ok(age) = SystemTime::now().duration_since(self.cached_at) {
            age > max_age
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedFileState {
    hash: String,
    modified_secs: u64,
    modified_nanos: u32,
    size: u64,
    config_hash: Option<String>,
    cached_at_secs: u64,
    cached_at_nanos: u32,
}

impl SerializedFileState {
    pub fn from_file_state(state: &FileState) -> Self {
        let duration = state
            .modified
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let cached_duration = state
            .cached_at
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Self {
            hash: format!("{}", state.hash),
            modified_secs: duration.as_secs(),
            modified_nanos: duration.subsec_nanos(),
            size: state.size,
            config_hash: state.config_hash.as_ref().map(|h| format!("{}", h)),
            cached_at_secs: cached_duration.as_secs(),
            cached_at_nanos: cached_duration.subsec_nanos(),
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
        let cached_at = SystemTime::UNIX_EPOCH
            + std::time::Duration::new(self.cached_at_secs, self.cached_at_nanos);

        let config_hash = self
            .config_hash
            .as_ref()
            .map(|h| h.parse().ok().unwrap_or_else(|| blake3::hash(&[])));

        Ok(FileState {
            hash,
            modified,
            size: self.size,
            config_hash,
            cached_at,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializedCache {
    version: u32,
    entries: Vec<(String, SerializedFileState)>,
}

impl SerializedCache {
    pub fn version() -> u32 {
        2 // Incremented for config-aware caching
    }
}

/// Enhanced hash-based content cache for incremental processing optimization.
#[derive(Debug)]
pub struct HashCache {
    cache: Arc<RwLock<HashMap<PathBuf, FileState>>>,
    cache_dir: Option<PathBuf>,
    /// Maximum age for cache entries before they're considered stale
    max_entry_age: Duration,
    /// Enable config-aware caching
    config_aware: bool,
}

impl HashCache {
    /// Create a new cache with default settings
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: None,
            max_entry_age: Duration::from_secs(24 * 60 * 60), // 24 hours default
            config_aware: false,
        }
    }

    /// Create a cache with a persistence directory
    pub fn with_cache_dir(cache_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_dir).ok();
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_dir: Some(cache_dir),
            max_entry_age: Duration::from_secs(24 * 60 * 60),
            config_aware: false,
        }
    }

    /// Enable config-aware caching
    pub fn with_config_aware(mut self, enabled: bool) -> Self {
        self.config_aware = enabled;
        self
    }

    /// Set maximum entry age for cache validation
    pub fn with_max_entry_age(mut self, age: Duration) -> Self {
        self.max_entry_age = age;
        self
    }

    pub fn cache_dir(&self) -> Option<&Path> {
        self.cache_dir.as_deref()
    }

    /// Serialize a path to a string for storage
    fn serialize_path(path: &Path) -> String {
        path.to_string_lossy().into_owned()
    }

    /// Deserialize a stored string to PathBuf
    fn deserialize_path(s: &str) -> PathBuf {
        PathBuf::from(s)
    }

    /// Save the cache to disk
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

    /// Load the cache from disk
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

        // Only load if version matches
        if serialized.version != SerializedCache::version() {
            tracing::info!(
                "Cache version mismatch: expected {}, got {}, skipping load",
                SerializedCache::version(),
                serialized.version
            );
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

    /// Asynchronously save cache to disk in background
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

    /// Compute the hash and state information for a file
    pub async fn compute_file_state(&self, path: &Path) -> Result<FileState> {
        use tokio::fs;

        let metadata = fs::metadata(path).await?;
        let content = fs::read(path).await?;
        let hash = blake3::hash(&content);

        Ok(FileState {
            hash,
            modified: metadata.modified()?,
            size: metadata.len(),
            config_hash: None,
            cached_at: SystemTime::now(),
        })
    }

    /// Compute file state with config hash for config-aware caching
    pub async fn compute_file_state_with_config(
        &self,
        path: &Path,
        config: &ZenithConfig,
    ) -> Result<FileState> {
        use tokio::fs;

        let metadata = fs::metadata(path).await?;
        let content = fs::read(path).await?;
        let hash = blake3::hash(&content);

        Ok(FileState::with_config(
            hash,
            metadata.modified()?,
            metadata.len(),
            config,
        ))
    }

    /// Check if a file needs processing
    pub async fn needs_processing(&self, path: &Path) -> Result<bool> {
        self.needs_processing_with_config(path, None).await
    }

    /// Check if a file needs processing with optional config awareness
    pub async fn needs_processing_with_config(
        &self,
        path: &Path,
        config: Option<&ZenithConfig>,
    ) -> Result<bool> {
        let current_state = if let Some(config) = config {
            self.compute_file_state_with_config(path, config).await?
        } else {
            self.compute_file_state(path).await?
        };

        let cache = self.cache.read().await;

        match cache.get(path) {
            Some(cached_state) => {
                if cached_state.is_expired(self.max_entry_age) {
                    tracing::debug!("Cache entry expired for {:?}", path);
                    return Ok(true);
                }

                let hash_changed = cached_state.hash != current_state.hash;

                let config_changed = if let Some(config) = config {
                    let current_config_hash =
                        blake3::hash(serde_json::to_string(config).unwrap_or_default().as_bytes());
                    cached_state.config_hash != Some(current_config_hash)
                } else {
                    false
                };

                tracing::debug!(
                    "Cache comparison for {:?}: hash_changed={}, config_changed={}",
                    path,
                    hash_changed,
                    config_changed
                );

                Ok(hash_changed || config_changed)
            }
            None => {
                tracing::debug!("File {:?} not in cache, needs processing", path);
                Ok(true) // File not in cache, needs processing
            }
        }
    }

    /// Update the cache for a file
    pub async fn update(&self, path: PathBuf, state: FileState) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.insert(path, state);
        Ok(())
    }

    /// Update cache with config awareness
    pub async fn update_with_config(&self, path: PathBuf, config: &ZenithConfig) -> Result<()> {
        let state = self.compute_file_state_with_config(&path, config).await?;
        self.update(path, state).await
    }

    /// Remove a file from the cache
    pub async fn remove(&self, path: &Path) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.remove(path);
        Ok(())
    }

    /// Clear the entire cache
    pub async fn clear(&self) -> Result<()> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let now = SystemTime::now();

        let mut expired_count = 0;
        let mut total_age = Duration::ZERO;
        let mut valid_count = 0;

        for state in cache.values() {
            if state.is_expired(self.max_entry_age) {
                expired_count += 1;
            } else {
                if let Ok(age) = now.duration_since(state.cached_at) {
                    total_age += age;
                }
                valid_count += 1;
            }
        }

        CacheStats {
            entries: cache.len(),
            expired_entries: expired_count,
            valid_entries: valid_count,
            average_age: if valid_count > 0 {
                Some(total_age / valid_count as u32)
            } else {
                None
            },
        }
    }

    /// Clean up expired cache entries
    pub async fn cleanup(&self) -> Result<usize> {
        let mut cache = self.cache.write().await;
        let now = SystemTime::now();
        let mut removed = 0;

        let keys_to_remove: Vec<PathBuf> = cache
            .iter()
            .filter(|(_, state)| {
                if let Ok(age) = now.duration_since(state.cached_at) {
                    age > self.max_entry_age
                } else {
                    false
                }
            })
            .map(|(path, _)| path.clone())
            .collect();

        for key in keys_to_remove {
            cache.remove(&key);
            removed += 1;
        }

        Ok(removed)
    }

    /// Batch check files for processing needs
    pub async fn batch_needs_processing(&self, paths: &[PathBuf]) -> Result<Vec<bool>> {
        let mut results = Vec::with_capacity(paths.len());

        for path in paths {
            results.push(self.needs_processing(path).await?);
        }

        Ok(results)
    }

    /// Batch update cache entries
    pub async fn batch_update(&self, updates: Vec<(PathBuf, FileState)>) -> Result<()> {
        let mut cache = self.cache.write().await;

        for (path, state) in updates {
            cache.insert(path, state);
        }

        Ok(())
    }

    /// Invalidate cache for files that match the given predicate
    pub async fn invalidate_matching<F>(&self, predicate: F) -> Result<usize>
    where
        F: Fn(&PathBuf) -> bool,
    {
        let mut cache = self.cache.write().await;
        let mut removed = 0;

        let keys_to_remove: Vec<PathBuf> = cache
            .keys()
            .filter(|path| predicate(path))
            .cloned()
            .collect();

        for key in keys_to_remove {
            cache.remove(&key);
            removed += 1;
        }

        Ok(removed)
    }

    /// Check if a file is in the cache
    pub async fn is_cached(&self, path: &Path) -> bool {
        let cache = self.cache.read().await;
        cache.contains_key(path)
    }

    /// Get cached state for a file
    pub async fn get_cached_state(&self, path: &Path) -> Option<FileState> {
        let cache = self.cache.read().await;
        cache.get(path).cloned()
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub entries: usize,
    pub expired_entries: usize,
    pub valid_entries: usize,
    pub average_age: Option<Duration>,
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

        // Write test content
        fs::write(path, b"test content").await.unwrap();

        let state = cache.compute_file_state(path).await.unwrap();
        assert_eq!(state.size, 12); // "test content" length
        assert!(state.hash != blake3::hash(b""));
    }

    #[tokio::test]
    async fn test_needs_processing_new_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();

        // New file needs processing
        assert!(cache.needs_processing(path).await.unwrap());
    }

    #[tokio::test]
    async fn test_needs_processing_unchanged_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        fs::write(&path, b"test content").await.unwrap();

        // Compute and cache file state
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        // File unchanged, no processing needed
        assert!(!cache.needs_processing(&path).await.unwrap());
    }

    #[tokio::test]
    async fn test_needs_processing_modified_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path().to_path_buf();

        // Write initial content and cache
        fs::write(&path, b"test content").await.unwrap();
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        // Modify file content
        fs::write(&path, b"modified content").await.unwrap();

        // File changed, needs processing
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
        assert_eq!(stats.expired_entries, 0);
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
    async fn test_cache_cleanup() {
        let temp_dir = tempfile::tempdir().unwrap();
        let cache_dir = temp_dir.path().to_path_buf();

        // Create cache with very short max entry age
        let cache = HashCache::with_cache_dir(cache_dir.clone())
            .with_max_entry_age(Duration::from_millis(1));

        let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
        let path = temp_file.path().to_path_buf();

        fs::write(&path, b"test content").await.unwrap();
        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path.clone(), state).await.unwrap();

        // Wait for entry to expire
        tokio::time::sleep(Duration::from_millis(10)).await;

        // Cleanup should remove the expired entry
        let removed = cache.cleanup().await.unwrap();
        assert_eq!(removed, 1);

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 0);
    }

    #[tokio::test]
    async fn test_config_aware_caching() {
        let cache = HashCache::new().with_config_aware(true);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();

        // Create two different configs
        let config1 = ZenithConfig::default();
        let config2 = ZenithConfig {
            custom_config_path: Some(PathBuf::from("/different/path")),
            ..Default::default()
        };

        // File should need processing with first config
        assert!(cache
            .needs_processing_with_config(path, Some(&config1))
            .await
            .unwrap());

        // Compute and cache with config1
        cache
            .update_with_config(path.to_path_buf(), &config1)
            .await
            .unwrap();

        // Same config, no change - should not need processing
        assert!(!cache
            .needs_processing_with_config(path, Some(&config1))
            .await
            .unwrap());

        // Different config - should need processing
        assert!(cache
            .needs_processing_with_config(path, Some(&config2))
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_invalidate_matching() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();

        // Create multiple test files with predictable names
        for i in 0..5 {
            let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
            let path = temp_file.path().to_path_buf();
            // Rename to include the number in a predictable way
            let new_path = temp_dir.path().join(format!("test_file_{}.txt", i));
            tokio::fs::rename(&path, &new_path).await.unwrap();
            fs::write(&new_path, format!("test content {}", i).as_bytes())
                .await
                .unwrap();
            let state = cache.compute_file_state(&new_path).await.unwrap();
            cache.update(new_path, state).await.unwrap();
        }

        // Invalidate files with "_0" in the name (should match test_file_0.txt)
        let removed = cache
            .invalidate_matching(|p| p.to_string_lossy().contains("_0"))
            .await
            .unwrap();
        assert_eq!(removed, 1);

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 4);
    }

    #[tokio::test]
    async fn test_is_cached() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        assert!(!cache.is_cached(path).await);

        fs::write(path, b"test content").await.unwrap();
        let state = cache.compute_file_state(path).await.unwrap();
        cache.update(path.to_path_buf(), state).await.unwrap();

        assert!(cache.is_cached(path).await);
    }

    #[tokio::test]
    async fn test_empty_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"").await.unwrap();

        let state = cache.compute_file_state(path).await.unwrap();
        assert_eq!(state.size, 0);
        assert_ne!(state.hash, blake3::hash(b"test"));
    }

    #[tokio::test]
    async fn test_large_file() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let large_content = vec![b'x'; 1024 * 1024];
        fs::write(path, &large_content).await.unwrap();

        let state = cache.compute_file_state(path).await.unwrap();
        assert_eq!(state.size, 1024 * 1024);
    }

    #[tokio::test]
    async fn test_batch_needs_processing() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let mut temp_files = Vec::new();
        let mut paths = Vec::new();

        for i in 0..3 {
            let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
            let path = temp_file.path().to_path_buf();
            fs::write(&path, format!("content {}", i).as_bytes())
                .await
                .unwrap();
            temp_files.push(temp_file);
            paths.push(path);
        }

        let results = cache.batch_needs_processing(&paths).await.unwrap();
        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|b| *b));
    }

    #[tokio::test]
    async fn test_batch_update() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let mut updates = Vec::new();

        for i in 0..3 {
            let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
            let path = temp_file.path().to_path_buf();
            fs::write(&path, format!("content {}", i).as_bytes())
                .await
                .unwrap();
            let state = cache.compute_file_state(&path).await.unwrap();
            updates.push((path, state));
        }

        cache.batch_update(updates).await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 3);
    }

    #[tokio::test]
    async fn test_remove_from_cache() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();
        let state = cache.compute_file_state(path).await.unwrap();
        cache.update(path.to_path_buf(), state).await.unwrap();

        assert!(cache.is_cached(path).await);

        cache.remove(path).await.unwrap();

        assert!(!cache.is_cached(path).await);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();

        for i in 0..5 {
            let temp_file = NamedTempFile::new_in(&temp_dir).unwrap();
            let path = temp_file.path().to_path_buf();
            fs::write(&path, format!("content {}", i).as_bytes())
                .await
                .unwrap();
            let state = cache.compute_file_state(&path).await.unwrap();
            cache.update(path, state).await.unwrap();
        }

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 5);

        cache.clear().await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 0);
    }

    #[tokio::test]
    async fn test_get_cached_state() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();
        let state = cache.compute_file_state(path).await.unwrap();
        cache
            .update(path.to_path_buf(), state.clone())
            .await
            .unwrap();

        let retrieved = cache.get_cached_state(path).await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().hash, state.hash);
    }

    #[tokio::test]
    async fn test_nonexistent_path_needs_processing() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent/file.txt");

        let result = cache.needs_processing(&nonexistent).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cache_with_special_characters_in_path() {
        let cache = HashCache::new();
        let temp_dir = tempfile::tempdir().unwrap();

        let special_name = "file-with-dashes_and_underscores.123.txt";
        let path = temp_dir.path().join(special_name);

        fs::write(&path, b"special content").await.unwrap();

        let state = cache.compute_file_state(&path).await.unwrap();
        cache.update(path, state).await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 1);
    }

    #[tokio::test]
    async fn test_config_aware_cache_with_none_config() {
        let cache = HashCache::new().with_config_aware(true);
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();

        assert!(cache
            .needs_processing_with_config(path, None)
            .await
            .unwrap());

        let state = cache.compute_file_state(path).await.unwrap();
        cache.update(path.to_path_buf(), state).await.unwrap();

        assert!(!cache
            .needs_processing_with_config(path, None)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_cache_without_persistence_dir() {
        let cache = HashCache::new();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        fs::write(path, b"test content").await.unwrap();

        let state = cache.compute_file_state(path).await.unwrap();
        cache.update(path.to_path_buf(), state).await.unwrap();

        cache.save().await.unwrap();

        let stats = cache.stats().await;
        assert_eq!(stats.entries, 1);
    }

    #[tokio::test]
    async fn test_file_state_with_config() {
        let config = ZenithConfig {
            custom_config_path: Some(PathBuf::from("/test/config")),
            ..Default::default()
        };

        let hash = blake3::hash(b"content");
        let modified = SystemTime::now();
        let size = 100;

        let state = FileState::with_config(hash, modified, size, &config);

        assert!(state.config_hash.is_some());
        assert_ne!(state.config_hash, Some(hash));
    }
}
