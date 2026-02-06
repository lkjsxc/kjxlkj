//! Extended text objects: sentence, paragraph, argument with boundary detection.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Position, Range};

/// Find sentence boundaries. Ends at `.`, `!`, or `?` followed by whitespace/EOL.
pub fn find_sentence(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let total = buf.line_count();
    let mut text = String::new();
    let mut line_offsets = Vec::with_capacity(total);
    for i in 0..total {
        line_offsets.push(text.len());
        text.push_str(&buf.line_to_string(i));
        if i + 1 < total { text.push('\n'); }
    }
    let cursor = line_offsets.get(pos.line).map(|o| o + pos.col)?;
    let bytes = text.as_bytes();
    // Find sentence start (search backward for sentence-ending punctuation + whitespace)
    let start = find_sentence_start(bytes, cursor);
    // Find sentence end (search forward for sentence-ending punctuation)
    let end = find_sentence_end(bytes, cursor);
    let (s, e) = if inner {
        // Inner: trim whitespace
        let s = skip_whitespace_forward(bytes, start);
        let e = end.min(bytes.len());
        (s, e)
    } else {
        // Around: include trailing whitespace
        let e = skip_whitespace_forward(bytes, end);
        (start, e)
    };
    Some(Range::new(
        offset_to_pos(&line_offsets, s),
        offset_to_pos(&line_offsets, e),
    ))
}

/// Find paragraph boundaries (blank-line delimited).
pub fn find_paragraph_ext(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let max = buf.line_count();
    if max == 0 { return None; }
    let mut start = pos.line;
    while start > 0 && !is_blank_line(buf, start - 1) { start -= 1; }
    let mut end = pos.line;
    while end + 1 < max && !is_blank_line(buf, end + 1) { end += 1; }
    if !inner {
        // Include trailing blank lines
        while end + 1 < max && is_blank_line(buf, end + 1) { end += 1; }
    }
    let end_col = buf.line_len(end);
    Some(Range::new(Position::new(start, 0), Position::new(end, end_col)))
}

/// Find function argument text object — content between matching `(`,`)` or `,`.
pub fn find_argument(buf: &TextBuffer, pos: Position, inner: bool) -> Option<Range> {
    let line = buf.line_to_string(pos.line);
    let chars: Vec<char> = line.chars().collect();
    let col = pos.col.min(chars.len().saturating_sub(1));
    // Find enclosing parens
    let mut depth = 0i32;
    let mut open = None;
    for i in (0..=col).rev() {
        match chars[i] {
            ')' => depth += 1,
            '(' => { if depth == 0 { open = Some(i); break; } depth -= 1; }
            _ => {}
        }
    }
    let open_idx = open?;
    depth = 0;
    let mut close = None;
    for i in col..chars.len() {
        match chars[i] {
            '(' => depth += 1,
            ')' => { if depth == 0 { close = Some(i); break; } depth -= 1; }
            _ => {}
        }
    }
    let close_idx = close?;
    // Find comma boundaries within the parens
    let mut arg_start = open_idx + 1;
    let mut arg_end = close_idx;
    depth = 0;
    for i in (open_idx + 1)..close_idx {
        match chars[i] {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 && i < col => arg_start = i + 1,
            ',' if depth == 0 && i >= col => { arg_end = i; break; }
            _ => {}
        }
    }
    if inner {
        // Trim whitespace
        while arg_start < arg_end && chars[arg_start].is_whitespace() { arg_start += 1; }
        while arg_end > arg_start && chars[arg_end - 1].is_whitespace() { arg_end -= 1; }
    }
    Some(Range::new(
        Position::new(pos.line, arg_start),
        Position::new(pos.line, arg_end),
    ))
}

fn is_blank_line(buf: &TextBuffer, line: usize) -> bool {
    buf.line_to_string(line).trim().is_empty()
}

fn find_sentence_start(bytes: &[u8], from: usize) -> usize {
    if from == 0 { return 0; }
    for i in (0..from).rev() {
        if is_sentence_end_char(bytes[i]) {
            return skip_whitespace_forward(bytes, i + 1);
        }
    }
    0
}

fn find_sentence_end(bytes: &[u8], from: usize) -> usize {
    for i in from..bytes.len() {
        if is_sentence_end_char(bytes[i]) { return i + 1; }
    }
    bytes.len()
}

fn is_sentence_end_char(b: u8) -> bool { b == b'.' || b == b'!' || b == b'?' }

fn skip_whitespace_forward(bytes: &[u8], from: usize) -> usize {
    let mut i = from;
    while i < bytes.len() && (bytes[i] == b' ' || bytes[i] == b'\t' || bytes[i] == b'\n') { i += 1; }
    i
}

fn offset_to_pos(line_offsets: &[usize], offset: usize) -> Position {
    for (i, &lo) in line_offsets.iter().enumerate().rev() {
        if offset >= lo { return Position::new(i, offset - lo); }
    }
    Position::new(0, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buf(text: &str) -> TextBuffer { TextBuffer::from_text(text) }

    #[test]
    fn sentence_single_line() {
        let b = buf("Hello world. Goodbye world.");
        let r = find_sentence(&b, Position::new(0, 14), true).unwrap();
        assert_eq!(r.start.col, 13);
    }

    #[test]
    fn paragraph_single() {
        let b = buf("Line one\nLine two\n\nLine four");
        let r = find_paragraph_ext(&b, Position::new(0, 0), true).unwrap();
        assert_eq!(r.start.line, 0);
        assert_eq!(r.end.line, 1);
    }

    #[test]
    fn paragraph_around_includes_trailing() {
        let b = buf("Line one\nLine two\n\nLine four");
        let r = find_paragraph_ext(&b, Position::new(0, 0), false).unwrap();
        assert_eq!(r.start.line, 0);
        assert!(r.end.line >= 2);
    }

    #[test]
    fn argument_inner() {
        let b = buf("fn(a, b, c)");
        let r = find_argument(&b, Position::new(0, 6), true).unwrap();
        assert_eq!(r.start.col, 6);
        assert_eq!(r.end.col, 7);
    }

    #[test]
    fn argument_first() {
        let b = buf("fn(alpha, beta)");
        let r = find_argument(&b, Position::new(0, 3), true).unwrap();
        assert_eq!(r.start.col, 3);
        assert_eq!(r.end.col, 8);
    }

    #[test]
    fn argument_around() {
        let b = buf("fn(a, b, c)");
        let r = find_argument(&b, Position::new(0, 3), false).unwrap();
        assert_eq!(r.start.col, 3);
    }

    #[test]
    fn sentence_at_start() {
        let b = buf("First sentence. Second sentence.");
        let r = find_sentence(&b, Position::new(0, 0), true).unwrap();
        assert_eq!(r.start.col, 0);
    }
}
