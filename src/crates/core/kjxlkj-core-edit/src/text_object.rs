//! Text object types.

/// Text object scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObjectScope {
    /// Inner text object (excludes delimiters).
    Inner,
    /// Around text object (includes delimiters).
    Around,
}

/// Text object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextObject {
    /// Word text object.
    Word,
    /// WORD text object.
    BigWord,
    /// Sentence text object.
    Sentence,
    /// Paragraph text object.
    Paragraph,
    /// Parentheses/block text object.
    Parens,
    /// Square brackets text object.
    Brackets,
    /// Curly braces text object.
    Braces,
    /// Angle brackets text object.
    AngleBrackets,
    /// Double quote text object.
    DoubleQuote,
    /// Single quote text object.
    SingleQuote,
    /// Backtick text object.
    Backtick,
    /// Tag text object.
    Tag,
}

impl TextObject {
    /// Get the delimiter for paired text objects.
    pub fn delimiters(&self) -> Option<(char, char)> {
        match self {
            TextObject::Parens => Some(('(', ')')),
            TextObject::Brackets => Some(('[', ']')),
            TextObject::Braces => Some(('{', '}')),
            TextObject::AngleBrackets => Some(('<', '>')),
            TextObject::DoubleQuote => Some(('"', '"')),
            TextObject::SingleQuote => Some(('\'', '\'')),
            TextObject::Backtick => Some(('`', '`')),
            _ => None,
        }
    }
}

/// Parse a text object from a character.
pub fn parse_text_object(c: char) -> Option<TextObject> {
    match c {
        'w' => Some(TextObject::Word),
        'W' => Some(TextObject::BigWord),
        's' => Some(TextObject::Sentence),
        'p' => Some(TextObject::Paragraph),
        '(' | ')' | 'b' => Some(TextObject::Parens),
        '[' | ']' => Some(TextObject::Brackets),
        '{' | '}' | 'B' => Some(TextObject::Braces),
        '<' | '>' => Some(TextObject::AngleBrackets),
        '"' => Some(TextObject::DoubleQuote),
        '\'' => Some(TextObject::SingleQuote),
        '`' => Some(TextObject::Backtick),
        't' => Some(TextObject::Tag),
        _ => None,
    }
}
