//! Big-WORD motions: W, B, E, gE.
//!
//! See /docs/spec/editing/motions/word-WORD.md.

use crate::cursor::Cursor;
use crate::motion_word::{
    is_whitespace_g, word_end_backward,
};
use kjxlkj_core_text::Buffer;
use unicode_segmentation::UnicodeSegmentation;

/// Move forward to next WORD start (W).
pub(crate) fn big_word_forward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    while col < gs.len()
        && !is_whitespace_g(gs[col])
        && gs[col] != "\n"
    {
        col += 1;
    }
    while col < gs.len() && is_whitespace_g(gs[col]) {
        col += 1;
    }
    if col < gs.len() && gs[col] != "\n" {
        c.col = col;
        c.desired_col = col;
    } else if c.line + 1 < buffer.line_count() {
        c.line += 1;
        c.col = first_nonblank(buffer, c.line);
        c.desired_col = c.col;
    }
}

/// Move backward to previous WORD start (B).
pub(crate) fn big_word_backward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    if c.col > 0 {
        let text = buffer.line(c.line).unwrap_or_default();
        let gs: Vec<&str> = text.graphemes(true).collect();
        let mut col = c.col - 1;
        while col > 0 && is_whitespace_g(gs[col]) {
            col -= 1;
        }
        while col > 0 && !is_whitespace_g(gs[col - 1]) {
            col -= 1;
        }
        c.col = col;
        c.desired_col = col;
    } else if c.line > 0 {
        c.line -= 1;
        let max =
            super::motion::line_max_col(buffer, c.line);
        c.col = max;
        c.desired_col = max;
    }
}

/// Move forward to end of WORD (E).
pub(crate) fn big_word_end_forward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    while col < gs.len() && is_whitespace_g(gs[col]) {
        col += 1;
    }
    while col + 1 < gs.len()
        && !is_whitespace_g(gs[col + 1])
        && gs[col + 1] != "\n"
    {
        col += 1;
    }
    if col < gs.len() && gs[col] != "\n" {
        c.col = col;
        c.desired_col = col;
    } else if c.line + 1 < buffer.line_count() {
        c.line += 1;
        c.col = first_nonblank(buffer, c.line);
        c.desired_col = c.col;
    }
}

/// Move backward to end of previous WORD (gE).
pub(crate) fn big_word_end_backward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    word_end_backward(c, buffer);
}

fn first_nonblank(buffer: &Buffer, line: usize) -> usize {
    super::motion::first_nonblank_col(buffer, line)
}
