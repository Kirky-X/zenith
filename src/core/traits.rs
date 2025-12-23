use crate::core::types::ZenithConfig;
use crate::error::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Zenith: Send + Sync {
    /// 格式化器名称
    fn name(&self) -> &str;

    /// 支持的文件扩展名
    fn extensions(&self) -> &[&str];

    /// 格式化文件内容
    async fn format(&self, content: &[u8], config: &ZenithConfig) -> Result<Vec<u8>>;

    /// 验证内容是否合法 (可选实现)
    async fn validate(&self, _content: &[u8]) -> Result<bool> {
        Ok(true)
    }
}
