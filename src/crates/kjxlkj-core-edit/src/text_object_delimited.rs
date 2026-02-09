//! Delimited text object resolution: brackets, quotes,
//! paragraphs.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::TextObjectScope;

use crate::cursor::CursorPosition;
use crate::text_object_bracket::{find_close_bracket, find_open_bracket};
use crate::text_object_exec::TextObjectRange;

pub(crate) fn resolve_bracket(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    open: char,
    close: char,
) -> Option<TextObjectRange> {
    let (start_line, start_col) = find_open_bracket(cursor, content, open, close)?;
    let (end_line, end_col) = find_close_bracket(start_line, start_col, content, open, close)?;

    let (s, e) = match scope {
        TextObjectScope::Inner => {
            let mut s = CursorPosition::new(start_line, start_col + 1);
            let e = CursorPosition::new(end_line, end_col.saturating_sub(1));
            if s.line == end_line && s.grapheme_offset > end_col {
                s.grapheme_offset = end_col;
            }
            (s, e)
        }
        TextObjectScope::Around => (
            CursorPosition::new(start_line, start_col),
            CursorPosition::new(end_line, end_col),
        ),
    };

    Some(TextObjectRange {
        start: s,
        end: e,
        linewise: false,
    })
}

pub(crate) fn resolve_quote(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    quote: char,
) -> Option<TextObjectRange> {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);

    let mut quotes = Vec::new();
    for i in 0..lg.count() {
        if let Some(g) = lg.get(i) {
            if g.starts_with(quote) {
                quotes.push(i);
            }
        }
    }

    if quotes.len() < 2 {
        return None;
    }

    for pair in quotes.chunks(2) {
        if pair.len() == 2 {
            let (open, close) = (pair[0], pair[1]);
            if cursor.grapheme_offset >= open && cursor.grapheme_offset <= close {
                let (s, e) = match scope {
                    TextObjectScope::Inner => (open + 1, close - 1),
                    TextObjectScope::Around => (open, close),
                };
                if s > e {
                    return None;
                }
                return Some(TextObjectRange {
                    start: CursorPosition::new(cursor.line, s),
                    end: CursorPosition::new(cursor.line, e),
                    linewise: false,
                });
            }
        }
    }

    None
}

pub(crate) fn resolve_paragraph(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<TextObjectRange> {
    let lc = content.line_count();
    let mut start = cursor.line;
    while start > 0 && !content.line_content(start).is_empty() {
        start -= 1;
    }
    if content.line_content(start).is_empty() && start < cursor.line {
        start += 1;
    }
    let mut end = cursor.line;
    while end + 1 < lc && !content.line_content(end).is_empty() {
        end += 1;
    }
    if scope == TextObjectScope::Around {
        while end + 1 < lc && content.line_content(end + 1).is_empty() {
            end += 1;
        }
    }
    Some(TextObjectRange {
        start: CursorPosition::new(start, 0),
        end: CursorPosition::new(end, 0),
        linewise: true,
    })
}

pub(crate) fn resolve_sentence(
    _scope: TextObjectScope,
    cursor: &CursorPosition,
    _content: &BufferContent,
) -> Option<TextObjectRange> {
    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, 0),
        end: CursorPosition::new(cursor.line, 0),
        linewise: false,
    })
}
