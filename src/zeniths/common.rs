// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::error::{Result, ZenithError};
use crate::utils::path::sanitize_path_for_log;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::process::Command;
use tokio::time::timeout;
use tracing::{debug, error};

#[derive(Debug, Clone)]
pub struct StdioFormatter {
    pub tool_name: &'static str,
    pub args: Vec<String>,
    /// Optional timeout for command execution (in seconds)
    pub timeout_seconds: Option<u64>,
}

impl Default for StdioFormatter {
    fn default() -> Self {
        Self {
            tool_name: "",
            args: Vec::new(),
            timeout_seconds: Some(30), // Default 30 second timeout
        }
    }
}

impl StdioFormatter {
    /// Create a new StdioFormatter with default timeout
    pub fn new(tool_name: &'static str, args: Vec<String>) -> Self {
        Self {
            tool_name,
            args,
            timeout_seconds: Some(30),
        }
    }

    /// Set custom timeout in seconds
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    /// Disable timeout (use with caution)
    pub fn without_timeout(mut self) -> Self {
        self.timeout_seconds = None;
        self
    }

    /// Core implementation shared between format_with_stdio and format_with_stdio_no_path
    async fn execute_command(
        &self,
        content: &[u8],
        path: Option<&Path>,
        extra_args: Option<Vec<String>>,
    ) -> Result<Vec<u8>> {
        let path_str = path.map(sanitize_path_for_log).unwrap_or_default();
        debug!(
            "Executing formatter '{}' with args: {:?}, extra_args: {:?}, path: {}",
            self.tool_name, self.args, extra_args, path_str
        );

        let mut cmd = Command::new(self.tool_name);

        // Add base arguments
        for arg in &self.args {
            cmd.arg(arg);
        }

        // Add extra arguments
        if let Some(extra) = extra_args {
            for arg in extra {
                cmd.arg(arg);
            }
        }

        // Add path argument if provided
        if let Some(p) = path {
            cmd.arg(p);
        }

        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| {
            error!("Failed to spawn formatter '{}': {}", self.tool_name, e);
            ZenithError::ToolNotFound {
                tool: self.tool_name.into(),
            }
        })?;

        // Write content to stdin
        if let Some(mut stdin) = child.stdin.take() {
            let mut writer = BufWriter::new(&mut stdin);
            writer.write_all(content).await.map_err(|e| {
                error!(
                    "Failed to write to formatter '{}' stdin: {}",
                    self.tool_name, e
                );
                ZenithError::Io(e)
            })?;
            writer.flush().await.map_err(|e| {
                error!(
                    "Failed to flush formatter '{}' stdin: {}",
                    self.tool_name, e
                );
                ZenithError::Io(e)
            })?;
        }

        // Execute command - always wait for output first
        let output_result = child.wait_with_output().await;

        // Apply timeout if configured
        let output = match (self.timeout_seconds, output_result) {
            (Some(timeout_secs), Ok(child_output)) => {
                let duration = Duration::from_secs(timeout_secs);
                match timeout(duration, async { Ok::<_, std::io::Error>(child_output) }).await {
                    Ok(Ok(output)) => output,
                    Ok(Err(e)) => {
                        error!("Failed to wait for formatter '{}': {}", self.tool_name, e);
                        return Err(ZenithError::Io(e));
                    }
                    Err(_) => {
                        return Err(ZenithError::ZenithFailed {
                            name: self.tool_name.into(),
                            reason: format!("Command timed out after {} seconds", timeout_secs),
                        });
                    }
                }
            }
            (Some(_), Err(e)) => {
                error!("Failed to wait for formatter '{}': {}", self.tool_name, e);
                return Err(ZenithError::Io(e));
            }
            (None, Ok(output)) => output,
            (None, Err(e)) => {
                error!("Failed to wait for formatter '{}': {}", self.tool_name, e);
                return Err(ZenithError::Io(e));
            }
        };

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

    pub async fn format_with_stdio(
        &self,
        content: &[u8],
        path: &Path,
        extra_args: Option<Vec<String>>,
    ) -> Result<Vec<u8>> {
        self.execute_command(content, Some(path), extra_args).await
    }

    pub async fn format_with_stdio_no_path(
        &self,
        content: &[u8],
        _path: &Path,
        extra_args: Option<Vec<String>>,
    ) -> Result<Vec<u8>> {
        self.execute_command(content, None, extra_args).await
    }
}
