//! Grapheme cluster utilities.
//!
//! Converts between grapheme indices and byte/char offsets
//! using `unicode-segmentation`.

use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

/// Count grapheme clusters in a rope slice.
pub fn grapheme_count(slice: RopeSlice<'_>) -> usize {
    let text: String = slice.chars().collect();
    text.graphemes(true).count()
}

/// Convert a grapheme index on a line to a byte offset within that line.
///
/// Returns `None` if the grapheme index is out of bounds.
pub fn grapheme_to_byte_offset(
    slice: RopeSlice<'_>,
    grapheme_idx: usize,
) -> Option<usize> {
    let text: String = slice.chars().collect();
    let mut byte_offset = 0;
    for (i, g) in text.graphemes(true).enumerate() {
        if i == grapheme_idx {
            return Some(byte_offset);
        }
        byte_offset += g.len();
    }
    if grapheme_idx == text.graphemes(true).count() {
        Some(byte_offset)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn ascii_grapheme_count() {
        let r = Rope::from_str("hello");
        assert_eq!(grapheme_count(r.slice(..)), 5);
    }

    #[test]
    fn cjk_grapheme_count() {
        let r = Rope::from_str("日本語");
        assert_eq!(grapheme_count(r.slice(..)), 3);
    }

    #[test]
    fn grapheme_to_byte_basic() {
        let r = Rope::from_str("hello");
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 0), Some(0));
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 3), Some(3));
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 5), Some(5));
    }

    #[test]
    fn grapheme_to_byte_multibyte() {
        let r = Rope::from_str("日本語");
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 0), Some(0));
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 1), Some(3));
        assert_eq!(grapheme_to_byte_offset(r.slice(..), 2), Some(6));
    }
}
