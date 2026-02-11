//! Word-level motion helpers.
//!
//! Implements w, b, e, ge motions.
//! Big word motions (W, B, E, gE) are in motion_big_word.rs.
//! See /docs/spec/editing/motions/word-WORD.md.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;
use unicode_segmentation::UnicodeSegmentation;

/// Move cursor forward to the next word boundary (w).
pub(crate) fn word_forward(c: &mut Cursor, buffer: &Buffer) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    while col < gs.len() {
        if gs[col] == "\n" {
            break;
        }
        if is_word_boundary(gs.get(col - 1).copied(), Some(gs[col]))
            && !is_whitespace_g(gs[col])
        {
            c.col = col;
            c.desired_col = col;
            return;
        }
        col += 1;
    }
    if c.line + 1 < buffer.line_count() {
        c.line += 1;
        c.col = first_nonblank(buffer, c.line);
        c.desired_col = c.col;
    }
}

/// Move cursor backward to the previous word start (b).
pub(crate) fn word_backward(c: &mut Cursor, buffer: &Buffer) {
    if c.col > 0 {
        let text = buffer.line(c.line).unwrap_or_default();
        let gs: Vec<&str> = text.graphemes(true).collect();
        let mut col = c.col - 1;
        while col > 0 {
            if is_word_boundary(
                gs.get(col - 1).copied(),
                gs.get(col).copied(),
            ) && !is_whitespace_g(gs[col])
            {
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

/// Move cursor forward to end of current/next word (e).
pub(crate) fn word_end_forward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    let mut col = c.col + 1;
    while col < gs.len() && is_whitespace_g(gs[col]) {
        col += 1;
    }
    while col + 1 < gs.len() {
        if gs[col + 1] == "\n" || is_word_boundary(
            Some(gs[col]),
            gs.get(col + 1).copied(),
        ) {
            c.col = col;
            c.desired_col = col;
            return;
        }
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

/// Move cursor backward to end of previous word (ge).
pub(crate) fn word_end_backward(
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
        c.col = col;
        c.desired_col = col;
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
            char_class(p) != char_class(c)
        }
        _ => false,
    }
}

fn char_class(g: &str) -> u8 {
    let c = g.chars().next().unwrap_or(' ');
    if c.is_alphanumeric() || c == '_' {
        1
    } else if c.is_whitespace() {
        0
    } else {
        2
    }
}

pub(crate) fn is_whitespace_g(g: &str) -> bool {
    g.chars().all(|c| c == ' ' || c == '\t')
}

fn first_nonblank(buffer: &Buffer, line: usize) -> usize {
    super::motion::first_nonblank_col(buffer, line)
}
