//! Word-level motion helpers.
//!
//! Extracted from motion.rs to keep each file ≤ 200 lines.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;
use unicode_segmentation::UnicodeSegmentation;

/// Move cursor forward to the next word boundary.
pub(crate) fn word_forward(c: &mut Cursor, buffer: &Buffer) {
    let text = buffer.line(c.line).unwrap_or_default();
    let graphemes: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    // Skip to next word boundary or next line.
    while col < graphemes.len() {
        let g = graphemes[col];
        if g == "\n" {
            break;
        }
        if is_word_boundary(
            graphemes.get(col.wrapping_sub(1)).copied(),
            Some(g),
        ) {
            c.col = col;
            c.desired_col = col;
            return;
        }
        col += 1;
    }
    // Move to next line.
    if c.line + 1 < buffer.line_count() {
        c.line += 1;
        c.col = super::motion::first_nonblank_col(buffer, c.line);
        c.desired_col = c.col;
    }
}

/// Move cursor backward to the previous word boundary.
pub(crate) fn word_backward(c: &mut Cursor, buffer: &Buffer) {
    if c.col > 0 {
        let text = buffer.line(c.line).unwrap_or_default();
        let graphemes: Vec<&str> = text.graphemes(true).collect();
        let mut col = c.col - 1;
        while col > 0 {
            if is_word_boundary(
                graphemes.get(col.wrapping_sub(1)).copied(),
                graphemes.get(col).copied(),
            ) {
                c.col = col;
                c.desired_col = col;
                return;
            }
            col -= 1;
        }
        c.col = 0;
        c.desired_col = 0;
    } else if c.line > 0 {
        c.line -= 1;
        let max = super::motion::line_max_col(buffer, c.line);
        c.col = max;
        c.desired_col = max;
    }
}

fn is_word_boundary(
    prev: Option<&str>,
    curr: Option<&str>,
) -> bool {
    match (prev, curr) {
        (Some(p), Some(c)) => {
            let p_word = is_word_char(p);
            let c_word = is_word_char(c);
            p_word != c_word
        }
        _ => false,
    }
}

fn is_word_char(g: &str) -> bool {
    g.chars()
        .next()
        .map(|c| c.is_alphanumeric() || c == '_')
        .unwrap_or(false)
}
