//! Line wrapping algorithm with wide-character boundary padding.
//!
//! Implements the normative wrapping procedure from viewport.md:
//! - Width-2 graphemes MUST NOT split across display rows.
//! - If only 1 column remains, a padding cell fills the gap and the
//!   wide character starts on the next display row.

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// A segment of a wrapped line for rendering.
#[derive(Debug, Clone)]
pub struct WrapSegment {
    /// Grapheme cluster text.
    pub grapheme: String,
    /// Display width (1 or 2).
    pub width: usize,
}

/// A padding cell at the end of a display row.
#[derive(Debug, Clone, Copy)]
pub struct PadCell;

/// A display row produced by wrapping a single buffer line.
#[derive(Debug, Clone)]
pub struct WrappedRow {
    /// The grapheme segments in this display row.
    pub segments: Vec<WrapSegment>,
    /// Whether this row ends with a padding cell (width-2 boundary).
    pub has_pad: bool,
    /// Total display width used (excluding padding).
    pub used_cols: usize,
}

/// Wrap a single buffer line into display rows.
///
/// `text_cols` is the available display width per row.
/// Returns at least one row (empty line produces one empty row).
pub fn wrap_line(line: &str, text_cols: usize) -> Vec<WrappedRow> {
    if text_cols == 0 {
        return vec![WrappedRow {
            segments: Vec::new(),
            has_pad: false,
            used_cols: 0,
        }];
    }

    let mut rows: Vec<WrappedRow> = Vec::new();
    let mut current_segments: Vec<WrapSegment> = Vec::new();
    let mut col = 0usize;

    let trimmed = line.trim_end_matches(&['\n', '\r'][..]);

    for grapheme in trimmed.graphemes(true) {
        let w = UnicodeWidthStr::width(grapheme);
        if w == 0 {
            // Zero-width grapheme: attach to current position
            current_segments.push(WrapSegment {
                grapheme: grapheme.to_string(),
                width: 0,
            });
            continue;
        }

        if col + w > text_cols {
            // Need to wrap: check if this is a width-2 boundary split
            let has_pad = w == 2 && col + 1 == text_cols;
            rows.push(WrappedRow {
                segments: std::mem::take(&mut current_segments),
                has_pad,
                used_cols: col,
            });
            col = 0;
        }

        current_segments.push(WrapSegment {
            grapheme: grapheme.to_string(),
            width: w,
        });
        col += w;
    }

    // Final row (always present, even if empty)
    rows.push(WrappedRow {
        segments: current_segments,
        has_pad: false,
        used_cols: col,
    });

    rows
}

/// Count the total display rows a line occupies when wrapped.
pub fn display_row_count(line: &str, text_cols: usize) -> usize {
    wrap_line(line, text_cols).len()
}

/// Find which display row and column offset a grapheme index maps to.
pub fn grapheme_to_display_pos(
    line: &str,
    grapheme_offset: usize,
    text_cols: usize,
) -> (usize, usize) {
    if text_cols == 0 {
        return (0, 0);
    }
    let trimmed = line.trim_end_matches(&['\n', '\r'][..]);
    let mut row = 0usize;
    let mut col = 0usize;

    for (gi, grapheme) in trimmed.graphemes(true).enumerate() {
        let w = UnicodeWidthStr::width(grapheme);
        // Check if this grapheme would wrap to the next row
        if w > 0 && col + w > text_cols {
            row += 1;
            col = 0;
        }
        if gi == grapheme_offset {
            return (row, col);
        }
        col += w;
    }
    (row, col)
}
