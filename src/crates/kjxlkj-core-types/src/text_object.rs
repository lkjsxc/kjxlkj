//! Text object types for selection.

use serde::{Deserialize, Serialize};

/// Text object scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObjectScope {
    /// Inner - exclude delimiters.
    Inner,
    /// Around - include delimiters.
    Around,
}

/// A text object that defines a region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextObject {
    // Word objects
    Word(TextObjectScope),
    BigWord(TextObjectScope),

    // Pair objects
    Parentheses(TextObjectScope),
    Brackets(TextObjectScope),
    Braces(TextObjectScope),
    AngleBrackets(TextObjectScope),
    DoubleQuotes(TextObjectScope),
    SingleQuotes(TextObjectScope),
    BackTicks(TextObjectScope),
    Tag(TextObjectScope),

    // Block objects
    Paragraph(TextObjectScope),
    Sentence(TextObjectScope),
    Block(TextObjectScope),

    // Line object
    Line,

    // Indent object
    Indent(TextObjectScope),

    // Argument object (for function arguments)
    Argument(TextObjectScope),

    // Comment object
    Comment(TextObjectScope),

    // Entire buffer
    Buffer,
}

impl TextObject {
    /// Creates an inner word object.
    pub fn inner_word() -> Self {
        Self::Word(TextObjectScope::Inner)
    }

    /// Creates an around word object.
    pub fn around_word() -> Self {
        Self::Word(TextObjectScope::Around)
    }

    /// Creates a text object from a character with scope.
    pub fn from_char(c: char, scope: TextObjectScope) -> Option<Self> {
        match c {
            'w' => Some(Self::Word(scope)),
            'W' => Some(Self::BigWord(scope)),
            '(' | ')' | 'b' => Some(Self::Parentheses(scope)),
            '[' | ']' => Some(Self::Brackets(scope)),
            '{' | '}' | 'B' => Some(Self::Braces(scope)),
            '<' | '>' => Some(Self::AngleBrackets(scope)),
            '"' => Some(Self::DoubleQuotes(scope)),
            '\'' => Some(Self::SingleQuotes(scope)),
            '`' => Some(Self::BackTicks(scope)),
            't' => Some(Self::Tag(scope)),
            'p' => Some(Self::Paragraph(scope)),
            's' => Some(Self::Sentence(scope)),
            _ => None,
        }
    }

    /// Whether this text object is linewise.
    pub fn is_linewise(&self) -> bool {
        matches!(self, Self::Paragraph(_) | Self::Line | Self::Buffer)
    }

    /// Whether this is an inner selection.
    pub fn is_inner(&self) -> bool {
        match self {
            Self::Word(s)
            | Self::BigWord(s)
            | Self::Parentheses(s)
            | Self::Brackets(s)
            | Self::Braces(s)
            | Self::AngleBrackets(s)
            | Self::DoubleQuotes(s)
            | Self::SingleQuotes(s)
            | Self::BackTicks(s)
            | Self::Tag(s)
            | Self::Paragraph(s)
            | Self::Sentence(s)
            | Self::Block(s)
            | Self::Indent(s)
            | Self::Argument(s)
            | Self::Comment(s) => matches!(s, TextObjectScope::Inner),
            Self::Line | Self::Buffer => true,
        }
    }
}

/// Range defined by a text object.
#[derive(Debug, Clone, Copy)]
pub struct TextRange {
    /// Start line (0-based).
    pub start_line: usize,
    /// Start column (0-based).
    pub start_col: usize,
    /// End line (0-based).
    pub end_line: usize,
    /// End column (0-based, inclusive).
    pub end_col: usize,
    /// Whether this range is linewise.
    pub linewise: bool,
}

impl TextRange {
    /// Creates a new text range.
    pub fn new(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
        linewise: bool,
    ) -> Self {
        Self { start_line, start_col, end_line, end_col, linewise }
    }

    /// Creates a charwise range.
    pub fn charwise(start_line: usize, start_col: usize, end_line: usize, end_col: usize) -> Self {
        Self::new(start_line, start_col, end_line, end_col, false)
    }

    /// Creates a linewise range.
    pub fn linewise(start_line: usize, end_line: usize) -> Self {
        Self::new(start_line, 0, end_line, 0, true)
    }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool {
        self.start_line == self.end_line && self.start_col == self.end_col
    }
}
