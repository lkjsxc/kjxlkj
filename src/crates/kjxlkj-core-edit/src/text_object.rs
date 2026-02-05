//! Text object implementations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::CursorPosition;

/// Text object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectType {
    /// A word.
    Word,
    /// A WORD (space-delimited).
    BigWord,
    /// A sentence.
    Sentence,
    /// A paragraph.
    Paragraph,
    /// Quoted string.
    Quoted(char),
    /// Bracketed content.
    Bracket(char, char),
    /// A tag (HTML/XML).
    Tag,
}

/// Text object selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextObjectRange {
    /// Start position.
    pub start: CursorPosition,
    /// End position (inclusive).
    pub end: CursorPosition,
}

/// Select a text object.
pub fn select_text_object(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    object: TextObjectType,
    inner: bool,
) -> Option<TextObjectRange> {
    match object {
        TextObjectType::Word => select_word(buffer, cursor, inner),
        TextObjectType::BigWord => select_big_word(buffer, cursor, inner),
        TextObjectType::Sentence => select_sentence(buffer, cursor, inner),
        TextObjectType::Paragraph => select_paragraph(buffer, cursor, inner),
        TextObjectType::Quoted(quote) => select_quoted(buffer, cursor, quote, inner),
        TextObjectType::Bracket(open, close) => select_bracket(buffer, cursor, open, close, inner),
        TextObjectType::Tag => select_tag(buffer, cursor, inner),
    }
}

fn select_word(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    inner: bool,
) -> Option<TextObjectRange> {
    let line = buffer.line(cursor.line)?;
    let line = line.trim_end_matches('\n');
    let chars: Vec<char> = line.chars().collect();

    if cursor.column >= chars.len() {
        return None;
    }

    let c = chars[cursor.column];
    let is_word = is_word_char(c);

    let mut start = cursor.column;
    let mut end = cursor.column;

    if is_word {
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }
        while end < chars.len() - 1 && is_word_char(chars[end + 1]) {
            end += 1;
        }
    } else if c.is_whitespace() {
        while start > 0 && chars[start - 1].is_whitespace() {
            start -= 1;
        }
        while end < chars.len() - 1 && chars[end + 1].is_whitespace() {
            end += 1;
        }
    } else {
        while start > 0 && !is_word_char(chars[start - 1]) && !chars[start - 1].is_whitespace() {
            start -= 1;
        }
        while end < chars.len() - 1
            && !is_word_char(chars[end + 1])
            && !chars[end + 1].is_whitespace()
        {
            end += 1;
        }
    }

    if !inner {
        while end < chars.len() - 1 && chars[end + 1].is_whitespace() {
            end += 1;
        }
    }

    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, start),
        end: CursorPosition::new(cursor.line, end),
    })
}

fn select_big_word(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    inner: bool,
) -> Option<TextObjectRange> {
    let line = buffer.line(cursor.line)?;
    let line = line.trim_end_matches('\n');
    let chars: Vec<char> = line.chars().collect();

    if cursor.column >= chars.len() {
        return None;
    }

    let mut start = cursor.column;
    let mut end = cursor.column;

    while start > 0 && !chars[start - 1].is_whitespace() {
        start -= 1;
    }
    while end < chars.len() - 1 && !chars[end + 1].is_whitespace() {
        end += 1;
    }

    if !inner {
        while end < chars.len() - 1 && chars[end + 1].is_whitespace() {
            end += 1;
        }
    }

    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, start),
        end: CursorPosition::new(cursor.line, end),
    })
}

fn select_sentence(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    _inner: bool,
) -> Option<TextObjectRange> {
    let line = buffer.line(cursor.line)?;
    let line = line.trim_end_matches('\n');
    let chars: Vec<char> = line.chars().collect();

    let mut start = cursor.column;
    let mut end = cursor.column;

    while start > 0 && !is_sentence_end(chars[start - 1]) {
        start -= 1;
    }
    while end < chars.len() - 1 && !is_sentence_end(chars[end]) {
        end += 1;
    }

    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, start),
        end: CursorPosition::new(cursor.line, end),
    })
}

fn select_paragraph(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    _inner: bool,
) -> Option<TextObjectRange> {
    let mut start_line = cursor.line;
    let mut end_line = cursor.line;

    while start_line > 0 && !is_blank_line(buffer, start_line - 1) {
        start_line -= 1;
    }
    while end_line < buffer.line_count() - 1 && !is_blank_line(buffer, end_line + 1) {
        end_line += 1;
    }

    let end_col = buffer.line_len(end_line).saturating_sub(1).max(0);

    Some(TextObjectRange {
        start: CursorPosition::new(start_line, 0),
        end: CursorPosition::new(end_line, end_col),
    })
}

fn select_quoted(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    quote: char,
    inner: bool,
) -> Option<TextObjectRange> {
    let line = buffer.line(cursor.line)?;
    let line = line.trim_end_matches('\n');
    let chars: Vec<char> = line.chars().collect();

    let mut start = None;
    let mut end = None;
    let mut in_quote = false;

    for (i, &c) in chars.iter().enumerate() {
        if c == quote {
            if !in_quote {
                start = Some(i);
                in_quote = true;
            } else {
                end = Some(i);
                if i >= cursor.column && start.unwrap() <= cursor.column {
                    break;
                }
                in_quote = false;
                start = None;
            }
        }
    }

    match (start, end) {
        (Some(s), Some(e)) if s <= cursor.column && e >= cursor.column => {
            let (start, end) = if inner { (s + 1, e - 1) } else { (s, e) };
            Some(TextObjectRange {
                start: CursorPosition::new(cursor.line, start),
                end: CursorPosition::new(cursor.line, end.max(start)),
            })
        }
        _ => None,
    }
}

fn select_bracket(
    buffer: &TextBuffer,
    cursor: CursorPosition,
    open: char,
    close: char,
    inner: bool,
) -> Option<TextObjectRange> {
    let text = buffer.text();
    let chars: Vec<char> = text.chars().collect();
    let cursor_idx = buffer.pos_to_char(cursor);

    let mut start = None;
    let mut depth = 0;

    for i in (0..=cursor_idx).rev() {
        if chars[i] == close {
            depth += 1;
        } else if chars[i] == open {
            if depth == 0 {
                start = Some(i);
                break;
            }
            depth -= 1;
        }
    }

    let start = start?;
    depth = 1;

    for (i, &c) in chars.iter().enumerate().skip(start + 1) {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                let (s, e) = if inner {
                    (start + 1, i - 1)
                } else {
                    (start, i)
                };
                return Some(TextObjectRange {
                    start: buffer.char_to_pos(s),
                    end: buffer.char_to_pos(e.max(s)),
                });
            }
        }
    }

    None
}

fn select_tag(buffer: &TextBuffer, cursor: CursorPosition, inner: bool) -> Option<TextObjectRange> {
    select_bracket(buffer, cursor, '<', '>', inner)
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn is_sentence_end(c: char) -> bool {
    matches!(c, '.' | '!' | '?')
}

fn is_blank_line(buffer: &TextBuffer, line: usize) -> bool {
    buffer
        .line(line)
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::{BufferId, BufferName};

    #[test]
    fn test_select_word() {
        let buf = TextBuffer::from_text(BufferId::new(1), BufferName::new("test"), "hello world");
        let result =
            select_text_object(&buf, CursorPosition::new(0, 2), TextObjectType::Word, true);
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start, CursorPosition::new(0, 0));
        assert_eq!(range.end, CursorPosition::new(0, 4));
    }

    #[test]
    fn test_select_quoted() {
        let buf = TextBuffer::from_text(
            BufferId::new(1),
            BufferName::new("test"),
            "say \"hello\" now",
        );
        let result = select_text_object(
            &buf,
            CursorPosition::new(0, 6),
            TextObjectType::Quoted('"'),
            true,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start, CursorPosition::new(0, 5));
        assert_eq!(range.end, CursorPosition::new(0, 9));
    }
}
