//! Text object types.

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text_object_finder::find_text_object;
    use kjxlkj_core_types::Position;

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

    #[test]
    fn test_text_object_with_count() {
        let obj = TextObject::inner(TextObjectKind::Word).with_count(3);
        assert_eq!(obj.count, 3);
    }
}

