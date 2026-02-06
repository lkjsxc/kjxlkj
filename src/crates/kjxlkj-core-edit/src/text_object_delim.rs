//! Delimiter and quote text object helpers.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};

pub(crate) fn find_quoted(
    buf: &TextBuffer, pos: Position, quote: char, inner: bool,
) -> Option<Range> {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    let col = pos.col;
    let mut open = None;
    let mut i = 0;
    let mut in_quote = false;
    let mut start = 0;
    while i < chars.len() {
        if chars[i] == quote && (i == 0 || chars[i - 1] != '\\') {
            if !in_quote { start = i; in_quote = true; }
            else {
                if col >= start && col <= i { open = Some((start, i)); break; }
                in_quote = false;
            }
        }
        i += 1;
    }
    if open.is_none() {
        let mut j = col;
        let mut found_open = false;
        while j < chars.len() {
            if chars[j] == quote {
                if !found_open { start = j; found_open = true; }
                else { open = Some((start, j)); break; }
            }
            j += 1;
        }
    }
    let (s, e) = open?;
    if inner {
        Some(Range::new(Position::new(pos.line, s + 1), Position::new(pos.line, e)))
    } else {
        Some(Range::new(Position::new(pos.line, s), Position::new(pos.line, e + 1)))
    }
}

pub(crate) fn find_delimited(
    buf: &TextBuffer, pos: Position, open: char, close: char, inner: bool,
) -> Option<Range> {
    let mut depth = 0i32;
    let mut sp = pos;
    let mut found_open = false;
    loop {
        if let Some(c) = buf.char_at(sp) {
            if c == close && sp != pos { depth += 1; }
            if c == open {
                if depth == 0 { found_open = true; break; }
                depth -= 1;
            }
        }
        if sp.col > 0 { sp.col -= 1; }
        else if sp.line > 0 { sp.line -= 1; sp.col = buf.line_len(sp.line); }
        else { break; }
    }
    if !found_open { return None; }

    depth = 0;
    let mut ep = pos;
    let mut found_close = false;
    let max = buf.line_count();
    loop {
        if let Some(c) = buf.char_at(ep) {
            if c == open && ep != sp { depth += 1; }
            if c == close {
                if depth == 0 { found_close = true; break; }
                depth -= 1;
            }
        }
        ep.col += 1;
        if ep.col > buf.line_len(ep.line) {
            ep.line += 1; ep.col = 0;
            if ep.line >= max { break; }
        }
    }
    if !found_close { return None; }

    if inner {
        let mut start = sp;
        start.col += 1;
        Some(Range::new(start, ep))
    } else {
        let mut end = ep;
        end.col += 1;
        Some(Range::new(sp, end))
    }
}
