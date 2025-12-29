// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 命令行程序的入口文件。
//! 负责解析命令行参数、初始化配置、注册内置和外部插件，并执行相应的命令。

use clap::Parser;
use colored::*;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, warn, Level};
use zenith::config::load_config;
use zenith::error::Result;
use zenith::internal::{
    BackupService, Cli, Commands, EnvironmentChecker, FileWatcher, HashCache, McpServer,
    PluginLoader, WatchConfig, ZenithRegistry, ZenithService,
};
use zenith::plugins::loader::PluginSecurityConfig;
use zenith::prelude::FormatResult;

#[cfg(feature = "c")]
use zenith::internal::ClangZenith;
#[cfg(feature = "ini")]
use zenith::internal::IniZenith;
#[cfg(feature = "java")]
use zenith::internal::JavaZenith;
#[cfg(feature = "markdown")]
use zenith::internal::MarkdownZenith;
#[cfg(feature = "prettier")]
use zenith::internal::PrettierZenith;
#[cfg(feature = "python")]
use zenith::internal::PythonZenith;
#[cfg(feature = "rust")]
use zenith::internal::RustZenith;
#[cfg(feature = "shell")]
use zenith::internal::ShellZenith;
#[cfg(feature = "toml")]
use zenith::internal::TomlZenith;

/// 程序的入口点。
///
/// # 返回值
///
/// 如果执行成功返回 `Ok(())`，否则返回包含错误信息的 `Result`。
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // 设置日志级别
    let log_level = match cli.log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    // 加载配置文件
    let mut config = load_config(cli.config)?;

    // 初始化插件加载器，应用安全配置
    let security_config = PluginSecurityConfig {
        allowed_commands: config.security.allowed_plugin_commands.clone(),
        allow_absolute_paths: config.security.allow_absolute_paths,
        allow_relative_paths: config.security.allow_relative_paths,
    };
    let mut plugin_loader = PluginLoader::with_security_config(security_config);

    // 从配置目录加载外部插件
    let plugins_dir = std::path::Path::new(&config.global.config_dir).join("plugins");
    if let Err(e) = plugin_loader.load_plugins_from_dir(&plugins_dir).await {
        error!("加载外部插件失败: {}", e);
    } else {
        info!(
            "外部插件加载完成，共 {} 个插件",
            plugin_loader.list_plugins().len()
        );
    }

    // 初始化统一注册中心
    let registry = Arc::new(ZenithRegistry::new());

    // 注册内置插件 (Built-in Zeniths)
    #[cfg(feature = "rust")]
    registry.register(Arc::new(RustZenith));

    #[cfg(feature = "python")]
    registry.register(Arc::new(PythonZenith));

    #[cfg(feature = "markdown")]
    registry.register(Arc::new(MarkdownZenith));

    #[cfg(feature = "prettier")]
    registry.register(Arc::new(PrettierZenith));

    #[cfg(feature = "c")]
    registry.register(Arc::new(ClangZenith));

    #[cfg(feature = "java")]
    registry.register(Arc::new(JavaZenith));

    #[cfg(feature = "ini")]
    registry.register(Arc::new(IniZenith));

    #[cfg(feature = "toml")]
    registry.register(Arc::new(TomlZenith));

    #[cfg(feature = "shell")]
    registry.register(Arc::new(ShellZenith));

    // 注册已加载的外部插件
    for plugin_info in plugin_loader.list_plugins() {
        if let Some(plugin) = plugin_loader.get_plugin(&plugin_info.name) {
            registry.register(plugin);
        }
    }

    // 根据命令执行相应的逻辑
    match cli.command {
        Commands::Format {
            paths,
            recursive,
            no_backup,
            workers,
            check,
            watch,
        } => {
            // 更新全局配置
            if recursive {
                config.global.recursive = true;
            }
            if no_backup {
                config.global.backup_enabled = false;
            }
            if let Some(w) = workers {
                config.concurrency.workers = w;
            }

            let mode_str = if check {
                "检查模式 (CHECK MODE)"
            } else if watch {
                "监听模式 (WATCH MODE)"
            } else {
                "写入模式 (WRITE MODE)"
            };
            info!(
                "正在启动 Zenith，模式：{}，工作线程数：{}...",
                mode_str, config.concurrency.workers
            );

            // 初始化服务组件
            let backup_service = Arc::new(BackupService::new(config.backup.clone()));
            let hash_cache = Arc::new(HashCache::new());
            let service = Arc::new(ZenithService::new(
                config.clone(),
                registry,
                backup_service,
                hash_cache,
                check,
            ));

            // 如果是监听模式，启动文件监听
            if watch {
                info!("启动文件监听模式，监控路径: {:?}", paths);

                // 首先格式化所有现有文件
                let string_paths: Vec<String> = paths
                    .clone()
                    .into_iter()
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect();
                let initial_results = service.format_paths(string_paths).await?;

                // 统计初始格式化结果
                let total = initial_results.len();
                let changed = initial_results.iter().filter(|r| r.changed).count();
                println!(
                    "\n{}",
                    format!("初始格式化完成: {} 个文件中 {} 个已修改", total, changed).green()
                );

                // 设置文件监听
                let watch_config = WatchConfig {
                    paths: paths.clone(),
                    debounce_duration: Duration::from_millis(100),
                    recursive,
                };

                let mut watcher = match FileWatcher::new(watch_config, service.clone()) {
                    Ok(w) => w,
                    Err(e) => {
                        error!("创建文件监听器失败: {}", e);
                        println!("{}", format!("创建文件监听器失败: {}", e).red());
                        std::process::exit(1);
                    }
                };

                info!(
                    "正在监听 {} 个路径，按 Ctrl+C 停止...",
                    watcher.watched_paths()
                );
                println!("\n{}", "监听中... (按 Ctrl+C 停止)".cyan());

                // 启动监听循环
                watcher
                    .start(move |path| {
                        let service = service.clone();
                        async move {
                            // 检查文件是否需要格式化
                            if !service.is_cached(&path).await {
                                let result = service.format_file(path).await;
                                if result.changed {
                                    println!(
                                        "{}",
                                        format!("  已格式化: {}", result.file_path.display())
                                            .green()
                                    );
                                } else if result.success {
                                    tracing::debug!("文件无需格式化: {:?}", result.file_path);
                                } else if let Some(err) = &result.error {
                                    if !err.starts_with("Skipped") {
                                        println!(
                                            "{}",
                                            format!(
                                                "  格式化失败: {} -> {}",
                                                result.file_path.display(),
                                                err
                                            )
                                            .red()
                                        );
                                    }
                                }
                                result
                            } else {
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
                        }
                    })
                    .await;
            } else {
                // 非监听模式，一次性格式化
                let string_paths: Vec<String> = paths
                    .into_iter()
                    .map(|p| p.to_string_lossy().into_owned())
                    .collect();
                let results = service.format_paths(string_paths).await?;

                // 统计执行结果
                let total = results.len();
                let success = results.iter().filter(|r| r.success).count();
                let changed = results.iter().filter(|r| r.changed).count();
                let failed = total - success;

                println!("\n{}", "执行摘要:".bold().underline());
                println!("  文件总数: {}", total);
                println!("  格式化成功: {}", success.to_string().green());
                println!("  已修改:     {}", changed.to_string().yellow());
                println!("  失败:       {}", failed.to_string().red());

                // 打印失败详情
                if failed > 0 {
                    println!("\n{}", "失败详情:".red().bold());
                    for res in results.iter().filter(|r| !r.success) {
                        if let Some(err) = &res.error {
                            if !err.starts_with("Skipped") {
                                println!("  {} -> {}", res.file_path.display(), err);
                            }
                        }
                    }
                }

                // 如果是检查模式且有文件需要格式化，则以非零状态码退出
                if check && changed > 0 {
                    println!("\n{}", "检查失败：部分文件需要格式化。".red());
                    std::process::exit(1);
                }
            }
        }
        Commands::Doctor { verbose } => {
            info!("正在检查系统环境...");
            let results = EnvironmentChecker::check_all(registry);
            let summary = EnvironmentChecker::print_results(&results, verbose);

            println!();

            if summary.missing_tools > 0 {
                let msg = format!(
                    "警告: 缺失 {} 个工具。某些格式化功能可能无法正常工作。",
                    summary.missing_tools
                );
                warn!("{}", msg);
                println!("{}", msg.yellow());
                std::process::exit(1);
            } else {
                println!("{}", "所有工具均可用！".green());
                info!("环境检查完成，所有工具均可用");
            }
        }
        Commands::ListBackups => {
            let backup_service = BackupService::new(config.backup.clone());
            match backup_service.list_backups().await {
                Ok(backups) => {
                    if backups.is_empty() {
                        println!("未发现备份。");
                    } else {
                        println!(
                            "{:<30} | {:<20} | {:<10}",
                            "备份 ID (Backup ID)", "创建时间", "大小"
                        );
                        println!("{:-<30}-|-{:-<20}-|-{:-<10}", "", "", "");
                        for (id, time, size) in backups {
                            let datetime: chrono::DateTime<chrono::Local> = time.into();
                            let size_mb = size as f64 / 1024.0 / 1024.0;
                            println!(
                                "{:<30} | {:<20} | {:.2} MB",
                                id,
                                datetime.format("%Y-%m-%d %H:%M"),
                                size_mb
                            );
                        }
                    }
                }
                Err(e) => error!("列出备份失败: {}", e),
            }
        }
        Commands::Recover { backup_id, target } => {
            info!("正在恢复备份 '{}'...", backup_id);
            let backup_service = BackupService::new(config.backup.clone());
            println!("正在恢复备份 '{}'...", backup_id);
            match backup_service.recover(&backup_id, target).await {
                Ok(count) => {
                    let msg = format!("成功恢复 {} 个文件。", count);
                    println!("{}", msg.green());
                    info!("{}", msg);
                }
                Err(e) => {
                    error!("恢复失败: {}", e);
                    println!("{}", format!("恢复失败: {}", e).red());
                }
            }
        }
        Commands::CleanBackups { days } => {
            info!("正在清理 {} 天前的备份...", days);
            let backup_service = BackupService::new(config.backup.clone());
            println!("正在清理 {} 天前的备份...", days);
            match backup_service.clean_backups(days).await {
                Ok(count) => {
                    let msg = format!("已移除 {} 个旧备份。", count);
                    println!("{}", msg.green());
                    info!("{}", msg);
                }
                Err(e) => {
                    error!("清理失败: {}", e);
                    println!("{}", format!("清理失败: {}", e).red());
                }
            }
        }
        Commands::Mcp { addr } => {
            let socket_addr: std::net::SocketAddr = addr
                .parse()
                .map_err(|_| zenith::error::ZenithError::Config("无效的地址".into()))?;

            let hash_cache = Arc::new(HashCache::new());
            let server = McpServer::new(config, registry, hash_cache);
            server.run(socket_addr).await?;
        }
        Commands::AutoRollback => {
            info!("正在启动自动回滚到最新备份...");

            let backup_service = Arc::new(BackupService::new(config.backup.clone()));
            let hash_cache = Arc::new(HashCache::new());
            let service = ZenithService::new(config, registry, backup_service, hash_cache, false);

            match service.auto_rollback().await {
                Ok(recovered_files) => {
                    let msg = format!("成功自动回滚 {} 个文件。", recovered_files.len());
                    println!("{}", msg.green());
                    info!("{}", msg);
                    if !recovered_files.is_empty() {
                        println!("\n已恢复的文件:");
                        for file_path in recovered_files {
                            println!("  - {}", file_path);
                        }
                    }
                }
                Err(e) => {
                    error!("自动回滚失败: {}", e);
                    println!("{}", format!("自动回滚失败: {}", e).red());
                }
            }
        }
    }

    Ok(())
}
