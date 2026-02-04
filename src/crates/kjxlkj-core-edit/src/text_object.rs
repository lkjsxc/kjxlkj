//! Text object definitions.

/// A text object kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectKind {
    /// Word.
    Word,
    /// WORD (whitespace-delimited).
    BigWord,
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
    AngleBrackets,
    /// Double quotes "".
    DoubleQuotes,
    /// Single quotes ''.
    SingleQuotes,
    /// Backticks ``.
    Backticks,
    /// Tag (XML/HTML).
    Tag,
}

/// A text object with inner/around modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextObject {
    /// The text object kind.
    pub kind: TextObjectKind,
    /// Whether to include surrounding delimiters.
    pub around: bool,
    /// Count (number of nesting levels to include).
    pub count: usize,
}

impl TextObject {
    /// Create an inner text object.
    pub fn inner(kind: TextObjectKind) -> Self {
        Self {
            kind,
            around: false,
            count: 1,
        }
    }

    /// Create an around text object.
    pub fn around(kind: TextObjectKind) -> Self {
        Self {
            kind,
            around: true,
            count: 1,
        }
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_object_inner() {
        let to = TextObject::inner(TextObjectKind::Word);
        assert!(!to.around);
        assert_eq!(to.kind, TextObjectKind::Word);
    }

    #[test]
    fn text_object_around() {
        let to = TextObject::around(TextObjectKind::Parens);
        assert!(to.around);
        assert_eq!(to.kind, TextObjectKind::Parens);
    }
}
