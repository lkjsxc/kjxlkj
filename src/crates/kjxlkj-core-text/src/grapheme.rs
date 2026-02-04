//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Count grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Calculate display width of a string.
pub fn grapheme_width(s: &str) -> usize {
    s.graphemes(true).map(UnicodeWidthStr::width).sum()
}

/// Get byte offset of the nth grapheme cluster.
pub fn nth_grapheme_offset(s: &str, n: usize) -> Option<usize> {
    let mut offset = 0;
    for (i, g) in s.grapheme_indices(true).enumerate() {
        if i == n {
            return Some(g.0);
        }
        offset = g.0 + g.1.len();
    }
    if n == grapheme_count(s) {
        Some(offset)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_count() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count("héllo"), 5);
        assert_eq!(grapheme_count(""), 0);
    }

    #[test]
    fn test_grapheme_width() {
        assert_eq!(grapheme_width("hello"), 5);
        assert_eq!(grapheme_width("你好"), 4);
    }

    #[test]
    fn test_nth_grapheme_offset() {
        assert_eq!(nth_grapheme_offset("hello", 0), Some(0));
        assert_eq!(nth_grapheme_offset("hello", 2), Some(2));
        assert_eq!(nth_grapheme_offset("hello", 5), Some(5));
        assert_eq!(nth_grapheme_offset("hello", 6), None);
    }
}
