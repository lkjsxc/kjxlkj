//! Text buffer implementation using rope data structure.

use kjxlkj_core_types::{BufferId, BufferVersion, Position};
use ropey::Rope;
use std::path::PathBuf;

/// A text buffer backed by a rope.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    /// Unique buffer identifier.
    id: BufferId,
    /// Buffer version (incremented on each edit).
    version: BufferVersion,
    /// Optional file path.
    path: Option<PathBuf>,
    /// The rope containing text content.
    rope: Rope,
    /// Whether buffer has unsaved changes.
    modified: bool,
}

impl TextBuffer {
    /// Create a new empty buffer.
    pub fn new(id: BufferId) -> Self {
        Self {
            id,
            version: BufferVersion::default(),
            path: None,
            rope: Rope::new(),
            modified: false,
        }
    }

    /// Create a buffer from string content.
    pub fn from_str(id: BufferId, content: &str) -> Self {
        Self {
            id,
            version: BufferVersion::default(),
            path: None,
            rope: Rope::from_str(content),
            modified: false,
        }
    }

    /// Get the buffer ID.
    pub fn id(&self) -> BufferId {
        self.id
    }

    /// Get the buffer version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Get the file path if any.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Set the file path.
    pub fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    /// Check if buffer has unsaved changes.
    pub fn is_modified(&self) -> bool {
        self.modified
    }

    /// Mark buffer as saved.
    pub fn mark_saved(&mut self) {
        self.modified = false;
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total character count.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get a line as a string (without trailing newline).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        let line = self.rope.line(line_idx);
        let s = line.to_string();
        Some(s.trim_end_matches(&['\n', '\r'][..]).to_string())
    }

    /// Get line length (excluding newline).
    pub fn line_len(&self, line_idx: usize) -> usize {
        self.line(line_idx).map(|s| s.chars().count()).unwrap_or(0)
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) {
        if let Some(char_idx) = self.pos_to_char_idx(pos) {
            self.rope.insert(char_idx, text);
            self.version.increment();
            self.modified = true;
        }
    }

    /// Delete a range of text.
    pub fn delete_range(&mut self, start: Position, end: Position) {
        let start_idx = self.pos_to_char_idx(start);
        let end_idx = self.pos_to_char_idx(end);
        if let (Some(s), Some(e)) = (start_idx, end_idx) {
            if s < e && e <= self.rope.len_chars() {
                self.rope.remove(s..e);
                self.version.increment();
                self.modified = true;
            }
        }
    }

    /// Get text in a range.
    pub fn text_range(&self, start: Position, end: Position) -> Option<String> {
        let start_idx = self.pos_to_char_idx(start)?;
        let end_idx = self.pos_to_char_idx(end)?;
        if start_idx <= end_idx && end_idx <= self.rope.len_chars() {
            Some(self.rope.slice(start_idx..end_idx).to_string())
        } else {
            None
        }
    }

    /// Get all text as a string.
    pub fn text(&self) -> String {
        self.rope.to_string()
    }

    /// Convert position to character index.
    pub fn pos_to_char_idx(&self, pos: Position) -> Option<usize> {
        if pos.line >= self.line_count() {
            return None;
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.line_len(pos.line);
        let col = pos.col.min(line_len);
        Some(line_start + col)
    }

    /// Convert character index to position.
    pub fn char_idx_to_pos(&self, char_idx: usize) -> Position {
        if char_idx >= self.rope.len_chars() {
            let last_line = self.line_count().saturating_sub(1);
            let last_col = self.line_len(last_line);
            return Position::new(last_line, last_col);
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        Position::new(line, col)
    }

    /// Get a reference to the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new(BufferId::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_insert_and_read() {
        let mut buf = TextBuffer::new(BufferId::new(1));
        buf.insert(Position::new(0, 0), "Hello, World!");
        assert_eq!(buf.line(0), Some("Hello, World!".to_string()));
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn buffer_multiline() {
        let buf = TextBuffer::from_str(BufferId::new(1), "Line 1\nLine 2\nLine 3");
        assert_eq!(buf.line_count(), 3);
        assert_eq!(buf.line(0), Some("Line 1".to_string()));
        assert_eq!(buf.line(1), Some("Line 2".to_string()));
        assert_eq!(buf.line(2), Some("Line 3".to_string()));
    }

    #[test]
    fn buffer_delete_range() {
        let mut buf = TextBuffer::from_str(BufferId::new(1), "Hello, World!");
        buf.delete_range(Position::new(0, 5), Position::new(0, 7));
        assert_eq!(buf.line(0), Some("HelloWorld!".to_string()));
    }

    #[test]
    fn buffer_position_conversion() {
        let buf = TextBuffer::from_str(BufferId::new(1), "AB\nCD\nEF");
        assert_eq!(buf.pos_to_char_idx(Position::new(0, 0)), Some(0));
        assert_eq!(buf.pos_to_char_idx(Position::new(1, 0)), Some(3));
        assert_eq!(buf.char_idx_to_pos(3), Position::new(1, 0));
    }
}
