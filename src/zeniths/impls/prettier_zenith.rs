// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::{Result, ZenithError};
use crate::utils::path::sanitize_path_for_log;
use crate::utils::version;
use async_trait::async_trait;
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::{debug, error};

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

        let sanitized_path = sanitize_path_for_log(path);
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let parser = match ext {
            "js" | "jsx" => "babel",
            "ts" | "tsx" => "typescript",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "md" => "markdown",
            "css" => "css",
            "scss" => "scss",
            "html" => "html",
            "vue" => "vue",
            _ => "babel",
        };

        let mut content_with_newline = content.to_vec();
        if !content.is_empty() && content[content.len() - 1] != b'\n' {
            content_with_newline.push(b'\n');
        }

        debug!(
            "Executing formatter 'prettier' with parser: {}, path: {}",
            parser, sanitized_path
        );

        let mut cmd = Command::new("prettier");
        cmd.args(&["--parser", parser])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            error!("Failed to spawn formatter 'prettier': {}", e);
            ZenithError::ToolNotFound {
                tool: "prettier".into(),
            }
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(&content_with_newline).map_err(|e| {
                error!("Failed to write to formatter 'prettier' stdin: {}", e);
                ZenithError::Io(e)
            })?;
        }

        let output = child.wait_with_output().map_err(|e| {
            error!("Failed to wait for formatter 'prettier': {}", e);
            ZenithError::Io(e)
        })?;

        if output.status.success() {
            debug!(
                "Formatter 'prettier' executed successfully, output size: {} bytes",
                output.stdout.len()
            );
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(
                "Formatter 'prettier' failed with exit code: {:?}, stderr: {}",
                output.status.code(),
                stderr
            );
            Err(ZenithError::ZenithFailed {
                name: "prettier".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
