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
    use std::time::Duration;

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
}
