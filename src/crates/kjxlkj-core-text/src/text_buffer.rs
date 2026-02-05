//! Text buffer implementation.

use kjxlkj_core_types::{Position, Range};
use ropey::Rope;
use std::fmt;
use std::str::FromStr;
use unicode_segmentation::UnicodeSegmentation;

/// Text buffer backed by a rope.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    version: u64,
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for TextBuffer {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rope: Rope::from_str(s),
            version: 0,
        })
    }
}

impl fmt::Display for TextBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rope)
    }
}

impl TextBuffer {
    /// Create an empty text buffer.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: 0,
        }
    }

    /// Get the buffer version (increments on each edit).
    pub fn version(&self) -> u64 {
        self.version
    }

    /// Get the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Get a line by index (0-based), without trailing newline.
    pub fn line(&self, idx: usize) -> Option<String> {
        if idx >= self.rope.len_lines() {
            return None;
        }
        let line = self.rope.line(idx);
        let s = line.to_string();
        Some(s.trim_end_matches(&['\r', '\n'][..]).to_string())
    }

    /// Get the length of a line in grapheme clusters.
    pub fn line_len(&self, idx: usize) -> usize {
        self.line(idx)
            .map(|s| s.graphemes(true).count())
            .unwrap_or(0)
    }

    /// Insert text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) {
        let char_idx = self.pos_to_char(pos);
        self.rope.insert(char_idx, text);
        self.version += 1;
    }

    /// Delete a range of text.
    pub fn delete(&mut self, range: Range) -> String {
        let start = self.pos_to_char(range.start);
        let end = self.pos_to_char(range.end);
        let deleted = self.rope.slice(start..end).to_string();
        self.rope.remove(start..end);
        self.version += 1;
        deleted
    }

    /// Replace text in a range.
    pub fn replace(&mut self, range: Range, text: &str) -> String {
        let deleted = self.delete(range);
        self.insert(range.start, text);
        deleted
    }

    /// Convert a position to a character index.
    pub fn pos_to_char(&self, pos: Position) -> usize {
        if pos.line >= self.rope.len_lines() {
            return self.rope.len_chars();
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line = self.rope.line(pos.line);
        let line_str = line.to_string();
        let mut char_offset = 0;
        for (i, g) in line_str.graphemes(true).enumerate() {
            if i >= pos.column {
                break;
            }
            char_offset += g.chars().count();
        }
        line_start + char_offset
    }

    /// Convert a character index to a position.
    pub fn char_to_pos(&self, char_idx: usize) -> Position {
        if char_idx >= self.rope.len_chars() {
            let line = self.rope.len_lines().saturating_sub(1);
            return Position::new(line, self.line_len(line));
        }
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let line_slice = self.rope.line(line);
        let line_str = line_slice.to_string();
        let offset = char_idx - line_start;
        let mut col = 0;
        let mut chars_counted = 0;
        for g in line_str.graphemes(true) {
            if chars_counted >= offset {
                break;
            }
            chars_counted += g.chars().count();
            col += 1;
        }
        Position::new(line, col)
    }

    /// Get a slice of the buffer.
    pub fn slice(&self, range: Range) -> String {
        let start = self.pos_to_char(range.start);
        let end = self.pos_to_char(range.end);
        self.rope.slice(start..end).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buf = TextBuffer::new();
        assert_eq!(buf.line_count(), 1);
        assert_eq!(buf.line(0), Some(String::new()));
    }

    #[test]
    fn test_from_str() {
        let buf: TextBuffer = "hello\nworld".parse().unwrap();
        assert_eq!(buf.line_count(), 2);
        assert_eq!(buf.line(0), Some("hello".to_string()));
        assert_eq!(buf.line(1), Some("world".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut buf = TextBuffer::new();
        buf.insert(Position::origin(), "hello");
        assert_eq!(buf.line(0), Some("hello".to_string()));
    }

    #[test]
    fn test_delete() {
        let mut buf: TextBuffer = "hello world".parse().unwrap();
        let range = Range::new(Position::new(0, 0), Position::new(0, 6));
        let deleted = buf.delete(range);
        assert_eq!(deleted, "hello ");
        assert_eq!(buf.line(0), Some("world".to_string()));
    }
}
