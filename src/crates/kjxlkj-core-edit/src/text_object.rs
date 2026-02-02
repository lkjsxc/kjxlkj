//! Text object types.

use kjxlkj_core_types::{Position, Range};
use serde::{Deserialize, Serialize};

/// Kind of text object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObjectKind {
    // Word objects
    Word,
    BigWord,

    // Quote objects
    SingleQuote,
    DoubleQuote,
    BackQuote,

    // Bracket objects
    Parentheses,
    Brackets,
    Braces,
    AngleBrackets,

    // Block objects
    Sentence,
    Paragraph,

    // Tag objects
    Tag,

    // Indent objects
    Indent,

    // Entire buffer
    Entire,

    // Line
    Line,
}

/// Modifier for text objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObjectModifier {
    /// Inner (inside delimiters).
    Inner,
    /// Around (including delimiters).
    Around,
}

/// A text object selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextObject {
    /// Kind of text object.
    pub kind: TextObjectKind,
    /// Modifier (inner/around).
    pub modifier: TextObjectModifier,
    /// Repeat count.
    pub count: usize,
}

impl TextObject {
    /// Creates a new inner text object.
    pub fn inner(kind: TextObjectKind) -> Self {
        Self {
            kind,
            modifier: TextObjectModifier::Inner,
            count: 1,
        }
    }

    /// Creates a new around text object.
    pub fn around(kind: TextObjectKind) -> Self {
        Self {
            kind,
            modifier: TextObjectModifier::Around,
            count: 1,
        }
    }

    /// Sets the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Returns true if this is an inner text object.
    pub fn is_inner(&self) -> bool {
        matches!(self.modifier, TextObjectModifier::Inner)
    }

    /// Returns true if this is an around text object.
    pub fn is_around(&self) -> bool {
        matches!(self.modifier, TextObjectModifier::Around)
    }
}

/// Finds the range of a text object in the given text.
pub fn find_text_object(text: &str, pos: Position, obj: &TextObject) -> Option<Range> {
    let byte_pos = position_to_byte(text, pos)?;
    let around = obj.is_around();

    match obj.kind {
        TextObjectKind::Word => find_word(text, byte_pos, around),
        TextObjectKind::BigWord => find_bigword(text, byte_pos, around),
        TextObjectKind::Parentheses => find_pair(text, byte_pos, '(', ')', around),
        TextObjectKind::Brackets => find_pair(text, byte_pos, '[', ']', around),
        TextObjectKind::Braces => find_pair(text, byte_pos, '{', '}', around),
        TextObjectKind::AngleBrackets => find_pair(text, byte_pos, '<', '>', around),
        TextObjectKind::SingleQuote => find_quote(text, byte_pos, '\'', around),
        TextObjectKind::DoubleQuote => find_quote(text, byte_pos, '"', around),
        TextObjectKind::BackQuote => find_quote(text, byte_pos, '`', around),
        _ => None,
    }
}

/// Converts position to byte offset.
fn position_to_byte(text: &str, pos: Position) -> Option<usize> {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.char_indices() {
        if line == pos.line && col == pos.col {
            return Some(i);
        }
        if ch == '\n' {
            if line == pos.line {
                return Some(i);
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    if line == pos.line && col == pos.col {
        Some(text.len())
    } else {
        None
    }
}

/// Finds a word at position.
fn find_word(text: &str, byte_pos: usize, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    if byte_pos >= bytes.len() {
        return None;
    }

    let is_word_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let start = (0..=byte_pos)
        .rev()
        .find(|&i| i == 0 || !is_word_char(bytes[i - 1]))
        .unwrap_or(0);
    let end = (byte_pos..bytes.len())
        .find(|&i| !is_word_char(bytes[i]))
        .unwrap_or(bytes.len());

    if start >= end {
        return None;
    }

    let (final_start, final_end) = if around {
        let ws_end = (end..bytes.len())
            .find(|&i| !bytes[i].is_ascii_whitespace() || bytes[i] == b'\n')
            .unwrap_or(bytes.len());
        (start, ws_end)
    } else {
        (start, end)
    };

    byte_range_to_position(text, final_start, final_end)
}

/// Finds a WORD at position (whitespace-delimited).
fn find_bigword(text: &str, byte_pos: usize, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    if byte_pos >= bytes.len() {
        return None;
    }

    let is_word_char = |b: u8| !b.is_ascii_whitespace();
    let start = (0..=byte_pos)
        .rev()
        .find(|&i| i == 0 || !is_word_char(bytes[i - 1]))
        .unwrap_or(0);
    let end = (byte_pos..bytes.len())
        .find(|&i| !is_word_char(bytes[i]))
        .unwrap_or(bytes.len());

    if start >= end {
        return None;
    }

    let (final_start, final_end) = if around {
        let ws_end = (end..bytes.len())
            .find(|&i| !bytes[i].is_ascii_whitespace() || bytes[i] == b'\n')
            .unwrap_or(bytes.len());
        (start, ws_end)
    } else {
        (start, end)
    };

    byte_range_to_position(text, final_start, final_end)
}

/// Finds a matching pair of brackets.
fn find_pair(text: &str, byte_pos: usize, open: char, close: char, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    let open_b = open as u8;
    let close_b = close as u8;

    let mut depth = 0i32;
    let mut start = None;
    for i in (0..=byte_pos.min(bytes.len().saturating_sub(1))).rev() {
        if bytes[i] == close_b {
            depth += 1;
        } else if bytes[i] == open_b {
            if depth == 0 {
                start = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    let start = start?;

    depth = 0;
    let mut end = None;
    for i in (byte_pos.max(start + 1))..bytes.len() {
        if bytes[i] == open_b {
            depth += 1;
        } else if bytes[i] == close_b {
            if depth == 0 {
                end = Some(i);
                break;
            }
            depth -= 1;
        }
    }
    let end = end?;

    let (final_start, final_end) = if around {
        (start, end + 1)
    } else {
        (start + 1, end)
    };

    byte_range_to_position(text, final_start, final_end)
}

/// Finds a quoted string.
fn find_quote(text: &str, byte_pos: usize, quote: char, around: bool) -> Option<Range> {
    let bytes = text.as_bytes();
    let quote_b = quote as u8;
    let line_start = text[..byte_pos].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let line_end = text[byte_pos..].find('\n').map(|i| byte_pos + i).unwrap_or(text.len());

    let mut quotes = vec![];
    for i in line_start..line_end {
        if bytes.get(i) == Some(&quote_b) {
            quotes.push(i);
        }
    }

    for pair in quotes.chunks(2) {
        if pair.len() == 2 && pair[0] <= byte_pos && byte_pos <= pair[1] {
            let (final_start, final_end) = if around {
                (pair[0], pair[1] + 1)
            } else {
                (pair[0] + 1, pair[1])
            };
            return byte_range_to_position(text, final_start, final_end);
        }
    }

    None
}

/// Converts byte range to position range.
fn byte_range_to_position(text: &str, start: usize, end: usize) -> Option<Range> {
    if start >= end {
        return None;
    }
    let start_pos = byte_to_position(text, start)?;
    let end_pos = byte_to_position(text, end)?;
    Some(Range::new(start_pos, end_pos))
}

/// Converts byte offset to position.
fn byte_to_position(text: &str, byte_pos: usize) -> Option<Position> {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.char_indices() {
        if i >= byte_pos {
            return Some(Position::new(line, col));
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    if byte_pos == text.len() {
        Some(Position::new(line, col))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_object_inner() {
        let obj = TextObject::inner(TextObjectKind::Word);
        assert!(obj.is_inner());
        assert!(!obj.is_around());
    }

    #[test]
    fn test_text_object_around() {
        let obj = TextObject::around(TextObjectKind::Parentheses);
        assert!(obj.is_around());
        assert!(!obj.is_inner());
    }

    #[test]
    fn test_find_inner_word() {
        let text = "hello world";
        let obj = TextObject::inner(TextObjectKind::Word);
        let r = find_text_object(text, Position::new(0, 1), &obj).unwrap();
        assert_eq!(r.start, Position::new(0, 0));
        assert_eq!(r.end, Position::new(0, 5));
    }

    #[test]
    fn test_find_around_word() {
        let text = "hello world";
        let obj = TextObject::around(TextObjectKind::Word);
        let r = find_text_object(text, Position::new(0, 1), &obj).unwrap();
        assert_eq!(r.start, Position::new(0, 0));
        assert_eq!(r.end, Position::new(0, 6));
    }

    #[test]
    fn test_find_inner_parens() {
        let text = "foo(bar)baz";
        let obj = TextObject::inner(TextObjectKind::Parentheses);
        let r = find_text_object(text, Position::new(0, 4), &obj).unwrap();
        assert_eq!(r.start, Position::new(0, 4));
        assert_eq!(r.end, Position::new(0, 7));
    }

    #[test]
    fn test_find_around_parens() {
        let text = "foo(bar)baz";
        let obj = TextObject::around(TextObjectKind::Parentheses);
        let r = find_text_object(text, Position::new(0, 4), &obj).unwrap();
        assert_eq!(r.start, Position::new(0, 3));
        assert_eq!(r.end, Position::new(0, 8));
    }

    #[test]
    fn test_find_inner_quote() {
        let text = r#"foo "bar" baz"#;
        let obj = TextObject::inner(TextObjectKind::DoubleQuote);
        let r = find_text_object(text, Position::new(0, 5), &obj).unwrap();
        assert_eq!(r.start, Position::new(0, 5));
        assert_eq!(r.end, Position::new(0, 8));
    }
}

