//! Text object types for operator-pending mode.

use serde::{Deserialize, Serialize};

/// Whether the text object selects inner or around content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObjectScope {
    /// Inner: excludes surrounding delimiters/whitespace.
    Inner,
    /// Around: includes surrounding delimiters/whitespace.
    Around,
}

/// The kind of text object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObjectKind {
    /// `w` — word.
    Word,
    /// `W` — WORD.
    BigWord,
    /// `s` — sentence.
    Sentence,
    /// `p` — paragraph.
    Paragraph,
    /// `(` or `)` — parentheses.
    Parens,
    /// `[` or `]` — square brackets.
    Brackets,
    /// `{` or `}` — curly braces.
    Braces,
    /// `<` or `>` — angle brackets.
    AngleBrackets,
    /// `"` — double quotes.
    DoubleQuote,
    /// `'` — single quotes.
    SingleQuote,
    /// `` ` `` — backtick.
    Backtick,
    /// `t` — XML/HTML tag.
    Tag,
    /// `a` — function argument.
    Argument,
}

/// A text object selection combining scope and kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextObject {
    /// Inner or around selection.
    pub scope: TextObjectScope,
    /// What kind of object.
    pub kind: TextObjectKind,
}

impl TextObject {
    /// Create a new text object.
    pub fn new(scope: TextObjectScope, kind: TextObjectKind) -> Self {
        Self { scope, kind }
    }

    /// Whether the text object uses bracket-pair matching.
    pub fn is_bracket_pair(&self) -> bool {
        matches!(
            self.kind,
            TextObjectKind::Parens
                | TextObjectKind::Brackets
                | TextObjectKind::Braces
                | TextObjectKind::AngleBrackets
        )
    }

    /// Whether the text object uses quote matching.
    pub fn is_quote(&self) -> bool {
        matches!(
            self.kind,
            TextObjectKind::DoubleQuote
                | TextObjectKind::SingleQuote
                | TextObjectKind::Backtick
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_pair_detection() {
        let obj = TextObject::new(
            TextObjectScope::Inner,
            TextObjectKind::Parens,
        );
        assert!(obj.is_bracket_pair());
    }

    #[test]
    fn quote_detection() {
        let obj = TextObject::new(
            TextObjectScope::Around,
            TextObjectKind::DoubleQuote,
        );
        assert!(obj.is_quote());
    }
}
