//! Grapheme cluster operations.
//!
//! This module provides grapheme cluster iteration and navigation
//! for proper Unicode text handling.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Iterator over grapheme clusters in a string.
pub struct GraphemeIter<'a> {
    inner: Box<dyn Iterator<Item = (usize, &'a str)> + 'a>,
}

impl<'a> GraphemeIter<'a> {
    /// Creates a new grapheme iterator from a string slice.
    pub fn new(s: &'a str) -> Self {
        Self {
            inner: Box::new(s.grapheme_indices(true)),
        }
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// Counts grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Returns the nth grapheme cluster in a string.
pub fn nth_grapheme(s: &str, n: usize) -> Option<&str> {
    s.graphemes(true).nth(n)
}

/// Returns the byte offset of the nth grapheme cluster.
pub fn grapheme_offset(s: &str, n: usize) -> Option<usize> {
    s.grapheme_indices(true).nth(n).map(|(offset, _)| offset)
}

/// Returns the grapheme index at a given byte offset.
pub fn offset_to_grapheme(s: &str, byte_offset: usize) -> usize {
    s.grapheme_indices(true)
        .take_while(|(offset, _)| *offset <= byte_offset)
        .count()
        .saturating_sub(1)
}

/// Returns the display width of a string in terminal cells.
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Returns the display width of a character.
pub fn char_width(c: char) -> usize {
    unicode_width::UnicodeWidthChar::width(c).unwrap_or(0)
}

/// Finds the next grapheme cluster boundary after the given byte offset.
pub fn next_grapheme_boundary(s: &str, byte_offset: usize) -> Option<usize> {
    s.grapheme_indices(true)
        .find(|(offset, _)| *offset > byte_offset)
        .map(|(offset, _)| offset)
}

/// Finds the previous grapheme cluster boundary before the given byte offset.
pub fn prev_grapheme_boundary(s: &str, byte_offset: usize) -> Option<usize> {
    s.grapheme_indices(true)
        .rev()
        .find(|(offset, _)| *offset < byte_offset)
        .map(|(offset, _)| offset)
}

/// Returns an iterator of grapheme clusters with their display widths.
pub fn graphemes_with_widths(s: &str) -> impl Iterator<Item = (&str, usize)> {
    s.graphemes(true).map(|g| (g, display_width(g)))
}

/// Truncates a string to fit within a given display width.
pub fn truncate_to_width(s: &str, max_width: usize) -> &str {
    let mut width = 0;
    let mut end_byte = 0;

    for (offset, grapheme) in s.grapheme_indices(true) {
        let gw = display_width(grapheme);
        if width + gw > max_width {
            break;
        }
        width += gw;
        end_byte = offset + grapheme.len();
    }

    &s[..end_byte]
}

/// Pads a string to a given display width using spaces.
pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let current = display_width(s);
    if current >= target_width {
        s.to_string()
    } else {
        format!("{}{}", s, " ".repeat(target_width - current))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn test_grapheme_count_emoji() {
        // Family emoji is one grapheme cluster
        assert_eq!(grapheme_count("👨‍👩‍👧‍👦"), 1);
    }

    #[test]
    fn test_grapheme_count_combining() {
        // é as e + combining accent
        assert_eq!(grapheme_count("e\u{0301}"), 1);
    }

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn test_display_width_cjk() {
        // CJK characters are double-width
        assert_eq!(display_width("中文"), 4);
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("hello world", 5), "hello");
    }

    #[test]
    fn test_truncate_cjk() {
        // "中文" = 4 width, should fit in 4
        assert_eq!(truncate_to_width("中文字", 4), "中文");
    }

    #[test]
    fn test_pad_to_width() {
        assert_eq!(pad_to_width("hi", 5), "hi   ");
    }

    #[test]
    fn test_nth_grapheme() {
        assert_eq!(nth_grapheme("abc", 1), Some("b"));
    }
}
