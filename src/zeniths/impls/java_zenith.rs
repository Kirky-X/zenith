// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::Result;
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::path::Path;

pub struct JavaZenith;

#[async_trait]
impl Zenith for JavaZenith {
    fn name(&self) -> &str {
        "google-java-format"
    }

    fn extensions(&self) -> &[&str] {
        &["java"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        let formatter = StdioFormatter {
            tool_name: "google-java-format",
            args: vec!["--stdin-filename".into()],
        };
        formatter.format_with_stdio(content, path, None).await
    }
}
