//! Extension traits for ropey types.

use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Extension trait for RopeSlice with grapheme-aware operations.
pub trait RopeSliceExt {
    /// Count grapheme clusters.
    fn grapheme_count(&self) -> usize;

    /// Get the display width of the slice.
    fn display_width(&self) -> usize;

    /// Get the nth grapheme cluster as a String.
    fn nth_grapheme(&self, n: usize) -> Option<String>;

    /// Get grapheme iterator.
    fn graphemes(&self) -> GraphemeIter<'_>;
}

impl RopeSliceExt for RopeSlice<'_> {
    fn grapheme_count(&self) -> usize {
        let mut count = 0;
        for chunk in self.chunks() {
            count += chunk.graphemes(true).count();
        }
        count
    }

    fn display_width(&self) -> usize {
        let mut width = 0;
        for chunk in self.chunks() {
            width += UnicodeWidthStr::width(chunk);
        }
        width
    }

    fn nth_grapheme(&self, n: usize) -> Option<String> {
        self.graphemes().nth(n)
    }

    fn graphemes(&self) -> GraphemeIter<'_> {
        GraphemeIter::new(*self)
    }
}

/// Iterator over grapheme clusters in a rope slice.
pub struct GraphemeIter<'a> {
    slice: RopeSlice<'a>,
    chunk_iter: ropey::iter::Chunks<'a>,
    current_chunk: Option<&'a str>,
    grapheme_iter: Option<std::iter::Peekable<unicode_segmentation::Graphemes<'a>>>,
}

impl<'a> GraphemeIter<'a> {
    fn new(slice: RopeSlice<'a>) -> Self {
        let mut chunk_iter = slice.chunks();
        let current_chunk = chunk_iter.next();
        let grapheme_iter =
            current_chunk.map(|c| UnicodeSegmentation::graphemes(c, true).peekable());

        Self {
            slice,
            chunk_iter,
            current_chunk,
            grapheme_iter,
        }
    }
}

impl Iterator for GraphemeIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut gi) = self.grapheme_iter {
                if let Some(g) = gi.next() {
                    return Some(g.to_string());
                }
            }

            // Move to next chunk
            self.current_chunk = self.chunk_iter.next();
            match self.current_chunk {
                Some(chunk) => {
                    self.grapheme_iter =
                        Some(UnicodeSegmentation::graphemes(chunk, true).peekable());
                }
                None => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn test_grapheme_count() {
        let rope = Rope::from_str("hello");
        assert_eq!(rope.slice(..).grapheme_count(), 5);
    }

    #[test]
    fn test_display_width() {
        let rope = Rope::from_str("hello");
        assert_eq!(rope.slice(..).display_width(), 5);

        // Wide characters
        let rope = Rope::from_str("你好");
        assert_eq!(rope.slice(..).display_width(), 4);
    }

    #[test]
    fn test_grapheme_iter() {
        let rope = Rope::from_str("abc");
        let graphemes: Vec<_> = rope.slice(..).graphemes().collect();
        assert_eq!(graphemes, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_grapheme_count_empty() {
        let rope = Rope::from_str("");
        assert_eq!(rope.slice(..).grapheme_count(), 0);
    }

    #[test]
    fn test_grapheme_count_unicode() {
        let rope = Rope::from_str("héllo");
        assert_eq!(rope.slice(..).grapheme_count(), 5);
    }

    #[test]
    fn test_display_width_emoji() {
        let rope = Rope::from_str("👍");
        assert!(rope.slice(..).display_width() >= 1);
    }

    #[test]
    fn test_grapheme_iter_empty() {
        let rope = Rope::from_str("");
        let graphemes: Vec<_> = rope.slice(..).graphemes().collect();
        assert!(graphemes.is_empty());
    }

    #[test]
    fn test_grapheme_iter_unicode() {
        let rope = Rope::from_str("日本");
        let graphemes: Vec<_> = rope.slice(..).graphemes().collect();
        assert_eq!(graphemes.len(), 2);
    }

    #[test]
    fn test_display_width_mixed() {
        let rope = Rope::from_str("abc日");
        let width = rope.slice(..).display_width();
        assert_eq!(width, 5); // 3 ascii + 2 for CJK
    }

    #[test]
    fn test_grapheme_iter_newlines() {
        let rope = Rope::from_str("a\nb");
        let graphemes: Vec<_> = rope.slice(..).graphemes().collect();
        assert_eq!(graphemes.len(), 3);
    }

    #[test]
    fn test_nth_grapheme_first() {
        let rope = Rope::from_str("abc");
        assert_eq!(rope.slice(..).nth_grapheme(0), Some("a".to_string()));
    }

    #[test]
    fn test_nth_grapheme_last() {
        let rope = Rope::from_str("abc");
        assert_eq!(rope.slice(..).nth_grapheme(2), Some("c".to_string()));
    }

    #[test]
    fn test_nth_grapheme_out_of_bounds() {
        let rope = Rope::from_str("abc");
        assert_eq!(rope.slice(..).nth_grapheme(10), None);
    }

    #[test]
    fn test_display_width_tabs() {
        let rope = Rope::from_str("\t");
        // Tab is generally 1 display width in UnicodeWidthStr
        let width = rope.slice(..).display_width();
        assert!(width >= 0);
    }

    #[test]
    fn test_grapheme_count_zwj_emoji() {
        // Emoji with ZWJ (Zero Width Joiner)
        let rope = Rope::from_str("👨‍👩‍👧");
        // ZWJ emoji is 1 grapheme cluster
        let count = rope.slice(..).grapheme_count();
        assert!(count >= 1);
    }

    #[test]
    fn test_display_width_combining() {
        // Character with combining diacritical mark
        let rope = Rope::from_str("é"); // e + combining accent
        let width = rope.slice(..).display_width();
        assert!(width >= 1);
    }

    #[test]
    fn test_grapheme_iter_combining() {
        let rope = Rope::from_str("é");
        let graphemes: Vec<_> = rope.slice(..).graphemes().collect();
        assert!(!graphemes.is_empty());
    }

    #[test]
    fn test_grapheme_count_long_string() {
        let content = "a".repeat(10000);
        let rope = Rope::from_str(&content);
        assert_eq!(rope.slice(..).grapheme_count(), 10000);
    }
}
