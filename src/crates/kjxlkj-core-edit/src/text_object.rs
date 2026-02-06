//! Text object detection.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range, TextObjectKind};
use crate::text_object_delim::{find_quoted, find_delimited};

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
    Some(Range::new(Position::new(pos.line, 0), Position::new(pos.line, buf.line_len(pos.line))))
}
