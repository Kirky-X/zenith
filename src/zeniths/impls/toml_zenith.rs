use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::Result;
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use std::path::Path;

pub struct TomlZenith;

#[async_trait]
impl Zenith for TomlZenith {
    fn name(&self) -> &str {
        "taplo"
    }

    fn extensions(&self) -> &[&str] {
        &["toml"]
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        let formatter = StdioFormatter {
            tool_name: "taplo",
            args: vec!["fmt".into(), "--stdin-filepath".into()],
        };
        formatter.format_with_stdio(content, path, None).await
    }
}
