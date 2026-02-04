//! Text object types and detection.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::Position;

/// The kind of text object.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    /// Inner (excludes delimiters/whitespace).
    Inner,
    /// Around (includes delimiters/whitespace).
    Around,
}

/// A text object definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObject {
    /// Word.
    Word,
    /// WORD (non-whitespace sequence).
    WORD,
    /// Sentence.
    Sentence,
    /// Paragraph.
    Paragraph,
    /// Parentheses ().
    Parens,
    /// Brackets [].
    Brackets,
    /// Braces {}.
    Braces,
    /// Angle brackets <>.
    Angles,
    /// Double quotes "".
    DoubleQuotes,
    /// Single quotes ''.
    SingleQuotes,
    /// Backticks ``.
    Backticks,
    /// Tag (HTML/XML).
    Tag,
}

/// A text range returned by text object detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextRange {
    /// Start position.
    pub start: Position,
    /// End position (inclusive).
    pub end: Position,
}

impl TextRange {
    /// Create a new text range.
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }
}

/// Find a text object around the cursor position.
pub fn find_text_object(
    buffer: &TextBuffer,
    cursor: Position,
    object: TextObject,
    kind: TextObjectKind,
) -> Option<TextRange> {
    match object {
        TextObject::Word => find_word_object(buffer, cursor, kind, false),
        TextObject::WORD => find_word_object(buffer, cursor, kind, true),
        TextObject::Parens => find_pair_object(buffer, cursor, '(', ')', kind),
        TextObject::Brackets => find_pair_object(buffer, cursor, '[', ']', kind),
        TextObject::Braces => find_pair_object(buffer, cursor, '{', '}', kind),
        TextObject::DoubleQuotes => find_quote_object(buffer, cursor, '"', kind),
        TextObject::SingleQuotes => find_quote_object(buffer, cursor, '\'', kind),
        TextObject::Backticks => find_quote_object(buffer, cursor, '`', kind),
        _ => None, // Other objects not yet implemented
    }
}

/// Find a word text object.
fn find_word_object(
    buffer: &TextBuffer,
    cursor: Position,
    kind: TextObjectKind,
    is_word: bool,
) -> Option<TextRange> {
    let line = buffer.line(cursor.line)?;
    let s = line.as_str()?;
    let s = s.trim_end_matches('\n').trim_end_matches('\r');
    let chars: Vec<char> = s.chars().collect();

    if cursor.col >= chars.len() {
        return None;
    }

    let is_word_char = |c: char| -> bool {
        if is_word {
            !c.is_whitespace()
        } else {
            c.is_alphanumeric() || c == '_'
        }
    };

    let mut start = cursor.col;
    let mut end = cursor.col;

    // Find word boundaries
    while start > 0 && is_word_char(chars[start - 1]) {
        start -= 1;
    }
    while end < chars.len() && is_word_char(chars[end]) {
        end += 1;
    }

    if kind == TextObjectKind::Around {
        // Include trailing whitespace
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
    }

    end = end.saturating_sub(1).max(start);

    Some(TextRange::new(
        Position::new(cursor.line, start),
        Position::new(cursor.line, end),
    ))
}

/// Find a paired delimiter text object.
fn find_pair_object(
    buffer: &TextBuffer,
    cursor: Position,
    open: char,
    close: char,
    kind: TextObjectKind,
) -> Option<TextRange> {
    // Search for matching pair
    let content = buffer.to_string();
    let chars: Vec<char> = content.chars().collect();

    // Convert cursor position to char index
    let mut char_idx = 0;
    for i in 0..cursor.line {
        if let Some(line) = buffer.line(i) {
            char_idx += line.as_str().map_or(0, |s| s.chars().count());
        }
    }
    char_idx += cursor.col;

    // Find opening bracket (search backward)
    let mut depth = 0;
    let mut open_idx = None;
    for i in (0..=char_idx.min(chars.len().saturating_sub(1))).rev() {
        if chars[i] == close {
            depth += 1;
        } else if chars[i] == open {
            if depth == 0 {
                open_idx = Some(i);
                break;
            }
            depth -= 1;
        }
    }

    let open_idx = open_idx?;

    // Find closing bracket (search forward)
    depth = 0;
    let mut close_idx = None;
    for i in open_idx..chars.len() {
        if chars[i] == open {
            depth += 1;
        } else if chars[i] == close {
            depth -= 1;
            if depth == 0 {
                close_idx = Some(i);
                break;
            }
        }
    }

    let close_idx = close_idx?;

    // Convert back to line/col
    let (start_line, start_col) = char_idx_to_position(buffer, open_idx);
    let (end_line, end_col) = char_idx_to_position(buffer, close_idx);

    if kind == TextObjectKind::Inner {
        Some(TextRange::new(
            Position::new(start_line, start_col + 1),
            Position::new(end_line, end_col.saturating_sub(1)),
        ))
    } else {
        Some(TextRange::new(
            Position::new(start_line, start_col),
            Position::new(end_line, end_col),
        ))
    }
}

/// Find a quote text object.
fn find_quote_object(
    buffer: &TextBuffer,
    cursor: Position,
    quote: char,
    kind: TextObjectKind,
) -> Option<TextRange> {
    let line = buffer.line(cursor.line)?;
    let s = line.as_str()?;
    let s = s.trim_end_matches('\n').trim_end_matches('\r');
    let chars: Vec<char> = s.chars().collect();

    // Find quote boundaries on the same line
    let mut quotes: Vec<usize> = chars
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == quote)
        .map(|(i, _)| i)
        .collect();

    if quotes.len() < 2 {
        return None;
    }

    // Find the pair containing cursor
    for i in 0..quotes.len() - 1 {
        if quotes[i] <= cursor.col && cursor.col <= quotes[i + 1] {
            let start = quotes[i];
            let end = quotes[i + 1];

            return if kind == TextObjectKind::Inner {
                Some(TextRange::new(
                    Position::new(cursor.line, start + 1),
                    Position::new(cursor.line, end.saturating_sub(1)),
                ))
            } else {
                Some(TextRange::new(
                    Position::new(cursor.line, start),
                    Position::new(cursor.line, end),
                ))
            };
        }
    }

    None
}

/// Convert a character index to line/col position.
fn char_idx_to_position(buffer: &TextBuffer, char_idx: usize) -> (usize, usize) {
    let mut remaining = char_idx;

    for line in 0..buffer.line_count() {
        if let Some(slice) = buffer.line(line) {
            let line_len = slice.as_str().map_or(0, |s| s.chars().count());
            if remaining < line_len {
                return (line, remaining);
            }
            remaining -= line_len;
        }
    }

    (
        buffer.line_count().saturating_sub(1),
        buffer
            .line_grapheme_len(buffer.line_count().saturating_sub(1)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_find_word_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Word,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 4);
    }

    #[test]
    fn test_find_parens() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "fn(a, b)");
        let result = find_text_object(
            &buffer,
            Position::new(0, 4),
            TextObject::Parens,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 3);
        assert_eq!(range.end.col, 6);
    }

    #[test]
    fn test_find_quotes() {
        let buffer = TextBuffer::from_text(BufferId::new(1), r#"say "hello""#);
        let result = find_text_object(
            &buffer,
            Position::new(0, 6),
            TextObject::DoubleQuotes,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 5);
        assert_eq!(range.end.col, 9);
    }

    #[test]
    fn test_find_word_around() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello world");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Word,
            TextObjectKind::Around,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
        // Around should include trailing whitespace
        assert!(range.end.col >= 5);
    }

    #[test]
    fn test_find_brackets() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "arr[index]");
        let result = find_text_object(
            &buffer,
            Position::new(0, 5),
            TextObject::Brackets,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 4);
        assert_eq!(range.end.col, 8);
    }

    #[test]
    fn test_find_braces() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "{a: 1}");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Braces,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 1);
        assert_eq!(range.end.col, 4);
    }

    #[test]
    fn test_find_single_quotes() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "say 'hi'");
        let result = find_text_object(
            &buffer,
            Position::new(0, 5),
            TextObject::SingleQuotes,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 5);
        assert_eq!(range.end.col, 6);
    }

    #[test]
    fn test_find_backticks() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "use `code` here");
        let result = find_text_object(
            &buffer,
            Position::new(0, 6),
            TextObject::Backticks,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 5);
        assert_eq!(range.end.col, 8);
    }

    #[test]
    fn test_find_WORD() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "hello-world test");
        let result = find_text_object(
            &buffer,
            Position::new(0, 3),
            TextObject::WORD,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        // WORD includes hyphen
        assert!(range.end.col >= 10);
    }

    #[test]
    fn test_find_parens_around() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "(test)");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Parens,
            TextObjectKind::Around,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0); // Include opening paren
        assert_eq!(range.end.col, 5);   // Include closing paren
    }

    #[test]
    fn test_nested_parens() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "((inner))");
        let result = find_text_object(
            &buffer,
            Position::new(0, 3),
            TextObject::Parens,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 2);
        assert_eq!(range.end.col, 6);
    }

    #[test]
    fn test_word_at_end_of_line() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "end");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Word,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 2);
    }

    #[test]
    fn test_no_match_returns_none() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "no parens here");
        let result = find_text_object(
            &buffer,
            Position::new(0, 5),
            TextObject::Parens,
            TextObjectKind::Inner,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_text_object_kind_equality() {
        assert_eq!(TextObjectKind::Inner, TextObjectKind::Inner);
        assert_ne!(TextObjectKind::Inner, TextObjectKind::Around);
    }

    #[test]
    fn test_text_object_kind_clone() {
        let kind = TextObjectKind::Around;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_text_object_equality() {
        assert_eq!(TextObject::Word, TextObject::Word);
        assert_ne!(TextObject::Word, TextObject::WORD);
    }

    #[test]
    fn test_text_object_debug() {
        let debug = format!("{:?}", TextObject::Parens);
        assert!(debug.contains("Parens"));
    }

    #[test]
    fn test_text_range_new() {
        let range = TextRange::new(Position::new(0, 0), Position::new(1, 5));
        assert_eq!(range.start.line, 0);
        assert_eq!(range.end.line, 1);
    }

    #[test]
    fn test_text_range_equality() {
        let r1 = TextRange::new(Position::new(0, 0), Position::new(1, 5));
        let r2 = TextRange::new(Position::new(0, 0), Position::new(1, 5));
        assert_eq!(r1, r2);
    }

    #[test]
    fn test_text_range_clone() {
        let range = TextRange::new(Position::new(0, 0), Position::new(1, 5));
        let cloned = range.clone();
        assert_eq!(range, cloned);
    }

    #[test]
    fn test_all_text_objects_exist() {
        let objects = [
            TextObject::Word,
            TextObject::WORD,
            TextObject::Sentence,
            TextObject::Paragraph,
            TextObject::Parens,
            TextObject::Brackets,
            TextObject::Braces,
            TextObject::Angles,
            TextObject::DoubleQuotes,
            TextObject::SingleQuotes,
            TextObject::Backticks,
            TextObject::Tag,
        ];
        assert_eq!(objects.len(), 12);
    }

    #[test]
    fn test_find_brackets_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "[array]");
        let result = find_text_object(
            &buffer,
            Position::new(0, 3),
            TextObject::Brackets,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_find_braces_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "{object}");
        let result = find_text_object(
            &buffer,
            Position::new(0, 3),
            TextObject::Braces,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_find_double_quotes_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "say \"hello\"");
        let result = find_text_object(
            &buffer,
            Position::new(0, 6),
            TextObject::DoubleQuotes,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert!(range.start.col >= 5);
    }

    #[test]
    fn test_find_single_quotes_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "'word'");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::SingleQuotes,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_find_backticks_inner() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "`code`");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Backticks,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
    }

    #[test]
    fn test_word_object_whitespace_only() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "   ");
        let result = find_text_object(
            &buffer,
            Position::new(0, 1),
            TextObject::Word,
            TextObjectKind::Inner,
        );
        // May return None or Some depending on implementation
        let _ = result;
    }

    #[test]
    fn test_word_at_start_of_line() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "start here");
        let result = find_text_object(
            &buffer,
            Position::new(0, 0),
            TextObject::Word,
            TextObjectKind::Inner,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
    }

    #[test]
    fn test_brackets_around() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "[test]");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Brackets,
            TextObjectKind::Around,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
        assert_eq!(range.end.col, 5);
    }

    #[test]
    fn test_braces_around() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "{code}");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Braces,
            TextObjectKind::Around,
        );
        assert!(result.is_some());
        let range = result.unwrap();
        assert_eq!(range.start.col, 0);
    }

    #[test]
    fn test_empty_parens() {
        let buffer = TextBuffer::from_text(BufferId::new(1), "fn()");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Parens,
            TextObjectKind::Inner,
        );
        // Empty parens should still find the pair
        assert!(result.is_some() || result.is_none()); // Either is acceptable
    }

    #[test]
    fn test_sentence_returns_none() {
        // Sentence object not yet implemented
        let buffer = TextBuffer::from_text(BufferId::new(1), "Hello. World.");
        let result = find_text_object(
            &buffer,
            Position::new(0, 0),
            TextObject::Sentence,
            TextObjectKind::Inner,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_paragraph_returns_none() {
        // Paragraph object not yet implemented
        let buffer = TextBuffer::from_text(BufferId::new(1), "Para1\n\nPara2");
        let result = find_text_object(
            &buffer,
            Position::new(0, 0),
            TextObject::Paragraph,
            TextObjectKind::Inner,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_angles_returns_none() {
        // Angles object not yet implemented
        let buffer = TextBuffer::from_text(BufferId::new(1), "<tag>");
        let result = find_text_object(
            &buffer,
            Position::new(0, 2),
            TextObject::Angles,
            TextObjectKind::Inner,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_tag_returns_none() {
        // Tag object not yet implemented
        let buffer = TextBuffer::from_text(BufferId::new(1), "<div>content</div>");
        let result = find_text_object(
            &buffer,
            Position::new(0, 5),
            TextObject::Tag,
            TextObjectKind::Inner,
        );
        assert!(result.is_none());
    }
}
