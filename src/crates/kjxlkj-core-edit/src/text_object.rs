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
}
