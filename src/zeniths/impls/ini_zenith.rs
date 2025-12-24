use crate::core::traits::Zenith;
use crate::core::types::ZenithConfig;
use crate::error::Result;
use async_trait::async_trait;
use std::path::Path;

pub struct IniZenith;

#[async_trait]
impl Zenith for IniZenith {
    fn name(&self) -> &str {
        "ini"
    }

    fn extensions(&self) -> &[&str] {
        &["ini", "conf"]
    }

    async fn format(
        &self,
        content: &[u8],
        _path: &Path,
        _config: &ZenithConfig,
    ) -> Result<Vec<u8>> {
        let text = String::from_utf8_lossy(content);
        let mut result = String::new();

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                result.push('\n');
                continue;
            }

            if trimmed.starts_with('[') && trimmed.ends_with(']') {
                // Section
                result.push_str(trimmed);
                result.push('\n');
            } else if trimmed.contains('=') {
                // Key-value pair
                let parts: Vec<&str> = trimmed.splitn(2, '=').collect();
                let key = parts[0].trim();
                let value = parts[1].trim();
                result.push_str(&format!("{} = {}\n", key, value));
            } else {
                // Comment or other
                result.push_str(trimmed);
                result.push('\n');
            }
        }

        Ok(result.into_bytes())
    }
}
