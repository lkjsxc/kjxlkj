//! Operator types for editing operations that act on motions or text objects.

use serde::{Deserialize, Serialize};

use crate::motion::Motion;
use crate::text_object::TextObjectType;

/// Editing operators that take a target (motion or text object).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Operator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    ToggleCase,
    UpperCase,
    LowerCase,
    Format,
}

/// What an operator acts upon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorTarget {
    /// A motion with a repeat count.
    Motion(Motion, usize),
    /// A text object; `bool` = true means inner, false means outer/around.
    TextObject(TextObjectType, bool),
    /// Doubled operator acts on the current line (e.g., `dd`, `yy`).
    Line,
    /// Operator to end of line (e.g., `D`, `C`).
    ToEndOfLine,
    /// Entire buffer (e.g., `gUae`).
    WholeBuffer,
}

impl Operator {
    /// Returns `true` if the operator deletes text from the buffer.
    pub fn removes_text(&self) -> bool {
        matches!(self, Self::Delete | Self::Change)
    }

    /// Returns the command character(s) for this operator.
    pub fn char_repr(&self) -> &'static str {
        match self {
            Self::Delete => "d",
            Self::Yank => "y",
            Self::Change => "c",
            Self::Indent => ">",
            Self::Outdent => "<",
            Self::ToggleCase => "g~",
            Self::UpperCase => "gU",
            Self::LowerCase => "gu",
            Self::Format => "gq",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_removes_text() {
        assert!(Operator::Delete.removes_text());
        assert!(!Operator::Yank.removes_text());
    }

    #[test]
    fn char_repr() {
        assert_eq!(Operator::UpperCase.char_repr(), "gU");
    }
}
