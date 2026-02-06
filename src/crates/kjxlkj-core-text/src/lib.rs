//! Text model backed by a rope data structure.

use kjxlkj_core_types::Position;
use ropey::Rope;

/// The primary text storage, wrapping a rope for efficient editing.
pub struct TextBuffer {
    rope: Rope,
}

impl TextBuffer {
    /// Create an empty text buffer.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
        }
    }

    /// Create a text buffer from a string.
    pub fn from_str(text: &str) -> Self {
        Self {
            rope: Rope::from_str(text),
        }
    }

    /// Return the number of lines in the buffer.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Return the total number of characters.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Return the text of a given line (zero-based), including the trailing newline if present.
    pub fn line(&self, line_idx: usize) -> Option<ropey::RopeSlice<'_>> {
        if line_idx < self.rope.len_lines() {
            Some(self.rope.line(line_idx))
        } else {
            None
        }
    }

    /// Insert text at the given character index.
    pub fn insert(&mut self, char_idx: usize, text: &str) {
        self.rope.insert(char_idx, text);
    }

    /// Remove text in the given character range.
    pub fn remove(&mut self, start: usize, end: usize) {
        self.rope.remove(start..end);
    }

    /// Convert a `Position` to a character index, returning `None` if out of bounds.
    pub fn pos_to_char_idx(&self, pos: Position) -> Option<usize> {
        if pos.line >= self.rope.len_lines() {
            return None;
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.rope.line(pos.line).len_chars();
        if pos.col > line_len {
            return None;
        }
        Some(line_start + pos.col)
    }

    /// Convert a character index to a `Position`.
    pub fn char_idx_to_pos(&self, char_idx: usize) -> Position {
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        Position::new(line, char_idx - line_start)
    }

    /// Get the full contents as a `String`.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Get a reference to the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
