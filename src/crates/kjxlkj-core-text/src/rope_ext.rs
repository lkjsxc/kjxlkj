//! Rope extension traits.

use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;

/// Extension trait for Rope.
pub trait RopeExt {
    /// Returns the number of grapheme clusters in a line.
    fn line_grapheme_count(&self, line_idx: usize) -> usize;

    /// Returns the byte offset for a grapheme index on a line.
    fn line_grapheme_to_byte(&self, line_idx: usize, grapheme_idx: usize) -> Option<usize>;

    /// Returns the grapheme index for a byte offset on a line.
    fn line_byte_to_grapheme(&self, line_idx: usize, byte_idx: usize) -> usize;

    /// Returns the content of a line without the line ending.
    fn line_content(&self, line_idx: usize) -> String;
}

impl RopeExt for Rope {
    fn line_grapheme_count(&self, line_idx: usize) -> usize {
        if line_idx >= self.len_lines() {
            return 0;
        }
        let line = self.line(line_idx);
        let line_str = line.to_string();
        let content = line_str.trim_end_matches(&['\n', '\r'][..]);
        content.graphemes(true).count()
    }

    fn line_grapheme_to_byte(&self, line_idx: usize, grapheme_idx: usize) -> Option<usize> {
        if line_idx >= self.len_lines() {
            return None;
        }
        let line = self.line(line_idx);
        let line_str = line.to_string();
        let mut byte_offset = 0;
        for (i, g) in line_str.graphemes(true).enumerate() {
            if i == grapheme_idx {
                return Some(byte_offset);
            }
            byte_offset += g.len();
        }
        Some(byte_offset)
    }

    fn line_byte_to_grapheme(&self, line_idx: usize, byte_idx: usize) -> usize {
        if line_idx >= self.len_lines() {
            return 0;
        }
        let line = self.line(line_idx);
        let line_str = line.to_string();
        let mut byte_offset = 0;
        for (i, g) in line_str.graphemes(true).enumerate() {
            if byte_offset >= byte_idx {
                return i;
            }
            byte_offset += g.len();
        }
        line_str.graphemes(true).count()
    }

    fn line_content(&self, line_idx: usize) -> String {
        if line_idx >= self.len_lines() {
            return String::new();
        }
        let line = self.line(line_idx);
        let line_str = line.to_string();
        line_str.trim_end_matches(&['\n', '\r'][..]).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_grapheme_count() {
        let rope = Rope::from_str("hello\nworld\n");
        assert_eq!(rope.line_grapheme_count(0), 5);
        assert_eq!(rope.line_grapheme_count(1), 5);
    }

    #[test]
    fn test_line_content() {
        let rope = Rope::from_str("hello\nworld\n");
        assert_eq!(rope.line_content(0), "hello");
        assert_eq!(rope.line_content(1), "world");
    }
}
