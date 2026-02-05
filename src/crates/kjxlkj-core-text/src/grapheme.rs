//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Count grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Get display width of a string in terminal columns.
pub fn grapheme_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Iterator over grapheme clusters with their byte offsets.
pub struct GraphemeIter<'a> {
    text: &'a str,
    cursor: usize,
}

impl<'a> GraphemeIter<'a> {
    /// Create a new grapheme iterator.
    pub fn new(text: &'a str) -> Self {
        Self { text, cursor: 0 }
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.text.len() {
            return None;
        }

        let remaining = &self.text[self.cursor..];
        let grapheme = remaining.graphemes(true).next()?;
        let start = self.cursor;
        self.cursor += grapheme.len();
        Some((start, grapheme))
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
        assert_eq!(grapheme_count("👋🏽"), 1);
    }

    #[test]
    fn test_grapheme_width() {
        assert_eq!(grapheme_width("hello"), 5);
        assert_eq!(grapheme_width("你好"), 4);
    }

    #[test]
    fn test_grapheme_iter() {
        let text = "aé";
        let items: Vec<_> = GraphemeIter::new(text).collect();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].1, "a");
        assert_eq!(items[1].1, "é");
    }
}
