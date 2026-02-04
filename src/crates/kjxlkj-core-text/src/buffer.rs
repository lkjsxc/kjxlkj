//! Text buffer backed by a rope.

use kjxlkj_core_types::Position;
use ropey::Rope;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;

/// A text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextBuffer {
    /// Create an empty buffer.
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Create a buffer from a string.
    pub fn from_text(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
        }
    }

    /// Load a buffer from a file (streaming).
    pub fn from_file(path: &Path) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let rope = Rope::from_reader(reader)?;
        Ok(Self { rope })
    }

    /// Save the buffer to a file.
    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        for chunk in self.rope.chunks() {
            writer.write_all(chunk.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total character count.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a line as a string (without trailing newline).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.line_count() {
            return None;
        }
        let line = self.rope.line(line_idx);
        let s = line.to_string();
        Some(s.trim_end_matches('\n').trim_end_matches('\r').to_string())
    }

    /// Get the length of a line (excluding newline).
    pub fn line_len(&self, line_idx: usize) -> usize {
        self.line(line_idx).map(|s| s.chars().count()).unwrap_or(0)
    }

    /// Convert a position to a character index.
    pub fn pos_to_char(&self, pos: Position) -> usize {
        if pos.line >= self.line_count() {
            return self.char_count();
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.line_len(pos.line);
        line_start + pos.col.min(line_len)
    }

    /// Convert a character index to a position.
    pub fn char_to_pos(&self, char_idx: usize) -> Position {
        let char_idx = char_idx.min(self.char_count());
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        Position::new(line, char_idx - line_start)
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) {
        let char_idx = self.pos_to_char(pos);
        self.rope.insert(char_idx, text);
    }

    /// Delete a range of text.
    pub fn delete(&mut self, start: Position, end: Position) -> String {
        let start_idx = self.pos_to_char(start);
        let end_idx = self.pos_to_char(end);
        let (start_idx, end_idx) = if start_idx <= end_idx {
            (start_idx, end_idx)
        } else {
            (end_idx, start_idx)
        };
        let deleted: String = self.rope.slice(start_idx..end_idx).to_string();
        self.rope.remove(start_idx..end_idx);
        deleted
    }

    /// Get a slice of text between two positions.
    pub fn slice(&self, start: Position, end: Position) -> String {
        let start_idx = self.pos_to_char(start);
        let end_idx = self.pos_to_char(end);
        let (start_idx, end_idx) = if start_idx <= end_idx {
            (start_idx, end_idx)
        } else {
            (end_idx, start_idx)
        };
        self.rope.slice(start_idx..end_idx).to_string()
    }

    /// Get the entire buffer as a string.
    pub fn contents(&self) -> String {
        self.rope.to_string()
    }

    /// Get a character at a position.
    pub fn char_at(&self, pos: Position) -> Option<char> {
        let idx = self.pos_to_char(pos);
        if idx < self.char_count() {
            self.rope.get_char(idx)
        } else {
            None
        }
    }

    /// Get lines in a range (for viewport rendering).
    pub fn lines_range(&self, start: usize, end: usize) -> Vec<String> {
        let end = end.min(self.line_count());
        (start..end).filter_map(|i| self.line(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_buffer() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_from_str() {
        let buf = TextBuffer::from_text("hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), Some("hello".to_string()));
        assert_eq!(buf.line(1), Some("world".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::from_text("hello");
        buf.insert(Position::new(0, 5), " world");
        assert_eq!(buf.line(0), Some("hello world".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut buf = TextBuffer::from_text("hello world");
        let deleted = buf.delete(Position::new(0, 5), Position::new(0, 11));
        assert_eq!(deleted, " world");
        assert_eq!(buf.line(0), Some("hello".to_string()));
    }

    #[test]
    fn test_position_conversion() {
        let buf = TextBuffer::from_text("abc\ndef\nghi");
        assert_eq!(buf.pos_to_char(Position::new(1, 1)), 5);
        assert_eq!(buf.char_to_pos(5), Position::new(1, 1));
    }
}
