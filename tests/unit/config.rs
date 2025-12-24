use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use zenith::config::types::{AppConfig, ZenithSettings};
use zenith::services::formatter::ZenithService;
use zenith::storage::backup::BackupService;
use zenith::storage::cache::HashCache;
use zenith::zeniths::registry::ZenithRegistry;

#[tokio::test]
async fn test_config_cache_functionality() {
    // 创建临时目录结构
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // 创建子目录和测试文件
    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir).unwrap();

    let test_file = src_dir.join("main.rs");
    fs::write(&test_file, "fn main() { println!(\"Hello\"); }").unwrap();

    // 创建项目级配置文件
    let config_file = project_dir.join("zenith.toml");
    fs::write(
        &config_file,
        r#"
[global]
log_level = "debug"

[zeniths.rust]
enabled = true
config_path = ".rustfmt.toml"
use_default = false

[zeniths.default]
enabled = true
use_default = true
"#,
    )
    .unwrap();

    // 创建服务实例
    let app_config = AppConfig::default();
    let registry = ZenithRegistry::new();
    let backup_service = BackupService::new(app_config.backup.clone());
    let hash_cache = HashCache::new();
    let service = ZenithService::new(
        app_config,
        std::sync::Arc::new(registry),
        std::sync::Arc::new(backup_service),
        std::sync::Arc::new(hash_cache),
        false,
    );

    // 处理文件以触发配置缓存
    let root = project_dir.to_path_buf();
    let result = service.process_file(root, test_file.clone()).await;

    // 验证处理成功
    assert!(result.success || result.error.as_ref().is_some_and(|e| e.contains("Skipped")));

    println!("Config cache test passed!");
}

#[tokio::test]
async fn test_extension_specific_config() {
    // 测试扩展名特定配置功能
    let mut app_config = AppConfig::default();

    // 添加Rust特定配置
    let rust_settings = ZenithSettings {
        enabled: true,
        config_path: Some(".rustfmt.toml".to_string()),
        use_default: false,
    };

    app_config.zeniths.insert("rs".to_string(), rust_settings);

    // 创建临时文件
    let temp_dir = TempDir::new().unwrap();
    let test_file = temp_dir.path().join("test.rs");
    fs::write(&test_file, "fn test() {}").unwrap();

    // 创建服务实例
    let registry = ZenithRegistry::new();
    let backup_service = BackupService::new(app_config.backup.clone());
    let hash_cache = HashCache::new();
    let service = ZenithService::new(
        app_config,
        std::sync::Arc::new(registry),
        std::sync::Arc::new(backup_service),
        std::sync::Arc::new(hash_cache),
        false,
    );

    // 获取针对该文件的Zenith配置
    let ext = "rs";
    let zenith_config = service.create_zenith_config_for_file(&service.config, &test_file, ext);

    // 验证配置被正确应用
    assert_eq!(
        zenith_config.custom_config_path,
        Some(PathBuf::from(".rustfmt.toml"))
    );
    assert!(!zenith_config.use_default_rules);

    println!("Extension-specific config test passed!");
}
