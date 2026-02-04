//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Get the display width of a grapheme cluster.
pub fn grapheme_width(grapheme: &str) -> usize {
    UnicodeWidthStr::width(grapheme)
}

/// Find the next grapheme boundary in a string.
pub fn next_grapheme_boundary(s: &str, byte_offset: usize) -> usize {
    if byte_offset >= s.len() {
        return s.len();
    }
    let remaining = &s[byte_offset..];
    let mut graphemes = remaining.grapheme_indices(true);
    graphemes.next(); // Skip current grapheme
    graphemes.next().map(|(i, _)| byte_offset + i).unwrap_or(s.len())
}

/// Find the previous grapheme boundary in a string.
pub fn prev_grapheme_boundary(s: &str, byte_offset: usize) -> usize {
    if byte_offset == 0 {
        return 0;
    }
    let prefix = &s[..byte_offset];
    prefix.grapheme_indices(true).last().map(|(i, _)| i).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_grapheme_width() {
        assert_eq!(grapheme_width("a"), 1);
        assert_eq!(grapheme_width(" "), 1);
    }

    #[test]
    fn cjk_grapheme_width() {
        assert_eq!(grapheme_width("中"), 2);
        assert_eq!(grapheme_width("あ"), 2);
    }

    #[test]
    fn grapheme_boundaries() {
        let s = "abc";
        assert_eq!(next_grapheme_boundary(s, 0), 1);
        assert_eq!(next_grapheme_boundary(s, 1), 2);
        assert_eq!(prev_grapheme_boundary(s, 2), 1);
        assert_eq!(prev_grapheme_boundary(s, 1), 0);
    }
}
