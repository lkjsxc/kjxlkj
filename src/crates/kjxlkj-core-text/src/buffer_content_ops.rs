//! Buffer content convenience methods for cursor and
//! char-level operations.

use crate::BufferContent;

impl BufferContent {
    /// Get total number of chars in the rope.
    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    /// Get the char index at the start of a line.
    pub fn line_start_offset(
        &self,
        line: usize,
    ) -> usize {
        if line >= self.line_count() {
            return self.rope.len_chars();
        }
        self.rope.line_to_char(line)
    }

    /// Get the char index at the end of a line
    /// (before the newline character).
    pub fn line_end_offset(
        &self,
        line: usize,
    ) -> usize {
        if line >= self.line_count() {
            return self.rope.len_chars();
        }
        let next = if line + 1 < self.line_count() {
            self.rope.line_to_char(line + 1)
        } else {
            self.rope.len_chars()
        };
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
    pub fn line_grapheme_count(
        &self,
        line: usize,
    ) -> usize {
        if line >= self.line_count() {
            return 0;
        }
        let lg = self.line_graphemes(line);
        lg.count()
    }

    /// Insert a single char at a char index.
    pub fn insert_char(
        &mut self,
        char_idx: usize,
        ch: char,
    ) {
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
    pub fn char_at(
        &self,
        char_idx: usize,
    ) -> Option<char> {
        if char_idx < self.rope.len_chars() {
            Some(self.rope.char(char_idx))
        } else {
            None
        }
    }

    /// Get the leading whitespace string from a line.
    pub fn line_leading_whitespace(
        &self,
        line: usize,
    ) -> String {
        let content = self.line_content(line);
        content
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect()
    }
}
