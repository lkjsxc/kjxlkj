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
        TextObjectKind::Tag => find_tag(buf, pos, inner),
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

/// Find `it` (inner tag) or `at` (around tag) text objects.
fn find_tag(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    // Collect all text with line offsets for searching
    let total = buf.line_count();
    let mut full_text = String::new();
    let mut line_offsets = Vec::with_capacity(total);
    for i in 0..total {
        line_offsets.push(full_text.len());
        full_text.push_str(&buf.line_to_string(i));
        if i + 1 < total { full_text.push('\n'); }
    }
    let cursor_offset = line_offsets.get(pos.line).map(|o| o + pos.col)?;
    // Search backward for opening tag
    let bytes = full_text.as_bytes();
    let mut search = cursor_offset;
    loop {
        let open_start = find_prev_char(bytes, b'<', search)?;
        if open_start + 1 >= bytes.len() || bytes[open_start + 1] == b'/' { 
            search = open_start.checked_sub(1)?; continue;
        }
        let open_end = find_next_char(bytes, b'>', open_start)?;
        // Extract tag name
        let tag_content = &full_text[open_start + 1..open_end];
        let tag_name = tag_content.split(|c: char| c.is_whitespace() || c == '/').next()?;
        if tag_name.is_empty() { search = open_start.checked_sub(1)?; continue; }
        // Find matching closing tag
        let close_tag = format!("</{}>", tag_name);
        let close_start = full_text[open_end + 1..].find(&close_tag)
            .map(|i| i + open_end + 1)?;
        let close_end = close_start + close_tag.len();
        // Check cursor is within range
        if cursor_offset > close_end { search = open_start.checked_sub(1)?; continue; }
        // Convert offsets back to positions
        return if inner {
            let s = offset_to_pos(&line_offsets, open_end + 1);
            let e = offset_to_pos(&line_offsets, close_start);
            Some(Range::new(s, e))
        } else {
            let s = offset_to_pos(&line_offsets, open_start);
            let e = offset_to_pos(&line_offsets, close_end);
            Some(Range::new(s, e))
        };
    }
}

fn find_prev_char(bytes: &[u8], c: u8, from: usize) -> Option<usize> {
    (0..=from).rev().find(|&i| bytes[i] == c)
}
fn find_next_char(bytes: &[u8], c: u8, from: usize) -> Option<usize> {
    (from..bytes.len()).find(|&i| bytes[i] == c)
}
fn offset_to_pos(line_offsets: &[usize], offset: usize) -> Position {
    for (i, &lo) in line_offsets.iter().enumerate().rev() {
        if offset >= lo { return Position::new(i, offset - lo); }
    }
    Position::new(0, offset)
}
