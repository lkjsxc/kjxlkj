//! Text buffer implementation using rope.

use crate::rope_ext::RopeSliceExt;
use kjxlkj_core_types::{BufferId, BufferName, BufferVersion};
use ropey::Rope;
use std::path::PathBuf;

/// A text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    /// Unique buffer identifier.
    id: BufferId,
    /// Display name.
    name: BufferName,
    /// Optional file path.
    path: Option<PathBuf>,
    /// The text content.
    content: Rope,
    /// Current version.
    version: BufferVersion,
    /// Whether the buffer has been modified since last save.
    modified: bool,
}

impl TextBuffer {
    /// Create a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::default(),
            path: None,
            content: Rope::new(),
            version: BufferVersion::INITIAL,
            modified: false,
        }
    }

    /// Create a buffer from text content.
    pub fn from_text(id: BufferId, text: &str) -> Self {
        Self {
            id,
            name: BufferName::default(),
            path: None,
            content: Rope::from_str(text),
            version: BufferVersion::INITIAL,
            modified: false,
        }
    }

    /// Create a buffer from a file path and content.
    pub fn from_file(id: BufferId, path: PathBuf, text: &str) -> Self {
        let name = path
            .file_name()
            .map(|n| BufferName::new(n.to_string_lossy()))
            .unwrap_or_default();
        Self {
            id,
            name,
            path: Some(path),
            content: Rope::from_str(text),
            version: BufferVersion::INITIAL,
            modified: false,
        }
    }

    /// Get the buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get the buffer name.
    pub fn name(&self) -> &BufferName {
        &self.name
    }

    /// Set the buffer name.
    pub fn set_name(&mut self, name: BufferName) {
        self.name = name;
    }

    /// Get the file path if set.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Set the file path.
    pub fn set_path(&mut self, path: PathBuf) {
        self.name = path
            .file_name()
            .map(|n| BufferName::new(n.to_string_lossy()))
            .unwrap_or_default();
        self.path = Some(path);
    }

    /// Get the current version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Check if the buffer has been modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark the buffer as saved.
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }

    /// Get the total character count.
    pub fn char_count(&self) -> usize {
        self.content.len_chars()
    }

    /// Get a line by index (0-based).
    pub fn line(&self, idx: usize) -> Option<ropey::RopeSlice<'_>> {
        if idx < self.line_count() {
            Some(self.content.line(idx))
        } else {
            None
        }
    }

    /// Get the length of a line in grapheme clusters.
    pub fn line_grapheme_len(&self, idx: usize) -> usize {
        self.line(idx)
            .map(|l| {
                // Use rope_grapheme_count for large lines that may span multiple chunks
                let mut len = crate::grapheme::rope_grapheme_count(l);
                // Exclude trailing newline from count
                if l.len_chars() > 0 {
                    let last_char_idx = l.len_chars() - 1;
                    let last_char = l.char(last_char_idx);
                    if last_char == '\n' {
                        len = len.saturating_sub(1);
                        // Also check for \r\n
                        if l.len_chars() > 1 {
                            let second_last_idx = l.len_chars() - 2;
                            if l.char(second_last_idx) == '\r' {
                                len = len.saturating_sub(1);
                            }
                        }
                    }
                }
                len
            })
            .unwrap_or(0)
    }

    /// Get the entire content as a string.
    pub fn to_string(&self) -> String {
        self.content.to_string()
    }

    /// Get a rope slice of the content.
    pub fn slice(&self) -> ropey::RopeSlice<'_> {
        self.content.slice(..)
    }

    /// Get the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.content
    }

    /// Insert text at a character index.
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        let idx = char_idx.min(self.content.len_chars());
        self.content.insert(idx, text);
        self.version = self.version.next();
        self.modified = true;
    }

    /// Insert text at a line and column.
    pub fn insert_at(&mut self, line: usize, col: usize, text: &str) {
        let line = line.min(self.line_count().saturating_sub(1).max(0));
        let line_start = self.content.line_to_char(line);
        let line_len = self.line_grapheme_len(line);
        let col = col.min(line_len);

        // Convert column (grapheme index) to char index
        let char_offset = if let Some(line_slice) = self.line(line) {
            let mut offset = 0;
            for (i, g) in line_slice.graphemes().enumerate() {
                if i == col {
                    break;
                }
                offset += g.chars().count();
            }
            offset
        } else {
            0
        };

        self.insert(line_start + char_offset, text);
    }

    /// Remove a range of characters.
    pub fn remove(&mut self, start: usize, end: usize) {
        let start = start.min(self.content.len_chars());
        let end = end.min(self.content.len_chars());
        if start < end {
            self.content.remove(start..end);
            self.version = self.version.next();
            self.modified = true;
        }
    }

    /// Remove a line by index.
    pub fn remove_line(&mut self, idx: usize) {
        let line_count = self.line_count();
        if idx >= line_count {
            return;
        }

        let start = self.content.line_to_char(idx);
        let end = if idx + 1 < line_count {
            // Not the last line: remove including the trailing newline
            self.content.line_to_char(idx + 1)
        } else if idx > 0 {
            // Last line and not the only line: also remove the preceding newline
            // Actually, we should remove from the previous line's end to this line's end
            self.content.len_chars()
        } else {
            // Only line in the buffer: remove everything
            self.content.len_chars()
        };

        // Special case: if removing last line and there are multiple lines,
        // also remove the newline before it
        if idx + 1 >= line_count && idx > 0 {
            // Remove including the previous newline
            let newline_pos = start.saturating_sub(1);
            self.remove(newline_pos, end);
        } else {
            self.remove(start, end);
        }
    }

    /// Replace all content.
    pub fn replace_all(&mut self, text: &str) {
        self.content = Rope::from_str(text);
        self.version = self.version.next();
        self.modified = true;
    }

    /// Get the character index for a line start.
    pub fn line_to_char(&self, line: usize) -> usize {
        if line >= self.line_count() {
            self.content.len_chars()
        } else {
            self.content.line_to_char(line)
        }
    }

    /// Get the line index for a character index.
    pub fn char_to_line(&self, char_idx: usize) -> usize {
        let idx = char_idx.min(self.content.len_chars());
        self.content.char_to_line(idx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = TextBuffer::new(BufferId::new(1));
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.char_count(), 0);
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_from_text() {
        let buf = TextBuffer::from_text(BufferId::new(1), "hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::new(BufferId::new(1));
        buf.insert(0, "hello");
        assert_eq!(buf.to_string(), "hello");
        assert!(buf.is_modified());
    }

    #[test]
    fn test_remove() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), "hello");
        buf.remove(0, 2);
        assert_eq!(buf.to_string(), "llo");
    }

    #[test]
    fn test_line_grapheme_len() {
        let buf = TextBuffer::from_text(BufferId::new(1), "héllo\nworld\n");
        assert_eq!(buf.line_grapheme_len(0), 5);
        assert_eq!(buf.line_grapheme_len(1), 5);
    }

    #[test]
    fn test_buffer_id() {
        let buf = TextBuffer::new(BufferId::new(42));
        assert_eq!(buf.id(), BufferId::new(42));
    }

    #[test]
    fn test_mark_saved() {
        let mut buf = TextBuffer::new(BufferId::new(1));
        buf.insert(0, "hello");
        assert!(buf.is_modified());
        buf.mark_saved();
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_char_count() {
        let buf = TextBuffer::from_text(BufferId::new(1), "hello\n");
        assert_eq!(buf.char_count(), 6); // 5 chars + newline
    }

    #[test]
    fn test_empty_buffer_line_count() {
        let buf = TextBuffer::from_text(BufferId::new(1), "");
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_line_to_char() {
        let buf = TextBuffer::from_text(BufferId::new(1), "hello\nworld\n");
        assert_eq!(buf.line_to_char(0), 0);
        assert_eq!(buf.line_to_char(1), 6);
        assert_eq!(buf.line_to_char(2), 12);
    }

    #[test]
    fn test_insert_newline() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), "hello");
        buf.insert(5, "\n");
        assert_eq!(buf.line_count(), 2);
    }

    #[test]
    fn test_remove_newline() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), "hello\nworld");
        buf.remove(5, 6); // Remove newline
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.to_string(), "helloworld");
    }

    #[test]
    fn test_multiple_inserts() {
        let mut buf = TextBuffer::new(BufferId::new(1));
        buf.insert(0, "a");
        buf.insert(1, "b");
        buf.insert(2, "c");
        assert_eq!(buf.to_string(), "abc");
    }

    #[test]
    fn test_insert_at_middle() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), "helloworld");
        buf.insert(5, " ");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_version_increment() {
        let mut buf = TextBuffer::new(BufferId::new(1));
        let v1 = buf.version();
        buf.insert(0, "x");
        let v2 = buf.version();
        assert_ne!(v1, v2);
    }
}
