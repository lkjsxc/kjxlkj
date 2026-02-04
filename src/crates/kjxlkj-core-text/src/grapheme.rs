//! Grapheme cluster utilities.

use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

/// Count the number of grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Get the byte offset of the nth grapheme cluster in a rope slice.
pub fn nth_grapheme_offset(slice: RopeSlice, n: usize) -> Option<usize> {
    let mut offset = 0;
    let mut count = 0;

    for chunk in slice.chunks() {
        for g in chunk.graphemes(true) {
            if count == n {
                return Some(offset);
            }
            offset += g.len();
            count += 1;
        }
    }

    if count == n {
        Some(offset)
    } else {
        None
    }
}

/// Count grapheme clusters in a rope slice.
pub fn rope_grapheme_count(slice: RopeSlice) -> usize {
    let mut count = 0;
    for chunk in slice.chunks() {
        count += chunk.graphemes(true).count();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn test_grapheme_count() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count("héllo"), 5);
        // Emoji with ZWJ sequence counts as one grapheme
        assert_eq!(grapheme_count("👨‍👩‍👧"), 1);
    }

    #[test]
    fn test_nth_grapheme_offset() {
        let rope = Rope::from_str("héllo");
        let slice = rope.slice(..);
        assert_eq!(nth_grapheme_offset(slice, 0), Some(0));
        assert_eq!(nth_grapheme_offset(slice, 1), Some(1));
        assert_eq!(nth_grapheme_offset(slice, 2), Some(3)); // 'é' is 2 bytes
    }

    #[test]
    fn test_grapheme_count_empty() {
        assert_eq!(grapheme_count(""), 0);
    }

    #[test]
    fn test_grapheme_count_single() {
        assert_eq!(grapheme_count("a"), 1);
    }

    #[test]
    fn test_grapheme_count_cjk() {
        assert_eq!(grapheme_count("你好"), 2);
        assert_eq!(grapheme_count("日本語"), 3);
    }

    #[test]
    fn test_grapheme_count_combining() {
        // e + combining acute accent = 1 grapheme
        assert_eq!(grapheme_count("e\u{0301}"), 1);
    }

    #[test]
    fn test_nth_grapheme_offset_empty() {
        let rope = Rope::from_str("");
        let slice = rope.slice(..);
        assert_eq!(nth_grapheme_offset(slice, 0), Some(0));
        assert_eq!(nth_grapheme_offset(slice, 1), None);
    }

    #[test]
    fn test_nth_grapheme_offset_beyond() {
        let rope = Rope::from_str("ab");
        let slice = rope.slice(..);
        assert_eq!(nth_grapheme_offset(slice, 10), None);
    }

    #[test]
    fn test_rope_grapheme_count() {
        let rope = Rope::from_str("hello");
        let slice = rope.slice(..);
        assert_eq!(rope_grapheme_count(slice), 5);
    }

    #[test]
    fn test_rope_grapheme_count_unicode() {
        let rope = Rope::from_str("héllo 世界");
        let slice = rope.slice(..);
        assert_eq!(rope_grapheme_count(slice), 8);
    }

    #[test]
    fn test_rope_grapheme_count_empty() {
        let rope = Rope::from_str("");
        let slice = rope.slice(..);
        assert_eq!(rope_grapheme_count(slice), 0);
    }

    #[test]
    fn test_rope_grapheme_count_newlines() {
        let rope = Rope::from_str("a\nb\nc");
        let slice = rope.slice(..);
        assert_eq!(rope_grapheme_count(slice), 5);
    }

    #[test]
    fn test_grapheme_count_emoji_zwj() {
        // Family emoji (multiple code points joined with ZWJ)
        assert_eq!(grapheme_count("👨‍👩‍👧‍👦"), 1);
    }

    #[test]
    fn test_grapheme_count_emoji_simple() {
        assert_eq!(grapheme_count("😀"), 1);
        assert_eq!(grapheme_count("😀😀😀"), 3);
    }

    #[test]
    fn test_grapheme_count_mixed() {
        assert_eq!(grapheme_count("abc123"), 6);
        assert_eq!(grapheme_count("a b c"), 5);
    }

    #[test]
    fn test_nth_grapheme_offset_multi_byte() {
        let rope = Rope::from_str("日本語");
        let slice = rope.slice(..);
        assert_eq!(nth_grapheme_offset(slice, 0), Some(0));
        assert_eq!(nth_grapheme_offset(slice, 1), Some(3));
        assert_eq!(nth_grapheme_offset(slice, 2), Some(6));
    }

    #[test]
    fn test_rope_grapheme_count_tab() {
        let rope = Rope::from_str("a\tb\tc");
        let slice = rope.slice(..);
        assert_eq!(rope_grapheme_count(slice), 5);
    }
}
