// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! 命令行命令定义模块。
//! 使用 `clap` 库定义程序的子命令及其参数。

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Zenith 命令行主结构体。
#[derive(Parser)]
#[command(name = "zenith", version, about = "高性能、可扩展的代码格式化与分析工具", long_about = None)]
pub struct Cli {
    /// 要执行的子命令。
    #[command(subcommand)]
    pub command: Commands,

    /// 配置文件路径。可以通过环境变量 `ZENITH_CONFIG` 设置。
    #[arg(short, long, env = "ZENITH_CONFIG")]
    pub config: Option<PathBuf>,

    /// 日志级别（debug, info, warn, error）。默认为 `info`。
    #[arg(short = 'L', long, env = "ZENITH_LOG_LEVEL", default_value = "info")]
    pub log_level: String,
}

/// 支持的子命令列表。
#[derive(Subcommand)]
pub enum Commands {
    /// 格式化文件或目录。
    Format {
        /// 要格式化的路径列表。
        #[arg(required = true)]
        paths: Vec<PathBuf>,

        /// 是否递归遍历子目录。
        #[arg(short, long)]
        recursive: bool,

        /// 是否禁用自动备份。
        #[arg(long)]
        no_backup: bool,

        /// 并发工作线程数。
        #[arg(short, long)]
        workers: Option<usize>,

        /// 运行在检查模式（dry-run），不修改文件内容。
        #[arg(long)]
        check: bool,

        /// 启用文件监听模式，监控文件变化并自动格式化。
        #[arg(long)]
        watch: bool,
    },

    /// 检查系统环境。
    Doctor {
        /// 是否输出详细信息。
        #[arg(short, long)]
        verbose: bool,
    },

    /// 列出所有可用的备份。
    ListBackups,

    /// 从备份中恢复文件。
    Recover {
        /// 要恢复的备份 ID。
        backup_id: String,

        /// 恢复的目标目录（默认为当前目录）。
        #[arg(short, long)]
        target: Option<PathBuf>,
    },

    /// 清理旧备份。
    CleanBackups {
        /// 备份保留天数。
        #[arg(short, long, default_value = "7")]
        days: u32,
    },

    /// 启动 MCP (Model Context Protocol) 服务。
    Mcp {
        /// 服务监听地址。
        #[arg(short, long, default_value = "127.0.0.1:9000")]
        addr: String,
    },

    /// 自动回滚到最新的备份。
    AutoRollback,
}
