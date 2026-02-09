use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

/// Count grapheme clusters in a rope slice (one line).
pub fn grapheme_count(slice: RopeSlice<'_>) -> usize {
    let s: std::borrow::Cow<str> = slice.into();
    s.graphemes(true).count()
}

/// Convert a grapheme offset within a line to a byte offset.
///
/// Returns the byte offset from the start of the rope slice.
/// If `grapheme_idx` >= number of graphemes, returns the
/// byte length of the slice.
pub fn grapheme_to_byte_offset(slice: RopeSlice<'_>, grapheme_idx: usize) -> usize {
    let s: std::borrow::Cow<str> = slice.into();
    let mut byte_offset = 0;
    for (i, g) in s.graphemes(true).enumerate() {
        if i == grapheme_idx {
            return byte_offset;
        }
        byte_offset += g.len();
    }
    byte_offset
}

/// Get the nth grapheme cluster from a rope slice.
///
/// Returns `None` if the index is out of bounds.
pub fn nth_grapheme(slice: RopeSlice<'_>, grapheme_idx: usize) -> Option<String> {
    let s: std::borrow::Cow<str> = slice.into();
    s.graphemes(true).nth(grapheme_idx).map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn test_grapheme_count_ascii() {
        let rope = Rope::from_str("hello");
        let line = rope.line(0);
        assert_eq!(grapheme_count(line), 5);
    }

    #[test]
    fn test_grapheme_count_cjk() {
        let rope = Rope::from_str("你好世界");
        let line = rope.line(0);
        assert_eq!(grapheme_count(line), 4);
    }

    #[test]
    fn test_grapheme_to_byte_ascii() {
        let rope = Rope::from_str("hello");
        let line = rope.line(0);
        assert_eq!(grapheme_to_byte_offset(line, 0), 0);
        assert_eq!(grapheme_to_byte_offset(line, 2), 2);
    }

    #[test]
    fn test_grapheme_to_byte_cjk() {
        let rope = Rope::from_str("你好世界");
        let line = rope.line(0);
        assert_eq!(grapheme_to_byte_offset(line, 0), 0);
        assert_eq!(grapheme_to_byte_offset(line, 1), 3);
        assert_eq!(grapheme_to_byte_offset(line, 2), 6);
    }

    #[test]
    fn test_nth_grapheme() {
        let rope = Rope::from_str("héllo");
        let line = rope.line(0);
        assert_eq!(nth_grapheme(line, 0), Some("h".to_string()));
        assert_eq!(nth_grapheme(line, 1), Some("é".to_string()));
        assert_eq!(nth_grapheme(line, 10), None);
    }
}
