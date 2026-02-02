//! Operator types for text manipulation.

use serde::{Deserialize, Serialize};
use super::ids::RegisterId;

/// An operator that acts on text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Operator {
    /// Delete text (d).
    Delete,
    /// Change text (c).
    Change,
    /// Yank/copy text (y).
    Yank,
    /// Indent right (>).
    IndentRight,
    /// Indent left (<).
    IndentLeft,
    /// Format text (gq).
    Format,
    /// Uppercase (gU).
    Uppercase,
    /// Lowercase (gu).
    Lowercase,
    /// Toggle case (g~).
    ToggleCase,
    /// Filter through external command (!).
    Filter { command: String },
    /// Replace characters (r).
    Replace { char: char },
    /// Substitute (s).
    Substitute,
    /// Auto-format (=).
    AutoIndent,
    /// Fold (zf).
    Fold,
    /// Comment toggle (gc).
    Comment,
}

impl Operator {
    /// Whether this operator deletes text.
    pub fn is_delete(&self) -> bool {
        matches!(self, Self::Delete | Self::Change | Self::Substitute)
    }

    /// Whether this operator changes mode after execution.
    pub fn enters_insert_mode(&self) -> bool {
        matches!(self, Self::Change | Self::Substitute)
    }

    /// Whether this operator is repeatable with dot.
    pub fn is_repeatable(&self) -> bool {
        !matches!(self, Self::Yank | Self::Fold)
    }

    /// Whether this operator requires a motion.
    pub fn requires_motion(&self) -> bool {
        !matches!(self, Self::Replace { .. } | Self::Substitute)
    }
}

/// A pending operator waiting for a motion/text-object.
#[derive(Debug, Clone)]
pub struct PendingOperator {
    /// The operator.
    pub operator: Operator,
    /// Count prefix (if any).
    pub count: Option<usize>,
    /// Register to use.
    pub register: RegisterId,
    /// Whether operator was doubled (e.g., dd, cc, yy).
    pub doubled: bool,
}

impl PendingOperator {
    /// Creates a new pending operator.
    pub fn new(operator: Operator) -> Self {
        Self {
            operator,
            count: None,
            register: RegisterId::Unnamed,
            doubled: false,
        }
    }

    /// Sets the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    /// Sets the register.
    pub fn with_register(mut self, register: RegisterId) -> Self {
        self.register = register;
        self
    }

    /// Marks as doubled (e.g., dd).
    pub fn doubled(mut self) -> Self {
        self.doubled = true;
        self
    }
}

/// Result of an operator execution.
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct OperatorResult {
    /// Text that was affected (for yank/delete).
    pub text: Option<String>,
    /// Whether the operation was linewise.
    pub linewise: bool,
    /// Number of lines affected.
    pub lines_affected: usize,
    /// Whether to enter insert mode after.
    pub enter_insert: bool,
}

