//! CJK support: wide character cursor clamping,
//! motion atomicity, and wrap boundary padding.

use kjxlkj_core_edit::CursorPosition;
use kjxlkj_core_text::display_width::grapheme_display_width;
use unicode_segmentation::UnicodeSegmentation;

/// Clamp a cursor position so it never lands on the
/// second column of a width-2 grapheme.
pub fn clamp_cursor_to_grapheme_boundary(line_str: &str, cursor: &mut CursorPosition) {
    let graphemes: Vec<&str> = line_str.graphemes(true).collect();
    let count = graphemes.len();
    if count == 0 {
        cursor.grapheme_offset = 0;
        return;
    }
    if cursor.grapheme_offset >= count {
        cursor.grapheme_offset = count.saturating_sub(1);
    }
    // Cursor is always on grapheme boundary by grapheme_offset
    // but verify clamping in case of external mutation
}

/// Move cursor right by one grapheme, skipping entire
/// width-2 graphemes atomically.
pub fn move_right_cjk(line_str: &str, cursor: &mut CursorPosition) -> bool {
    let count = line_str.graphemes(true).count();
    if cursor.grapheme_offset + 1 < count {
        cursor.grapheme_offset += 1;
        cursor.desired_col = None;
        true
    } else {
        false
    }
}

/// Move cursor left by one grapheme, skipping entire
/// width-2 graphemes atomically.
pub fn move_left_cjk(_line_str: &str, cursor: &mut CursorPosition) -> bool {
    if cursor.grapheme_offset > 0 {
        cursor.grapheme_offset -= 1;
        cursor.desired_col = None;
        true
    } else {
        false
    }
}

/// Compute the display width of the grapheme under cursor.
/// For CJK: returns 2. For ASCII: returns 1.
pub fn cursor_grapheme_width(line_str: &str, offset: usize) -> u8 {
    let graphemes: Vec<&str> = line_str.graphemes(true).collect();
    if offset < graphemes.len() {
        grapheme_display_width(graphemes[offset])
    } else {
        1
    }
}

/// Insert padding cells at wrap boundary when a width-2
/// grapheme would be split across lines.
///
/// Given a line of graphemes and the available columns,
/// returns wrapped display rows with padding where needed.
pub fn wrap_with_cjk_padding(line_str: &str, cols: u16) -> Vec<Vec<WrapCell>> {
    let cols = cols as usize;
    let mut rows: Vec<Vec<WrapCell>> = Vec::new();
    let mut current_row: Vec<WrapCell> = Vec::new();
    let mut col = 0;

    for g in line_str.graphemes(true) {
        let w = grapheme_display_width(g) as usize;
        if col + w > cols {
            // Pad remaining with spaces
            while col < cols {
                current_row.push(WrapCell::Padding);
                col += 1;
            }
            rows.push(current_row);
            current_row = Vec::new();
            col = 0;
        }
        current_row.push(WrapCell::Grapheme(g.to_string(), w as u8));
        col += w;
    }
    if !current_row.is_empty() || rows.is_empty() {
        rows.push(current_row);
    }
    rows
}

/// A cell in a wrapped display row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrapCell {
    /// A grapheme with its display width.
    Grapheme(String, u8),
    /// A padding cell inserted at wrap boundary.
    Padding,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_cursor() {
        let line = "aあb";
        let mut cursor = CursorPosition {
            line: 0,
            grapheme_offset: 10,
            desired_col: None,
        };
        clamp_cursor_to_grapheme_boundary(line, &mut cursor);
        assert_eq!(cursor.grapheme_offset, 2); // 3 graphemes
    }

    #[test]
    fn move_right_left() {
        let line = "あいう";
        let mut cursor = CursorPosition {
            line: 0,
            grapheme_offset: 0,
            desired_col: None,
        };
        assert!(move_right_cjk(line, &mut cursor));
        assert_eq!(cursor.grapheme_offset, 1);
        assert!(move_left_cjk(line, &mut cursor));
        assert_eq!(cursor.grapheme_offset, 0);
        assert!(!move_left_cjk(line, &mut cursor));
    }

    #[test]
    fn cursor_width() {
        let line = "aあb";
        assert_eq!(cursor_grapheme_width(line, 0), 1); // 'a'
        assert_eq!(cursor_grapheme_width(line, 1), 2); // 'あ'
        assert_eq!(cursor_grapheme_width(line, 2), 1); // 'b'
    }

    #[test]
    fn wrap_boundary_padding() {
        // 5 columns: "abあcd"
        // 'a'=1, 'b'=1, 'あ'=2, 'c'=1, 'd'=1
        // Row 1: a(1) b(1) -> col=2, あ needs 2 -> col=4 fits
        // Row 1: a b あ -> col=4, c(1) -> col=5 == cols
        // Row 1: [a, b, あ, c], Row 2: [d]
        let rows = wrap_with_cjk_padding("abあcd", 5);
        assert_eq!(rows.len(), 2);

        // 4 columns: "abあcd"
        // a(1) b(1) -> col=2, あ(2) -> col=4 == cols, fits
        // Row 1: [a, b, あ], Row 2: [c, d]
        let rows2 = wrap_with_cjk_padding("abあcd", 4);
        assert_eq!(rows2.len(), 2);

        // 3 columns: "abあcd"
        // a(1) b(1) -> col=2, あ(2) needs 2, col+2=4 > 3
        // Pad at col=2, that's 1 padding cell
        // Row 1: [a, b, Padding], Row 2: [あ, c], Row 3: [d]
        let rows3 = wrap_with_cjk_padding("abあcd", 3);
        assert_eq!(rows3.len(), 3);
        assert_eq!(rows3[0].len(), 3);
        assert_eq!(rows3[0][2], WrapCell::Padding);
    }
}
