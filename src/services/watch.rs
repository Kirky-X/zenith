// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! File watching service for watch mode.
//! Uses the `notify` crate to monitor file system changes.

use crate::config::types::FormatResult;
use crate::services::formatter::ZenithService;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::future::Future;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Events from the file watcher
#[derive(Debug, Clone, PartialEq)]
pub enum WatchEvent {
    /// File was created
    Created(PathBuf),
    /// File was modified
    Modified(PathBuf),
    /// File was deleted
    Deleted(PathBuf),
}

/// Configuration for the file watcher
#[derive(Debug, Clone)]
pub struct WatchConfig {
    /// Paths to watch
    pub paths: Vec<PathBuf>,
    /// Debounce duration for rapid events
    pub debounce_duration: Duration,
    /// Whether to watch recursively
    pub recursive: bool,
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            paths: Vec::new(),
            debounce_duration: Duration::from_millis(100),
            recursive: true,
        }
    }
}

/// File watcher service that monitors file changes and triggers formatting
pub struct FileWatcher {
    config: WatchConfig,
    watcher: Option<RecommendedWatcher>,
    event_receiver: mpsc::Receiver<WatchEvent>,
    _watcher_task: JoinHandle<()>,
}

impl FileWatcher {
    /// Create a new file watcher with the given configuration
    pub fn new(config: WatchConfig, _service: Arc<ZenithService>) -> Result<Self, notify::Error> {
        let (event_sender, event_receiver) = mpsc::channel(100);

        // Create a debounced watcher
        let mut watcher = RecommendedWatcher::new(
            move |result: notify::Result<notify::Event>| {
                if let Ok(event) = result {
                    let event_type = match event.kind {
                        notify::EventKind::Create(_) => WatchEvent::Created,
                        notify::EventKind::Modify(_) => WatchEvent::Modified,
                        notify::EventKind::Remove(_) => WatchEvent::Deleted,
                        _ => WatchEvent::Modified,
                    };

                    for path in event.paths {
                        let sender = event_sender.clone();
                        let event_type = event_type.clone();
                        let path = path;
                        tokio::task::spawn_blocking(move || {
                            let event = event_type(path);
                            if let Err(e) = sender.blocking_send(event) {
                                tracing::warn!("Failed to send watch event: {}", e);
                            }
                        });
                    }
                }
            },
            notify::Config::default(),
        )?;

        // Add paths to watch
        for path in &config.paths {
            watcher.watch(
                path,
                if config.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                },
            )?;
        }

        // Spawn the watcher task
        let watcher_task = tokio::spawn(async move {
            // The watcher runs in a separate task
            // Events are sent through the channel
        });

        Ok(Self {
            config,
            watcher: Some(watcher),
            event_receiver,
            _watcher_task: watcher_task,
        })
    }

    /// Start watching files and processing events
    pub async fn start<F, Fut>(&mut self, mut process_fn: F)
    where
        F: FnMut(PathBuf) -> Fut + Send + 'static,
        Fut: Future<Output = FormatResult> + Send + 'static,
    {
        while let Some(event) = self.event_receiver.recv().await {
            match event {
                WatchEvent::Modified(path) | WatchEvent::Created(path) => {
                    tracing::info!("File changed: {:?}", path);
                    let _ = process_fn(path).await;
                }
                WatchEvent::Deleted(path) => {
                    tracing::info!("File deleted: {:?}", path);
                    // Handle deletion if needed
                }
            }
        }
    }

    /// Add a new path to watch
    pub fn add_path(&mut self, path: &Path) -> notify::Result<()> {
        if let Some(ref mut watcher) = self.watcher {
            watcher.watch(
                path,
                if self.config.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                },
            )?;
        }
        self.config.paths.push(path.to_path_buf());
        Ok(())
    }

    /// Remove a path from watching
    pub fn remove_path(&mut self, path: &Path) -> notify::Result<()> {
        if let Some(ref mut watcher) = self.watcher {
            watcher.unwatch(path)?;
        }
        self.config.paths.retain(|p| p != path);
        Ok(())
    }

    /// Get the number of watched paths
    pub fn watched_paths(&self) -> usize {
        self.config.paths.len()
    }
}

#[allow(dead_code)]
/// Builder for creating FileWatcher with fluent API
pub struct FileWatcherBuilder {
    config: WatchConfig,
}

#[allow(dead_code)]
impl FileWatcherBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: WatchConfig::default(),
        }
    }

    /// Add paths to watch
    pub fn with_paths(mut self, paths: Vec<PathBuf>) -> Self {
        self.config.paths = paths;
        self
    }

    /// Set debounce duration
    pub fn with_debounce_duration(mut self, duration: Duration) -> Self {
        self.config.debounce_duration = duration;
        self
    }

    /// Set recursive watching mode
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.config.recursive = recursive;
        self
    }

    /// Build the FileWatcher
    pub fn build(self, service: Arc<ZenithService>) -> Result<FileWatcher, notify::Error> {
        FileWatcher::new(self.config, service)
    }
}

#[allow(dead_code)]
impl Default for FileWatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_watch_config_default() {
        let config = WatchConfig::default();
        assert!(config.paths.is_empty());
        assert_eq!(config.debounce_duration, Duration::from_millis(100));
        assert!(config.recursive);
    }

    #[tokio::test]
    async fn test_watch_builder() {
        let builder = FileWatcherBuilder::new()
            .with_paths(vec![PathBuf::from("/test")])
            .with_recursive(false);

        assert_eq!(builder.config.paths.len(), 1);
        assert!(!builder.config.recursive);
    }

    #[tokio::test]
    async fn test_watch_event_types() {
        let created = WatchEvent::Created(PathBuf::from("new_file.rs"));
        let modified = WatchEvent::Modified(PathBuf::from("existing_file.rs"));
        let deleted = WatchEvent::Deleted(PathBuf::from("deleted_file.rs"));

        match created {
            WatchEvent::Created(path) => assert_eq!(path, PathBuf::from("new_file.rs")),
            _ => panic!("Expected Created event"),
        }

        match modified {
            WatchEvent::Modified(path) => assert_eq!(path, PathBuf::from("existing_file.rs")),
            _ => panic!("Expected Modified event"),
        }

        match deleted {
            WatchEvent::Deleted(path) => assert_eq!(path, PathBuf::from("deleted_file.rs")),
            _ => panic!("Expected Deleted event"),
        }
    }

    #[tokio::test]
    async fn test_watch_config_with_paths() {
        let mut config = WatchConfig::default();
        config.paths = vec![PathBuf::from("/test/path")];
        config.debounce_duration = Duration::from_millis(500);
        config.recursive = false;

        assert_eq!(config.paths.len(), 1);
        assert_eq!(config.debounce_duration, Duration::from_millis(500));
        assert!(!config.recursive);
    }

    #[tokio::test]
    async fn test_watch_config_empty_paths() {
        let config = WatchConfig {
            paths: Vec::new(),
            debounce_duration: Duration::from_millis(200),
            recursive: true,
        };

        assert!(config.paths.is_empty());
        assert_eq!(config.debounce_duration, Duration::from_millis(200));
        assert!(config.recursive);
    }

    #[tokio::test]
    async fn test_watch_config_max_debounce() {
        let config = WatchConfig {
            paths: vec![PathBuf::from("/test")],
            debounce_duration: Duration::from_secs(10),
            recursive: true,
        };

        assert_eq!(config.debounce_duration, Duration::from_secs(10));
    }

    #[test]
    fn test_watch_event_equality() {
        let event1 = WatchEvent::Created(PathBuf::from("test.rs"));
        let event2 = WatchEvent::Created(PathBuf::from("test.rs"));
        let event3 = WatchEvent::Modified(PathBuf::from("test.rs"));

        assert_eq!(event1, event2);
        assert_ne!(event1, event3);
    }

    #[test]
    fn test_watch_builder_default_values() {
        let builder = FileWatcherBuilder::new();
        assert!(builder.config.paths.is_empty());
        assert_eq!(builder.config.debounce_duration, Duration::from_millis(100));
        assert!(builder.config.recursive);
    }

    #[tokio::test]
    async fn test_watch_builder_with_multiple_paths() {
        let builder = FileWatcherBuilder::new()
            .with_paths(vec![
                PathBuf::from("/path/to/file1.rs"),
                PathBuf::from("/path/to/file2.rs"),
                PathBuf::from("/path/to/file3.rs"),
            ])
            .with_recursive(true)
            .with_debounce_duration(Duration::from_millis(300));

        assert_eq!(builder.config.paths.len(), 3);
        assert!(builder.config.recursive);
        assert_eq!(builder.config.debounce_duration, Duration::from_millis(300));
    }

    #[tokio::test]
    async fn test_watch_builder_chain() {
        let builder = FileWatcherBuilder::new()
            .with_paths(vec![PathBuf::from("/test")])
            .with_recursive(false)
            .with_debounce_duration(Duration::from_millis(50));

        assert!(!builder.config.recursive);
        assert_eq!(builder.config.debounce_duration, Duration::from_millis(50));
    }
}
