//! Text object resolution: converts a TextObject to a cursor range.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::{TextObject, TextObjectKind, TextObjectScope};

use crate::cursor::CursorPosition;

/// Range produced by resolving a text object.
#[derive(Debug, Clone, Copy)]
pub struct TextObjectRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
    pub linewise: bool,
}

/// Resolve a text object at the current cursor position.
///
/// Returns the range that the text object selects, or None if
/// the text object doesn't match at the cursor position.
pub fn resolve_text_object(
    obj: TextObject,
    cursor: &CursorPosition,
    content: &BufferContent,
    _count: u32,
) -> Option<TextObjectRange> {
    match obj.kind {
        TextObjectKind::Word => resolve_word(obj.scope, cursor, content, false),
        TextObjectKind::BigWord => resolve_word(obj.scope, cursor, content, true),
        TextObjectKind::Sentence => resolve_sentence(obj.scope, cursor, content),
        TextObjectKind::Paragraph => resolve_paragraph(obj.scope, cursor, content),
        TextObjectKind::Parens => resolve_bracket(obj.scope, cursor, content, '(', ')'),
        TextObjectKind::Brackets => resolve_bracket(obj.scope, cursor, content, '[', ']'),
        TextObjectKind::Braces => resolve_bracket(obj.scope, cursor, content, '{', '}'),
        TextObjectKind::AngleBrackets => resolve_bracket(obj.scope, cursor, content, '<', '>'),
        TextObjectKind::DoubleQuote => resolve_quote(obj.scope, cursor, content, '"'),
        TextObjectKind::SingleQuote => resolve_quote(obj.scope, cursor, content, '\''),
        TextObjectKind::Backtick => resolve_quote(obj.scope, cursor, content, '`'),
        TextObjectKind::Tag => None, // Tag objects require tree-sitter; deferred
        TextObjectKind::Argument => resolve_argument(obj.scope, cursor, content),
    }
}

fn resolve_word(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    big_word: bool,
) -> Option<TextObjectRange> {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);
    if lg.count() == 0 {
        return None;
    }
    let idx = cursor.grapheme_offset.min(lg.count() - 1);

    // Classify current character
    let current_g = lg.get(idx)?;
    let c = current_g.chars().next()?;
    let current_kind = if big_word {
        if c.is_whitespace() { 0 } else { 1 }
    } else {
        kjxlkj_core_text::classify_word_char(c) as u8
    };

    // Find start of word
    let mut start = idx;
    while start > 0 {
        let g = lg.get(start - 1)?;
        let gc = g.chars().next()?;
        let kind = if big_word {
            if gc.is_whitespace() { 0 } else { 1 }
        } else {
            kjxlkj_core_text::classify_word_char(gc) as u8
        };
        if kind != current_kind {
            break;
        }
        start -= 1;
    }

    // Find end of word
    let mut end = idx;
    while end + 1 < lg.count() {
        let g = lg.get(end + 1)?;
        let gc = g.chars().next()?;
        let kind = if big_word {
            if gc.is_whitespace() { 0 } else { 1 }
        } else {
            kjxlkj_core_text::classify_word_char(gc) as u8
        };
        if kind != current_kind {
            break;
        }
        end += 1;
    }

    // For "around", extend to include surrounding whitespace
    if scope == TextObjectScope::Around {
        // Extend to trailing whitespace first
        while end + 1 < lg.count() {
            let g = lg.get(end + 1)?;
            let gc = g.chars().next()?;
            if !gc.is_whitespace() {
                break;
            }
            end += 1;
        }
    }

    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, start),
        end: CursorPosition::new(cursor.line, end),
        linewise: false,
    })
}

fn resolve_bracket(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    open: char,
    close: char,
) -> Option<TextObjectRange> {
    // Search backward for matching open bracket
    let mut depth = 0i32;
    let mut start_line = cursor.line;
    let mut start_col = cursor.grapheme_offset;
    let mut found_start = false;

    // Search from cursor backward
    for line in (0..=cursor.line).rev() {
        let lc = content.line_content(line);
        let lg = kjxlkj_core_text::LineGraphemes::from_str(&lc);
        let start_idx = if line == cursor.line {
            cursor.grapheme_offset
        } else {
            lg.count().saturating_sub(1)
        };
        for i in (0..=start_idx).rev() {
            if let Some(g) = lg.get(i) {
                let c = g.chars().next().unwrap_or(' ');
                if c == close {
                    depth += 1;
                } else if c == open {
                    if depth == 0 {
                        start_line = line;
                        start_col = i;
                        found_start = true;
                        break;
                    }
                    depth -= 1;
                }
            }
        }
        if found_start {
            break;
        }
    }

    if !found_start {
        return None;
    }

    // Search forward for matching close bracket
    depth = 0;
    let mut end_line = cursor.line;
    let mut end_col = cursor.grapheme_offset;
    let mut found_end = false;

    for line in start_line..content.line_count() {
        let lc = content.line_content(line);
        let lg = kjxlkj_core_text::LineGraphemes::from_str(&lc);
        let start_idx = if line == start_line {
            start_col
        } else {
            0
        };
        for i in start_idx..lg.count() {
            if let Some(g) = lg.get(i) {
                let c = g.chars().next().unwrap_or(' ');
                if c == open && !(line == start_line && i == start_col) {
                    depth += 1;
                } else if c == close {
                    if depth == 0 {
                        end_line = line;
                        end_col = i;
                        found_end = true;
                        break;
                    }
                    depth -= 1;
                }
            }
        }
        if found_end {
            break;
        }
    }

    if !found_end {
        return None;
    }

    let (s, e) = match scope {
        TextObjectScope::Inner => {
            // Exclude the brackets themselves
            let mut s = CursorPosition::new(start_line, start_col + 1);
            let e = CursorPosition::new(end_line, end_col.saturating_sub(1));
            if s.line == end_line && s.grapheme_offset > end_col {
                s.grapheme_offset = end_col;
            }
            (s, e)
        }
        TextObjectScope::Around => {
            (
                CursorPosition::new(start_line, start_col),
                CursorPosition::new(end_line, end_col),
            )
        }
    };

    Some(TextObjectRange {
        start: s,
        end: e,
        linewise: false,
    })
}

fn resolve_quote(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    quote: char,
) -> Option<TextObjectRange> {
    let line = content.line_content(cursor.line);
    let lg = kjxlkj_core_text::LineGraphemes::from_str(&line);

    // Find quote boundaries on the current line
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

    // Find the pair that contains or follows the cursor
    for pair in quotes.chunks(2) {
        if pair.len() == 2 {
            let (open, close) = (pair[0], pair[1]);
            if cursor.grapheme_offset >= open
                && cursor.grapheme_offset <= close
            {
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

fn resolve_paragraph(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<TextObjectRange> {
    let lc = content.line_count();
    // Find start of paragraph
    let mut start = cursor.line;
    while start > 0 && !content.line_content(start).is_empty() {
        start -= 1;
    }
    if content.line_content(start).is_empty() && start < cursor.line {
        start += 1;
    }
    // Find end
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

fn resolve_sentence(
    _scope: TextObjectScope,
    cursor: &CursorPosition,
    _content: &BufferContent,
) -> Option<TextObjectRange> {
    // Simplified: treat the whole line as a sentence
    Some(TextObjectRange {
        start: CursorPosition::new(cursor.line, 0),
        end: CursorPosition::new(cursor.line, 0),
        linewise: false,
    })
}

fn resolve_argument(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
) -> Option<TextObjectRange> {
    // Argument text object: find comma-separated argument
    resolve_bracket(scope, cursor, content, '(', ')')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_inner() {
        let content = BufferContent::from_str("hello world\n");
        let cursor = CursorPosition::new(0, 1);
        let obj = TextObject::new(
            TextObjectScope::Inner,
            TextObjectKind::Word,
        );
        let range = resolve_text_object(obj, &cursor, &content, 1);
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.grapheme_offset, 0);
        assert_eq!(r.end.grapheme_offset, 4);
    }

    #[test]
    fn bracket_inner() {
        let content = BufferContent::from_str("(hello)\n");
        let cursor = CursorPosition::new(0, 3);
        let obj = TextObject::new(
            TextObjectScope::Inner,
            TextObjectKind::Parens,
        );
        let range = resolve_text_object(obj, &cursor, &content, 1);
        assert!(range.is_some());
    }
}
