use crate::error::Result;
use std::path::{Path, PathBuf};

pub fn traverse_upwards<F>(start_dir: &Path, mut check_file: F) -> Result<Option<PathBuf>>
where
    F: FnMut(&Path) -> Option<PathBuf>,
{
    let mut current_dir = start_dir
        .parent()
        .ok_or_else(|| crate::error::ZenithError::Config("Invalid file path".to_string()))?;

    loop {
        if let Some(found_path) = check_file(current_dir) {
            return Ok(Some(found_path));
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent,
            None => break,
        }
    }

    Ok(None)
}

pub fn find_file_upwards<P: AsRef<Path>>(
    start_file: P,
    file_names: &[&str],
) -> Result<Option<PathBuf>> {
    let start_dir = start_file.as_ref();
    let file_names_vec = file_names.to_vec();

    traverse_upwards(start_dir, |dir| {
        for file_name in &file_names_vec {
            let candidate = dir.join(file_name);
            if candidate.exists() {
                return Some(candidate);
            }
        }
        None
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_file_upwards_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        fs::write(&test_file, "test").unwrap();

        let result = find_file_upwards(&test_file, &["nonexistent_file_12345.toml"]);
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_find_file_upwards_found() {
        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");
        fs::write(&config_file, "test = true").unwrap();

        let test_file = temp_dir.path().join("subdir").join("test.txt");
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(&test_file, "test").unwrap();

        let result = find_file_upwards(&test_file, &["config.toml"]);
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_traverse_upwards_custom_check() {
        let temp_dir = TempDir::new().unwrap();
        let marker_file = temp_dir.path().join("marker.txt");
        fs::write(&marker_file, "test").unwrap();

        let test_file = temp_dir.path().join("subdir").join("test.txt");
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(&test_file, "test").unwrap();

        let result = traverse_upwards(&test_file, |dir| {
            let marker = dir.join("marker.txt");
            if marker.exists() {
                Some(marker)
            } else {
                None
            }
        });

        assert!(result.unwrap().is_some());
    }
}
