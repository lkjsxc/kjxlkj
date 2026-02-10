//! Text buffer wrapping a Rope with buffer metadata.

use kjxlkj_core_types::{BufferId, BufferName, BufferVersion, Encoding, LineEnding};
use ropey::Rope;
use std::path::PathBuf;

/// A text buffer with metadata.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    pub id: BufferId,
    pub name: BufferName,
    pub path: Option<PathBuf>,
    pub content: Rope,
    pub modified: bool,
    pub version: BufferVersion,
    pub encoding: Encoding,
    pub line_ending: LineEnding,
    pub readonly: bool,
}

impl TextBuffer {
    /// Create a new empty scratch buffer.
    pub fn new_scratch(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::Scratch,
            path: None,
            content: Rope::new(),
            modified: false,
            version: BufferVersion::new(),
            encoding: Encoding::default(),
            line_ending: LineEnding::default(),
            readonly: false,
        }
    }

    /// Create a buffer from file content.
    pub fn from_file(id: BufferId, path: PathBuf, content: String) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        let line_ending = if content.contains("\r\n") {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        };
        Self {
            id,
            name: BufferName::File(name),
            path: Some(path),
            content: Rope::from_str(&content),
            modified: false,
            version: BufferVersion::new(),
            encoding: Encoding::Utf8,
            line_ending,
            readonly: false,
        }
    }

    /// Total line count.
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get line content as a string.
    pub fn line(&self, idx: usize) -> Option<String> {
        if idx < self.content.len_lines() {
            Some(self.content.line(idx).to_string())
        } else {
            None
        }
    }

    /// Insert text at a byte offset.
    pub fn insert(&mut self, byte_offset: usize, text: &str) {
        let char_idx = self.content.byte_to_char(byte_offset);
        self.content.insert(char_idx, text);
        self.version.increment();
        self.modified = true;
    }

    /// Insert text at a char index.
    pub fn insert_at_char(&mut self, char_idx: usize, text: &str) {
        self.content.insert(char_idx, text);
        self.version.increment();
        self.modified = true;
    }

    /// Remove a byte range.
    pub fn remove(&mut self, start: usize, end: usize) {
        let start_char = self.content.byte_to_char(start);
        let end_char = self.content.byte_to_char(end);
        self.content.remove(start_char..end_char);
        self.version.increment();
        self.modified = true;
    }

    /// Remove a char range.
    pub fn remove_char_range(&mut self, start: usize, end: usize) {
        self.content.remove(start..end);
        self.version.increment();
        self.modified = true;
    }

    /// Get content as string.
    pub fn to_string_content(&self) -> String {
        self.content.to_string()
    }

    /// Get the byte offset for a line start.
    pub fn line_to_byte(&self, line: usize) -> usize {
        self.content.line_to_byte(line)
    }

    /// Get the char index for a line start.
    pub fn line_to_char(&self, line: usize) -> usize {
        self.content.line_to_char(line)
    }

    /// Clone the rope for snapshots (cheap via structural sharing).
    pub fn snapshot_rope(&self) -> Rope {
        self.content.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_scratch() {
        let buf = TextBuffer::new_scratch(BufferId(1));
        assert_eq!(buf.name, BufferName::Scratch);
        assert!(!buf.modified);
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_from_file() {
        let buf = TextBuffer::from_file(
            BufferId(1),
            PathBuf::from("test.txt"),
            "hello\nworld\n".to_string(),
        );
        assert_eq!(buf.name, BufferName::File("test.txt".into()));
        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.line_ending, LineEnding::Lf);
    }

    #[test]
    fn test_insert_and_modify() {
        let mut buf = TextBuffer::new_scratch(BufferId(1));
        buf.insert_at_char(0, "hello");
        assert!(buf.modified);
        assert_eq!(buf.to_string_content(), "hello");
    }
}
