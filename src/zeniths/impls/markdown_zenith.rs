// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::config::types::ZenithConfig;
use crate::core::traits::Zenith;
use crate::error::{Result, ZenithError};
use crate::zeniths::common::StdioFormatter;
use async_trait::async_trait;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub struct MarkdownZenith;

#[async_trait]
impl Zenith for MarkdownZenith {
    fn name(&self) -> &str {
        "markdown"
    }

    fn extensions(&self) -> &[&str] {
        &["md"]
    }

    fn priority(&self) -> i32 {
        100
    }

    async fn format(&self, content: &[u8], path: &Path, _config: &ZenithConfig) -> Result<Vec<u8>> {
        let preprocessed = fix_table_newlines(content);
        let with_rust_formatted = format_rust_code_blocks(&preprocessed);
        let formatter = StdioFormatter {
            tool_name: "prettier",
            args: vec![
                "--stdin-filepath".into(),
                "--parser".into(),
                "markdown".into(),
            ],
        };
        formatter
            .format_with_stdio_no_path(&with_rust_formatted, path, None)
            .await
    }
}

fn fix_table_newlines(content: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(content);
    let fixed = text.replace("|| ", "|\n|");
    fixed.into_bytes()
}

fn format_rust_code_blocks(content: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(content);
    let rust_code_block_pattern = Regex::new(r"(?s)```rust\s*\n(.+?)\n```").unwrap();
    let mut result = text.to_string();
    let mut matches: Vec<(String, String)> = Vec::new();

    for cap in rust_code_block_pattern.captures_iter(&text) {
        let full_match = cap.get(0).unwrap().as_str().to_string();
        let code_content = cap.get(1).unwrap().as_str().to_string();
        if let Ok(formatted) = format_with_rustfmt(&code_content) {
            matches.push((full_match, formatted));
        }
    }

    for (original, formatted) in matches.into_iter().rev() {
        let replacement = format!("```rust\n{}\n```", formatted);
        if let Some(pos) = result.rfind(&original) {
            let before = &result[..pos];
            let after = &result[pos + original.len()..];
            result = format!("{}{}{}", before, replacement, after);
        }
    }

    result.into_bytes()
}

fn format_with_rustfmt(code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            ZenithError::Io(std::io::Error::other(format!(
                "Failed to spawn rustfmt: {}",
                e
            )))
        })?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(code.as_bytes()).map_err(|e| {
            ZenithError::Io(std::io::Error::other(format!(
                "Failed to write to rustfmt stdin: {}",
                e
            )))
        })?;
    }

    let output = child.wait_with_output().map_err(|e| {
        ZenithError::Io(std::io::Error::other(format!(
            "Failed to read rustfmt output: {}",
            e
        )))
    })?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(ZenithError::Utf8Conversion)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        Err(ZenithError::ZenithFailed {
            name: "rustfmt".to_string(),
            reason: error_msg.to_string(),
        })
    }
}
