//! File system read/write operations.

use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FsError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("encoding error for {path}")]
    Encoding { path: String },
}

/// Read a file to string.
pub fn read_file(path: &Path) -> Result<String, FsError> {
    tracing::debug!("reading file: {}", path.display());
    std::fs::read_to_string(path).map_err(FsError::from)
}

/// Write content to a file (creates or overwrites).
pub fn write_file(path: &Path, content: &str) -> Result<(), FsError> {
    tracing::debug!("writing file: {}", path.display());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content).map_err(FsError::from)
}

/// Check if a file exists.
pub fn file_exists(path: &Path) -> bool {
    path.exists()
}

/// Detect encoding heuristic: returns "utf-8" or "latin-1".
pub fn detect_encoding(bytes: &[u8]) -> &'static str {
    // Check for BOM
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return "utf-8";
    }
    // Try to validate as UTF-8
    if std::str::from_utf8(bytes).is_ok() {
        "utf-8"
    } else {
        "latin-1"
    }
}

/// Detect line ending style in content.
pub fn detect_line_ending(content: &str) -> &'static str {
    if content.contains("\r\n") {
        "crlf"
    } else if content.contains('\r') {
        "cr"
    } else {
        "lf"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_utf8() {
        assert_eq!(detect_encoding(b"hello world"), "utf-8");
        assert_eq!(detect_encoding(&[0xEF, 0xBB, 0xBF, b'a']), "utf-8");
    }

    #[test]
    fn detect_latin1() {
        // Invalid UTF-8 byte sequence.
        assert_eq!(detect_encoding(&[0xFF, 0xFE, 0x80]), "latin-1");
    }

    #[test]
    fn line_endings() {
        assert_eq!(detect_line_ending("a\nb\n"), "lf");
        assert_eq!(detect_line_ending("a\r\nb\r\n"), "crlf");
        assert_eq!(detect_line_ending("a\rb\r"), "cr");
    }
}
