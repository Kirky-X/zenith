// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::error::Result;
use async_trait::async_trait;
use std::path::Path;

#[async_trait]
pub trait Zenith: Send + Sync {
    fn name(&self) -> &str;

    fn extensions(&self) -> &[&str];

    fn priority(&self) -> i32 {
        0
    }

    async fn format(&self, content: &[u8], path: &Path, config: &ZenithConfig) -> Result<Vec<u8>>;

    async fn validate(&self, _content: &[u8]) -> Result<bool> {
        Ok(true)
    }
}
