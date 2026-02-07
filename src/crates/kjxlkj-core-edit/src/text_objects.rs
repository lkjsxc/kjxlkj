//! Text object range finding for motions and operators.

use kjxlkj_core_text::grapheme::is_word_char;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range, TextObjectScope, TextObjectType};

/// Find the range of a text object at the given position.
pub fn find_text_object(
    buffer: &TextBuffer,
    pos: Position,
    obj: TextObjectType,
    scope: TextObjectScope,
) -> Option<Range> {
    match obj {
        TextObjectType::Word => find_word(buffer, pos, scope, false),
        TextObjectType::BigWord => find_word(buffer, pos, scope, true),
        TextObjectType::DoubleQuote
        | TextObjectType::SingleQuote
        | TextObjectType::BacktickQuote => find_quote(buffer, pos, obj.delimiters()?.0, scope),
        TextObjectType::Paren
        | TextObjectType::Bracket
        | TextObjectType::Brace
        | TextObjectType::AngleBracket => {
            let (o, c) = obj.delimiters()?;
            find_bracket(buffer, pos, o, c, scope)
        }
        TextObjectType::Paragraph => find_paragraph(buffer, pos, scope),
        TextObjectType::Sentence => find_sentence(buffer, pos, scope),
        TextObjectType::Tag => find_tag(buffer, pos, scope),
        _ => None,
    }
}

fn find_word(buf: &TextBuffer, pos: Position, scope: TextObjectScope, big: bool) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let ch: Vec<char> = line.chars().collect();
    if pos.col >= ch.len() {
        return None;
    }
    let cls = |c: char| -> u8 {
        if big {
            u8::from(!c.is_whitespace())
        } else if is_word_char(c) {
            1
        } else if c.is_whitespace() {
            0
        } else {
            2
        }
    };
    let k = cls(ch[pos.col]);
    let mut s = pos.col;
    while s > 0 && cls(ch[s - 1]) == k {
        s -= 1;
    }
    let mut e = pos.col;
    while e + 1 < ch.len() && cls(ch[e + 1]) == k {
        e += 1;
    }
    let (rs, re) = match scope {
        TextObjectScope::Inner => (s, e + 1),
        TextObjectScope::Outer => {
            let mut e2 = e + 1;
            while e2 < ch.len() && ch[e2].is_whitespace() {
                e2 += 1;
            }
            (s, e2)
        }
    };
    Some(Range::new(
        Position::new(pos.line, rs),
        Position::new(pos.line, re),
    ))
}

fn find_quote(buf: &TextBuffer, pos: Position, q: char, scope: TextObjectScope) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let ch: Vec<char> = line.chars().collect();
    let ps: Vec<usize> = ch
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == q)
        .map(|(i, _)| i)
        .collect();
    if ps.len() < 2 {
        return None;
    }
    for pair in ps.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if pos.col >= a && pos.col <= b {
            let inner = scope == TextObjectScope::Inner;
            let s = if inner { a + 1 } else { a };
            let e = if inner { b } else { b + 1 };
            return Some(Range::new(
                Position::new(pos.line, s),
                Position::new(pos.line, e),
            ));
        }
    }
    None
}

fn find_bracket(
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

fn find_paragraph(buf: &TextBuffer, pos: Position, scope: TextObjectScope) -> Option<Range> {
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

fn find_sentence(buf: &TextBuffer, pos: Position, scope: TextObjectScope) -> Option<Range> {
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

fn find_tag(buf: &TextBuffer, pos: Position, scope: TextObjectScope) -> Option<Range> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;
    fn buf(text: &str) -> TextBuffer {
        TextBuffer::from_text(BufferId(1), "t".into(), text)
    }

    #[test]
    fn word_inner() {
        let b = buf("hello world");
        let r = find_text_object(
            &b,
            Position::new(0, 1),
            TextObjectType::Word,
            TextObjectScope::Inner,
        )
        .unwrap();
        assert_eq!(r.start.col, 0);
    }
    #[test]
    fn paren_inner() {
        let b = buf("fn(a, b)");
        let r = find_text_object(
            &b,
            Position::new(0, 4),
            TextObjectType::Paren,
            TextObjectScope::Inner,
        );
        assert!(r.is_some());
    }
}
