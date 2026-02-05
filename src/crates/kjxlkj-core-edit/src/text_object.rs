//! Text objects.

/// Text object types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObject {
    /// Inner word.
    InnerWord,
    /// A word (with surrounding whitespace).
    AWord,
    /// Inner WORD.
    InnerWORD,
    /// A WORD.
    AWORD,
    /// Inner sentence.
    InnerSentence,
    /// A sentence.
    ASentence,
    /// Inner paragraph.
    InnerParagraph,
    /// A paragraph.
    AParagraph,
    /// Inner parentheses.
    InnerParen,
    /// A parentheses (including delimiters).
    AParen,
    /// Inner brackets.
    InnerBracket,
    /// A brackets.
    ABracket,
    /// Inner braces.
    InnerBrace,
    /// A braces.
    ABrace,
    /// Inner quotes.
    InnerQuote,
    /// A quotes.
    AQuote,
    /// Inner double quotes.
    InnerDoubleQuote,
    /// A double quotes.
    ADoubleQuote,
    /// Inner backticks.
    InnerBacktick,
    /// A backticks.
    ABacktick,
    /// Inner tag.
    InnerTag,
    /// A tag.
    ATag,
}
