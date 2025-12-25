// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::Result;
use crate::utils::version;
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::path::Path;

pub struct PrettierZenith;

const PRETTIER_MIN_VERSION: &str = "2.0.0";

impl PrettierZenith {
    fn check_prettier_version() -> Result<()> {
        let version_str = version::get_tool_version("prettier")?;
        version::check_version("prettier", &version_str, PRETTIER_MIN_VERSION)?;
        Ok(())
    }
}

#[async_trait]
impl Zenith for PrettierZenith {
    fn name(&self) -> &str {
        "prettier"
    }

    fn extensions(&self) -> &[&str] {
        &[
            "js", "jsx", "ts", "tsx", "json", "css", "scss", "html", "vue", "yaml", "yml", "md",
        ]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        Self::check_prettier_version()?;

        let formatter = StdioFormatter {
            tool_name: "prettier",
            args: vec!["--stdin-filepath".into()],
        };
        formatter.format_with_stdio(content, path, None).await
    }
}
