//! Syntax highlight group definitions and matching.
//!
//! Defines the standard set of highlight groups used for syntax
//! highlighting and maps token types to groups.

/// Standard highlight group identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighlightGroup {
    Comment,
    Constant,
    String,
    Character,
    Number,
    Boolean,
    Float,
    Identifier,
    Function,
    Statement,
    Conditional,
    Repeat,
    Label,
    Operator,
    Keyword,
    Exception,
    PreProc,
    Include,
    Define,
    Macro,
    Type,
    StorageClass,
    Structure,
    Typedef,
    Special,
    SpecialChar,
    Delimiter,
    SpecialComment,
    Error,
    Todo,
    Normal,
}

/// RGB color value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
}

/// Style for a highlight group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightStyle {
    pub group: HighlightGroup,
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

/// A highlighted span within a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub group: HighlightGroup,
}

/// Map a token type string to a highlight group.
pub fn token_to_group(token_type: &str) -> HighlightGroup {
    match token_type {
        "comment" | "line_comment" | "block_comment" => HighlightGroup::Comment,
        "string" | "string_literal" => HighlightGroup::String,
        "number" | "integer_literal" | "float_literal" => HighlightGroup::Number,
        "boolean" => HighlightGroup::Boolean,
        "character" | "char_literal" => HighlightGroup::Character,
        "keyword" | "keyword_control" => HighlightGroup::Keyword,
        "function" | "function_call" | "method" => HighlightGroup::Function,
        "type" | "type_identifier" | "primitive_type" => HighlightGroup::Type,
        "operator" => HighlightGroup::Operator,
        "identifier" | "variable" => HighlightGroup::Identifier,
        "macro" | "macro_call" => HighlightGroup::Macro,
        "include" | "use" | "import" => HighlightGroup::Include,
        "conditional" | "if" | "else" | "match" => HighlightGroup::Conditional,
        "repeat" | "for" | "while" | "loop" => HighlightGroup::Repeat,
        "label" => HighlightGroup::Label,
        "exception" | "try" | "catch" => HighlightGroup::Exception,
        "delimiter" | "punctuation" => HighlightGroup::Delimiter,
        "special" => HighlightGroup::Special,
        "error" => HighlightGroup::Error,
        "todo" => HighlightGroup::Todo,
        _ => HighlightGroup::Normal,
    }
}

/// Build default highlight styles (a basic dark theme).
pub fn default_highlight_styles() -> Vec<HighlightStyle> {
    vec![
        HighlightStyle { group: HighlightGroup::Comment, fg: Some(Color::new(106, 153, 85)), bg: None, bold: false, italic: true, underline: false },
        HighlightStyle { group: HighlightGroup::String, fg: Some(Color::new(206, 145, 120)), bg: None, bold: false, italic: false, underline: false },
        HighlightStyle { group: HighlightGroup::Number, fg: Some(Color::new(181, 206, 168)), bg: None, bold: false, italic: false, underline: false },
        HighlightStyle { group: HighlightGroup::Keyword, fg: Some(Color::new(86, 156, 214)), bg: None, bold: true, italic: false, underline: false },
        HighlightStyle { group: HighlightGroup::Function, fg: Some(Color::new(220, 220, 170)), bg: None, bold: false, italic: false, underline: false },
        HighlightStyle { group: HighlightGroup::Type, fg: Some(Color::new(78, 201, 176)), bg: None, bold: false, italic: false, underline: false },
        HighlightStyle { group: HighlightGroup::Error, fg: Some(Color::new(244, 71, 71)), bg: None, bold: true, italic: false, underline: true },
    ]
}

/// Apply highlight groups to spans within a line of tokenized text.
pub fn highlight_line(tokens: &[(&str, &str)]) -> Vec<HighlightSpan> {
    let mut spans = Vec::new();
    let mut offset = 0;
    for (text, token_type) in tokens {
        let group = token_to_group(token_type);
        let len = text.len();
        if group != HighlightGroup::Normal {
            spans.push(HighlightSpan { start: offset, end: offset + len, group });
        }
        offset += len;
    }
    spans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_mapping_keywords() {
        assert_eq!(token_to_group("keyword"), HighlightGroup::Keyword);
        assert_eq!(token_to_group("comment"), HighlightGroup::Comment);
        assert_eq!(token_to_group("unknown_xyz"), HighlightGroup::Normal);
    }

    #[test]
    fn token_mapping_types() {
        assert_eq!(token_to_group("type"), HighlightGroup::Type);
        assert_eq!(token_to_group("function"), HighlightGroup::Function);
        assert_eq!(token_to_group("string"), HighlightGroup::String);
    }

    #[test]
    fn default_styles_not_empty() {
        let styles = default_highlight_styles();
        assert!(styles.len() >= 5);
        assert!(styles.iter().any(|s| s.group == HighlightGroup::Keyword));
    }

    #[test]
    fn highlight_line_spans() {
        let tokens = vec![("fn", "keyword"), (" ", "normal"), ("main", "function")];
        let spans = highlight_line(&tokens);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].group, HighlightGroup::Keyword);
        assert_eq!(spans[0].start, 0);
        assert_eq!(spans[0].end, 2);
        assert_eq!(spans[1].group, HighlightGroup::Function);
    }

    #[test]
    fn color_construction() {
        let c = Color::new(255, 128, 0);
        assert_eq!(c.r, 255);
        assert_eq!(c.g, 128);
        assert_eq!(c.b, 0);
    }

    #[test]
    fn highlight_span_positions() {
        let tokens = vec![("let", "keyword"), (" x = ", "normal"), ("42", "number")];
        let spans = highlight_line(&tokens);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[1].start, 8);
        assert_eq!(spans[1].end, 10);
    }

    #[test]
    fn conditional_and_loop_tokens() {
        assert_eq!(token_to_group("conditional"), HighlightGroup::Conditional);
        assert_eq!(token_to_group("repeat"), HighlightGroup::Repeat);
        assert_eq!(token_to_group("for"), HighlightGroup::Repeat);
    }
}
