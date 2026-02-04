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

    #[test]
    fn text_object_with_count() {
        let to = TextObject::inner(TextObjectKind::Braces).with_count(3);
        assert_eq!(to.count, 3);
        assert!(!to.around);
    }

    #[test]
    fn text_object_count_min_one() {
        let to = TextObject::inner(TextObjectKind::Word).with_count(0);
        assert_eq!(to.count, 1); // Minimum count is 1
    }

    #[test]
    fn text_object_kinds_exist() {
        let _ = TextObjectKind::Word;
        let _ = TextObjectKind::BigWord;
        let _ = TextObjectKind::Sentence;
        let _ = TextObjectKind::Paragraph;
        let _ = TextObjectKind::Parens;
        let _ = TextObjectKind::Brackets;
        let _ = TextObjectKind::Braces;
        let _ = TextObjectKind::DoubleQuotes;
        let _ = TextObjectKind::SingleQuotes;
    }

    #[test]
    fn text_object_debug_format() {
        let to = TextObject::around(TextObjectKind::DoubleQuotes);
        let debug = format!("{:?}", to);
        assert!(debug.contains("DoubleQuotes"));
    }

    #[test]
    fn text_object_equality() {
        let to1 = TextObject::inner(TextObjectKind::Word);
        let to2 = TextObject::inner(TextObjectKind::Word);
        assert_eq!(to1, to2);
    }

    #[test]
    fn text_object_inequality() {
        let to1 = TextObject::inner(TextObjectKind::Word);
        let to2 = TextObject::around(TextObjectKind::Word);
        assert_ne!(to1, to2);
    }

    #[test]
    fn text_object_clone() {
        let to = TextObject::around(TextObjectKind::Tag);
        let cloned = to.clone();
        assert_eq!(to, cloned);
    }

    #[test]
    fn text_object_kind_angle_brackets() {
        let to = TextObject::inner(TextObjectKind::AngleBrackets);
        assert_eq!(to.kind, TextObjectKind::AngleBrackets);
    }

    #[test]
    fn text_object_kind_backticks() {
        let to = TextObject::around(TextObjectKind::Backticks);
        assert!(to.around);
        assert_eq!(to.kind, TextObjectKind::Backticks);
    }

    #[test]
    fn text_object_sentence() {
        let to = TextObject::inner(TextObjectKind::Sentence);
        assert_eq!(to.kind, TextObjectKind::Sentence);
    }

    #[test]
    fn text_object_paragraph() {
        let to = TextObject::around(TextObjectKind::Paragraph);
        assert!(to.around);
        assert_eq!(to.kind, TextObjectKind::Paragraph);
    }

    #[test]
    fn text_object_count_default() {
        let to = TextObject::inner(TextObjectKind::Word);
        assert_eq!(to.count, 1);
    }

    #[test]
    fn text_object_big_word() {
        let to = TextObject::inner(TextObjectKind::BigWord);
        assert_eq!(to.kind, TextObjectKind::BigWord);
    }

    #[test]
    fn text_object_braces() {
        let to = TextObject::around(TextObjectKind::Braces);
        assert_eq!(to.kind, TextObjectKind::Braces);
    }

    #[test]
    fn text_object_parens() {
        let to = TextObject::inner(TextObjectKind::Parens);
        assert_eq!(to.kind, TextObjectKind::Parens);
    }

    #[test]
    fn text_object_quotes() {
        let to = TextObject::inner(TextObjectKind::DoubleQuotes);
        assert_eq!(to.kind, TextObjectKind::DoubleQuotes);
    }
}
