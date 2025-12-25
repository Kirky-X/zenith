use crate::error::{Result, ZenithError};
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

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

        let mut child = cmd.spawn().map_err(|_| ZenithError::ToolNotFound {
            tool: self.tool_name.into(),
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(content).await.map_err(ZenithError::Io)?;
        }

        let output = child.wait_with_output().await.map_err(ZenithError::Io)?;

        if output.status.success() {
            Ok(output.stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(ZenithError::ZenithFailed {
                name: self.tool_name.into(),
                reason: stderr.to_string(),
            })
        }
    }
}
