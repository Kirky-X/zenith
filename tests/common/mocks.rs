// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use zenith::config::types::ZenithConfig;
use zenith::core::traits::Zenith;
use zenith::error::ZenithError;

pub struct MockZenith {
    name: String,
    extensions: Vec<&'static str>,
}

impl MockZenith {
    pub fn new(name: &str, extensions: &[&'static str]) -> Self {
        Self {
            name: name.to_string(),
            extensions: extensions.to_vec(),
        }
    }
}

#[async_trait::async_trait]
impl Zenith for MockZenith {
    fn name(&self) -> &str {
        &self.name
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }

    async fn format(
        &self,
        _content: &[u8],
        _path: &std::path::Path,
        _config: &ZenithConfig,
    ) -> Result<Vec<u8>, ZenithError> {
        Ok(Vec::new())
    }

    async fn validate(&self, _content: &[u8]) -> Result<bool, ZenithError> {
        Ok(true)
    }
}

pub struct MockFormatter {
    name: String,
    extensions: Vec<&'static str>,
}

impl MockFormatter {
    pub fn new(name: &str, extensions: &[&'static str]) -> Self {
        Self {
            name: name.to_string(),
            extensions: extensions.to_vec(),
        }
    }
}

#[async_trait::async_trait]
impl Zenith for MockFormatter {
    fn name(&self) -> &str {
        &self.name
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }

    async fn format(
        &self,
        content: &[u8],
        _path: &std::path::Path,
        _config: &ZenithConfig,
    ) -> Result<Vec<u8>, ZenithError> {
        Ok(content.to_vec())
    }

    async fn validate(&self, _content: &[u8]) -> Result<bool, ZenithError> {
        Ok(true)
    }
}
