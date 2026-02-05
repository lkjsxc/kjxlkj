//! Text buffer implementation using Rope.

use kjxlkj_core_types::{BufferId, BufferMeta, BufferName, BufferVersion, CursorPosition};
use ropey::Rope;
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    /// Unique identifier.
    id: BufferId,
    /// Display name.
    name: BufferName,
    /// Optional file path.
    path: Option<PathBuf>,
    /// The rope containing text.
    rope: Rope,
    /// Whether the buffer has unsaved changes.
    modified: bool,
    /// Buffer version for snapshot tagging.
    version: BufferVersion,
}

impl TextBuffer {
    /// Create a new empty buffer.
    pub fn new(id: BufferId, name: BufferName) -> Self {
        Self {
            id,
            name,
            path: None,
            rope: Rope::new(),
            modified: false,
            version: BufferVersion::new(0),
        }
    }

    /// Create a buffer from text content.
    pub fn from_text(id: BufferId, name: BufferName, text: &str) -> Self {
        Self {
            id,
            name,
            path: None,
            rope: Rope::from_str(text),
            modified: false,
            version: BufferVersion::new(0),
        }
    }

    /// Create a buffer from a file.
    pub fn from_file(id: BufferId, path: PathBuf, content: &str) -> Self {
        let name = BufferName::new(
            path.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "[No Name]".to_string()),
        );
        Self {
            id,
            name,
            path: Some(path),
            rope: Rope::from_str(content),
            modified: false,
            version: BufferVersion::new(0),
        }
    }

    /// Get buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get buffer name.
    pub fn name(&self) -> &BufferName {
        &self.name
    }

    /// Get file path.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Set file path.
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    /// Check if modified.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark as saved.
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Get version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Get line count.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines().max(1)
    }

    /// Get total character count.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get a line's text content.
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.rope.len_lines() {
            return None;
        }
        let line = self.rope.line(line_idx);
        Some(line.to_string())
    }

    /// Get line length in characters (excluding newline).
    pub fn line_len(&self, line_idx: usize) -> usize {
        if line_idx >= self.rope.len_lines() {
            return 0;
        }
        let line = self.rope.line(line_idx);
        let len = line.len_chars();
        // Exclude trailing newline if present
        if len > 0 && line.char(len - 1) == '\n' {
            len - 1
        } else {
            len
        }
    }

    /// Get display width of a line.
    pub fn line_display_width(&self, line_idx: usize) -> usize {
        self.line(line_idx)
            .map(|s| s.trim_end_matches('\n').width())
            .unwrap_or(0)
    }

    /// Get grapheme count for a line.
    pub fn line_grapheme_count(&self, line_idx: usize) -> usize {
        self.line(line_idx)
            .map(|s| s.trim_end_matches('\n').graphemes(true).count())
            .unwrap_or(0)
    }

    /// Get all text.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Get text in range.
    pub fn text_range(&self, start_line: usize, end_line: usize) -> String {
        let start = self.line_to_char(start_line);
        let end = self.line_to_char(end_line.min(self.line_count()));
        self.rope.slice(start..end).to_string()
    }

    /// Convert line index to char index.
    pub fn line_to_char(&self, line_idx: usize) -> usize {
        if line_idx >= self.rope.len_lines() {
            self.rope.len_chars()
        } else {
            self.rope.line_to_char(line_idx)
        }
    }

    /// Insert text at position.
    pub fn insert(&mut self, pos: CursorPosition, text: &str) {
        let char_idx = self.pos_to_char(pos);
        self.rope.insert(char_idx, text);
        self.modified = true;
        self.version.increment();
    }

    /// Delete character at position.
    pub fn delete_char(&mut self, pos: CursorPosition) {
        let char_idx = self.pos_to_char(pos);
        if char_idx < self.rope.len_chars() {
            self.rope.remove(char_idx..char_idx + 1);
            self.modified = true;
            self.version.increment();
        }
    }

    /// Delete range of text.
    pub fn delete_range(&mut self, start: CursorPosition, end: CursorPosition) {
        let start_idx = self.pos_to_char(start);
        let end_idx = self.pos_to_char(end);
        if start_idx < end_idx && end_idx <= self.rope.len_chars() {
            self.rope.remove(start_idx..end_idx);
            self.modified = true;
            self.version.increment();
        }
    }

    /// Convert cursor position to char index.
    pub fn pos_to_char(&self, pos: CursorPosition) -> usize {
        let line = pos.line.min(self.line_count().saturating_sub(1));
        let line_start = self.line_to_char(line);
        let line_len = self.line_len(line);
        let col = pos.column.min(line_len);
        line_start + col
    }

    /// Convert char index to cursor position.
    pub fn char_to_pos(&self, char_idx: usize) -> CursorPosition {
        let char_idx = char_idx.min(self.rope.len_chars());
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let column = char_idx - line_start;
        CursorPosition::new(line, column)
    }

    /// Clamp cursor to valid position.
    pub fn clamp_cursor(&self, pos: CursorPosition, end_inclusive: bool) -> CursorPosition {
        let line = pos.line.min(self.line_count().saturating_sub(1));
        let line_len = self.line_len(line);
        let max_col = if end_inclusive {
            line_len
        } else {
            line_len.saturating_sub(1).max(0)
        };
        let column = pos.column.min(max_col);
        CursorPosition::new(line, column)
    }

    /// Get buffer metadata.
    pub fn meta(&self) -> BufferMeta {
        BufferMeta {
            id: self.id,
            name: self.name.clone(),
            path: self.path.clone(),
            modified: self.modified,
            line_count: self.line_count(),
            version: self.version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = TextBuffer::new(BufferId::new(1), BufferName::new("test"));
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.char_count(), 0);
        assert!(!buf.is_modified());
    }

    #[test]
    fn test_from_text() {
        let buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), Some("hello\n".to_string()));
        assert_eq!(buf.line(1), Some("world".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello");
        buf.insert(CursorPosition::new(0, 5), " world");
        assert_eq!(buf.text(), "hello world");
        assert!(buf.is_modified());
    }

    #[test]
    fn test_delete() {
        let mut buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello");
        buf.delete_char(CursorPosition::new(0, 0));
        assert_eq!(buf.text(), "ello");
    }

    #[test]
    fn test_line_len() {
        let buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello\nworld");
        assert_eq!(buf.line_len(0), 5);
        assert_eq!(buf.line_len(1), 5);
    }

    #[test]
    fn test_clamp_cursor() {
        let buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello");
        let clamped = buf.clamp_cursor(CursorPosition::new(0, 100), false);
        assert_eq!(clamped.column, 4);
        let clamped = buf.clamp_cursor(CursorPosition::new(0, 100), true);
        assert_eq!(clamped.column, 5);
    }
}
