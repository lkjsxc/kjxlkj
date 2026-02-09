//! Syntax highlighting types and highlight groups.

use serde::{Deserialize, Serialize};

use crate::Color;

/// A span of highlighted text in a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighlightSpan {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// Highlight group.
    pub group: HighlightGroup,
}

/// Semantic highlight groups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HighlightGroup {
    Normal,
    Keyword,
    Function,
    Type,
    String,
    Comment,
    Identifier,
    Constant,
    Operator,
    Delimiter,
    Field,
    Number,
    Boolean,
    PreProc,
    Include,
    Macro,
    Error,
    Warning,
    Info,
    Hint,
    Search,
    Visual,
    CursorLine,
    LineNr,
    StatusLine,
    StatusLineNC,
    VertSplit,
    Pmenu,
    PmenuSel,
    TabLine,
    TabLineSel,
    DiffAdd,
    DiffChange,
    DiffDelete,
    Special,
    Title,
    NonText,
    MatchParen,
}

/// Style for a highlight group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

impl Default for HighlightStyle {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
        }
    }
}

/// A fold region in the buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldRegion {
    /// Start line (0-indexed).
    pub start_line: usize,
    /// End line (0-indexed, inclusive).
    pub end_line: usize,
    /// Fold level (1+).
    pub level: u32,
}

/// Diagnostic severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// A diagnostic message for a buffer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column (0-indexed).
    pub col: usize,
    /// End line.
    pub end_line: usize,
    /// End column.
    pub end_col: usize,
    /// Severity.
    pub severity: DiagnosticSeverity,
    /// Message text.
    pub message: String,
    /// Source (e.g. "rustc", "pyright").
    pub source: String,
    /// Diagnostic code.
    pub code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn highlight_span() {
        let span = HighlightSpan {
            start: 0,
            end: 10,
            group: HighlightGroup::Keyword,
        };
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 10);
    }

    #[test]
    fn diagnostic() {
        let diag = Diagnostic {
            line: 5,
            col: 0,
            end_line: 5,
            end_col: 10,
            severity: DiagnosticSeverity::Error,
            message: "undeclared".into(),
            source: "rustc".into(),
            code: Some("E0425".into()),
        };
        assert_eq!(diag.severity, DiagnosticSeverity::Error);
    }
}
