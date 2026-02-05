//! Text object implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Cursor, Range};
use unicode_segmentation::UnicodeSegmentation;

/// Kind of text object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextObjectKind {
    Inner,
    Around,
}

/// Text object types.
#[derive(Debug, Clone, PartialEq)]
pub enum TextObject {
    Word,
    BigWord,
    Sentence,
    Paragraph,
    Quote(char),
    Bracket(char, char),
    Tag,
}

/// Find the range of a text object at the cursor.
pub fn find_text_object(
    obj: &TextObject,
    kind: TextObjectKind,
    cursor: Cursor,
    buffer: &TextBuffer,
) -> Option<Range> {
    match obj {
        TextObject::Word => find_word(cursor, buffer, kind, false),
        TextObject::BigWord => find_word(cursor, buffer, kind, true),
        TextObject::Quote(q) => find_quote(cursor, buffer, kind, *q),
        TextObject::Bracket(open, close) => find_bracket(cursor, buffer, kind, *open, *close),
        TextObject::Sentence => find_sentence(cursor, buffer, kind),
        TextObject::Paragraph => find_paragraph(cursor, buffer, kind),
        TextObject::Tag => find_tag(cursor, buffer, kind),
    }
}

fn find_word(
    cursor: Cursor,
    buffer: &TextBuffer,
    kind: TextObjectKind,
    big: bool,
) -> Option<Range> {
    let line = buffer.line(cursor.line).ok()?;
    let graphemes: Vec<&str> = line.graphemes(true).collect();

    if cursor.column >= graphemes.len() {
        return None;
    }

    let is_word = |c: char| {
        if big {
            !c.is_whitespace()
        } else {
            c.is_alphanumeric() || c == '_'
        }
    };

    let ch = graphemes[cursor.column].chars().next()?;
    let in_word = is_word(ch);

    // Find start of word
    let mut start = cursor.column;
    while start > 0 {
        let prev = graphemes[start - 1].chars().next()?;
        if (in_word && is_word(prev)) || (!in_word && !prev.is_whitespace() && !is_word(prev)) {
            start -= 1;
        } else {
            break;
        }
    }

    // Find end of word
    let mut end = cursor.column;
    while end + 1 < graphemes.len() {
        let next = graphemes[end + 1].chars().next()?;
        if (in_word && is_word(next)) || (!in_word && !next.is_whitespace() && !is_word(next)) {
            end += 1;
        } else {
            break;
        }
    }

    // For "around", include trailing whitespace
    if kind == TextObjectKind::Around {
        while end + 1 < graphemes.len() {
            let next = graphemes[end + 1].chars().next()?;
            if next.is_whitespace() {
                end += 1;
            } else {
                break;
            }
        }
    }

    Some(Range::from_coords(cursor.line, start, cursor.line, end + 1))
}

fn find_quote(
    cursor: Cursor,
    buffer: &TextBuffer,
    kind: TextObjectKind,
    quote: char,
) -> Option<Range> {
    let line = buffer.line(cursor.line).ok()?;
    let graphemes: Vec<&str> = line.graphemes(true).collect();

    let mut start = None;
    let mut end = None;

    // Find quotes on the line
    for (i, g) in graphemes.iter().enumerate() {
        if g.starts_with(quote) {
            if start.is_none() {
                start = Some(i);
            } else {
                end = Some(i);
                if i >= cursor.column {
                    break;
                }
            }
        }
    }

    let (s, e) = match (start, end) {
        (Some(s), Some(e)) if s <= cursor.column && cursor.column <= e => (s, e),
        _ => return None,
    };

    match kind {
        TextObjectKind::Inner => Some(Range::from_coords(cursor.line, s + 1, cursor.line, e)),
        TextObjectKind::Around => Some(Range::from_coords(cursor.line, s, cursor.line, e + 1)),
    }
}

fn find_bracket(
    cursor: Cursor,
    buffer: &TextBuffer,
    kind: TextObjectKind,
    open: char,
    close: char,
) -> Option<Range> {
    // Find matching brackets
    let mut depth = 0;
    let mut start = None;

    // Search backward for opening bracket
    let mut line = cursor.line;
    let mut col = cursor.column;

    loop {
        let text = buffer.line(line).ok()?;
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        while col > 0 || (col == 0 && line == cursor.line) {
            if col < graphemes.len() {
                let c = graphemes[col].chars().next()?;
                if c == close {
                    depth += 1;
                } else if c == open {
                    if depth == 0 {
                        start = Some((line, col));
                        break;
                    }
                    depth -= 1;
                }
            }
            if col == 0 {
                break;
            }
            col -= 1;
        }

        if start.is_some() {
            break;
        }

        if line == 0 {
            break;
        }
        line -= 1;
        col = buffer.line_len(line).ok()?.saturating_sub(1);
    }

    let (start_line, start_col) = start?;

    // Search forward for closing bracket
    depth = 0;
    line = start_line;
    col = start_col;

    loop {
        let text = buffer.line(line).ok()?;
        let graphemes: Vec<&str> = text.graphemes(true).collect();

        while col < graphemes.len() {
            let c = graphemes[col].chars().next()?;
            if c == open {
                depth += 1;
            } else if c == close {
                depth -= 1;
                if depth == 0 {
                    return match kind {
                        TextObjectKind::Inner => {
                            Some(Range::from_coords(start_line, start_col + 1, line, col))
                        }
                        TextObjectKind::Around => {
                            Some(Range::from_coords(start_line, start_col, line, col + 1))
                        }
                    };
                }
            }
            col += 1;
        }

        if line + 1 >= buffer.line_count() {
            break;
        }
        line += 1;
        col = 0;
    }

    None
}

fn find_sentence(_cursor: Cursor, _buffer: &TextBuffer, _kind: TextObjectKind) -> Option<Range> {
    // Simplified: not fully implemented
    None
}

fn find_paragraph(cursor: Cursor, buffer: &TextBuffer, kind: TextObjectKind) -> Option<Range> {
    let line_count = buffer.line_count();

    // Find paragraph start
    let mut start = cursor.line;
    while start > 0 {
        let line = buffer.line(start - 1).ok()?;
        if line.trim().is_empty() {
            break;
        }
        start -= 1;
    }

    // Find paragraph end
    let mut end = cursor.line;
    while end + 1 < line_count {
        let line = buffer.line(end + 1).ok()?;
        if line.trim().is_empty() {
            break;
        }
        end += 1;
    }

    let end_col = buffer.line_len(end).ok()?;

    match kind {
        TextObjectKind::Inner => Some(Range::from_coords(start, 0, end, end_col)),
        TextObjectKind::Around => {
            // Include trailing blank line
            let final_line = if end + 1 < line_count { end + 1 } else { end };
            Some(Range::from_coords(start, 0, final_line + 1, 0))
        }
    }
}

fn find_tag(_cursor: Cursor, _buffer: &TextBuffer, _kind: TextObjectKind) -> Option<Range> {
    // Simplified: not fully implemented
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inner_word() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 2);
        let range = find_text_object(&TextObject::Word, TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 0);
        assert_eq!(r.end.column, 5);
    }

    #[test]
    fn test_around_word() {
        let buf = TextBuffer::from_str("hello world");
        let cursor = Cursor::new(0, 2);
        let range = find_text_object(&TextObject::Word, TextObjectKind::Around, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 0);
        assert_eq!(r.end.column, 6); // includes trailing space
    }

    #[test]
    fn test_inner_quote() {
        let buf = TextBuffer::from_str("say \"hello\" please");
        let cursor = Cursor::new(0, 7);
        let range = find_text_object(&TextObject::Quote('"'), TextObjectKind::Inner, cursor, &buf);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 5);
        assert_eq!(r.end.column, 10);
    }

    #[test]
    fn test_inner_bracket() {
        let buf = TextBuffer::from_str("foo(bar)baz");
        let cursor = Cursor::new(0, 5);
        let range = find_text_object(
            &TextObject::Bracket('(', ')'),
            TextObjectKind::Inner,
            cursor,
            &buf,
        );
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.column, 4);
        assert_eq!(r.end.column, 7);
    }
}
