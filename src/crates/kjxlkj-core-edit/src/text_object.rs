//! Text object detection.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range, TextObjectKind};

/// Find the range of a text object from cursor position.
/// `inner` = true for inner (i), false for around (a).
pub fn find_text_object(
    buf: &TextBuffer,
    pos: Position,
    kind: TextObjectKind,
    inner: bool,
) -> Option<Range> {
    match kind {
        TextObjectKind::Word => find_word(buf, pos, inner),
        TextObjectKind::WORD => find_word_big(buf, pos, inner),
        TextObjectKind::DoubleQuote => find_quoted(buf, pos, '"', inner),
        TextObjectKind::SingleQuote => find_quoted(buf, pos, '\'', inner),
        TextObjectKind::BackTick => find_quoted(buf, pos, '`', inner),
        TextObjectKind::Paren => find_delimited(buf, pos, '(', ')', inner),
        TextObjectKind::Bracket => find_delimited(buf, pos, '[', ']', inner),
        TextObjectKind::Brace => find_delimited(buf, pos, '{', '}', inner),
        TextObjectKind::AngleBracket => find_delimited(buf, pos, '<', '>', inner),
        TextObjectKind::Paragraph => find_paragraph(buf, pos, inner),
        TextObjectKind::Sentence => find_sentence(buf, pos, inner),
        TextObjectKind::Tag => None, // Requires XML parsing
    }
}

fn find_word(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    if chars.is_empty() { return None; }
    let col = pos.col.min(chars.len().saturating_sub(1));
    let is_word = |c: char| c.is_alphanumeric() || c == '_';
    let cur_is_word = is_word(chars[col]);
    let predicate: Box<dyn Fn(char) -> bool> = if cur_is_word {
        Box::new(is_word)
    } else if chars[col].is_whitespace() {
        Box::new(|c: char| c.is_whitespace())
    } else {
        Box::new(|c: char| !is_word(c) && !c.is_whitespace())
    };

    let mut start = col;
    while start > 0 && predicate(chars[start - 1]) {
        start -= 1;
    }
    let mut end = col;
    while end + 1 < chars.len() && predicate(chars[end + 1]) {
        end += 1;
    }
    end += 1; // exclusive

    if !inner {
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

fn find_word_big(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    if chars.is_empty() { return None; }
    let col = pos.col.min(chars.len().saturating_sub(1));
    let is_ws = chars[col].is_whitespace();
    let predicate: Box<dyn Fn(char) -> bool> = if is_ws {
        Box::new(|c: char| c.is_whitespace())
    } else {
        Box::new(|c: char| !c.is_whitespace())
    };

    let mut start = col;
    while start > 0 && predicate(chars[start - 1]) {
        start -= 1;
    }
    let mut end = col;
    while end + 1 < chars.len() && predicate(chars[end + 1]) {
        end += 1;
    }
    end += 1;

    if !inner {
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
    }
    Some(Range::new(
        Position::new(pos.line, start),
        Position::new(pos.line, end),
    ))
}

fn find_quoted(
    buf: &TextBuffer,
    pos: Position,
    quote: char,
    inner: bool,
) -> Option<Range> {
    let line_str = buf.line_to_string(pos.line);
    let chars: Vec<char> = line_str.chars().collect();
    let col = pos.col;
    // Find opening quote (at or before cursor)
    let mut open = None;
    let mut i = 0;
    let mut in_quote = false;
    let mut start = 0;
    while i < chars.len() {
        if chars[i] == quote && (i == 0 || chars[i - 1] != '\\') {
            if !in_quote {
                start = i;
                in_quote = true;
            } else {
                if col >= start && col <= i {
                    open = Some((start, i));
                    break;
                }
                in_quote = false;
            }
        }
        i += 1;
    }
    // If cursor is not inside quotes, search forward
    if open.is_none() {
        let mut j = col;
        let mut found_open = false;
        while j < chars.len() {
            if chars[j] == quote {
                if !found_open {
                    start = j;
                    found_open = true;
                } else {
                    open = Some((start, j));
                    break;
                }
            }
            j += 1;
        }
    }
    let (s, e) = open?;
    if inner {
        Some(Range::new(
            Position::new(pos.line, s + 1),
            Position::new(pos.line, e),
        ))
    } else {
        Some(Range::new(
            Position::new(pos.line, s),
            Position::new(pos.line, e + 1),
        ))
    }
}

fn find_delimited(
    buf: &TextBuffer,
    pos: Position,
    open: char,
    close: char,
    inner: bool,
) -> Option<Range> {
    // Search backward for opening delimiter
    let mut depth = 0i32;
    let mut sp = pos;
    let mut found_open = false;
    loop {
        if let Some(c) = buf.char_at(sp) {
            if c == close && sp != pos { depth += 1; }
            if c == open {
                if depth == 0 {
                    found_open = true;
                    break;
                }
                depth -= 1;
            }
        }
        if sp.col > 0 {
            sp.col -= 1;
        } else if sp.line > 0 {
            sp.line -= 1;
            sp.col = buf.line_len(sp.line);
        } else {
            break;
        }
    }
    if !found_open { return None; }

    // Search forward for closing delimiter
    depth = 0;
    let mut ep = pos;
    let mut found_close = false;
    let max = buf.line_count();
    loop {
        if let Some(c) = buf.char_at(ep) {
            if c == open && ep != sp { depth += 1; }
            if c == close {
                if depth == 0 {
                    found_close = true;
                    break;
                }
                depth -= 1;
            }
        }
        ep.col += 1;
        if ep.col > buf.line_len(ep.line) {
            ep.line += 1;
            ep.col = 0;
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

fn find_paragraph(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let max = buf.line_count();
    let mut start = pos.line;
    // Find start of paragraph (first non-blank above)
    while start > 0 && buf.line_len(start.saturating_sub(1)) > 0 {
        start -= 1;
    }
    let mut end = pos.line;
    while end + 1 < max && buf.line_len(end + 1) > 0 {
        end += 1;
    }
    if !inner {
        while end + 1 < max && buf.line_len(end + 1) == 0 {
            end += 1;
        }
    }
    Some(Range::new(
        Position::new(start, 0),
        Position::new(end + 1, 0),
    ))
}

fn find_sentence(buf: &TextBuffer, pos: Position, _inner: bool) -> Option<Range> {
    // Simplified sentence: terminated by .!? followed by whitespace
    Some(Range::new(
        Position::new(pos.line, 0),
        Position::new(pos.line, buf.line_len(pos.line)),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_word() {
        let buf = TextBuffer::from_text("hello world");
        let r = find_text_object(&buf, Position::new(0, 1), TextObjectKind::Word, true);
        assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 5))));
    }

    #[test]
    fn around_word() {
        let buf = TextBuffer::from_text("hello world");
        let r = find_text_object(&buf, Position::new(0, 1), TextObjectKind::Word, false);
        // "hello " including trailing space
        assert_eq!(r, Some(Range::new(Position::new(0, 0), Position::new(0, 6))));
    }

    #[test]
    fn inner_double_quote() {
        let buf = TextBuffer::from_text(r#"say "hello" there"#);
        let r = find_text_object(&buf, Position::new(0, 6), TextObjectKind::DoubleQuote, true);
        assert_eq!(r, Some(Range::new(Position::new(0, 5), Position::new(0, 10))));
    }

    #[test]
    fn around_paren() {
        let buf = TextBuffer::from_text("fn(a, b)");
        let r = find_text_object(&buf, Position::new(0, 4), TextObjectKind::Paren, false);
        assert_eq!(r, Some(Range::new(Position::new(0, 2), Position::new(0, 8))));
    }

    #[test]
    fn inner_brace() {
        let buf = TextBuffer::from_text("{ hello }");
        let r = find_text_object(&buf, Position::new(0, 3), TextObjectKind::Brace, true);
        assert!(r.is_some());
    }
}
