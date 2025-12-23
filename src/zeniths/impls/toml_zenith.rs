use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::{Result, ZenithError};
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct TomlZenith;

#[async_trait]
impl Zenith for TomlZenith {
    fn name(&self) -> &str {
        "taplo"
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }

    async fn format(&self, content: &[u8], _path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        let mut child = Command::new("taplo")
            .arg("fmt")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: "taplo".into(),
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
                name: "taplo".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
