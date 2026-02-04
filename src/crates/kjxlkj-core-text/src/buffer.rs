//! Text buffer using a rope.

use crate::grapheme::{grapheme_count, nth_grapheme_offset};
use kjxlkj_core_types::Cursor;
use ropey::Rope;
use std::io::{BufRead, BufReader, Read};

/// A text buffer backed by a rope.
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
    /// Create an empty text buffer.
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Create a buffer from a string.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
        }
    }

    /// Load content from a reader (streaming).
    pub fn from_reader<R: Read>(reader: R) -> std::io::Result<Self> {
        let mut rope = Rope::new();
        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            let line = line?;
            rope.append(Rope::from_str(&line));
            rope.append(Rope::from_str("\n"));
        }
        // Remove trailing newline if added
        let len = rope.len_chars();
        if len > 0 && rope.char(len - 1) == '\n' {
            // Keep it, standard behavior
        }
        Ok(Self { rope })
    }

    /// Get the total length in bytes.
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Get the total length in characters.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the number of lines.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Check if buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Get a line by index (0-based).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.len_lines() {
            return None;
        }
        Some(self.rope.line(line_idx).to_string())
    }

    /// Get line content without trailing newline.
    pub fn line_content(&self, line_idx: usize) -> Option<String> {
        self.line(line_idx).map(|s| {
            s.trim_end_matches('\n')
                .trim_end_matches('\r')
                .to_string()
        })
    }

    /// Get grapheme count for a line.
    pub fn line_grapheme_count(&self, line_idx: usize) -> usize {
        self.line_content(line_idx)
            .map(|s| grapheme_count(&s))
            .unwrap_or(0)
    }

    /// Convert cursor to byte offset.
    pub fn cursor_to_byte(&self, cursor: Cursor) -> Option<usize> {
        if cursor.line >= self.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_byte(cursor.line);
        let line = self.line_content(cursor.line)?;
        let col_offset = nth_grapheme_offset(&line, cursor.col)?;
        Some(line_start + col_offset)
    }

    /// Convert cursor to char offset.
    pub fn cursor_to_char(&self, cursor: Cursor) -> Option<usize> {
        let byte = self.cursor_to_byte(cursor)?;
        Some(self.rope.byte_to_char(byte))
    }

    /// Insert text at a byte offset.
    pub fn insert(&mut self, byte_idx: usize, text: &str) {
        let char_idx = self.rope.byte_to_char(byte_idx);
        self.rope.insert(char_idx, text);
    }

    /// Insert text at cursor position.
    pub fn insert_at_cursor(&mut self, cursor: Cursor, text: &str) {
        if let Some(byte) = self.cursor_to_byte(cursor) {
            self.insert(byte, text);
        }
    }

    /// Delete a range of bytes.
    pub fn delete_range(&mut self, start_byte: usize, end_byte: usize) {
        let start_char = self.rope.byte_to_char(start_byte);
        let end_char = self.rope.byte_to_char(end_byte);
        self.rope.remove(start_char..end_char);
    }

    /// Delete text from start to end cursor.
    pub fn delete_cursor_range(&mut self, start: Cursor, end: Cursor) -> Option<String> {
        let start_byte = self.cursor_to_byte(start)?;
        let end_byte = self.cursor_to_byte(end)?;
        let (s, e) = if start_byte <= end_byte {
            (start_byte, end_byte)
        } else {
            (end_byte, start_byte)
        };
        let deleted = self.slice_bytes(s, e);
        self.delete_range(s, e);
        Some(deleted)
    }

    /// Get a slice of text by byte range.
    pub fn slice_bytes(&self, start: usize, end: usize) -> String {
        let start_char = self.rope.byte_to_char(start);
        let end_char = self.rope.byte_to_char(end);
        self.rope.slice(start_char..end_char).to_string()
    }

    /// Get the entire text as a string.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Replace entire contents.
    pub fn set_text(&mut self, text: &str) {
        self.rope = Rope::from_str(text);
    }

    /// Clamp cursor to valid position.
    pub fn clamp_cursor(&self, cursor: Cursor) -> Cursor {
        let line = cursor.line.min(self.len_lines().saturating_sub(1));
        let max_col = self.line_grapheme_count(line).saturating_sub(1);
        let col = cursor.col.min(max_col);
        Cursor::new(line, col)
    }

    /// Clamp cursor for insert mode (can be one past end).
    pub fn clamp_cursor_insert(&self, cursor: Cursor) -> Cursor {
        let line = cursor.line.min(self.len_lines().saturating_sub(1));
        let max_col = self.line_grapheme_count(line);
        let col = cursor.col.min(max_col);
        Cursor::new(line, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
    }

    #[test]
    fn test_from_str() {
        let buf = TextBuffer::from_str("hello\nworld");
        assert_eq!(buf.len_lines(), 2);
        assert_eq!(buf.line_content(0), Some("hello".to_string()));
        assert_eq!(buf.line_content(1), Some("world".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::from_str("hello");
        buf.insert(5, " world");
        assert_eq!(buf.to_string(), "hello world");
    }

    #[test]
    fn test_cursor_to_byte() {
        let buf = TextBuffer::from_str("hello\nworld");
        assert_eq!(buf.cursor_to_byte(Cursor::new(0, 0)), Some(0));
        assert_eq!(buf.cursor_to_byte(Cursor::new(0, 5)), Some(5));
        assert_eq!(buf.cursor_to_byte(Cursor::new(1, 0)), Some(6));
    }
}
