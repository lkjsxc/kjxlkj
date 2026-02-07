//! Syntax highlight groups and token-to-group mapping.

use crate::theme_full::Face;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::theme_full::{Rgb, ThemeColor};

/// Syntax highlight group (31 groups).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HighlightGroup {
    Comment,
    String,
    Number,
    Keyword,
    Function,
    Type,
    Variable,
    Constant,
    Operator,
    Delimiter,
    Identifier,
    Statement,
    PreProc,
    Include,
    Define,
    Macro,
    StorageClass,
    Structure,
    Typedef,
    Special,
    SpecialChar,
    Tag,
    Error,
    Todo,
    MatchParen,
    Search,
    Visual,
    CursorLine,
    LineNr,
    CursorLineNr,
    StatusLine,
}

/// Map a token type name to a highlight group.
pub fn token_to_group(token: &str) -> Option<HighlightGroup> {
    Some(match token {
        "comment" => HighlightGroup::Comment,
        "string" => HighlightGroup::String,
        "number" => HighlightGroup::Number,
        "keyword" => HighlightGroup::Keyword,
        "function" => HighlightGroup::Function,
        "type" => HighlightGroup::Type,
        "variable" => HighlightGroup::Variable,
        "constant" => HighlightGroup::Constant,
        "operator" => HighlightGroup::Operator,
        "delimiter" => HighlightGroup::Delimiter,
        "identifier" => HighlightGroup::Identifier,
        "statement" => HighlightGroup::Statement,
        "preproc" => HighlightGroup::PreProc,
        "include" => HighlightGroup::Include,
        "define" => HighlightGroup::Define,
        "macro" => HighlightGroup::Macro,
        "storage_class" => HighlightGroup::StorageClass,
        "structure" => HighlightGroup::Structure,
        "typedef" => HighlightGroup::Typedef,
        "special" => HighlightGroup::Special,
        "special_char" => HighlightGroup::SpecialChar,
        "tag" => HighlightGroup::Tag,
        "error" => HighlightGroup::Error,
        "todo" => HighlightGroup::Todo,
        "match_paren" => HighlightGroup::MatchParen,
        "search" => HighlightGroup::Search,
        "visual" => HighlightGroup::Visual,
        "cursor_line" => HighlightGroup::CursorLine,
        "line_nr" => HighlightGroup::LineNr,
        "cursor_line_nr" => HighlightGroup::CursorLineNr,
        "status_line" => HighlightGroup::StatusLine,
        _ => return None,
    })
}

/// A highlighted span within a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightSpan {
    pub start_col: usize,
    pub end_col: usize,
    pub group: HighlightGroup,
}

/// Convert token spans `(start, end, token_type)` to [`HighlightSpan`]s.
pub fn highlight_line(tokens: &[(usize, usize, &str)]) -> Vec<HighlightSpan> {
    tokens
        .iter()
        .filter_map(|(start, end, tok)| {
            token_to_group(tok).map(|g| HighlightSpan {
                start_col: *start,
                end_col: *end,
                group: g,
            })
        })
        .collect()
}

fn face(fg_hex: &str, bold: bool, italic: bool) -> Face {
    Face {
        fg: ThemeColor::Rgb(Rgb::from_hex(fg_hex).unwrap()),
        bg: ThemeColor::Default,
        bold,
        italic,
        underline: false,
        strikethrough: false,
    }
}

/// Default dark-theme styles for at least 7 highlight groups.
pub fn default_highlight_styles() -> HashMap<HighlightGroup, Face> {
    let mut m = HashMap::new();
    m.insert(HighlightGroup::Comment, face("#6a9955", false, true));
    m.insert(HighlightGroup::String, face("#ce9178", false, false));
    m.insert(HighlightGroup::Number, face("#b5cea8", false, false));
    m.insert(HighlightGroup::Keyword, face("#569cd6", true, false));
    m.insert(HighlightGroup::Function, face("#dcdcaa", false, false));
    m.insert(HighlightGroup::Type, face("#4ec9b0", false, false));
    m.insert(HighlightGroup::Operator, face("#d4d4d4", false, false));
    m.insert(HighlightGroup::Error, face("#f44747", true, false));
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_to_group_valid() {
        assert_eq!(token_to_group("keyword"), Some(HighlightGroup::Keyword));
        assert_eq!(token_to_group("comment"), Some(HighlightGroup::Comment));
    }

    #[test]
    fn token_to_group_invalid() {
        assert_eq!(token_to_group("nonexistent"), None);
    }

    #[test]
    fn highlight_line_basic() {
        let tokens = vec![(0, 5, "keyword"), (6, 10, "string")];
        let spans = highlight_line(&tokens);
        assert_eq!(spans.len(), 2);
        assert_eq!(spans[0].group, HighlightGroup::Keyword);
    }

    #[test]
    fn highlight_line_skips_unknown() {
        let tokens = vec![(0, 5, "unknown_token")];
        assert!(highlight_line(&tokens).is_empty());
    }

    #[test]
    fn default_styles_count() {
        let styles = default_highlight_styles();
        assert!(styles.len() >= 7);
    }
}
