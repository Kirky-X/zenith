// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 命令行程序的入口文件。
//! 负责解析命令行参数、初始化配置、注册内置和外部插件，并执行相应的命令。

use clap::Parser;
use colored::*;
use std::sync::Arc;
use tracing::{error, info, Level};
use zenith::config::load_config;
use zenith::error::Result;
use zenith::internal::{
    BackupService, ClangZenith, Cli, Commands, EnvironmentChecker, HashCache, IniZenith,
    JavaZenith, MarkdownZenith, McpServer, PluginLoader, PrettierZenith, PythonZenith, RustZenith,
    ShellZenith, TomlZenith, ZenithRegistry, ZenithService,
};
use zenith::plugins::loader::PluginSecurityConfig;

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
        eprintln!("加载外部插件失败: {}", e);
    }

    // 初始化统一注册中心
    let registry = Arc::new(ZenithRegistry::new());

    // 注册内置插件 (Built-in Zeniths)
    registry.register(Arc::new(RustZenith));
    registry.register(Arc::new(PythonZenith));
    registry.register(Arc::new(MarkdownZenith));
    registry.register(Arc::new(PrettierZenith));
    registry.register(Arc::new(ClangZenith));
    registry.register(Arc::new(JavaZenith));
    registry.register(Arc::new(IniZenith));
    registry.register(Arc::new(TomlZenith));
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
            let service = ZenithService::new(config, registry, backup_service, hash_cache, check);

            // 将 PathBuf 转换为 String 以保持 format_paths 兼容性
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
        Commands::Doctor { verbose } => {
            info!("正在检查系统环境...");
            let results = EnvironmentChecker::check_all(registry);
            let summary = EnvironmentChecker::print_results(&results, verbose);

            println!();

            if summary.missing_tools > 0 {
                println!(
                    "{}",
                    format!(
                        "警告: 缺失 {} 个工具。某些格式化功能可能无法正常工作。",
                        summary.missing_tools
                    )
                    .yellow()
                );
                std::process::exit(1);
            } else {
                println!("{}", "所有工具均可用！".green());
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
            let backup_service = BackupService::new(config.backup.clone());
            println!("正在恢复备份 '{}'...", backup_id);
            match backup_service.recover(&backup_id, target).await {
                Ok(count) => println!("{}", format!("成功恢复 {} 个文件。", count).green()),
                Err(e) => error!("恢复失败: {}", e),
            }
        }
        Commands::CleanBackups { days } => {
            let backup_service = BackupService::new(config.backup.clone());
            println!("正在清理 {} 天前的备份...", days);
            match backup_service.clean_backups(days).await {
                Ok(count) => println!("{}", format!("已移除 {} 个旧备份。", count).green()),
                Err(e) => error!("清理失败: {}", e),
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
                    println!(
                        "{}",
                        format!("成功自动回滚 {} 个文件。", recovered_files.len()).green()
                    );
                    if !recovered_files.is_empty() {
                        println!("\n已恢复的文件:");
                        for file_path in recovered_files {
                            println!("  - {}", file_path);
                        }
                    }
                }
                Err(e) => error!("自动回滚失败: {}", e),
            }
        }
    }

    Ok(())
}
