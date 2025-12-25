use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::Result;
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::path::Path;

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
        let formatter = StdioFormatter {
            tool_name: "clang-format",
            args: vec!["--assume-filename".into()],
        };
        formatter.format_with_stdio(content, path, None).await
    }
}
