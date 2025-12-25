// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

use crate::error::{Result, ZenithError};
use std::path::{Component, Path};

pub fn validate_path(path: &Path) -> Result<()> {
    for component in path.components() {
        if let Component::ParentDir = component {
            return Err(ZenithError::PathTraversal(path.to_path_buf()));
        }
    }
    Ok(())
}

pub fn validate_path_strict(path: &Path) -> Result<()> {
    validate_path(path)?;

    let path_str = path.to_string_lossy();

    if path_str.len() > 4096 {
        return Err(ZenithError::Config(format!(
            "Path too long: {} characters",
            path_str.len()
        )));
    }

    let dangerous_patterns = [
        "/etc/",
        "/sys/",
        "/proc/",
        "/dev/",
        "/root/",
        "/boot/",
        "/lib/",
        "/lib64/",
        "/usr/bin/",
        "/usr/sbin/",
        "/bin/",
        "/sbin/",
    ];

    for pattern in &dangerous_patterns {
        if path_str.contains(pattern) {
            return Err(ZenithError::Config(format!(
                "Access to system directory is not allowed: {}",
                pattern
            )));
        }
    }

    Ok(())
}

pub fn is_safe_path(path: &Path) -> bool {
    validate_path(path).is_ok()
}

pub fn is_safe_path_strict(path: &Path) -> bool {
    validate_path_strict(path).is_ok()
}

pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}

pub fn sanitize_path_for_log(path: &Path) -> String {
    if let Some(file_name) = path.file_name() {
        let name = file_name.to_string_lossy();
        if let Some(parent) = path.parent() {
            if let Some(parent_name) = parent.file_name() {
                return format!("{}/{}", parent_name.to_string_lossy(), name);
            }
        }
        return name.to_string();
    }
    path.display().to_string()
}
