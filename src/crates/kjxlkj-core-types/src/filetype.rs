//! File type detection.

use std::path::Path;

use crate::filetype_detect::{detect_from_extension, detect_from_filename, detect_from_shebang};

/// Detected file type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileType {
    name: String,
}

impl FileType {
    /// Creates a new file type.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Plain text file type.
    pub fn text() -> Self {
        Self::new("text")
    }
}

impl Default for FileType {
    fn default() -> Self {
        Self::text()
    }
}

/// Detects file type from path and content.
pub fn detect(path: Option<&Path>, content: Option<&str>) -> FileType {
    if let Some(path) = path {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            if let Some(ft) = detect_from_extension(ext) {
                return ft;
            }
        }
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if let Some(ft) = detect_from_filename(name) {
                return ft;
            }
        }
    }
    if let Some(content) = content {
        if let Some(ft) = detect_from_shebang(content) {
            return ft;
        }
    }
    FileType::text()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust() {
        let ft = detect(Some(Path::new("main.rs")), None);
        assert_eq!(ft.name(), "rust");
    }

    #[test]
    fn test_detect_python() {
        let ft = detect(Some(Path::new("script.py")), None);
        assert_eq!(ft.name(), "python");
    }

    #[test]
    fn test_detect_makefile() {
        let ft = detect(Some(Path::new("Makefile")), None);
        assert_eq!(ft.name(), "make");
    }

    #[test]
    fn test_detect_fallback() {
        let ft = detect(None, None);
        assert_eq!(ft.name(), "text");
    }
}
