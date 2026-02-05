//! Text buffer implementation using rope.

use kjxlkj_core_types::{BufferVersion, Position, Range};
use ropey::Rope;
use thiserror::Error;
use unicode_segmentation::UnicodeSegmentation;

use crate::grapheme_count;

/// Errors that can occur during text operations.
#[derive(Debug, Error)]
pub enum TextError {
    #[error("line {0} out of bounds (max {1})")]
    LineOutOfBounds(usize, usize),

    #[error("column {0} out of bounds for line (max {1})")]
    ColumnOutOfBounds(usize, usize),

    #[error("invalid character index")]
    InvalidCharIndex,
}

/// A text buffer backed by a rope data structure.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    version: BufferVersion,
}

impl TextBuffer {
    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: BufferVersion::initial(),
        }
    }

    /// Create a buffer from a string.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
            version: BufferVersion::initial(),
        }
    }

    /// Get the current version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Increment and return the new version.
    fn bump_version(&mut self) -> BufferVersion {
        self.version = self.version.next();
        self.version
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total number of characters.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the total number of bytes.
    pub fn byte_count(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a line as a string (without trailing newline).
    pub fn line(&self, line_idx: usize) -> Result<String, TextError> {
        if line_idx >= self.line_count() {
            return Err(TextError::LineOutOfBounds(
                line_idx,
                self.line_count().saturating_sub(1),
            ));
        }
        let line = self.rope.line(line_idx);
        let s = line.to_string();
        Ok(s.trim_end_matches(&['\n', '\r'][..]).to_string())
    }

    /// Get the length of a line (excluding newline).
    pub fn line_len(&self, line_idx: usize) -> Result<usize, TextError> {
        let line = self.line(line_idx)?;
        Ok(grapheme_count(&line))
    }

    /// Get all text as a string.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Convert a position to a character index.
    pub fn pos_to_char(&self, pos: Position) -> Result<usize, TextError> {
        if pos.line >= self.line_count() {
            return Err(TextError::LineOutOfBounds(
                pos.line,
                self.line_count().saturating_sub(1),
            ));
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line = self.line(pos.line)?;
        let line_graphemes: Vec<&str> = line.graphemes(true).collect();

        if pos.column > line_graphemes.len() {
            return Err(TextError::ColumnOutOfBounds(pos.column, line_graphemes.len()));
        }

        // Convert grapheme offset to char offset
        let char_offset: usize = line_graphemes[..pos.column]
            .iter()
            .map(|g| g.chars().count())
            .sum();

        Ok(line_start + char_offset)
    }

    /// Convert a character index to a position.
    pub fn char_to_pos(&self, char_idx: usize) -> Result<Position, TextError> {
        if char_idx > self.char_count() {
            return Err(TextError::InvalidCharIndex);
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let char_in_line = char_idx - line_start;

        // Convert char offset to grapheme offset
        let line_text = self.line(line)?;
        let mut grapheme_col = 0;
        let mut char_count = 0;
        for g in line_text.graphemes(true) {
            if char_count >= char_in_line {
                break;
            }
            char_count += g.chars().count();
            grapheme_col += 1;
        }

        Ok(Position::new(line, grapheme_col))
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) -> Result<BufferVersion, TextError> {
        let char_idx = self.pos_to_char(pos)?;
        self.rope.insert(char_idx, text);
        Ok(self.bump_version())
    }

    /// Insert a character at a position.
    pub fn insert_char(&mut self, pos: Position, ch: char) -> Result<BufferVersion, TextError> {
        let char_idx = self.pos_to_char(pos)?;
        self.rope.insert_char(char_idx, ch);
        Ok(self.bump_version())
    }

    /// Delete a range of text.
    pub fn delete(&mut self, range: Range) -> Result<(String, BufferVersion), TextError> {
        let start_idx = self.pos_to_char(range.start)?;
        let end_idx = self.pos_to_char(range.end)?;

        if start_idx >= end_idx {
            return Ok((String::new(), self.version));
        }

        let deleted: String = self.rope.slice(start_idx..end_idx).to_string();
        self.rope.remove(start_idx..end_idx);
        Ok((deleted, self.bump_version()))
    }

    /// Delete a single character at a position.
    pub fn delete_char(&mut self, pos: Position) -> Result<(char, BufferVersion), TextError> {
        let char_idx = self.pos_to_char(pos)?;
        if char_idx >= self.char_count() {
            return Err(TextError::InvalidCharIndex);
        }
        let ch = self.rope.char(char_idx);
        self.rope.remove(char_idx..char_idx + 1);
        Ok((ch, self.bump_version()))
    }

    /// Get text in a range.
    pub fn slice(&self, range: Range) -> Result<String, TextError> {
        let start_idx = self.pos_to_char(range.start)?;
        let end_idx = self.pos_to_char(range.end)?;
        Ok(self.rope.slice(start_idx..end_idx).to_string())
    }

    /// Replace text in a range.
    pub fn replace(&mut self, range: Range, text: &str) -> Result<BufferVersion, TextError> {
        let start_idx = self.pos_to_char(range.start)?;
        let end_idx = self.pos_to_char(range.end)?;
        self.rope.remove(start_idx..end_idx);
        self.rope.insert(start_idx, text);
        Ok(self.bump_version())
    }

    /// Get lines in a range (for viewport rendering).
    pub fn lines_in_range(&self, start_line: usize, end_line: usize) -> Vec<String> {
        let start = start_line.min(self.line_count());
        let end = end_line.min(self.line_count());
        (start..end)
            .filter_map(|i| self.line(i).ok())
            .collect()
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.line_count(), 1);
    }

    #[test]
    fn test_from_str() {
        let buf = TextBuffer::from_str("hello\nworld");
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0).unwrap(), "hello");
        assert_eq!(buf.line(1).unwrap(), "world");
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::new();
        buf.insert(Position::origin(), "hello").unwrap();
        assert_eq!(buf.to_string(), "hello");
    }

    #[test]
    fn test_insert_char() {
        let mut buf = TextBuffer::from_str("hllo");
        buf.insert_char(Position::new(0, 1), 'e').unwrap();
        assert_eq!(buf.line(0).unwrap(), "hello");
    }

    #[test]
    fn test_delete() {
        let mut buf = TextBuffer::from_str("hello");
        let (deleted, _) = buf.delete(Range::from_coords(0, 1, 0, 3)).unwrap();
        assert_eq!(deleted, "el");
        assert_eq!(buf.line(0).unwrap(), "hlo");
    }

    #[test]
    fn test_line_len() {
        let buf = TextBuffer::from_str("hello\n");
        assert_eq!(buf.line_len(0).unwrap(), 5);
    }

    #[test]
    fn test_version_increments() {
        let mut buf = TextBuffer::new();
        let v0 = buf.version();
        buf.insert(Position::origin(), "a").unwrap();
        let v1 = buf.version();
        assert!(v1 > v0);
    }

    #[test]
    fn test_lines_in_range() {
        let buf = TextBuffer::from_str("a\nb\nc\nd");
        let lines = buf.lines_in_range(1, 3);
        assert_eq!(lines, vec!["b", "c"]);
    }
}
