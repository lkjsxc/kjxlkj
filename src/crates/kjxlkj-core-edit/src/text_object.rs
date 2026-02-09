/// Text object definitions for operator composition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextObject {
    pub kind: TextObjectKind,
    pub inner: bool, // true = inner, false = a (around)
}

/// Kinds of text objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextObjectKind {
    Word,
    BigWord,
    Sentence,
    Paragraph,
    Paren,
    Bracket,
    Brace,
    Angle,
    SingleQuote,
    DoubleQuote,
    BackQuote,
    Tag,
}

impl TextObject {
    pub fn inner(kind: TextObjectKind) -> Self {
        Self { kind, inner: true }
    }

    pub fn around(kind: TextObjectKind) -> Self {
        Self { kind, inner: false }
    }
}
