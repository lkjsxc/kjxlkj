use crate::grapheme::{grapheme_count, grapheme_to_byte_offset};
use ropey::Rope;

/// Extension trait for Rope with grapheme-aware operations.
pub trait RopeExt {
    /// Insert text at a given (line, grapheme_offset) position.
    fn insert_at_grapheme(&mut self, line: usize, grapheme_offset: usize, text: &str);

    /// Delete text between two (line, grapheme) positions.
    fn delete_grapheme_range(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
    );

    /// Get line count (wrapping ropey method).
    fn line_count(&self) -> usize;

    /// Count graphemes in a specific line.
    fn line_grapheme_count(&self, line: usize) -> usize;

    /// Convert (line, grapheme) to byte offset in rope.
    fn grapheme_pos_to_byte(&self, line: usize, grapheme: usize) -> usize;
}

impl RopeExt for Rope {
    fn insert_at_grapheme(&mut self, line: usize, grapheme_offset: usize, text: &str) {
        let byte_offset = self.grapheme_pos_to_byte(line, grapheme_offset);
        let char_idx = self.byte_to_char(byte_offset);
        self.insert(char_idx, text);
    }

    fn delete_grapheme_range(
        &mut self,
        start_line: usize,
        start_grapheme: usize,
        end_line: usize,
        end_grapheme: usize,
    ) {
        let start_byte = self.grapheme_pos_to_byte(start_line, start_grapheme);
        let end_byte = self.grapheme_pos_to_byte(end_line, end_grapheme);
        if start_byte < end_byte {
            let start_char = self.byte_to_char(start_byte);
            let end_char = self.byte_to_char(end_byte);
            self.remove(start_char..end_char);
        }
    }

    fn line_count(&self) -> usize {
        self.len_lines()
    }

    fn line_grapheme_count(&self, line: usize) -> usize {
        if line >= self.len_lines() {
            return 0;
        }
        grapheme_count(self.line(line))
    }

    fn grapheme_pos_to_byte(&self, line: usize, grapheme: usize) -> usize {
        let line_start_byte = self.line_to_byte(line);
        let line_slice = self.line(line);
        let offset = grapheme_to_byte_offset(line_slice, grapheme);
        line_start_byte + offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_grapheme() {
        let mut rope = Rope::from_str("hello world");
        rope.insert_at_grapheme(0, 5, ",");
        assert_eq!(rope.to_string(), "hello, world");
    }

    #[test]
    fn test_delete_grapheme_range() {
        let mut rope = Rope::from_str("hello world");
        rope.delete_grapheme_range(0, 5, 0, 11);
        assert_eq!(rope.to_string(), "hello");
    }

    #[test]
    fn test_line_grapheme_count() {
        let rope = Rope::from_str("hello\nworld\n");
        assert_eq!(rope.line_grapheme_count(0), 6); // "hello\n"
        assert_eq!(rope.line_grapheme_count(1), 6); // "world\n"
    }
}
