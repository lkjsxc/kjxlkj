//! Text object resolution: converts a TextObject to a
//! cursor range.

use kjxlkj_core_text::BufferContent;
use kjxlkj_core_types::{
    TextObject, TextObjectKind, TextObjectScope,
};

use crate::cursor::CursorPosition;
use crate::text_object_delimited::{
    resolve_bracket, resolve_paragraph, resolve_quote,
    resolve_sentence,
};
use crate::text_object_argument::resolve_argument;
use crate::text_object_tag::resolve_tag;

/// Range produced by resolving a text object.
#[derive(Debug, Clone, Copy)]
pub struct TextObjectRange {
    pub start: CursorPosition,
    pub end: CursorPosition,
    pub linewise: bool,
}

/// Resolve a text object at the current cursor position.
pub fn resolve_text_object(
    obj: TextObject,
    cursor: &CursorPosition,
    content: &BufferContent,
    _count: u32,
) -> Option<TextObjectRange> {
    match obj.kind {
        TextObjectKind::Word => {
            resolve_word(
                obj.scope, cursor, content, false,
            )
        }
        TextObjectKind::BigWord => {
            resolve_word(
                obj.scope, cursor, content, true,
            )
        }
        TextObjectKind::Sentence => {
            resolve_sentence(obj.scope, cursor, content)
        }
        TextObjectKind::Paragraph => {
            resolve_paragraph(obj.scope, cursor, content)
        }
        TextObjectKind::Parens => {
            resolve_bracket(
                obj.scope, cursor, content, '(', ')',
            )
        }
        TextObjectKind::Brackets => {
            resolve_bracket(
                obj.scope, cursor, content, '[', ']',
            )
        }
        TextObjectKind::Braces => {
            resolve_bracket(
                obj.scope, cursor, content, '{', '}',
            )
        }
        TextObjectKind::AngleBrackets => {
            resolve_bracket(
                obj.scope, cursor, content, '<', '>',
            )
        }
        TextObjectKind::DoubleQuote => {
            resolve_quote(
                obj.scope, cursor, content, '"',
            )
        }
        TextObjectKind::SingleQuote => {
            resolve_quote(
                obj.scope, cursor, content, '\'',
            )
        }
        TextObjectKind::Backtick => {
            resolve_quote(
                obj.scope, cursor, content, '`',
            )
        }
        TextObjectKind::Tag => {
            resolve_tag(obj.scope, cursor, content)
        }
        TextObjectKind::Argument => {
            resolve_argument(obj.scope, cursor, content)
        }
    }
}

fn resolve_word(
    scope: TextObjectScope,
    cursor: &CursorPosition,
    content: &BufferContent,
    big_word: bool,
) -> Option<TextObjectRange> {
    let line = content.line_content(cursor.line);
    let lg =
        kjxlkj_core_text::LineGraphemes::from_str(&line);
    if lg.count() == 0 {
        return None;
    }
    let idx =
        cursor.grapheme_offset.min(lg.count() - 1);

    let current_g = lg.get(idx)?;
    let c = current_g.chars().next()?;
    let current_kind = if big_word {
        if c.is_whitespace() { 0 } else { 1 }
    } else {
        kjxlkj_core_text::classify_word_char(c) as u8
    };

    let mut start = idx;
    while start > 0 {
        let g = lg.get(start - 1)?;
        let gc = g.chars().next()?;
        let kind = if big_word {
            if gc.is_whitespace() { 0 } else { 1 }
        } else {
            kjxlkj_core_text::classify_word_char(gc)
                as u8
        };
        if kind != current_kind {
            break;
        }
        start -= 1;
    }

    let mut end = idx;
    while end + 1 < lg.count() {
        let g = lg.get(end + 1)?;
        let gc = g.chars().next()?;
        let kind = if big_word {
            if gc.is_whitespace() { 0 } else { 1 }
        } else {
            kjxlkj_core_text::classify_word_char(gc)
                as u8
        };
        if kind != current_kind {
            break;
        }
        end += 1;
    }

    if scope == TextObjectScope::Around {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_inner() {
        let content =
            BufferContent::from_str("hello world\n");
        let cursor = CursorPosition::new(0, 1);
        let obj = TextObject::new(
            TextObjectScope::Inner,
            TextObjectKind::Word,
        );
        let range = resolve_text_object(
            obj, &cursor, &content, 1,
        );
        assert!(range.is_some());
        let r = range.unwrap();
        assert_eq!(r.start.grapheme_offset, 0);
        assert_eq!(r.end.grapheme_offset, 4);
    }

    #[test]
    fn bracket_inner() {
        let content =
            BufferContent::from_str("(hello)\n");
        let cursor = CursorPosition::new(0, 3);
        let obj = TextObject::new(
            TextObjectScope::Inner,
            TextObjectKind::Parens,
        );
        let range = resolve_text_object(
            obj, &cursor, &content, 1,
        );
        assert!(range.is_some());
    }
}
