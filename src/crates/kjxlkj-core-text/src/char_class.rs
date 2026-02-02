//! Word character classification.
//!
//! Types for classifying characters in word motions.

/// Character classification for word motions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharClass {
    /// Whitespace.
    Whitespace,
    /// Word character (alphanumeric or _).
    Word,
    /// Punctuation/symbol.
    Punctuation,
}

impl CharClass {
    /// Classifies a character.
    pub fn classify(ch: char) -> Self {
        if ch.is_whitespace() {
            Self::Whitespace
        } else if ch.is_alphanumeric() || ch == '_' {
            Self::Word
        } else {
            Self::Punctuation
        }
    }

    /// Returns whether two characters are in the same class.
    pub fn same_class(a: char, b: char) -> bool {
        Self::classify(a) == Self::classify(b)
    }
}
