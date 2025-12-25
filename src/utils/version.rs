// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::error::{Result, ZenithError};
use std::process::Command;

pub fn parse_version(version_str: &str) -> Option<Vec<u32>> {
    let version: Vec<u32> = version_str
        .split(|c: char| !c.is_ascii_digit())
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    if version.is_empty() {
        None
    } else {
        Some(version)
    }
}

pub fn check_version(tool: &str, version_str: &str, min_version: &str) -> Result<()> {
    let current = parse_version(version_str).ok_or_else(|| ZenithError::ZenithFailed {
        name: tool.to_string(),
        reason: format!("Failed to parse version: {}", version_str),
    })?;

    let min = parse_version(min_version).ok_or_else(|| ZenithError::ZenithFailed {
        name: tool.to_string(),
        reason: format!("Failed to parse minimum version: {}", min_version),
    })?;

    if current < min {
        return Err(ZenithError::VersionIncompatible {
            tool: tool.to_string(),
            required: format!(">= {}", min_version),
            actual: version_str.to_string(),
        });
    }

    Ok(())
}

pub fn get_tool_version(tool: &str) -> Result<String> {
    let output =
        Command::new(tool)
            .arg("--version")
            .output()
            .map_err(|_| ZenithError::ToolNotFound {
                tool: tool.to_string(),
            })?;

    if !output.status.success() {
        return Err(ZenithError::ZenithFailed {
            name: tool.to_string(),
            reason: "Failed to get version".to_string(),
        });
    }

    let version_str = String::from_utf8_lossy(&output.stdout);
    let version = version_str
        .lines()
        .next()
        .and_then(|line| {
            line.split_whitespace()
                .nth(1)
                .or_else(|| line.split_whitespace().next())
        })
        .ok_or_else(|| ZenithError::ZenithFailed {
            name: tool.to_string(),
            reason: "No version information found".to_string(),
        })?;

    Ok(version.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("1.5.0"), Some(vec![1, 5, 0]));
        assert_eq!(parse_version("2.0"), Some(vec![2, 0]));
        assert_eq!(parse_version("3"), Some(vec![3]));
        assert_eq!(parse_version("invalid"), None);
    }

    #[test]
    fn test_check_version_compatible() {
        assert!(check_version("test", "1.5.0", "1.0.0").is_ok());
        assert!(check_version("test", "2.0.0", "1.5.0").is_ok());
        assert!(check_version("test", "1.5.0", "1.5.0").is_ok());
    }

    #[test]
    fn test_check_version_incompatible() {
        let result = check_version("test", "1.0.0", "1.5.0");
        assert!(result.is_err());
        if let Err(ZenithError::VersionIncompatible {
            tool,
            required,
            actual,
        }) = result
        {
            assert_eq!(tool, "test");
            assert_eq!(required, ">= 1.5.0");
            assert_eq!(actual, "1.0.0");
        } else {
            panic!("Expected VersionIncompatible error");
        }
    }
}
