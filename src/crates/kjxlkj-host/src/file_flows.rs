//! File operation flows: path resolution, encoding detection, and op sequencing.

use std::path::PathBuf;

/// Result of a file operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileResult {
    Success,
    NotFound,
    PermissionDenied,
    IoError(String),
}

/// Options for opening a file.
#[derive(Debug, Clone)]
pub struct OpenOptions {
    pub encoding: String,
    pub readonly: bool,
    pub create: bool,
}

impl Default for OpenOptions {
    fn default() -> Self {
        Self { encoding: "utf-8".into(), readonly: false, create: false }
    }
}

/// Options for writing a file.
#[derive(Debug, Clone)]
pub struct WriteOptions {
    pub encoding: String,
    pub line_ending: String,
    pub force: bool,
    pub backup: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            encoding: "utf-8".into(),
            line_ending: "\n".into(),
            force: false,
            backup: false,
        }
    }
}

/// Individual file operations composable into flows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOp {
    Open,
    Edit,
    Write,
    WriteQuit,
    SaveAs,
}

/// Resolve a path with tilde expansion and canonicalization attempt.
pub fn resolve_path(path: &str) -> String {
    let expanded = if path.starts_with('~') {
        if let Some(home) = home_dir() {
            path.replacen('~', &home, 1)
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    };
    match PathBuf::from(&expanded).canonicalize() {
        Ok(canon) => canon.to_string_lossy().to_string(),
        Err(_) => expanded,
    }
}

/// Detect encoding from a byte slice (BOM-based heuristic).
pub fn detect_encoding(bytes: &[u8]) -> String {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        "utf-8-bom".into()
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        "utf-16-le".into()
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        "utf-16-be".into()
    } else {
        "utf-8".into()
    }
}

/// Detect the dominant line-ending style.
pub fn detect_line_ending(content: &str) -> String {
    let crlf = content.matches("\r\n").count();
    let lf = content.matches('\n').count().saturating_sub(crlf);
    if crlf > lf { "\r\n".into() } else { "\n".into() }
}

/// Validate that a path is suitable for writing.
pub fn validate_write_target(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("empty path".into());
    }
    let p = PathBuf::from(path);
    if let Some(parent) = p.parent() {
        if !parent.as_os_str().is_empty() && !parent.exists() {
            return Err(format!("parent directory does not exist: {}", parent.display()));
        }
    }
    Ok(())
}

/// Build the sequence of operations for an edit flow.
pub fn build_edit_flow(path: &str) -> Vec<FileOp> {
    let _ = path; // used for future validation
    vec![FileOp::Open, FileOp::Edit]
}

/// Build the sequence of operations for a write-quit flow.
pub fn build_wq_flow(path: &str) -> Vec<FileOp> {
    let _ = path;
    vec![FileOp::Open, FileOp::Edit, FileOp::WriteQuit]
}

fn home_dir() -> Option<String> {
    std::env::var("HOME").ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_encodings() {
        assert_eq!(detect_encoding(b"hello"), "utf-8");
        assert_eq!(detect_encoding(&[0xEF, 0xBB, 0xBF, b'h']), "utf-8-bom");
        assert_eq!(detect_encoding(&[0xFF, 0xFE, 0, 0]), "utf-16-le");
        assert_eq!(detect_encoding(&[0xFE, 0xFF, 0, 0]), "utf-16-be");
    }

    #[test]
    fn line_endings() {
        assert_eq!(detect_line_ending("a\nb\nc\n"), "\n");
        assert_eq!(detect_line_ending("a\r\nb\r\nc\r\n"), "\r\n");
    }

    #[test]
    fn validate_paths() {
        assert!(validate_write_target("").is_err());
        assert!(validate_write_target("/tmp/test.txt").is_ok());
    }

    #[test]
    fn flows() {
        assert_eq!(build_edit_flow("x.rs"), vec![FileOp::Open, FileOp::Edit]);
        let f = build_wq_flow("x.rs");
        assert_eq!(f.len(), 3);
        assert_eq!(f[2], FileOp::WriteQuit);
    }

    #[test]
    fn resolve_path_absolute() {
        assert!(resolve_path("/tmp").starts_with('/'));
    }

    #[test]
    fn option_defaults() {
        let o = OpenOptions::default();
        assert_eq!(o.encoding, "utf-8");
        let w = WriteOptions::default();
        assert_eq!(w.line_ending, "\n");
    }

    #[test]
    fn file_result_eq() {
        assert_eq!(FileResult::Success, FileResult::Success);
        assert_ne!(FileResult::Success, FileResult::NotFound);
    }
}
