use clap::Parser;
use colored::*;
use std::sync::Arc;
use tracing::{error, info, Level};
use zenith::cli::commands::{Cli, Commands};
use zenith::config::load_config;
use zenith::error::Result;
use zenith::mcp::server::McpServer;
use zenith::services::formatter::ZenithService;
use zenith::storage::backup::BackupService;
use zenith::utils::environment::EnvironmentChecker;
use zenith::zeniths::impls::{
    c_zenith::ClangZenith, ini_zenith::IniZenith, java_zenith::JavaZenith,
    prettier_zenith::PrettierZenith, python_zenith::PythonZenith, rust_zenith::RustZenith,
    shell_zenith::ShellZenith, toml_zenith::TomlZenith,
};
use zenith::zeniths::registry::ZenithRegistry;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = match cli.log_level.to_lowercase().as_str() {
        "debug" => Level::DEBUG,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();

    let mut config = load_config(cli.config)?;

    // 统一注册中心
    let registry = Arc::new(ZenithRegistry::new());
    registry.register(Arc::new(RustZenith));
    registry.register(Arc::new(PythonZenith));
    registry.register(Arc::new(PrettierZenith));
    registry.register(Arc::new(ClangZenith));
    registry.register(Arc::new(JavaZenith));
    registry.register(Arc::new(IniZenith));
    registry.register(Arc::new(TomlZenith));
    registry.register(Arc::new(ShellZenith));

    match cli.command {
        Commands::Format {
            paths,
            recursive,
            no_backup,
            workers,
            check,
        } => {
            if recursive {
                config.global.recursive = true;
            }
            if no_backup {
                config.global.backup_enabled = false;
            }
            if let Some(w) = workers {
                config.concurrency.workers = w;
            }

            let mode_str = if check { "CHECK MODE" } else { "WRITE MODE" };
            info!(
                "Starting Zenith in {} with {} workers...",
                mode_str, config.concurrency.workers
            );

            let backup_service = Arc::new(BackupService::new(config.backup.clone()));
            let service = ZenithService::new(config, registry, backup_service, check);

            let results = service.format_paths(paths).await?;

            let total = results.len();
            let success = results.iter().filter(|r| r.success).count();
            let changed = results.iter().filter(|r| r.changed).count();
            let failed = total - success;

            println!("\n{}", "Summary:".bold().underline());
            println!("  Total files: {}", total);
            println!("  Formatted:   {}", success.to_string().green());
            println!("  Changed:     {}", changed.to_string().yellow());
            println!("  Failed:      {}", failed.to_string().red());

            if failed > 0 {
                println!("\n{}", "Failures:".red().bold());
                for res in results.iter().filter(|r| !r.success) {
                    if let Some(err) = &res.error {
                        if !err.starts_with("Skipped") {
                            println!("  {} -> {}", res.file_path.display(), err);
                        }
                    }
                }
            }

            if check && changed > 0 {
                println!("\n{}", "Check failed: Files need formatting.".red());
                std::process::exit(1);
            }
        }
        Commands::Doctor { verbose } => {
            info!("Checking system environment...");
            let results = EnvironmentChecker::check_all(registry);

            println!("\n{}", "Tool Environment Check:".bold().underline());
            for res in results {
                let status = if res.available {
                    "✅ Available".green()
                } else {
                    "❌ Not Found".red()
                };

                print!("  {:<20} {}", res.name.bold(), status);
                if let Some(v) = res.version {
                    if verbose {
                        print!(" ({})", v.dimmed());
                    }
                }
                println!();
            }
            println!();
        }
        Commands::ListBackups => {
            let backup_service = BackupService::new(config.backup.clone());
            match backup_service.list_backups().await {
                Ok(backups) => {
                    if backups.is_empty() {
                        println!("No backups found.");
                    } else {
                        println!("{:<30} | {:<20} | {:<10}", "Backup ID", "Created", "Size");
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
                Err(e) => error!("Failed to list backups: {}", e),
            }
        }
        Commands::Recover { backup_id, target } => {
            let backup_service = BackupService::new(config.backup.clone());
            println!("Recovering backup '{}'...", backup_id);
            match backup_service.recover(&backup_id, target).await {
                Ok(count) => println!(
                    "{}",
                    format!("Successfully restored {} files.", count).green()
                ),
                Err(e) => error!("Recovery failed: {}", e),
            }
        }
        Commands::CleanBackups { days } => {
            let backup_service = BackupService::new(config.backup.clone());
            println!("Cleaning backups older than {} days...", days);
            match backup_service.clean_backups(days).await {
                Ok(count) => println!("{}", format!("Removed {} old backups.", count).green()),
                Err(e) => error!("Cleanup failed: {}", e),
            }
        }
        Commands::Mcp { addr } => {
            let socket_addr: std::net::SocketAddr = addr
                .parse()
                .map_err(|_| zenith::error::ZenithError::Config("Invalid address".into()))?;

            let server = McpServer::new(config, registry);
            server.run(socket_addr).await?;
        }
    }

    Ok(())
}
