//! Operator types for editing commands.

use serde::{Deserialize, Serialize};

/// Editing operators that compose with motions and text objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    /// `d` — delete text.
    Delete,
    /// `c` — change text (delete + enter Insert).
    Change,
    /// `y` — yank (copy) text.
    Yank,
    /// `>` — indent text.
    Indent,
    /// `<` — dedent text.
    Dedent,
    /// `=` — auto-reindent text.
    Reindent,
    /// `gq` — format text.
    Format,
    /// `g~` — toggle case.
    ToggleCase,
    /// `gu` — lowercase.
    Lowercase,
    /// `gU` — uppercase.
    Uppercase,
}

impl Operator {
    /// Whether this operator deletes text from the buffer.
    pub fn is_destructive(&self) -> bool {
        matches!(
            self,
            Operator::Delete | Operator::Change
        )
    }

    /// Whether this operator enters insert mode after execution.
    pub fn enters_insert(&self) -> bool {
        matches!(self, Operator::Change)
    }

    /// Human-readable name for display.
    pub fn display_name(&self) -> &'static str {
        match self {
            Operator::Delete => "delete",
            Operator::Change => "change",
            Operator::Yank => "yank",
            Operator::Indent => "indent",
            Operator::Dedent => "dedent",
            Operator::Reindent => "reindent",
            Operator::Format => "format",
            Operator::ToggleCase => "toggle-case",
            Operator::Lowercase => "lowercase",
            Operator::Uppercase => "uppercase",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destructive_operators() {
        assert!(Operator::Delete.is_destructive());
        assert!(Operator::Change.is_destructive());
        assert!(!Operator::Yank.is_destructive());
    }

    #[test]
    fn change_enters_insert() {
        assert!(Operator::Change.enters_insert());
        assert!(!Operator::Delete.enters_insert());
    }
}
