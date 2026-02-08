//! Buffer content: rope wrapper with grapheme-aware operations.
//!
//! Wraps `ropey::Rope` to provide the edit operations specified in
//! /docs/spec/editor/buffers.md.

use ropey::Rope;

use crate::grapheme::LineGraphemes;
use crate::line_ops::LineEnding;

/// The text content of a buffer stored as a rope.
///
/// Provides O(log n) inserts/deletes, efficient line indexing,
/// and cheap snapshot cloning via structural sharing.
#[derive(Debug, Clone)]
pub struct BufferContent {
    /// The underlying rope.
    rope: Rope,
    /// Detected line ending style.
    line_ending: LineEnding,
}

impl BufferContent {
    /// Create an empty buffer content.
    pub fn empty() -> Self {
        Self {
            rope: Rope::new(),
            line_ending: LineEnding::default(),
        }
    }

    /// Create from a string.
    pub fn from_str(s: &str) -> Self {
        let line_ending = crate::line_ops::detect_line_ending(s);
        Self {
            rope: Rope::from_str(s),
            line_ending,
        }
    }

    /// Get a reference to the underlying rope.
    pub fn rope(&self) -> &Rope {
        &self.rope
    }

    /// Get a mutable reference to the underlying rope.
    pub fn rope_mut(&mut self) -> &mut Rope {
        &mut self.rope
    }

    /// Clone the rope (cheap via structural sharing).
    pub fn snapshot(&self) -> Rope {
        self.rope.clone()
    }

    /// Line ending style.
    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    /// Set the line ending style.
    pub fn set_line_ending(&mut self, le: LineEnding) {
        self.line_ending = le;
    }

    /// Number of lines in the buffer.
    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    /// Total number of characters.
    pub fn char_count(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get a line as a string (including line ending).
    pub fn line_str(&self, line: usize) -> String {
        if line >= self.rope.len_lines() {
            return String::new();
        }
        let slice = self.rope.line(line);
        slice.chunks().collect()
    }

    /// Get grapheme decomposition for a line.
    pub fn line_graphemes(&self, line: usize) -> LineGraphemes {
        if line >= self.rope.len_lines() {
            return LineGraphemes::from_str("");
        }
        LineGraphemes::from_rope_slice(self.rope.line(line))
    }

    /// Convert (line, grapheme_offset) to a char index in the rope.
    pub fn grapheme_to_char(
        &self,
        line: usize,
        grapheme_offset: usize,
    ) -> usize {
        let line_start = self.rope.line_to_char(line);
        let line_str = self.line_str(line);
        let lg = LineGraphemes::from_str(&line_str);
        let byte_off = lg
            .byte_offset_at(grapheme_offset)
            .unwrap_or(lg.total_bytes());
        // Convert byte offset within line to char offset
        let line_prefix = &line_str[..byte_off];
        let char_offset = line_prefix.chars().count();
        line_start + char_offset
    }

    /// Insert text at (line, grapheme_offset).
    pub fn insert(
        &mut self,
        line: usize,
        grapheme_offset: usize,
        text: &str,
    ) {
        let char_idx = self.grapheme_to_char(line, grapheme_offset);
        self.rope.insert(char_idx, text);
    }

    /// Delete a range from (start line, start grapheme) to (end line, end grapheme).
    pub fn delete(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
    ) {
        let start = self.grapheme_to_char(start_line, start_grapheme);
        let end = self.grapheme_to_char(end_line, end_grapheme);
        if start < end && end <= self.rope.len_chars() {
            self.rope.remove(start..end);
        }
    }

    /// Delete a range of lines.
    pub fn delete_lines(&mut self, start: usize, end: usize) {
        if start >= self.line_count() {
            return;
        }
        let end = end.min(self.line_count());
        let start_char = self.rope.line_to_char(start);
        let end_char = if end >= self.line_count() {
            self.rope.len_chars()
        } else {
            self.rope.line_to_char(end)
        };
        if start_char < end_char {
            self.rope.remove(start_char..end_char);
        }
    }

    /// Replace a range with new text.
    pub fn replace(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
        text: &str,
    ) {
        self.delete(start_line, start_grapheme, end_line, end_grapheme);
        self.insert(start_line, start_grapheme, text);
    }

    /// Get the full text as a string.
    pub fn to_string(&self) -> String {
        self.rope.chunks().collect()
    }

    /// Get a line without the line ending.
    pub fn line_content(&self, line: usize) -> String {
        let s = self.line_str(line);
        s.trim_end_matches(['\n', '\r']).to_string()
    }

    /// Get total number of chars in the rope.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the char index at the start of a line.
    pub fn line_start_offset(&self, line: usize) -> usize {
        if line >= self.line_count() {
            return self.rope.len_chars();
        }
        self.rope.line_to_char(line)
    }

    /// Get the char index at the end of a line (before newline).
    pub fn line_end_offset(&self, line: usize) -> usize {
        if line >= self.line_count() {
            return self.rope.len_chars();
        }
        let next = if line + 1 < self.line_count() {
            self.rope.line_to_char(line + 1)
        } else {
            self.rope.len_chars()
        };
        // Back up over line ending.
        if next > 0 {
            let ch = self.rope.char(next - 1);
            if ch == '\n' {
                return next - 1;
            }
        }
        next
    }

    /// Convert (line, grapheme_offset) to a char index.
    pub fn line_grapheme_to_offset(
        &self,
        line: usize,
        grapheme_offset: usize,
    ) -> usize {
        self.grapheme_to_char(line, grapheme_offset)
    }

    /// Count graphemes in a line.
    pub fn line_grapheme_count(&self, line: usize) -> usize {
        if line >= self.line_count() {
            return 0;
        }
        let lg = self.line_graphemes(line);
        lg.count()
    }

    /// Insert a single char at a char index.
    pub fn insert_char(&mut self, char_idx: usize, ch: char) {
        let idx = char_idx.min(self.rope.len_chars());
        let mut buf = [0u8; 4];
        let s = ch.encode_utf8(&mut buf);
        self.rope.insert(idx, s);
    }

    /// Delete a range of chars [start, end).
    pub fn delete_range(
        &mut self,
        start: usize,
        end: usize,
    ) {
        let s = start.min(self.rope.len_chars());
        let e = end.min(self.rope.len_chars());
        if s < e {
            self.rope.remove(s..e);
        }
    }

    /// Get char at a char index.
    pub fn char_at(&self, char_idx: usize) -> Option<char> {
        if char_idx < self.rope.len_chars() {
            Some(self.rope.char(char_idx))
        } else {
            None
        }
    }
}

impl Default for BufferContent {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_buffer() {
        let b = BufferContent::empty();
        assert_eq!(b.line_count(), 1);
        assert_eq!(b.char_count(), 0);
    }

    #[test]
    fn from_str_and_lines() {
        let b = BufferContent::from_str("hello\nworld\n");
        assert_eq!(b.line_count(), 3);
        assert_eq!(b.line_content(0), "hello");
        assert_eq!(b.line_content(1), "world");
    }

    #[test]
    fn insert_text() {
        let mut b = BufferContent::from_str("hello\n");
        b.insert(0, 5, " world");
        assert_eq!(b.line_content(0), "hello world");
    }

    #[test]
    fn delete_text() {
        let mut b = BufferContent::from_str("hello world\n");
        b.delete(0, 5, 0, 11);
        assert_eq!(b.line_content(0), "hello");
    }

    #[test]
    fn cjk_insert() {
        let mut b = BufferContent::from_str("あいう\n");
        b.insert(0, 1, "X");
        assert_eq!(b.line_content(0), "あXいう");
    }
}
