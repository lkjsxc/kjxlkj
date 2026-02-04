//! Text utility functions.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Calculate display width of a string (accounting for wide characters).
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Count grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Iterate over grapheme clusters in a line.
pub fn line_graphemes(s: &str) -> impl Iterator<Item = &str> {
    s.graphemes(true)
}

/// Get the grapheme at a specific index.
#[allow(dead_code)]
pub fn grapheme_at(s: &str, index: usize) -> Option<&str> {
    s.graphemes(true).nth(index)
}

/// Convert grapheme index to byte offset.
#[allow(dead_code)]
pub fn grapheme_to_byte_offset(s: &str, grapheme_idx: usize) -> usize {
    s.graphemes(true)
        .take(grapheme_idx)
        .map(|g| g.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_display_width() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn wide_char_display_width() {
        assert_eq!(display_width("你好"), 4);
    }

    #[test]
    fn grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn grapheme_count_emoji() {
        // Family emoji is one grapheme cluster
        assert_eq!(grapheme_count("👨‍👩‍👧"), 1);
    }

    #[test]
    fn grapheme_at_works() {
        assert_eq!(grapheme_at("hello", 0), Some("h"));
        assert_eq!(grapheme_at("hello", 4), Some("o"));
        assert_eq!(grapheme_at("hello", 5), None);
    }
}
