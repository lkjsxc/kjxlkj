//! LineGraphemes: grapheme-level view of a text line.

use crate::display_width::grapheme_display_width;
use unicode_segmentation::UnicodeSegmentation;

/// Grapheme-level view of a line.
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
    pub fn from_rope_slice(
        slice: ropey::RopeSlice<'_>,
    ) -> Self {
        let line_str: String =
            slice.chunks().collect();
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

    /// Number of grapheme clusters.
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

    /// Starting display column of grapheme at index.
    pub fn display_col_at(
        &self,
        idx: usize,
    ) -> Option<usize> {
        self.display_cols.get(idx).copied()
    }

    /// Total display width of the line.
    pub fn total_width(&self) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            let last = self.graphemes.len() - 1;
            self.display_cols[last]
                + self.widths[last] as usize
        }
    }

    /// Find grapheme index at a display column.
    pub fn grapheme_at_col(
        &self,
        col: usize,
    ) -> usize {
        for (i, &dc) in
            self.display_cols.iter().enumerate()
        {
            let w = self.widths[i] as usize;
            if col < dc + w {
                return i;
            }
        }
        self.graphemes.len()
    }

    /// Get the byte offset of grapheme at index.
    pub fn byte_offset_at(
        &self,
        idx: usize,
    ) -> Option<usize> {
        self.byte_offsets.get(idx).copied()
    }

    /// Byte offset for the end of the last grapheme.
    pub fn total_bytes(&self) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            let last = self.graphemes.len() - 1;
            self.byte_offsets[last]
                + self.graphemes[last].len()
        }
    }

    /// Clamp a grapheme index to valid range.
    pub fn clamp_exclusive(
        &self,
        idx: usize,
    ) -> usize {
        if self.graphemes.is_empty() {
            0
        } else {
            idx.min(self.graphemes.len() - 1)
        }
    }

    /// Clamp a grapheme index to valid inclusive range.
    pub fn clamp_inclusive(
        &self,
        idx: usize,
    ) -> usize {
        idx.min(self.graphemes.len())
    }
}
