//! Operator types and execution.

use kjxlkj_core_types::CursorPosition;

/// Operator type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Delete operator.
    Delete,
    /// Change operator (delete and enter insert).
    Change,
    /// Yank operator.
    Yank,
    /// Indent right.
    IndentRight,
    /// Indent left.
    IndentLeft,
    /// Auto-format.
    Format,
    /// Toggle case.
    ToggleCase,
    /// Lowercase.
    Lowercase,
    /// Uppercase.
    Uppercase,
}

impl Operator {
    /// Check if this operator enters insert mode after execution.
    pub fn enters_insert(&self) -> bool {
        matches!(self, Operator::Change)
    }

    /// Check if this operator modifies text.
    pub fn modifies_text(&self) -> bool {
        !matches!(self, Operator::Yank)
    }
}

/// Region to operate on.
#[derive(Debug, Clone, Copy)]
pub struct OperatorRegion {
    /// Start position.
    pub start: CursorPosition,
    /// End position.
    pub end: CursorPosition,
    /// Whether the region is line-wise.
    pub linewise: bool,
}

impl OperatorRegion {
    /// Create a character-wise region.
    pub fn charwise(start: CursorPosition, end: CursorPosition) -> Self {
        Self {
            start,
            end,
            linewise: false,
        }
    }

    /// Create a line-wise region.
    pub fn linewise(start: CursorPosition, end: CursorPosition) -> Self {
        Self {
            start,
            end,
            linewise: true,
        }
    }

    /// Normalize the region so start <= end.
    pub fn normalize(&self) -> Self {
        let (start, end) = if self.start.line < self.end.line
            || (self.start.line == self.end.line && self.start.grapheme <= self.end.grapheme)
        {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        };
        Self {
            start,
            end,
            linewise: self.linewise,
        }
    }
}
