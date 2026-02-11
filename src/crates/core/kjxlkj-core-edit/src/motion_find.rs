//! Character-find, paragraph, and match-paren motions.
//!
//! Extracted from motion.rs to keep files under 200 lines.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) fn paragraph_forward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    let total = buffer.line_count();
    let mut line = c.line + 1;
    while line < total {
        let text = buffer.line(line).unwrap_or_default();
        if text.trim() == "" || text.trim() == "\n" {
            c.line = line;
            c.col = 0;
            c.desired_col = 0;
            return;
        }
        line += 1;
    }
    c.line = total.saturating_sub(1);
    c.col = 0;
    c.desired_col = 0;
}

pub(crate) fn paragraph_backward(
    c: &mut Cursor,
    buffer: &Buffer,
) {
    if c.line == 0 {
        return;
    }
    let mut line = c.line - 1;
    loop {
        let text = buffer.line(line).unwrap_or_default();
        if text.trim() == "" || text.trim() == "\n" {
            c.line = line;
            c.col = 0;
            c.desired_col = 0;
            return;
        }
        if line == 0 {
            break;
        }
        line -= 1;
    }
    c.line = 0;
    c.col = 0;
    c.desired_col = 0;
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
    if c.col >= gs.len() {
        return;
    }
    let ch = gs[c.col];
    let (target, forward) = match ch {
        "(" => (")", true),
        ")" => ("(", false),
        "[" => ("]", true),
        "]" => ("[", false),
        "{" => ("}", true),
        "}" => ("{", false),
        _ => return,
    };
    let mut depth = 1i32;
    if forward {
        match_forward(c, buffer, ch, target, &mut depth);
    } else {
        match_backward(c, buffer, ch, target, &mut depth);
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
