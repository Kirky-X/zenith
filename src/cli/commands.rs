use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "zenith", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, env = "ZENITH_CONFIG")]
    pub config: Option<PathBuf>,

    #[arg(short = 'L', long, env = "ZENITH_LOG_LEVEL", default_value = "info")]
    pub log_level: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Format files or directories
    Format {
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        #[arg(short, long)]
        recursive: bool,

        #[arg(long)]
        no_backup: bool,

        #[arg(short, long)]
        workers: Option<usize>,

        /// Run in check mode (dry-run), do not write files
        #[arg(long)]
        check: bool,
    },

    /// Check system environment
    Doctor {
        #[arg(short, long)]
        verbose: bool,
    },

    /// List available backups
    ListBackups,

    /// Recover from a backup
    Recover {
        /// The backup ID to recover from
        backup_id: String,

        /// Target directory (defaults to current directory)
        #[arg(short, long)]
        target: Option<PathBuf>,
    },

    /// Clean old backups
    CleanBackups {
        /// Days to retain backups
        #[arg(short, long, default_value = "7")]
        days: u32,
    },

    /// Start MCP Server
    Mcp {
        /// Server address
        #[arg(short, long, default_value = "127.0.0.1:9000")]
        addr: String,
    },
}
