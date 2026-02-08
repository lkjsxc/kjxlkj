//! Grapheme cluster iteration and line-level grapheme accessors.
//!
//! Provides iterators over grapheme clusters for rope lines,
//! with bidirectional mapping between grapheme indices and byte offsets.

use ropey::RopeSlice;
use unicode_segmentation::UnicodeSegmentation;

use crate::display_width::grapheme_display_width;

/// Information about a single grapheme cluster in a line.
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

/// Grapheme-level view of a line from a rope slice.
pub struct LineGraphemes {
    /// All grapheme clusters in the line.
    graphemes: Vec<String>,
    /// Display widths per grapheme.
    widths: Vec<u8>,
    /// Cumulative display column per grapheme.
    display_cols: Vec<usize>,
    /// Byte offsets per grapheme.
    byte_offsets: Vec<usize>,
}

impl LineGraphemes {
    /// Build from a rope line slice.
    pub fn from_rope_slice(slice: RopeSlice<'_>) -> Self {
        let line_str: String = slice.chunks().collect();
        Self::from_str(&line_str)
    }

    /// Build from a string.
    pub fn from_str(s: &str) -> Self {
        let mut graphemes = Vec::new();
        let mut widths = Vec::new();
        let mut display_cols = Vec::new();
        let mut byte_offsets = Vec::new();
        let mut col = 0usize;
        let mut byte_off = 0usize;

        for g in s.graphemes(true) {
            // Skip trailing newline characters for grapheme counting.
            if g == "\n" || g == "\r\n" || g == "\r" {
                break;
            }
            let w = grapheme_display_width(g);
            graphemes.push(g.to_string());
            widths.push(w);
            display_cols.push(col);
            byte_offsets.push(byte_off);
            col += w as usize;
            byte_off += g.len();
        }

        Self {
            graphemes,
            widths,
            display_cols,
            byte_offsets,
        }
    }

    /// Number of grapheme clusters (excluding line ending).
    pub fn count(&self) -> usize {
        self.graphemes.len()
    }

    /// Get the grapheme at a given index.
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.graphemes.get(idx).map(|s| s.as_str())
    }

    /// Get the display width of the grapheme at index.
    pub fn width_at(&self, idx: usize) -> Option<u8> {
        self.widths.get(idx).copied()
    }

    /// Get the starting display column of grapheme at index.
    pub fn display_col_at(&self, idx: usize) -> Option<usize> {
        self.display_cols.get(idx).copied()
    }

    /// Total display width of the line.
    pub fn total_width(&self) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            let last = self.graphemes.len() - 1;
            self.display_cols[last] + self.widths[last] as usize
        }
    }

    /// Find grapheme index at a display column.
    pub fn grapheme_at_col(&self, col: usize) -> usize {
        for (i, &dc) in self.display_cols.iter().enumerate() {
            let w = self.widths[i] as usize;
            if col < dc + w {
                return i;
            }
        }
        self.graphemes.len()
    }

    /// Get the byte offset of grapheme at index.
    pub fn byte_offset_at(&self, idx: usize) -> Option<usize> {
        self.byte_offsets.get(idx).copied()
    }

    /// Byte offset for the end of the last grapheme.
    pub fn total_bytes(&self) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            let last = self.graphemes.len() - 1;
            self.byte_offsets[last] + self.graphemes[last].len()
        }
    }

    /// Clamp a grapheme index to valid end-exclusive range.
    pub fn clamp_exclusive(&self, idx: usize) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            idx.min(self.graphemes.len() - 1)
        }
    }

    /// Clamp a grapheme index to valid end-inclusive range.
    pub fn clamp_inclusive(&self, idx: usize) -> usize {
        idx.min(self.graphemes.len())
    }
}

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
        assert_eq!(lg.grapheme_at_col(1), 0); // second col of あ
        assert_eq!(lg.grapheme_at_col(2), 1); // first col of い
    }

    #[test]
    fn clamp_exclusive() {
        let lg = LineGraphemes::from_str("abc");
        assert_eq!(lg.clamp_exclusive(5), 2);
        assert_eq!(lg.clamp_exclusive(1), 1);
    }
}
