//! Navigation-based text objects: brackets, paragraphs, sentences, tags.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range, TextObjectScope};

pub(crate) fn find_bracket(
    buf: &TextBuffer,
    pos: Position,
    open: char,
    close: char,
    scope: TextObjectScope,
) -> Option<Range> {
    let (sl, sc) = find_open(buf, pos, open, close)?;
    let (el, ec) = find_close(buf, pos, open, close)?;
    let inner = scope == TextObjectScope::Inner;
    let s = if inner {
        Position::new(sl, sc + 1)
    } else {
        Position::new(sl, sc)
    };
    let e = if inner {
        Position::new(el, ec)
    } else {
        Position::new(el, ec + 1)
    };
    Some(Range::new(s, e))
}

fn find_open(buf: &TextBuffer, pos: Position, open: char, close: char) -> Option<(usize, usize)> {
    let mut depth: i32 = 0;
    let mut li = pos.line;
    loop {
        let ch: Vec<char> = buf.line(li)?.chars().collect();
        let sc = if li == pos.line {
            pos.col
        } else {
            ch.len().saturating_sub(1)
        };
        for col in (0..=sc.min(ch.len().saturating_sub(1))).rev() {
            if ch[col] == close && !(li == pos.line && col == pos.col) {
                depth += 1;
            }
            if ch[col] == open {
                if depth == 0 {
                    return Some((li, col));
                }
                depth -= 1;
            }
        }
        if li == 0 {
            return None;
        }
        li -= 1;
    }
}

fn find_close(buf: &TextBuffer, pos: Position, open: char, close: char) -> Option<(usize, usize)> {
    let mut depth: i32 = 0;
    for li in pos.line..buf.line_count() {
        let ch: Vec<char> = buf.line(li)?.chars().collect();
        let sc = if li == pos.line { pos.col + 1 } else { 0 };
        for (col, &c) in ch.iter().enumerate().skip(sc) {
            if c == open {
                depth += 1;
            }
            if c == close {
                if depth == 0 {
                    return Some((li, col));
                }
                depth -= 1;
            }
        }
    }
    None
}

pub(crate) fn find_paragraph(
    buf: &TextBuffer,
    pos: Position,
    scope: TextObjectScope,
) -> Option<Range> {
    let lc = buf.line_count();
    if lc == 0 {
        return None;
    }
    let blank = |i: usize| buf.line(i).is_none_or(|l| l.trim().is_empty());
    let mut s = pos.line;
    while s > 0 && !blank(s - 1) {
        s -= 1;
    }
    let mut e = pos.line;
    while e + 1 < lc && !blank(e + 1) {
        e += 1;
    }
    if scope == TextObjectScope::Outer {
        let mut e2 = e + 1;
        while e2 < lc && blank(e2) {
            e2 += 1;
        }
        e = e2.saturating_sub(1);
    }
    Some(Range::new(
        Position::new(s, 0),
        Position::new(e, buf.line_len(e)),
    ))
}

pub(crate) fn find_sentence(
    buf: &TextBuffer,
    pos: Position,
    scope: TextObjectScope,
) -> Option<Range> {
    let ch: Vec<char> = buf.line(pos.line)?.chars().collect();
    let se = |c: char| matches!(c, '.' | '!' | '?');
    let mut s = pos.col;
    while s > 0 && !se(ch[s - 1]) {
        s -= 1;
    }
    while s < ch.len() && ch[s].is_whitespace() {
        s += 1;
    }
    let mut e = pos.col;
    while e < ch.len() && !se(ch[e]) {
        e += 1;
    }
    if e < ch.len() {
        e += 1;
    }
    if scope == TextObjectScope::Outer {
        while e < ch.len() && ch[e].is_whitespace() {
            e += 1;
        }
    }
    Some(Range::new(
        Position::new(pos.line, s),
        Position::new(pos.line, e),
    ))
}

pub(crate) fn find_tag(buf: &TextBuffer, pos: Position, scope: TextObjectScope) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let os = line[..=pos.col.min(line.len().saturating_sub(1))].rfind('<')?;
    let oe = line[os..].find('>')? + os;
    let name = line[os + 1..oe].split_whitespace().next()?;
    if name.starts_with('/') {
        return None;
    }
    let ct = format!("</{}>", name);
    for sl in pos.line..buf.line_count() {
        let l = buf.line(sl)?;
        let sf = if sl == pos.line { oe + 1 } else { 0 };
        if sf < l.len() {
            if let Some(ci) = l[sf..].find(&ct) {
                let cs = ci + sf;
                let inner = scope == TextObjectScope::Inner;
                let s = if inner {
                    Position::new(pos.line, oe + 1)
                } else {
                    Position::new(pos.line, os)
                };
                let e = if inner {
                    Position::new(sl, cs)
                } else {
                    Position::new(sl, cs + ct.len())
                };
                return Some(Range::new(s, e));
            }
        }
    }
    None
}
