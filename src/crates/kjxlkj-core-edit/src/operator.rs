//! Operator types.

use serde::{Deserialize, Serialize};

/// Kind of operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperatorKind {
    /// Delete text.
    Delete,
    /// Change text (delete and enter insert mode).
    Change,
    /// Yank text.
    Yank,
    /// Indent right.
    IndentRight,
    /// Indent left.
    IndentLeft,
    /// Auto-indent.
    AutoIndent,
    /// Convert to uppercase.
    Uppercase,
    /// Convert to lowercase.
    Lowercase,
    /// Toggle case.
    ToggleCase,
    /// Format text.
    Format,
    /// Comment/uncomment.
    Comment,
}

/// An operator with register and count.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Operator {
    /// Kind of operator.
    pub kind: OperatorKind,
    /// Target register.
    pub register: Option<char>,
    /// Repeat count.
    pub count: usize,
}

impl Operator {
    /// Creates a new operator.
    pub fn new(kind: OperatorKind) -> Self {
        Self {
            kind,
            register: None,
            count: 1,
        }
    }

    /// Sets the register.
    pub fn with_register(mut self, register: char) -> Self {
        self.register = Some(register);
        self
    }

    /// Sets the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }

    /// Returns true if this operator modifies text.
    pub fn is_mutating(&self) -> bool {
        !matches!(self.kind, OperatorKind::Yank)
    }

    /// Returns true if this operator enters insert mode.
    pub fn enters_insert(&self) -> bool {
        matches!(self.kind, OperatorKind::Change)
    }
}
