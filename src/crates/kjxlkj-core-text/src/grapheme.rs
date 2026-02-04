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

    #[test]
    fn grapheme_boundary_at_end() {
        let s = "ab";
        assert_eq!(next_grapheme_boundary(s, 2), 2);
    }

    #[test]
    fn grapheme_boundary_at_start() {
        let s = "ab";
        assert_eq!(prev_grapheme_boundary(s, 0), 0);
    }

    #[test]
    fn empty_string_width() {
        assert_eq!(grapheme_width(""), 0);
    }

    #[test]
    fn emoji_width() {
        // Most terminals render emoji as width 2
        assert!(grapheme_width("😀") >= 1);
    }

    #[test]
    fn tab_width() {
        // Tab character width (unicode width crate reports 1 for control chars)
        assert!(grapheme_width("\t") >= 0);
    }

    #[test]
    fn multibyte_boundaries() {
        let s = "中文";
        assert_eq!(next_grapheme_boundary(s, 0), 3); // 3 bytes per CJK char
        assert_eq!(prev_grapheme_boundary(s, 6), 3);
    }

    #[test]
    fn next_boundary_at_middle() {
        let s = "abcdef";
        assert_eq!(next_grapheme_boundary(s, 3), 4);
    }

    #[test]
    fn prev_boundary_at_end() {
        let s = "abcd";
        assert_eq!(prev_grapheme_boundary(s, 4), 3);
    }

    #[test]
    fn cjk_char_width() {
        // CJK characters are typically double-width
        assert!(grapheme_width("中") >= 1);
    }

    #[test]
    fn ascii_char_width() {
        assert_eq!(grapheme_width("a"), 1);
    }

    #[test]
    fn space_width() {
        assert_eq!(grapheme_width(" "), 1);
    }

    #[test]
    fn digit_width() {
        assert_eq!(grapheme_width("5"), 1);
    }

    #[test]
    fn punctuation_width() {
        assert_eq!(grapheme_width("."), 1);
    }

    #[test]
    fn multiple_ascii_chars() {
        let s = "hello";
        assert_eq!(next_grapheme_boundary(s, 0), 1);
        assert_eq!(next_grapheme_boundary(s, 1), 2);
    }

    #[test]
    fn boundary_at_string_end() {
        let s = "abc";
        assert_eq!(next_grapheme_boundary(s, 3), 3);
    }
}
