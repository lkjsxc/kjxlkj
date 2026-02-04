//! Text object definitions.

/// Text object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    /// Word (w).
    Word,
    /// WORD (W).
    BigWord,
    /// Sentence (s).
    Sentence,
    /// Paragraph (p).
    Paragraph,
    /// Quoted string (", ', `).
    Quote(char),
    /// Parentheses ().
    Parens,
    /// Brackets [].
    Brackets,
    /// Braces {}.
    Braces,
    /// Angle brackets <>.
    AngleBrackets,
    /// Tag (XML/HTML).
    Tag,
}

/// Text object selection type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectType {
    /// Inner text object (i).
    Inner,
    /// Around text object (a).
    Around,
}

/// Text object with selection type.
#[derive(Debug, Clone)]
pub struct TextObject {
    /// Object kind.
    pub kind: TextObjectKind,
    /// Selection type (inner or around).
    pub obj_type: TextObjectType,
    /// Repeat count.
    pub count: usize,
}

impl TextObject {
    /// Create a new text object.
    pub fn new(kind: TextObjectKind, obj_type: TextObjectType) -> Self {
        Self {
            kind,
            obj_type,
            count: 1,
        }
    }

    /// Create an inner word text object.
    pub fn inner_word() -> Self {
        Self::new(TextObjectKind::Word, TextObjectType::Inner)
    }

    /// Create an around word text object.
    pub fn around_word() -> Self {
        Self::new(TextObjectKind::Word, TextObjectType::Around)
    }

    /// Create an inner quoted text object.
    pub fn inner_quote(quote: char) -> Self {
        Self::new(TextObjectKind::Quote(quote), TextObjectType::Inner)
    }

    /// Create an around quoted text object.
    pub fn around_quote(quote: char) -> Self {
        Self::new(TextObjectKind::Quote(quote), TextObjectType::Around)
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_word() {
        let obj = TextObject::inner_word();
        assert_eq!(obj.kind, TextObjectKind::Word);
        assert_eq!(obj.obj_type, TextObjectType::Inner);
    }

    #[test]
    fn around_quote() {
        let obj = TextObject::around_quote('"');
        assert_eq!(obj.kind, TextObjectKind::Quote('"'));
        assert_eq!(obj.obj_type, TextObjectType::Around);
    }
}
