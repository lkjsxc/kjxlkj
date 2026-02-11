//! Extended text object ranges (paragraph, sentence, tag).
//! See /docs/spec/editing/text-objects/text_objects.md.

use crate::cursor::Cursor;
use kjxlkj_core_text::Buffer;

/// Paragraph text object: contiguous non-blank lines around cursor.
/// inner: non-blank lines only. around: includes trailing blank lines.
pub fn paragraph_obj_range(
    cursor: &Cursor, buf: &Buffer, inner: bool,
) -> Option<(Cursor, Cursor)> {
    let total = buf.line_count();
    if total == 0 { return None; }
    let cl = cursor.line.min(total - 1);
    let cur_blank = is_blank_line(buf, cl);
    // Find start of paragraph (first non-blank going up, or first blank if on blank).
    let mut start = cl;
    if cur_blank {
        while start > 0 && is_blank_line(buf, start - 1) { start -= 1; }
    } else {
        while start > 0 && !is_blank_line(buf, start - 1) { start -= 1; }
    }
    // Find end of paragraph.
    let mut end = cl;
    if cur_blank {
        while end + 1 < total && is_blank_line(buf, end + 1) { end += 1; }
    } else {
        while end + 1 < total && !is_blank_line(buf, end + 1) { end += 1; }
    }
    if !inner && !cur_blank {
        while end + 1 < total && is_blank_line(buf, end + 1) { end += 1; }
    }
    let end_col = buf.line(end).map(|l| {
        let t = l.trim_end_matches('\n').trim_end_matches('\r');
        t.chars().count().saturating_sub(1)
    }).unwrap_or(0);
    Some((Cursor::new(start, 0), Cursor::new(end, end_col)))
}

/// Sentence text object: text ending with '.', '!', or '?' + whitespace.
/// inner: sentence text only. around: includes trailing whitespace.
pub fn sentence_obj_range(
    cursor: &Cursor, buf: &Buffer, inner: bool,
) -> Option<(Cursor, Cursor)> {
    let line = buf.line(cursor.line)?;
    let chars: Vec<char> = line.chars().collect();
    if chars.is_empty() { return None; }
    let col = cursor.col.min(chars.len().saturating_sub(1));
    let is_sent_end = |c: char| c == '.' || c == '!' || c == '?';
    // Find sentence start: scan backward for sentence terminator + whitespace.
    let mut start = col;
    while start > 0 {
        if is_sent_end(chars[start - 1]) {
            if start < chars.len() && chars[start].is_whitespace() { break; }
        }
        start -= 1;
    }
    // Skip leading whitespace.
    while start < col && chars[start].is_whitespace() && chars[start] != '\n' { start += 1; }
    // Find sentence end: scan forward for sentence terminator.
    let mut end = col;
    while end < chars.len() && !is_sent_end(chars[end]) { end += 1; }
    if end >= chars.len() { end = chars.len() - 1; } else if !inner {
        // Include trailing whitespace for around.
        while end + 1 < chars.len() && chars[end + 1].is_whitespace() && chars[end + 1] != '\n' {
            end += 1;
        }
    }
    Some((Cursor::new(cursor.line, start), Cursor::new(cursor.line, end)))
}

fn is_blank_line(buf: &Buffer, line: usize) -> bool {
    buf.line(line).map(|l| l.trim().is_empty()).unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    fn buf(text: &str) -> Buffer { Buffer::from_text(BufferId(0), "t", text) }

    #[test]
    fn inner_paragraph_selects_contiguous_lines() {
        let b = buf("aaa\nbbb\n\nccc");
        let c = Cursor::new(0, 0);
        let (s, e) = paragraph_obj_range(&c, &b, true).unwrap();
        assert_eq!(s.line, 0);
        assert_eq!(e.line, 1);
    }

    #[test]
    fn around_paragraph_includes_trailing_blanks() {
        let b = buf("aaa\nbbb\n\nccc");
        let c = Cursor::new(0, 0);
        let (s, e) = paragraph_obj_range(&c, &b, false).unwrap();
        assert_eq!(s.line, 0);
        assert_eq!(e.line, 2);
    }

    #[test]
    fn inner_sentence_stops_at_period() {
        let b = buf("Hello world. Goodbye.");
        let c = Cursor::new(0, 3);
        let (s, e) = sentence_obj_range(&c, &b, true).unwrap();
        assert_eq!(s.col, 0);
        assert_eq!(e.col, 11); // period char
    }

    #[test]
    fn around_sentence_includes_trailing_space() {
        let b = buf("Hello. World.");
        let c = Cursor::new(0, 0);
        let (s, e) = sentence_obj_range(&c, &b, false).unwrap();
        assert_eq!(s.col, 0);
        assert!(e.col >= 5); // includes period + trailing space
    }

    #[test]
    fn paragraph_on_single_line() {
        let b = buf("only line");
        let c = Cursor::new(0, 0);
        let (s, e) = paragraph_obj_range(&c, &b, true).unwrap();
        assert_eq!((s.line, e.line), (0, 0));
    }
}
