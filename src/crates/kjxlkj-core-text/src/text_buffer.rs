//! Text buffer implementation.

use kjxlkj_core_types::{BufferVersion, Position, Range};
use ropey::Rope;

use crate::RopeExt;

/// A text buffer backed by a rope.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    rope: Rope,
    version: BufferVersion,
}

impl TextBuffer {
    /// Creates an empty text buffer.
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            version: BufferVersion::initial(),
        }
    }

    /// Creates a text buffer from a string.
    pub fn from_str(s: &str) -> Self {
        Self {
            rope: Rope::from_str(s),
            version: BufferVersion::initial(),
        }
    }

    /// Returns the current version.
    pub fn version(&self) -> BufferVersion {
        self.version
    }

    /// Returns the number of lines.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Returns the total character count.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Returns a line as a string (without line ending).
    pub fn line(&self, line_idx: usize) -> String {
        self.rope.line_content(line_idx)
    }

    /// Returns the grapheme count for a line.
    pub fn line_grapheme_count(&self, line_idx: usize) -> usize {
        self.rope.line_grapheme_count(line_idx)
    }

    /// Inserts text at a position.
    pub fn insert(&mut self, pos: Position, text: &str) {
        let char_idx = self.position_to_char_idx(pos);
        self.rope.insert(char_idx, text);
        self.version = self.version.next();
    }

    /// Deletes text in a range.
    pub fn delete(&mut self, range: Range) {
        let start_idx = self.position_to_char_idx(range.start);
        let end_idx = self.position_to_char_idx(range.end);
        if end_idx > start_idx {
            self.rope.remove(start_idx..end_idx);
            self.version = self.version.next();
        }
    }

    /// Returns the text in a range.
    pub fn slice(&self, range: Range) -> String {
        let start_idx = self.position_to_char_idx(range.start);
        let end_idx = self.position_to_char_idx(range.end);
        self.rope.slice(start_idx..end_idx).to_string()
    }

    /// Converts a position to a character index.
    pub fn position_to_char_idx(&self, pos: Position) -> usize {
        if pos.line >= self.rope.len_lines() {
            return self.rope.len_chars();
        }
        let line_start = self.rope.line_to_char(pos.line);
        let line_len = self.rope.line(pos.line).len_chars();
        line_start + pos.col.min(line_len)
    }

    /// Converts a character index to a position.
    pub fn char_idx_to_position(&self, char_idx: usize) -> Position {
        let line = self.rope.char_to_line(char_idx);
        let line_start = self.rope.line_to_char(line);
        let col = char_idx - line_start;
        Position::new(line, col)
    }

    /// Returns the full text content.
    pub fn to_string(&self) -> String {
        self.rope.to_string()
    }

    /// Returns a reference to the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.rope
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}
