use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::{Result, ZenithError};
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct JavaZenith;

#[async_trait]
impl Zenith for JavaZenith {
    fn name(&self) -> &str {
        "google-java-format"
    }

    fn extensions(&self) -> &[&str] {
        &["java"]
    }

    async fn format(&self, content: &[u8], _path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        // 使用 google-java-format -
        let mut child = Command::new("google-java-format")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: "google-java-format".into(),
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
                name: "google-java-format".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
