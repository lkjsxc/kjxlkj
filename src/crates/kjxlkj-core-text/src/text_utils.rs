//! Text utility functions.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Returns the display width of a grapheme cluster.
pub fn grapheme_width(grapheme: &str) -> usize {
    UnicodeWidthStr::width(grapheme)
}

/// Returns the number of grapheme clusters in a line.
pub fn line_grapheme_count(line: &str) -> usize {
    line.graphemes(true).count()
}

/// Returns the display width of a line.
pub fn line_display_width(line: &str) -> usize {
    line.graphemes(true).map(grapheme_width).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_width() {
        assert_eq!(grapheme_width("a"), 1);
        assert_eq!(line_display_width("hello"), 5);
    }

    #[test]
    fn grapheme_count() {
        assert_eq!(line_grapheme_count("hello"), 5);
        assert_eq!(line_grapheme_count("é"), 1);
    }
}
