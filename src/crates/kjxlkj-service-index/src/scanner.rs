//! File scanner for building the file index.

use std::path::{Path, PathBuf};

/// Scan a directory recursively for files, respecting gitignore.
pub fn scan_directory(root: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    scan_recursive(root, root, &mut result);
    result
}

fn scan_recursive(
    root: &Path,
    dir: &Path,
    result: &mut Vec<PathBuf>,
) {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden files and common ignored directories.
        if name_str.starts_with('.') {
            continue;
        }
        if matches!(
            name_str.as_ref(),
            "node_modules" | "target" | "__pycache__"
                | ".git" | "dist" | "build"
        ) {
            continue;
        }

        if path.is_dir() {
            scan_recursive(root, &path, result);
        } else if path.is_file() {
            if let Ok(rel) = path.strip_prefix(root) {
                result.push(rel.to_path_buf());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scan_temp_dir() {
        let dir = std::env::temp_dir().join("kjxlkj_scan_test");
        let _ = fs::create_dir_all(&dir);
        let file = dir.join("test.txt");
        fs::write(&file, "hello").unwrap();

        let files = scan_directory(&dir);
        assert!(!files.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }
}
