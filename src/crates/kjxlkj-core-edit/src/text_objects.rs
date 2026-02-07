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
            crate::text_objects_nav::find_bracket(buffer, pos, o, c, scope)
        }
        TextObjectType::Paragraph => crate::text_objects_nav::find_paragraph(buffer, pos, scope),
        TextObjectType::Sentence => crate::text_objects_nav::find_sentence(buffer, pos, scope),
        TextObjectType::Tag => crate::text_objects_nav::find_tag(buffer, pos, scope),
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
