use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::{Result, ZenithError};
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct RustZenith;

#[async_trait]
impl Zenith for RustZenith {
    fn name(&self) -> &str {
        "rust"
    }

    fn extensions(&self) -> &[&str] {
        &["rs"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        // 调用 rustfmt，使用 --stdin-filename 以支持自动查找配置文件
        let mut child = Command::new("rustfmt")
            .arg("--emit")
            .arg("stdout")
            .arg("--stdin-filename")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: "rustfmt".into(),
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
                name: "rustfmt".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
