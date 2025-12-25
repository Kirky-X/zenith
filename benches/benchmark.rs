// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Benchmark tests using Criterion
//! Evaluates performance of core components

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use tempfile::TempDir;
use zenith::storage::cache::HashCache;
use zenith::zeniths::registry::ZenithRegistry;

fn bench_hash_cache_compute_state(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let cache = HashCache::new();

    let mut group = c.benchmark_group("hash_cache_compute");

    for size in [1024, 10_240, 102_400, 1_048_576].iter() {
        let test_file = temp_dir.path().join(format!("test_{}.txt", size));
        let content = vec![b'a'; *size];
        std::fs::write(&test_file, content).unwrap();

        let rt = tokio::runtime::Runtime::new().unwrap();
        let cache_ref = &cache;
        let test_file_ref = test_file.clone();

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| rt.block_on(cache_ref.compute_file_state(black_box(&test_file_ref))));
        });
    }
    group.finish();
}

fn bench_hash_cache_needs_processing(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let cache = HashCache::new();

    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, vec![b'a'; 10_240]).unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let state = cache.compute_file_state(&test_file).await.unwrap();
        cache.update(test_file.clone(), state).await.unwrap();
    });

    let cache_ref = &cache;
    let test_file_ref = test_file.clone();

    c.bench_function("hash_cache_needs_processing", |b| {
        b.iter(|| rt.block_on(cache_ref.needs_processing(black_box(&test_file_ref))));
    });
}

fn bench_hash_cache_batch_operations(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let cache = HashCache::new();

    let mut paths = Vec::new();
    for i in 0..100 {
        let test_file = temp_dir.path().join(format!("test_{}.txt", i));
        std::fs::write(&test_file, vec![b'a'; 1024]).unwrap();
        paths.push(test_file);
    }

    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache_ref = &cache;
    let paths_ref = paths.clone();

    c.bench_function("hash_cache_batch_check_100_files", |b| {
        b.iter(|| rt.block_on(cache_ref.batch_needs_processing(black_box(&paths_ref))));
    });
}

fn bench_registry_lookup(c: &mut Criterion) {
    let registry = Arc::new(ZenithRegistry::new());

    static RUST_EXTENSIONS: &[&str] = &["rs", "rust"];
    let mock_formatter = Arc::new(MockZenith::new("rust", RUST_EXTENSIONS));
    registry.register(mock_formatter);

    let mut group = c.benchmark_group("registry_lookup");

    group.bench_function("existing_extension", |b| {
        b.iter(|| registry.get_by_extension(black_box("rs")));
    });

    group.bench_function("nonexistent_extension", |b| {
        b.iter(|| registry.get_by_extension(black_box("xyz")));
    });

    group.finish();
}

fn bench_registry_list_all(c: &mut Criterion) {
    let registry = Arc::new(ZenithRegistry::new());

    static EXTENSIONS: [[&str; 1]; 10] = [
        ["ext_0"],
        ["ext_1"],
        ["ext_2"],
        ["ext_3"],
        ["ext_4"],
        ["ext_5"],
        ["ext_6"],
        ["ext_7"],
        ["ext_8"],
        ["ext_9"],
    ];

    for (i, ext) in EXTENSIONS.iter().enumerate() {
        let formatter = Arc::new(MockZenith::new(&format!("formatter_{}", i), ext));
        registry.register(formatter);
    }

    c.bench_function("registry_list_all_10_formatters", |b| {
        b.iter(|| registry.list_all());
    });
}

fn bench_path_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_validation");

    group.bench_function("safe_path", |b| {
        b.iter(|| {
            zenith::utils::path::validate_path(black_box(std::path::Path::new(
                "/home/user/project/src/main.rs",
            )))
        });
    });

    group.bench_function("path_with_parent_dir", |b| {
        b.iter(|| {
            zenith::utils::path::validate_path(black_box(std::path::Path::new(
                "/home/user/project/../etc/passwd",
            )))
        });
    });

    group.finish();
}

fn bench_is_hidden(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    let hidden_file = temp_dir.path().join(".hidden");
    std::fs::write(&hidden_file, "content").unwrap();

    let normal_file = temp_dir.path().join("normal.txt");
    std::fs::write(&normal_file, "content").unwrap();

    let hidden_entry = walkdir::WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == ".hidden")
        .unwrap();

    let normal_entry = walkdir::WalkDir::new(temp_dir.path())
        .into_iter()
        .filter_map(|e| e.ok())
        .find(|e| e.file_name() == "normal.txt")
        .unwrap();

    let mut group = c.benchmark_group("is_hidden_check");

    group.bench_function("hidden_file", |b| {
        b.iter(|| zenith::utils::path::is_hidden(black_box(&hidden_entry)));
    });

    group.bench_function("normal_file", |b| {
        b.iter(|| zenith::utils::path::is_hidden(black_box(&normal_entry)));
    });

    group.finish();
}

fn bench_config_cache_operations(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let app_config = zenith::config::types::AppConfig::default();

    let test_file = temp_dir.path().join("test.txt");
    std::fs::write(&test_file, "test").unwrap();

    c.bench_function("config_cache_get_config", |b| {
        let mut cache = zenith::config::cache::ConfigCache::new();
        b.iter(|| cache.get_config_for_file(black_box(&app_config), black_box(&test_file)));
    });
}

fn bench_config_cache_find_project_dir(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    let git_dir = temp_dir.path().join(".git");
    std::fs::create_dir(&git_dir).unwrap();

    let src_dir = temp_dir.path().join("src");
    std::fs::create_dir(&src_dir).unwrap();

    let test_file = src_dir.join("main.rs");
    std::fs::write(&test_file, "fn main() {}").unwrap();

    let cache = zenith::config::cache::ConfigCache::new();

    c.bench_function("config_cache_find_project_dir", |b| {
        b.iter(|| cache.find_project_directory(black_box(&test_file)));
    });
}

// Mock Zenith formatter for benchmarking
struct MockZenith {
    name: String,
    extensions: &'static [&'static str],
}

impl MockZenith {
    fn new(name: &str, extensions: &'static [&'static str]) -> Self {
        Self {
            name: name.to_string(),
            extensions,
        }
    }
}

#[async_trait::async_trait]
impl zenith::core::traits::Zenith for MockZenith {
    fn name(&self) -> &str {
        &self.name
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }

    async fn format(
        &self,
        content: &[u8],
        _path: &std::path::Path,
        _config: &zenith::config::types::ZenithConfig,
    ) -> zenith::error::Result<Vec<u8>> {
        Ok(content.to_vec())
    }
}

criterion_group!(
    benches,
    bench_hash_cache_compute_state,
    bench_hash_cache_needs_processing,
    bench_hash_cache_batch_operations,
    bench_registry_lookup,
    bench_registry_list_all,
    bench_path_validation,
    bench_is_hidden,
    bench_config_cache_operations,
    bench_config_cache_find_project_dir
);

criterion_main!(benches);
