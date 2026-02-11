//! Character-find, paragraph, and match-paren motions.
//!
//! Extracted from motion.rs to keep files under 200 lines.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn paragraph_forward(c: &mut Cursor, buffer: &Buffer) {
    let total = buffer.line_count();
    for line in (c.line + 1)..total {
        let text = buffer.line(line).unwrap_or_default();
        if text.trim().is_empty() || text.trim() == "\n" {
            c.line = line; c.col = 0; c.desired_col = 0; return;
        }
    }
    c.line = total.saturating_sub(1); c.col = 0; c.desired_col = 0;
}

pub(crate) fn paragraph_backward(c: &mut Cursor, buffer: &Buffer) {
    if c.line == 0 { return; }
    for line in (0..c.line).rev() {
        let text = buffer.line(line).unwrap_or_default();
        if text.trim().is_empty() || text.trim() == "\n" {
            c.line = line; c.col = 0; c.desired_col = 0; return;
        }
    }
    c.line = 0; c.col = 0; c.desired_col = 0;
}

pub(crate) fn find_char_forward(
    c: &mut Cursor,
    buffer: &Buffer,
    ch: char,
) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    for i in (c.col + 1)..gs.len() {
        if gs[i].starts_with(ch) {
            c.col = i;
            c.desired_col = i;
            return;
        }
    }
}

pub(crate) fn find_char_backward(
    c: &mut Cursor,
    buffer: &Buffer,
    ch: char,
) {
    if c.col == 0 {
        return;
    }
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    for i in (0..c.col).rev() {
        if gs[i].starts_with(ch) {
            c.col = i;
            c.desired_col = i;
            return;
        }
    }
}

pub(crate) fn match_paren(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    let text = buffer.line(c.line).unwrap_or_default();
    let gs: Vec<&str> = text.graphemes(true).collect();
    if c.col >= gs.len() { return; }
    // Try current char first, then scan forward on line.
    let start_col = if bracket_pair(gs[c.col]).is_some() { c.col } else {
        match (c.col + 1..gs.len()).find(|&i| bracket_pair(gs[i]).is_some()) {
            Some(i) => { c.col = i; c.desired_col = i; i }
            None => return,
        }
    };
    let (target, forward) = bracket_pair(gs[start_col]).unwrap();
    let ch = gs[start_col];
    let mut depth = 1i32;
    if forward { match_forward(c, buffer, ch, target, &mut depth); }
    else { match_backward(c, buffer, ch, target, &mut depth); }
}

fn bracket_pair(ch: &str) -> Option<(&'static str, bool)> {
    match ch {
        "(" => Some((")", true)), ")" => Some(("(", false)),
        "[" => Some(("]", true)), "]" => Some(("[", false)),
        "{" => Some(("}", true)), "}" => Some(("{", false)),
        _ => None,
    }
}

fn match_forward(
    c: &mut Cursor,
    buffer: &Buffer,
    ch: &str,
    target: &str,
    depth: &mut i32,
) {
    let mut line = c.line;
    let mut col = c.col + 1;
    while line < buffer.line_count() {
        let lt = buffer.line(line).unwrap_or_default();
        let lgs: Vec<&str> =
            lt.graphemes(true).collect();
        while col < lgs.len() {
            if lgs[col] == ch {
                *depth += 1;
            } else if lgs[col] == target {
                *depth -= 1;
                if *depth == 0 {
                    c.line = line;
                    c.col = col;
                    c.desired_col = col;
                    return;
                }
            }
            col += 1;
        }
        line += 1;
        col = 0;
    }
}

fn match_backward(
    c: &mut Cursor,
    buffer: &Buffer,
    ch: &str,
    target: &str,
    depth: &mut i32,
) {
    let mut line = c.line;
    let mut col = if c.col > 0 { c.col - 1 } else { 0 };
    let mut first = true;
    loop {
        let lt = buffer.line(line).unwrap_or_default();
        let lgs: Vec<&str> =
            lt.graphemes(true).collect();
        if !first {
            col = if lgs.is_empty() {
                0
            } else {
                lgs.len() - 1
            };
        }
        first = false;
        loop {
            if col < lgs.len() {
                if lgs[col] == ch {
                    *depth += 1;
                } else if lgs[col] == target {
                    *depth -= 1;
                    if *depth == 0 {
                        c.line = line;
                        c.col = col;
                        c.desired_col = col;
                        return;
                    }
                }
            }
            if col == 0 {
                break;
            }
            col -= 1;
        }
        if line == 0 {
            break;
        }
        line -= 1;
    }
}
