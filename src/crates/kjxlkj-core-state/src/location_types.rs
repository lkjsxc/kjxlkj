//! Location entry type.

use std::path::PathBuf;

/// A single location list entry.
#[derive(Debug, Clone)]
pub struct LocationEntry {
    /// File path.
    pub path: PathBuf,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub col: usize,
    /// Entry text/message.
    pub text: String,
}

impl LocationEntry {
    /// Creates a new location entry.
    pub fn new(path: PathBuf, line: usize, col: usize, text: &str) -> Self {
        Self {
            path,
            line,
            col,
            text: text.to_string(),
        }
    }

    /// Returns formatted location string.
    pub fn location(&self) -> String {
        format!("{}:{}:{}", self.path.display(), self.line, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_entry_location() {
        let entry = LocationEntry::new(PathBuf::from("src/lib.rs"), 42, 10, "test");
        assert_eq!(entry.location(), "src/lib.rs:42:10");
    }

    #[test]
    fn test_location_entry_new() {
        let entry = LocationEntry::new(PathBuf::from("test.rs"), 1, 1, "hello");
        assert_eq!(entry.line, 1);
        assert_eq!(entry.text, "hello");
    }
}
