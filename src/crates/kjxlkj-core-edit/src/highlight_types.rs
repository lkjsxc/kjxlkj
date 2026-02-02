//! Highlight types and groups.

use serde::{Deserialize, Serialize};

/// Highlight group names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HighlightGroup {
    /// Normal text.
    Normal,
    /// Comment.
    Comment,
    /// Constant (strings, numbers).
    Constant,
    /// String literal.
    String,
    /// Character literal.
    Character,
    /// Number literal.
    Number,
    /// Boolean.
    Boolean,
    /// Identifier.
    Identifier,
    /// Function name.
    Function,
    /// Statement (if, for, etc).
    Statement,
    /// Conditional (if, else).
    Conditional,
    /// Repeat (for, while).
    Repeat,
    /// Label.
    Label,
    /// Operator.
    Operator,
    /// Keyword.
    Keyword,
    /// Exception (try, catch).
    Exception,
    /// PreProc (preprocessor).
    PreProc,
    /// Include (#include).
    Include,
    /// Define (#define).
    Define,
    /// Macro.
    Macro,
    /// Type.
    Type,
    /// StorageClass (static, const).
    StorageClass,
    /// Structure.
    Structure,
    /// Typedef.
    Typedef,
    /// Special.
    Special,
    /// SpecialChar.
    SpecialChar,
    /// Tag.
    Tag,
    /// Delimiter.
    Delimiter,
    /// SpecialComment.
    SpecialComment,
    /// Debug.
    Debug,
    /// Underlined.
    Underlined,
    /// Error.
    Error,
    /// Todo.
    Todo,
    /// Search match.
    Search,
    /// Current search match.
    IncSearch,
    /// Visual selection.
    Visual,
    /// Line number.
    LineNr,
    /// Current line number.
    CursorLineNr,
    /// Sign column.
    SignColumn,
    /// Status line.
    StatusLine,
    /// Tab line.
    TabLine,
    /// Popup menu.
    Pmenu,
    /// Popup menu selected.
    PmenuSel,
    /// Non-text (eg ~).
    NonText,
    /// EOL markers.
    EndOfBuffer,
}

/// A highlight span.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightSpan {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset.
    pub end: usize,
    /// Highlight group.
    pub group: HighlightGroup,
}

impl HighlightSpan {
    /// Creates a new highlight span.
    pub fn new(start: usize, end: usize, group: HighlightGroup) -> Self {
        Self { start, end, group }
    }

    /// Returns the length of the span.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_span() {
        let span = HighlightSpan::new(0, 10, HighlightGroup::Comment);
        assert_eq!(span.len(), 10);
        assert!(!span.is_empty());
    }

    #[test]
    fn test_highlight_span_empty() {
        let span = HighlightSpan::new(5, 5, HighlightGroup::Normal);
        assert!(span.is_empty());
    }

    #[test]
    fn test_highlight_group_eq() {
        assert_eq!(HighlightGroup::Keyword, HighlightGroup::Keyword);
        assert_ne!(HighlightGroup::Keyword, HighlightGroup::Comment);
    }
}
