use crate::error::{Result, ZenithError};
use std::path::{Component, Path};

pub fn validate_path(path: &Path) -> Result<()> {
    // 检查路径是否包含 '..' 组件，防止遍历到父目录
    for component in path.components() {
        if let Component::ParentDir = component {
            return Err(ZenithError::PathTraversal(path.to_path_buf()));
        }
    }
    Ok(())
}

pub fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && s != "." && s != "..")
        .unwrap_or(false)
}
