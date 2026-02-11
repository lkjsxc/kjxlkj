//! Text object range computation (see /docs/spec/editing/text-objects/README.md).

use crate::cursor::Cursor;
use crate::text_object_ext;
use kjxlkj_core_text::Buffer;

pub fn text_obj_range(
    cursor: &Cursor, buf: &Buffer, ch: char, inner: bool,
) -> Option<(Cursor, Cursor)> {
    match ch {
        'w' => word_obj_range(cursor, buf, inner, false),
        'W' => word_obj_range(cursor, buf, inner, true),
        '(' | ')' | 'b' => bracket_obj_range(cursor, buf, '(', ')', inner),
        '{' | '}' | 'B' => bracket_obj_range(cursor, buf, '{', '}', inner),
        '[' | ']' => bracket_obj_range(cursor, buf, '[', ']', inner),
        '<' | '>' => bracket_obj_range(cursor, buf, '<', '>', inner),
        '"' => quote_obj_range(cursor, buf, '"', inner),
        '\'' => quote_obj_range(cursor, buf, '\'', inner),
        '`' => quote_obj_range(cursor, buf, '`', inner),
        'p' => text_object_ext::paragraph_obj_range(cursor, buf, inner),
        's' => text_object_ext::sentence_obj_range(cursor, buf, inner),
        _ => None,
    }
}

fn word_obj_range(
    cursor: &Cursor, buf: &Buffer, inner: bool, big: bool,
) -> Option<(Cursor, Cursor)> {
    let line = buf.line(cursor.line)?;
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col.min(chars.len().saturating_sub(1));
    if chars.is_empty() { return None; }
    let is_word = |c: char| if big { !c.is_whitespace() } else { c.is_alphanumeric() || c == '_' };
    let cur_is_word = is_word(chars[col]);
    let mut start = col;
    while start > 0 && is_word(chars[start - 1]) == cur_is_word && !chars[start - 1].is_whitespace() {
        start -= 1;
    }
    let mut end = col;
    while end + 1 < chars.len() && is_word(chars[end + 1]) == cur_is_word && !chars[end + 1].is_whitespace() {
        end += 1;
    }
    if !inner {
        while end + 1 < chars.len() && chars[end + 1].is_whitespace() && chars[end + 1] != '\n' {
            end += 1;
        }
    }
    Some((Cursor::new(cursor.line, start), Cursor::new(cursor.line, end)))
}

fn bracket_obj_range(
    cursor: &Cursor, buf: &Buffer, open: char, close: char, inner: bool,
) -> Option<(Cursor, Cursor)> {
    let mut cursor_offset = 0usize;
    for l in 0..cursor.line {
        cursor_offset += buf.line(l).map(|s| s.len()).unwrap_or(0);
    }
    let cur_line = buf.line(cursor.line).unwrap_or_default();
    let col_byte: usize = cur_line.char_indices().nth(cursor.col).map(|(i, _)| i)
        .unwrap_or(cur_line.len());
    cursor_offset += col_byte;
    let mut full = String::new();
    for l in 0..buf.line_count() {
        full.push_str(&buf.line(l).unwrap_or_default());
    }
    let bytes = full.as_bytes();
    let (ob, cb) = (open as u8, close as u8);
    let mut depth = 0i32;
    let mut open_pos = None;
    let mut i = cursor_offset;
    loop {
        if bytes[i] == cb && i != cursor_offset { depth += 1; }
        if bytes[i] == ob {
            if depth == 0 { open_pos = Some(i); break; }
            depth -= 1;
        }
        if i == 0 { break; }
        i -= 1;
    }
    let open_byte = open_pos?;
    depth = 0;
    let mut close_pos = None;
    for j in (open_byte + 1)..full.len() {
        if bytes[j] == ob { depth += 1; }
        if bytes[j] == cb {
            if depth == 0 { close_pos = Some(j); break; }
            depth -= 1;
        }
    }
    let close_byte = close_pos?;
    if inner {
        let mut si = open_byte + 1;
        let mut ei = close_byte.saturating_sub(1);
        if si < full.len() && bytes[si] == b'\n' { si += 1; }
        if ei > si && bytes[ei] == b'\n' { ei = ei.saturating_sub(1); }
        Some((byte_to_cursor(buf, si), byte_to_cursor(buf, ei)))
    } else {
        Some((byte_to_cursor(buf, open_byte), byte_to_cursor(buf, close_byte)))
    }
}

fn quote_obj_range(
    cursor: &Cursor, buf: &Buffer, q: char, inner: bool,
) -> Option<(Cursor, Cursor)> {
    let line = buf.line(cursor.line)?;
    let chars: Vec<char> = line.chars().collect();
    let col = cursor.col;
    let mut positions: Vec<usize> = Vec::new();
    for (i, &c) in chars.iter().enumerate() { if c == q { positions.push(i); } }
    let mut pair = None;
    for chunk in positions.chunks(2) {
        if chunk.len() == 2 && chunk[0] <= col && col <= chunk[1] { pair = Some((chunk[0], chunk[1])); break; }
    }
    if pair.is_none() {
        for chunk in positions.chunks(2) {
            if chunk.len() == 2 && chunk[0] > col { pair = Some((chunk[0], chunk[1])); break; }
        }
    }
    let (qopen, qclose) = pair?;
    let (s, e) = if inner { (qopen + 1, qclose.saturating_sub(1)) } else { (qopen, qclose) };
    Some((Cursor::new(cursor.line, s), Cursor::new(cursor.line, e)))
}

fn byte_to_cursor(buf: &Buffer, byte_pos: usize) -> Cursor {
    let mut remaining = byte_pos;
    for l in 0..buf.line_count() {
        let line = buf.line(l).unwrap_or_default();
        if remaining < line.len() {
            let col = line[..remaining].chars().count();
            return Cursor::new(l, col);
        }
        remaining -= line.len();
    }
    let last = buf.line_count().saturating_sub(1);
    let last_len = buf.line(last).map(|s| s.chars().count().saturating_sub(1)).unwrap_or(0);
    Cursor::new(last, last_len)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    fn buf(text: &str) -> Buffer { Buffer::from_text(BufferId(0), "t", text) }

    #[test]
    fn word_objects() {
        let b = buf("hello world");
        let (s, e) = text_obj_range(&Cursor::new(0, 0), &b, 'w', true).unwrap();
        assert_eq!((s.col, e.col), (0, 4));
        let (s, e) = text_obj_range(&Cursor::new(0, 0), &b, 'w', false).unwrap();
        assert_eq!((s.col, e.col), (0, 5));
    }

    #[test]
    fn inner_paren() {
        let b = buf("fn(abc)");
        let c = Cursor::new(0, 3);
        let (s, e) = text_obj_range(&c, &b, '(', true).unwrap();
        assert_eq!((s.col, e.col), (3, 5));
    }

    #[test]
    fn around_paren() {
        let b = buf("fn(abc)");
        let c = Cursor::new(0, 3);
        let (s, e) = text_obj_range(&c, &b, '(', false).unwrap();
        assert_eq!((s.col, e.col), (2, 6));
    }

    #[test]
    fn inner_double_quote() {
        let b = buf("say \"hello\" end");
        let c = Cursor::new(0, 6);
        let (s, e) = text_obj_range(&c, &b, '"', true).unwrap();
        assert_eq!((s.col, e.col), (5, 9));
    }

    #[test]
    fn around_double_quote() {
        let b = buf("say \"hello\" end");
        let c = Cursor::new(0, 6);
        let (s, e) = text_obj_range(&c, &b, '"', false).unwrap();
        assert_eq!((s.col, e.col), (4, 10));
    }

    #[test]
    fn inner_brace_multiline() {
        let b = buf("if {\n  x\n}");
        let c = Cursor::new(1, 2);
        let (s, e) = text_obj_range(&c, &b, '{', true).unwrap();
        assert_eq!((s.line, s.col), (1, 0));
        assert_eq!((e.line, e.col), (1, 2));
    }
}
