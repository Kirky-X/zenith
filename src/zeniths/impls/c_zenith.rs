use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::{Result, ZenithError};
use async_trait::async_trait;
use std::path::Path;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct ClangZenith;

#[async_trait]
impl Zenith for ClangZenith {
    fn name(&self) -> &str {
        "clang-format"
    }

    fn extensions(&self) -> &[&str] {
        &["c", "cpp", "cc", "h", "hpp"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        // 使用 clang-format，并传入 --assume-filename 以支持配置文件探测
        let mut child = Command::new("clang-format")
            .arg("--assume-filename")
            .arg(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: "clang-format".into(),
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
                name: "clang-format".into(),
                reason: stderr.to_string(),
            })
        }
    }
}
