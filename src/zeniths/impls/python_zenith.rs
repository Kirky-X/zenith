use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::{Result, ZenithError};
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct PythonZenith;

#[async_trait]
impl Zenith for PythonZenith {
    fn name(&self) -> &str {
        "python"
    }

    fn extensions(&self) -> &[&str] {
        &["py", "pyi"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        // 使用 ruff format -，并传入 --stdin-filename 以支持配置文件探测
        let mut child = Command::new("ruff")
            .arg("format")
            .arg("-") // 从 stdin 读取
            .arg("--stdin-filename")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: "ruff".into(),
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
                name: "ruff".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
