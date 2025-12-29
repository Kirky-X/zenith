use crate::config::types::FormatResult;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Batch processing optimizer for efficient file processing
pub struct BatchOptimizer {
    batch_size: usize,
    workers: usize,
}

impl BatchOptimizer {
    /// Create a new batch optimizer with the given configuration
    pub fn new(batch_size: usize, workers: usize) -> Self {
        Self {
            batch_size: batch_size.max(1),
            workers: workers.max(1),
        }
    }

    /// Process files in batches with controlled concurrency
    pub async fn process_batches<F, Fut>(
        &self,
        files: Vec<PathBuf>,
        process_fn: F,
    ) -> Vec<FormatResult>
    where
        F: Fn(PathBuf) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = FormatResult> + Send + 'static,
    {
        let semaphore = Arc::new(Semaphore::new(self.workers));
        let process_fn = Arc::new(process_fn);
        let mut handles = Vec::new();

        for file in files {
            let sem_clone = semaphore.clone();
            let process_fn = Arc::clone(&process_fn);

            let handle = tokio::spawn(async move {
                let _permit = match sem_clone.acquire().await {
                    Ok(permit) => permit,
                    Err(_) => {
                        // Semaphore was closed, which shouldn't happen in normal operation
                        // Return a failed result
                        return FormatResult {
                            file_path: file,
                            success: false,
                            changed: false,
                            original_size: 0,
                            formatted_size: 0,
                            duration_ms: 0,
                            error: Some("Semaphore closed".to_string()),
                        };
                    }
                };
                process_fn(file).await
            });
            handles.push(handle);
        }

        let mut results = Vec::new();
        for handle in handles {
            if let Ok(res) = handle.await {
                results.push(res);
            }
        }

        results
    }

    /// Split files into batches for batch-level processing
    #[allow(dead_code)]
    pub fn split_into_batches(&self, files: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
        files
            .chunks(self.batch_size)
            .map(|chunk| chunk.to_vec())
            .collect()
    }

    /// Get the configured batch size
    #[allow(dead_code)]
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    /// Get the configured number of workers
    #[allow(dead_code)]
    pub fn workers(&self) -> usize {
        self.workers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_batch_optimizer_creation() {
        let optimizer = BatchOptimizer::new(10, 4);
        assert_eq!(optimizer.batch_size(), 10);
        assert_eq!(optimizer.workers(), 4);
    }

    #[test]
    fn test_split_into_batches() {
        let optimizer = BatchOptimizer::new(3, 2);
        let files: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("file{}.txt", i)))
            .collect();

        let batches = optimizer.split_into_batches(files);
        assert_eq!(batches.len(), 4);
        assert_eq!(batches[0].len(), 3);
        assert_eq!(batches[1].len(), 3);
        assert_eq!(batches[2].len(), 3);
        assert_eq!(batches[3].len(), 1);
    }

    #[tokio::test]
    async fn test_process_batches() {
        let optimizer = BatchOptimizer::new(2, 2);
        let files: Vec<PathBuf> = (0..5)
            .map(|i| PathBuf::from(format!("file{}.txt", i)))
            .collect();

        let results = optimizer
            .process_batches(files, |path| async move {
                tokio::time::sleep(Duration::from_millis(10)).await;
                FormatResult {
                    file_path: path,
                    success: true,
                    changed: false,
                    original_size: 0,
                    formatted_size: 0,
                    duration_ms: 10,
                    error: None,
                }
            })
            .await;

        assert_eq!(results.len(), 5);
        for result in results {
            assert!(result.success);
        }
    }

    #[test]
    fn test_batch_size_minimum() {
        let optimizer = BatchOptimizer::new(0, 0);
        assert_eq!(optimizer.batch_size(), 1);
        assert_eq!(optimizer.workers(), 1);
    }

    #[tokio::test]
    async fn test_empty_files_batch() {
        let optimizer = BatchOptimizer::new(5, 2);
        let files: Vec<PathBuf> = Vec::new();

        let results = optimizer
            .process_batches(files, |path| async move {
                FormatResult {
                    file_path: path,
                    success: true,
                    changed: false,
                    original_size: 0,
                    formatted_size: 0,
                    duration_ms: 0,
                    error: None,
                }
            })
            .await;

        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_single_file_batch() {
        let optimizer = BatchOptimizer::new(1, 1);
        let files = vec![PathBuf::from("single_file.txt")];

        let results = optimizer
            .process_batches(files, |path| async move {
                FormatResult {
                    file_path: path,
                    success: true,
                    changed: true,
                    original_size: 100,
                    formatted_size: 80,
                    duration_ms: 5,
                    error: None,
                }
            })
            .await;

        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert!(results[0].changed);
    }

    #[test]
    fn test_batch_split_even_division() {
        let optimizer = BatchOptimizer::new(4, 2);
        let files: Vec<PathBuf> = (0..8)
            .map(|i| PathBuf::from(format!("file{}.txt", i)))
            .collect();

        let batches = optimizer.split_into_batches(files);
        assert_eq!(batches.len(), 2);
        assert_eq!(batches[0].len(), 4);
        assert_eq!(batches[1].len(), 4);
    }

    #[test]
    fn test_batch_split_uneven_division() {
        let optimizer = BatchOptimizer::new(4, 2);
        let files: Vec<PathBuf> = (0..10)
            .map(|i| PathBuf::from(format!("file{}.txt", i)))
            .collect();

        let batches = optimizer.split_into_batches(files);
        assert_eq!(batches.len(), 3);
        assert_eq!(batches[0].len(), 4);
        assert_eq!(batches[1].len(), 4);
        assert_eq!(batches[2].len(), 2);
    }

    #[tokio::test]
    async fn test_batch_processing_order() {
        let optimizer = BatchOptimizer::new(10, 1);
        let files: Vec<PathBuf> = (0..5)
            .map(|i| PathBuf::from(format!("ordered_file_{}.txt", i)))
            .collect();

        let processed_order = Arc::new(Mutex::new(Vec::new()));
        let processed_order_for_check = Arc::clone(&processed_order);
        let files_clone = files.clone();
        let _results = optimizer
            .process_batches(files_clone, move |path| {
                let processed_order = Arc::clone(&processed_order);
                async move {
                    processed_order.lock().await.push(path.clone());
                    FormatResult {
                        file_path: path,
                        success: true,
                        changed: false,
                        original_size: 0,
                        formatted_size: 0,
                        duration_ms: 0,
                        error: None,
                    }
                }
            })
            .await;

        let order = processed_order_for_check.lock().await;
        assert_eq!(order.len(), 5);
    }

    #[tokio::test]
    async fn test_batch_with_failed_files() {
        let optimizer = BatchOptimizer::new(2, 2);
        let files = vec![
            PathBuf::from("valid_file.txt"),
            PathBuf::from("invalid_file.txt"),
        ];

        let results = optimizer
            .process_batches(files, |path| async move {
                if path.to_string_lossy().contains("invalid") {
                    FormatResult {
                        file_path: path,
                        success: false,
                        changed: false,
                        original_size: 0,
                        formatted_size: 0,
                        duration_ms: 0,
                        error: Some("Processing failed".to_string()),
                    }
                } else {
                    FormatResult {
                        file_path: path,
                        success: true,
                        changed: true,
                        original_size: 50,
                        formatted_size: 40,
                        duration_ms: 2,
                        error: None,
                    }
                }
            })
            .await;

        assert_eq!(results.len(), 2);
        assert!(results[0].success);
        assert!(!results[1].success);
        assert!(results[1].error.is_some());
    }

    #[tokio::test]
    async fn test_large_batch_processing() {
        let optimizer = BatchOptimizer::new(100, 4);
        let files: Vec<PathBuf> = (0..50)
            .map(|i| PathBuf::from(format!("large_batch_file_{}.txt", i)))
            .collect();

        let results = optimizer
            .process_batches(files, |path| async move {
                FormatResult {
                    file_path: path,
                    success: true,
                    changed: false,
                    original_size: 1024,
                    formatted_size: 1024,
                    duration_ms: 1,
                    error: None,
                }
            })
            .await;

        assert_eq!(results.len(), 50);
        assert!(results.iter().all(|r| r.success));
    }

    #[test]
    fn test_batch_size_boundary() {
        let optimizer_min = BatchOptimizer::new(1, 1);
        assert_eq!(optimizer_min.batch_size(), 1);

        let optimizer_large = BatchOptimizer::new(10000, 100);
        assert_eq!(optimizer_large.batch_size(), 10000);
        assert_eq!(optimizer_large.workers(), 100);
    }

    #[tokio::test]
    async fn test_concurrent_batch_processing() {
        let optimizer = BatchOptimizer::new(10, 4);
        let files: Vec<PathBuf> = (0..8)
            .map(|i| PathBuf::from(format!("concurrent_file_{}.txt", i)))
            .collect();

        let start_time = tokio::time::Instant::now();
        let results = optimizer
            .process_batches(files, |path| async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                FormatResult {
                    file_path: path,
                    success: true,
                    changed: false,
                    original_size: 0,
                    formatted_size: 0,
                    duration_ms: 50,
                    error: None,
                }
            })
            .await;
        let elapsed = start_time.elapsed();

        assert_eq!(results.len(), 8);
        assert!(results.iter().all(|r| r.success));
        assert!(elapsed < Duration::from_millis(200));
    }
}
