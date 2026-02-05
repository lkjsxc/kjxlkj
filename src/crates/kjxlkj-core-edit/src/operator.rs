//! Operators for editing.

/// Editing operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Delete.
    Delete,
    /// Change (delete and enter insert).
    Change,
    /// Yank (copy).
    Yank,
    /// Indent right.
    IndentRight,
    /// Indent left.
    IndentLeft,
    /// Format.
    Format,
    /// Uppercase.
    Uppercase,
    /// Lowercase.
    Lowercase,
}
