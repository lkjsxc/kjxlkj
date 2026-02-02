//! Grapheme cluster utilities.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Iterator over grapheme clusters in a string.
pub struct GraphemeIter<'a> {
    inner: unicode_segmentation::Graphemes<'a>,
}

impl<'a> GraphemeIter<'a> {
    /// Creates a new grapheme iterator.
    pub fn new(s: &'a str) -> Self {
        Self {
            inner: s.graphemes(true),
        }
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl<'a> DoubleEndedIterator for GraphemeIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

/// Counts grapheme clusters in a string.
pub fn grapheme_count(s: &str) -> usize {
    s.graphemes(true).count()
}

/// Returns the display width of a string.
pub fn grapheme_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grapheme_count() {
        assert_eq!(grapheme_count("hello"), 5);
        assert_eq!(grapheme_count("héllo"), 5);
        assert_eq!(grapheme_count("👨‍👩‍👧‍👦"), 1);
    }

    #[test]
    fn test_grapheme_width() {
        assert_eq!(grapheme_width("hello"), 5);
        assert_eq!(grapheme_width("日本語"), 6);
    }
}
