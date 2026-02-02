//! Text rope wrapper for efficient string operations.

use crate::grapheme;
use kjxlkj_core_types::position::Position;
use ropey::Rope;
use std::str::FromStr;

/// A text rope for efficient editing of large documents.
#[derive(Debug, Clone)]
pub struct TextRope {
    rope: Rope,
}

impl Default for TextRope {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRope {
    /// Creates an empty text rope.
    pub fn new() -> Self {
        Self { rope: Rope::new() }
    }

    /// Creates a text rope from a string slice.
    pub fn from_text(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
        }
    }

    /// Returns the total number of bytes.
    pub fn len_bytes(&self) -> usize {
        self.rope.len_bytes()
    }

    /// Returns the total number of characters.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns the number of grapheme clusters.
    pub fn len_graphemes(&self) -> usize {
        grapheme::grapheme_count(&self.rope.to_string())
    }

    /// Returns the display width of the entire text.
    pub fn display_width(&self) -> usize {
        grapheme::display_width(&self.rope.to_string())
    }

    /// Returns the number of lines.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns true if the rope is empty.
    pub fn is_empty(&self) -> bool {
        self.rope.len_chars() == 0
    }

    /// Returns the character at the given index.
    pub fn char_at(&self, idx: usize) -> Option<char> {
        if idx < self.len_chars() {
            Some(self.rope.char(idx))
        } else {
            None
        }
    }

    /// Returns a slice of the rope as a string.
    pub fn slice(&self, start: usize, end: usize) -> String {
        self.rope.slice(start..end).to_string()
    }

    /// Returns the line at the given index.
    pub fn line(&self, line_idx: usize) -> Option<String> {
        if line_idx < self.len_lines() {
            Some(self.rope.line(line_idx).to_string())
        } else {
            None
        }
    }

    /// Returns the number of lines (alias for len_lines).
    pub fn line_count(&self) -> usize {
        self.len_lines()
    }

    /// Returns the character index at the start of a line.
    pub fn line_to_char(&self, line_idx: usize) -> usize {
        self.rope.line_to_char(line_idx)
    }

    /// Returns the length of a line (including newline if present).
    pub fn line_len(&self, line_idx: usize) -> Option<usize> {
        if line_idx < self.len_lines() {
            Some(self.rope.line(line_idx).len_chars())
        } else {
            None
        }
    }

    /// Converts a line/column position to a character index.
    pub fn pos_to_char_idx(&self, pos: Position) -> Option<usize> {
        let line = pos.line.as_usize();
        if line >= self.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(line);
        let line_len = self.rope.line(line).len_chars();
        let col = pos.col.as_usize().min(line_len.saturating_sub(1));
        Some(line_start + col)
    }

    /// Converts a character index to a line/column position.
    pub fn char_idx_to_pos(&self, idx: usize) -> Option<Position> {
        if idx > self.len_chars() {
            return None;
        }
        let line = self.rope.char_to_line(idx);
        let line_start = self.rope.line_to_char(line);
        let col = idx - line_start;
        Some(Position::new(line, col))
    }

    /// Inserts text at the given character index.
    pub fn insert(&mut self, idx: usize, text: &str) {
        self.rope.insert(idx, text);
    }

    /// Removes text in the given character range.
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
    }

    /// Replaces text in the given range.
    pub fn replace(&mut self, start: usize, end: usize, text: &str) {
        self.rope.remove(start..end);
        self.rope.insert(start, text);
    }

    /// Returns the entire rope as a string.
    pub fn contents(&self) -> String {
        self.rope.to_string()
    }
}

impl std::fmt::Display for TextRope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rope)
    }
}

impl FromStr for TextRope {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_text(s))
    }
}

impl From<&str> for TextRope {
    fn from(s: &str) -> Self {
        Self::from_text(s)
    }
}

impl From<String> for TextRope {
    fn from(s: String) -> Self {
        Self::from_text(&s)
    }
}

#[cfg(test)]
#[path = "rope_tests.rs"]
mod tests;
