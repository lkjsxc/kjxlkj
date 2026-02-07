//! Text object types for selection targets.

use serde::{Deserialize, Serialize};
use std::fmt;

/// The kind of text object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextObjectType {
    Word,
    BigWord,
    DoubleQuote,
    SingleQuote,
    BacktickQuote,
    Paren,
    Bracket,
    Brace,
    AngleBracket,
    Paragraph,
    Sentence,
    Tag,
    Argument,
    IndentLevel,
    EntireBuffer,
    Line,
    Number,
    Url,
}

/// Whether the text object selects inner content or includes delimiters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TextObjectScope {
    /// Inner: content between delimiters, excluding them.
    Inner,
    /// Outer (around): content including delimiters/surrounding whitespace.
    Outer,
}

impl TextObjectType {
    /// Returns the delimiter pair for bracket-like text objects.
    pub fn delimiters(&self) -> Option<(char, char)> {
        match self {
            Self::Paren => Some(('(', ')')),
            Self::Bracket => Some(('[', ']')),
            Self::Brace => Some(('{', '}')),
            Self::AngleBracket => Some(('<', '>')),
            Self::DoubleQuote => Some(('"', '"')),
            Self::SingleQuote => Some(('\'', '\'')),
            Self::BacktickQuote => Some(('`', '`')),
            _ => None,
        }
    }
}

impl fmt::Display for TextObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Word => "word",
            Self::BigWord => "WORD",
            Self::DoubleQuote => "double-quote",
            Self::SingleQuote => "single-quote",
            Self::BacktickQuote => "backtick",
            Self::Paren => "paren",
            Self::Bracket => "bracket",
            Self::Brace => "brace",
            Self::AngleBracket => "angle-bracket",
            Self::Paragraph => "paragraph",
            Self::Sentence => "sentence",
            Self::Tag => "tag",
            Self::Argument => "argument",
            Self::IndentLevel => "indent-level",
            Self::EntireBuffer => "entire-buffer",
            Self::Line => "line",
            Self::Number => "number",
            Self::Url => "url",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for TextObjectScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inner => write!(f, "inner"),
            Self::Outer => write!(f, "around"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paren_delimiters() {
        assert_eq!(TextObjectType::Paren.delimiters(), Some(('(', ')')));
    }

    #[test]
    fn word_has_no_delimiters() {
        assert_eq!(TextObjectType::Word.delimiters(), None);
    }
}
