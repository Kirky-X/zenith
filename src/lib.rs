// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

//! Zenith 是一个高性能、可扩展的代码格式化与分析工具核心库。
//!
//! 该库提供了一套统一的接口来集成各种代码格式化工具（Zeniths），
//! 并支持插件系统、备份机制、缓存优化以及 MCP (Model Context Protocol) 协议。

pub mod config;
pub mod core;
pub mod error;
pub mod plugins;
pub mod prelude;
pub mod storage;
pub mod utils;
pub mod zeniths;

pub(crate) mod cli;
pub(crate) mod mcp;
pub(crate) mod services;

pub use mcp::protocol::{
    FileFormatResult, FormatParams, FormatResponseData, JsonRpcError, JsonRpcRequest,
    JsonRpcResponse, RecoverParams, RecoverResponseData,
};

#[doc(hidden)]
pub mod internal {
    pub use crate::cli::commands::{Cli, Commands};
    pub use crate::config::load_config;
    pub use crate::mcp::server::McpServer;
    pub use crate::plugins::PluginLoader;
    pub use crate::services::formatter::ZenithService;
    pub use crate::storage::backup::BackupService;
    pub use crate::storage::cache::HashCache;
    pub use crate::utils::environment::EnvironmentChecker;
    pub use crate::zeniths::registry::ZenithRegistry;

    #[cfg(feature = "c")]
    pub use crate::zeniths::impls::c_zenith::ClangZenith;
    #[cfg(feature = "ini")]
    pub use crate::zeniths::impls::ini_zenith::IniZenith;
    #[cfg(feature = "java")]
    pub use crate::zeniths::impls::java_zenith::JavaZenith;
    #[cfg(feature = "markdown")]
    pub use crate::zeniths::impls::markdown_zenith::MarkdownZenith;
    #[cfg(feature = "prettier")]
    pub use crate::zeniths::impls::prettier_zenith::PrettierZenith;
    #[cfg(feature = "python")]
    pub use crate::zeniths::impls::python_zenith::PythonZenith;
    #[cfg(feature = "rust")]
    pub use crate::zeniths::impls::rust_zenith::RustZenith;
    #[cfg(feature = "shell")]
    pub use crate::zeniths::impls::shell_zenith::ShellZenith;
    #[cfg(feature = "toml")]
    pub use crate::zeniths::impls::toml_zenith::TomlZenith;
}
