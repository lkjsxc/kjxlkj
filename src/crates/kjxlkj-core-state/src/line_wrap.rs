//! Line wrapping algorithm with CJK boundary padding.
//!
//! Handles soft-wrap for long lines, inserting padding
//! cells when a width-2 grapheme would be split across
//! display rows.

use kjxlkj_core_text::display_width::grapheme_display_width;
use unicode_segmentation::UnicodeSegmentation;

/// A wrapped display row with source mapping.
#[derive(Debug, Clone)]
pub struct WrapRow {
    /// Grapheme segments for this row.
    pub segments: Vec<WrapSegment>,
    /// Total display width occupied (including padding).
    pub display_width: usize,
    /// Whether this row ends with padding cells.
    pub has_padding: bool,
}

/// A segment within a wrapped row.
#[derive(Debug, Clone)]
pub enum WrapSegment {
    /// A grapheme from the source line.
    Grapheme {
        text: String,
        width: u8,
        source_offset: usize,
    },
    /// A padding cell inserted at wrap boundary.
    Padding,
}

/// Compute wrapped rows for a single buffer line.
pub fn wrap_line(line: &str, cols: usize) -> Vec<WrapRow> {
    if cols == 0 {
        return vec![WrapRow {
            segments: Vec::new(),
            display_width: 0,
            has_padding: false,
        }];
    }

    let mut rows = Vec::new();
    let mut current = WrapRow {
        segments: Vec::new(),
        display_width: 0,
        has_padding: false,
    };
    let mut col = 0;

    for (idx, g) in line.graphemes(true).enumerate() {
        let w = grapheme_display_width(g) as usize;

        if col + w > cols {
            // Need to wrap
            if w == 2 && col + 1 == cols {
                // Wide char would split — add padding
                current.segments.push(WrapSegment::Padding);
                current.display_width += 1;
                current.has_padding = true;
            }
            rows.push(current);
            current = WrapRow {
                segments: Vec::new(),
                display_width: 0,
                has_padding: false,
            };
            col = 0;
        }

        current.segments.push(WrapSegment::Grapheme {
            text: g.to_string(),
            width: w as u8,
            source_offset: idx,
        });
        current.display_width += w;
        col += w;
    }

    rows.push(current);
    rows
}

/// Compute the total display rows for a line at given width.
pub fn display_row_count(line: &str, cols: usize) -> usize {
    if cols == 0 {
        return 1;
    }
    wrap_line(line, cols).len()
}

/// Map a source grapheme offset to (display_row, display_col).
pub fn source_to_display(line: &str, cols: usize, grapheme_offset: usize) -> (usize, usize) {
    let rows = wrap_line(line, cols);
    for (row_idx, row) in rows.iter().enumerate() {
        for seg in &row.segments {
            if let WrapSegment::Grapheme { source_offset, .. } = seg {
                if *source_offset == grapheme_offset {
                    let col: usize = row
                        .segments
                        .iter()
                        .take_while(|s| {
                            !matches!(
                                s,
                                WrapSegment::Grapheme {
                                    source_offset: o,
                                    ..
                                } if *o == grapheme_offset
                            )
                        })
                        .map(|s| match s {
                            WrapSegment::Grapheme { width, .. } => *width as usize,
                            WrapSegment::Padding => 1,
                        })
                        .sum();
                    return (row_idx, col);
                }
            }
        }
    }
    (rows.len().saturating_sub(1), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_wrap_short_line() {
        let rows = wrap_line("hello", 80);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].display_width, 5);
    }

    #[test]
    fn wrap_at_boundary() {
        let rows = wrap_line("abcde", 3);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].display_width, 3);
        assert_eq!(rows[1].display_width, 2);
    }

    #[test]
    fn cjk_padding_at_boundary() {
        // 3 cols: "aあb" -> a(1), あ(2) needs 2, col=1+2=3 fits
        let rows = wrap_line("aあb", 3);
        assert_eq!(rows.len(), 2);

        // 2 cols: "aあb" -> a(1), あ(2) col=1+2=3 > 2
        //   pad at col=1, new row: あ(2) fits
        let rows2 = wrap_line("aあb", 2);
        assert_eq!(rows2.len(), 3);
        assert!(rows2[0].has_padding);
    }

    #[test]
    fn display_row_count_simple() {
        assert_eq!(display_row_count("hello world", 5), 3);
        assert_eq!(display_row_count("hi", 80), 1);
    }

    #[test]
    fn source_to_display_mapping() {
        // "abcdef" at 3 cols: [abc][def]
        let (row, col) = source_to_display("abcdef", 3, 4);
        assert_eq!(row, 1);
        assert_eq!(col, 1); // 'd' is at col 0, 'e' at col 1
    }
}
