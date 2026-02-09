//! Grapheme cluster iteration and info structures.
//!
//! Provides iterators over grapheme clusters for rope
//! lines.

use unicode_segmentation::UnicodeSegmentation;

use crate::display_width::grapheme_display_width;

/// Information about a single grapheme cluster.
#[derive(Debug, Clone)]
pub struct GraphemeInfo {
    /// The grapheme cluster string.
    pub grapheme: String,
    /// Zero-based grapheme index in the line.
    pub index: usize,
    /// Byte offset from the start of the line.
    pub byte_offset: usize,
    /// Display width in terminal columns (1 or 2).
    pub display_width: u8,
    /// Starting display column.
    pub display_col: usize,
}

/// Iterator over grapheme clusters of a line string.
pub struct GraphemeIter<'a> {
    inner: Vec<&'a str>,
    pos: usize,
    byte_offset: usize,
    display_col: usize,
}

impl<'a> GraphemeIter<'a> {
    /// Create from a string slice.
    pub fn new(s: &'a str) -> Self {
        let inner: Vec<&str> = s.graphemes(true).collect();
        Self {
            inner,
            pos: 0,
            byte_offset: 0,
            display_col: 0,
        }
    }
}

impl<'a> Iterator for GraphemeIter<'a> {
    type Item = GraphemeInfo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.len() {
            return None;
        }
        let g = self.inner[self.pos];
        let w = grapheme_display_width(g);
        let info = GraphemeInfo {
            grapheme: g.to_string(),
            index: self.pos,
            byte_offset: self.byte_offset,
            display_width: w,
            display_col: self.display_col,
        };
        self.pos += 1;
        self.byte_offset += g.len();
        self.display_col += w as usize;
        Some(info)
    }
}

// Re-export LineGraphemes from sibling module.
pub use crate::grapheme_line::LineGraphemes;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_line_graphemes() {
        let lg = LineGraphemes::from_str("hello\n");
        assert_eq!(lg.count(), 5);
        assert_eq!(lg.get(0), Some("h"));
        assert_eq!(lg.total_width(), 5);
    }

    #[test]
    fn cjk_line_graphemes() {
        let lg = LineGraphemes::from_str("あいう\n");
        assert_eq!(lg.count(), 3);
        assert_eq!(lg.width_at(0), Some(2));
        assert_eq!(lg.total_width(), 6);
    }

    #[test]
    fn mixed_line_graphemes() {
        let lg = LineGraphemes::from_str("aあb\n");
        assert_eq!(lg.count(), 3);
        assert_eq!(lg.display_col_at(0), Some(0));
        assert_eq!(lg.display_col_at(1), Some(1));
        assert_eq!(lg.display_col_at(2), Some(3));
    }

    #[test]
    fn grapheme_at_col_wide() {
        let lg = LineGraphemes::from_str("あいう");
        assert_eq!(lg.grapheme_at_col(0), 0);
        assert_eq!(lg.grapheme_at_col(1), 0);
        assert_eq!(lg.grapheme_at_col(2), 1);
    }

    #[test]
    fn clamp_exclusive() {
        let lg = LineGraphemes::from_str("abc");
        assert_eq!(lg.clamp_exclusive(5), 2);
        assert_eq!(lg.clamp_exclusive(1), 1);
    }
}
