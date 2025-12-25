// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::discovery::discover_formatter_config;
use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::Result;
use crate::utils::version;
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::path::Path;

pub struct RustZenith;

const RUSTFMT_MIN_VERSION: &str = "1.0.0";

impl RustZenith {
    fn check_rustfmt_version() -> Result<()> {
        let version_str = version::get_tool_version("rustfmt")?;
        version::check_version("rustfmt", &version_str, RUSTFMT_MIN_VERSION)?;
        Ok(())
    }
}

#[async_trait]
impl Zenith for RustZenith {
    fn name(&self) -> &str {
        "rust"
    }

    fn extensions(&self) -> &[&str] {
        &["rs"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        Self::check_rustfmt_version()?;

        let mut extra_args = vec!["--emit".into(), "stdout".into()];

        if let Some(config_path) = discover_formatter_config(path, "rust")? {
            extra_args.push("--config-path".into());
            extra_args.push(config_path.to_string_lossy().into());
        }

        let formatter = StdioFormatter {
            tool_name: "rustfmt",
            args: vec![],
        };
        formatter
            .format_with_stdio(content, path, Some(extra_args))
            .await
    }
}
