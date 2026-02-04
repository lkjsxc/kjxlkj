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
}
