//! Text object implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};

/// Text object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObject {
    /// Inner word (iw).
    InnerWord,
    /// Around word (aw).
    AroundWord,
    /// Inner WORD (iW).
    InnerWORD,
    /// Around WORD (aW).
    AroundWORD,
    /// Inner quotes (i").
    InnerDoubleQuote,
    /// Around quotes (a").
    AroundDoubleQuote,
    /// Inner single quotes (i').
    InnerSingleQuote,
    /// Around single quotes (a').
    AroundSingleQuote,
    /// Inner parentheses (i( or ib).
    InnerParen,
    /// Around parentheses (a( or ab).
    AroundParen,
    /// Inner brackets (i[).
    InnerBracket,
    /// Around brackets (a[).
    AroundBracket,
    /// Inner braces (i{ or iB).
    InnerBrace,
    /// Around braces (a{ or aB).
    AroundBrace,
}

/// Find the range for a text object.
pub fn find_text_object(buf: &TextBuffer, pos: Position, obj: TextObject) -> Option<Range> {
    match obj {
        TextObject::InnerWord => find_word(buf, pos, false),
        TextObject::AroundWord => find_word(buf, pos, true),
        TextObject::InnerWORD => find_word_big(buf, pos, false),
        TextObject::AroundWORD => find_word_big(buf, pos, true),
        TextObject::InnerDoubleQuote => find_quoted(buf, pos, '"', false),
        TextObject::AroundDoubleQuote => find_quoted(buf, pos, '"', true),
        TextObject::InnerSingleQuote => find_quoted(buf, pos, '\'', false),
        TextObject::AroundSingleQuote => find_quoted(buf, pos, '\'', true),
        TextObject::InnerParen => find_matched(buf, pos, '(', ')', false),
        TextObject::AroundParen => find_matched(buf, pos, '(', ')', true),
        TextObject::InnerBracket => find_matched(buf, pos, '[', ']', false),
        TextObject::AroundBracket => find_matched(buf, pos, '[', ']', true),
        TextObject::InnerBrace => find_matched(buf, pos, '{', '}', false),
        TextObject::AroundBrace => find_matched(buf, pos, '{', '}', true),
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn find_word(buf: &TextBuffer, pos: Position, around: bool) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let chars: Vec<char> = line.chars().collect();
    if pos.col >= chars.len() {
        return None;
    }

    let mut start = pos.col;
    let mut end = pos.col;

    // Expand to word boundaries
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }

    if around {
        // Include trailing whitespace
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
    }

    Some(Range::new(
        Position::new(pos.line, start),
        Position::new(pos.line, end),
    ))
}

fn find_word_big(buf: &TextBuffer, pos: Position, around: bool) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let chars: Vec<char> = line.chars().collect();
    if pos.col >= chars.len() {
        return None;
    }

    let mut start = pos.col;
    let mut end = pos.col;

    // WORD = non-whitespace
    while start > 0 && !chars[start - 1].is_whitespace() {
        start -= 1;
    }
    while end < chars.len() && !chars[end].is_whitespace() {
        end += 1;
    }

    if around {
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
    }

    Some(Range::new(
        Position::new(pos.line, start),
        Position::new(pos.line, end),
    ))
}

fn find_quoted(buf: &TextBuffer, pos: Position, quote: char, around: bool) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let chars: Vec<char> = line.chars().collect();

    // Find opening quote
    let mut start = None;
    for i in (0..=pos.col).rev() {
        if chars.get(i) == Some(&quote) {
            start = Some(i);
            break;
        }
    }
    let start = start?;

    // Find closing quote
    let mut end = None;
    for (i, c) in chars.iter().enumerate().skip(start + 1) {
        if *c == quote {
            end = Some(i);
            break;
        }
    }
    let end = end?;

    if around {
        Some(Range::new(
            Position::new(pos.line, start),
            Position::new(pos.line, end + 1),
        ))
    } else {
        Some(Range::new(
            Position::new(pos.line, start + 1),
            Position::new(pos.line, end),
        ))
    }
}

fn find_matched(
    buf: &TextBuffer,
    pos: Position,
    open: char,
    close: char,
    around: bool,
) -> Option<Range> {
    let line = buf.line(pos.line)?;
    let chars: Vec<char> = line.chars().collect();

    // Find opening bracket
    let mut depth = 0i32;
    let mut start = None;
    for i in (0..=pos.col).rev() {
        match chars.get(i) {
            Some(c) if *c == close => depth += 1,
            Some(c) if *c == open => {
                if depth == 0 {
                    start = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let start = start?;

    // Find closing bracket
    depth = 0;
    let mut end = None;
    for i in (start + 1)..chars.len() {
        match chars.get(i) {
            Some(c) if *c == open => depth += 1,
            Some(c) if *c == close => {
                if depth == 0 {
                    end = Some(i);
                    break;
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    let end = end?;

    if around {
        Some(Range::new(
            Position::new(pos.line, start),
            Position::new(pos.line, end + 1),
        ))
    } else {
        Some(Range::new(
            Position::new(pos.line, start + 1),
            Position::new(pos.line, end),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_word() {
        let buf = TextBuffer::from_text("hello world");
        let range = find_text_object(&buf, Position::new(0, 7), TextObject::InnerWord);
        assert_eq!(
            range,
            Some(Range::new(Position::new(0, 6), Position::new(0, 11)))
        );
    }

    #[test]
    fn test_inner_quotes() {
        let buf = TextBuffer::from_text("say \"hello\" there");
        let range = find_text_object(&buf, Position::new(0, 6), TextObject::InnerDoubleQuote);
        assert_eq!(
            range,
            Some(Range::new(Position::new(0, 5), Position::new(0, 10)))
        );
    }

    #[test]
    fn test_inner_paren() {
        let buf = TextBuffer::from_text("fn(arg1, arg2)");
        let range = find_text_object(&buf, Position::new(0, 5), TextObject::InnerParen);
        assert_eq!(
            range,
            Some(Range::new(Position::new(0, 3), Position::new(0, 13)))
        );
    }
}
