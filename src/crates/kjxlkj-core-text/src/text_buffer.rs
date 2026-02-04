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
                let s = l.as_str().unwrap_or("");
                // Exclude newline from count
                let s = s.trim_end_matches('\n').trim_end_matches('\r');
                crate::grapheme::grapheme_count(s)
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
        if idx < self.line_count() {
            let start = self.content.line_to_char(idx);
            let end = if idx + 1 < self.line_count() {
                self.content.line_to_char(idx + 1)
            } else {
                self.content.len_chars()
            };
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
}
