/// File open/edit/write flows — command and spec integration.

use std::path::{Path, PathBuf};

/// File operation result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileResult { Success, FileNotFound, PermissionDenied, ReadOnly, AlreadyExists, IoError(String) }

/// File open options.
#[derive(Debug, Clone)]
pub struct OpenOptions { pub readonly: bool, pub create: bool, pub encoding: String, pub binary: bool }

impl Default for OpenOptions {
    fn default() -> Self { Self { readonly: false, create: false, encoding: "utf-8".into(), binary: false } }
}

/// Write options for :w variants.
#[derive(Debug, Clone)]
pub struct WriteOptions { pub force: bool, pub append: bool, pub line_range: Option<(usize, usize)> }

impl Default for WriteOptions {
    fn default() -> Self { Self { force: false, append: false, line_range: None } }
}

/// File operation tracking.
#[derive(Debug)]
pub struct FileOperation { pub path: PathBuf, pub op: FileOp, pub result: FileResult }

/// Operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOp { Open, Edit, Write, WriteQuit, SaveAs }

/// Validate a write target path.
pub fn validate_write_target(path: &Path, force: bool) -> FileResult {
    if path.as_os_str().is_empty() { return FileResult::IoError("empty path".into()); }
    if !force && path.extension().is_none() {
        // allow, but note it
    }
    FileResult::Success
}

/// Resolve file path (expand ~ to home).
pub fn resolve_path(input: &str) -> PathBuf {
    if let Some(rest) = input.strip_prefix("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(input)
}

/// Detect file encoding by BOM.
pub fn detect_encoding(bytes: &[u8]) -> &'static str {
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) { "utf-8-bom" }
    else if bytes.starts_with(&[0xFF, 0xFE]) { "utf-16le" }
    else if bytes.starts_with(&[0xFE, 0xFF]) { "utf-16be" }
    else { "utf-8" }
}

/// Detect line ending from content.
pub fn detect_line_ending(content: &str) -> &'static str {
    if content.contains("\r\n") { "\r\n" } else if content.contains('\r') { "\r" } else { "\n" }
}

/// Build the :edit command flow.
pub fn build_edit_flow(path: &str) -> Vec<FileOp> {
    vec![FileOp::Open, FileOp::Edit]
}

/// Build the :wq command flow.
pub fn build_wq_flow() -> Vec<FileOp> { vec![FileOp::Write, FileOp::WriteQuit] }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_success() { assert_eq!(validate_write_target(Path::new("foo.rs"), false), FileResult::Success); }

    #[test]
    fn validate_empty() { assert!(matches!(validate_write_target(Path::new(""), false), FileResult::IoError(_))); }

    #[test]
    fn resolve_relative() { assert_eq!(resolve_path("foo.rs"), PathBuf::from("foo.rs")); }

    #[test]
    fn detect_utf8() { assert_eq!(detect_encoding(b"hello"), "utf-8"); }

    #[test]
    fn detect_utf8_bom() { assert_eq!(detect_encoding(&[0xEF, 0xBB, 0xBF, 0x41]), "utf-8-bom"); }

    #[test]
    fn detect_utf16le() { assert_eq!(detect_encoding(&[0xFF, 0xFE, 0x41, 0x00]), "utf-16le"); }

    #[test]
    fn line_ending_unix() { assert_eq!(detect_line_ending("line1\nline2"), "\n"); }

    #[test]
    fn line_ending_dos() { assert_eq!(detect_line_ending("line1\r\nline2"), "\r\n"); }

    #[test]
    fn edit_flow() { assert_eq!(build_edit_flow("test.rs").len(), 2); }

    #[test]
    fn wq_flow() { assert_eq!(build_wq_flow().len(), 2); }
}
