//! Rope-based text storage.

use kjxlkj_core_types::{ByteOffset, CharOffset, LineCol};
use ropey::Rope;

/// Text storage using a rope data structure.
#[derive(Debug, Clone)]
pub struct RopeText {
    rope: Rope,
}

impl RopeText {
    /// Create empty text.
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Create text from a string.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
        }
    }

    /// Get the number of lines.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get the total number of bytes.
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Get the total number of chars.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_bytes() == 0
    }

    /// Get a line as a string (without trailing newline).
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx >= self.len_lines() {
            return None;
        }
        let line = self.rope.line(line_idx);
        let s = line.to_string();
        Some(s.trim_end_matches(&['\n', '\r'][..]).to_string())
    }

    /// Get the length of a line in characters (without newline).
    pub fn line_len_chars(&self, line_idx: usize) -> Option<usize> {
        self.line(line_idx).map(|s| s.chars().count())
    }

    /// Convert line/col to char offset.
    pub fn linecol_to_char(&self, pos: LineCol) -> Option<CharOffset> {
        if pos.line >= self.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.line_len_chars(pos.line)?;
        let col = pos.col.min(line_len);
        Some(CharOffset::new(line_start + col))
    }

    /// Convert char offset to line/col.
    pub fn char_to_linecol(&self, offset: CharOffset) -> LineCol {
        let offset = offset.as_usize().min(self.len_chars().saturating_sub(1));
        let line = self.rope.char_to_line(offset);
        let line_start = self.rope.line_to_char(line);
        let col = offset - line_start;
        LineCol::new(line, col)
    }

    /// Convert byte offset to char offset.
    pub fn byte_to_char(&self, offset: ByteOffset) -> CharOffset {
        let byte = offset.as_usize().min(self.len_bytes());
        CharOffset::new(self.rope.byte_to_char(byte))
    }

    /// Convert char offset to byte offset.
    pub fn char_to_byte(&self, offset: CharOffset) -> ByteOffset {
        let ch = offset.as_usize().min(self.len_chars());
        ByteOffset::new(self.rope.char_to_byte(ch))
    }

    /// Insert text at a char offset.
    pub fn insert(&mut self, offset: CharOffset, text: &str) {
        let idx = offset.as_usize().min(self.len_chars());
        self.rope.insert(idx, text);
    }

    /// Delete a range of characters.
    pub fn delete(&mut self, start: CharOffset, end: CharOffset) {
        let start_idx = start.as_usize().min(self.len_chars());
        let end_idx = end.as_usize().min(self.len_chars());
        if start_idx < end_idx {
            self.rope.remove(start_idx..end_idx);
        }
    }

    /// Get the entire text as a string.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get a slice of text as a string.
    pub fn slice(&self, start: CharOffset, end: CharOffset) -> String {
        let start_idx = start.as_usize().min(self.len_chars());
        let end_idx = end.as_usize().min(self.len_chars());
        if start_idx >= end_idx {
            return String::new();
        }
        self.rope.slice(start_idx..end_idx).to_string()
    }

    /// Clamp a position to valid bounds.
    pub fn clamp_position(&self, pos: LineCol) -> LineCol {
        if self.is_empty() {
            return LineCol::origin();
        }
        let line = pos.line.min(self.len_lines().saturating_sub(1));
        let line_len = self.line_len_chars(line).unwrap_or(0);
        let col = pos.col.min(line_len.saturating_sub(1).max(0));
        LineCol::new(line, col)
    }
}

impl Default for RopeText {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text() {
        let text = RopeText::new();
        assert!(text.is_empty());
        assert_eq!(text.len_lines(), 1);
    }

    #[test]
    fn from_string() {
        let text = RopeText::from_str("hello\nworld");
        assert_eq!(text.len_lines(), 2);
        assert_eq!(text.line(0), Some("hello".to_string()));
        assert_eq!(text.line(1), Some("world".to_string()));
    }

    #[test]
    fn insert_text() {
        let mut text = RopeText::from_str("hello");
        text.insert(CharOffset::new(5), " world");
        assert_eq!(text.to_string(), "hello world");
    }

    #[test]
    fn delete_text() {
        let mut text = RopeText::from_str("hello world");
        text.delete(CharOffset::new(5), CharOffset::new(11));
        assert_eq!(text.to_string(), "hello");
    }

    #[test]
    fn linecol_conversion() {
        let text = RopeText::from_str("hello\nworld");
        let pos = LineCol::new(1, 2);
        let offset = text.linecol_to_char(pos).unwrap();
        let back = text.char_to_linecol(offset);
        assert_eq!(back, pos);
    }

    #[test]
    fn clamp_position() {
        let text = RopeText::from_str("ab\ncd");
        let clamped = text.clamp_position(LineCol::new(10, 10));
        assert_eq!(clamped.line, 1);
        assert_eq!(clamped.col, 1);
    }
}
