//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Get grapheme clusters from a string.
pub fn graphemes(s: &str) -> impl Iterator<Item = &str> {
    s.graphemes(true)
}

/// Count grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Get display width of a string in terminal cells.
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Get display width of a single grapheme cluster.
pub fn grapheme_width(grapheme: &str) -> usize {
    // Handle control characters and special cases.
    if grapheme.chars().any(|c| c.is_control()) {
        return 0;
    }
    UnicodeWidthStr::width(grapheme)
}

/// Convert grapheme offset to byte offset within a line.
pub fn grapheme_to_byte_offset(line: &str, grapheme_offset: usize) -> usize {
    line.graphemes(true)
        .take(grapheme_offset)
        .map(|g| g.len())
        .sum()
}

/// Convert byte offset to grapheme offset within a line.
pub fn byte_to_grapheme_offset(line: &str, byte_offset: usize) -> usize {
    let mut bytes = 0;
    for (i, g) in line.graphemes(true).enumerate() {
        if bytes >= byte_offset {
            return i;
        }
        bytes += g.len();
    }
    grapheme_count(line)
}

/// Convert grapheme offset to display column.
pub fn grapheme_to_display_col(line: &str, grapheme_offset: usize) -> usize {
    line.graphemes(true)
        .take(grapheme_offset)
        .map(grapheme_width)
        .sum()
}

/// Convert display column to grapheme offset.
/// Returns the grapheme that contains or starts at the given column.
pub fn display_col_to_grapheme(line: &str, display_col: usize) -> usize {
    let mut col = 0;
    for (i, g) in line.graphemes(true).enumerate() {
        if col >= display_col {
            return i;
        }
        col += grapheme_width(g);
    }
    grapheme_count(line)
}

/// Get the nth grapheme from a line.
pub fn nth_grapheme(line: &str, n: usize) -> Option<&str> {
    line.graphemes(true).nth(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_count_ascii() {
        assert_eq!(grapheme_count("hello"), 5);
    }

    #[test]
    fn test_grapheme_count_cjk() {
        assert_eq!(grapheme_count("日本語"), 3);
    }

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn test_display_width_cjk() {
        assert_eq!(display_width("日本語"), 6);
    }

    #[test]
    fn test_grapheme_to_display_col() {
        let line = "a日b";
        assert_eq!(grapheme_to_display_col(line, 0), 0);
        assert_eq!(grapheme_to_display_col(line, 1), 1);
        assert_eq!(grapheme_to_display_col(line, 2), 3);
    }
}
