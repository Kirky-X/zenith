// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::error::{Result, ZenithError};
use crate::utils::path::sanitize_path_for_log;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;
use tracing::{debug, error};

pub struct StdioFormatter {
    pub tool_name: &'static str,
    pub args: Vec<String>,
}

impl StdioFormatter {
    pub async fn format_with_stdio(
        &self,
        content: &[u8],
        path: &Path,
        extra_args: Option<Vec<String>>,
    ) -> Result<Vec<u8>> {
        let sanitized_path = sanitize_path_for_log(path);
        debug!(
            "Executing formatter '{}' with args: {:?}, extra_args: {:?}, path: {}",
            self.tool_name, self.args, extra_args, sanitized_path
        );

        let mut cmd = Command::new(self.tool_name);

        for arg in &self.args {
            cmd.arg(arg);
        }

        if let Some(extra) = extra_args {
            for arg in extra {
                cmd.arg(arg);
            }
        }

        cmd.arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            error!("Failed to spawn formatter '{}': {}", self.tool_name, e);
            ZenithError::ToolNotFound {
                tool: self.tool_name.into(),
            }
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content).await.map_err(|e| {
                error!(
                    "Failed to write to formatter '{}' stdin: {}",
                    self.tool_name, e
                );
                ZenithError::Io(e)
            })?;
        }

        let output = child.wait_with_output().await.map_err(|e| {
            error!("Failed to wait for formatter '{}': {}", self.tool_name, e);
            ZenithError::Io(e)
        })?;

        if output.status.success() {
            debug!(
                "Formatter '{}' executed successfully, output size: {} bytes",
                self.tool_name,
                output.stdout.len()
            );
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            error!(
                "Formatter '{}' failed with exit code: {:?}, stderr: {}",
                self.tool_name,
                output.status.code(),
                stderr
            );
            Err(ZenithError::ZenithFailed {
                name: self.tool_name.into(),
                reason: stderr.to_string(),
            })
        }
    }
}
