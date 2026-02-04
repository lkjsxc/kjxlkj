//! Operators for editing commands.

/// The kind of operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    /// Delete text.
    Delete,
    /// Yank (copy) text.
    Yank,
    /// Change text (delete and enter insert mode).
    Change,
    /// Indent text.
    Indent,
    /// Outdent text.
    Outdent,
    /// Toggle case.
    ToggleCase,
    /// Uppercase.
    Uppercase,
    /// Lowercase.
    Lowercase,
    /// Format/reflow text.
    Format,
}

/// A complete operator with count and kind.
#[derive(Debug, Clone, Copy)]
pub struct Operator {
    pub kind: OperatorKind,
    pub count: usize,
}

impl Operator {
    /// Create a new operator with count 1.
    pub fn new(kind: OperatorKind) -> Self {
        Self { kind, count: 1 }
    }

    /// Create an operator with a specific count.
    pub fn with_count(kind: OperatorKind, count: usize) -> Self {
        Self { kind, count }
    }

    /// Whether this operator enters insert mode after completion.
    pub fn enters_insert(&self) -> bool {
        matches!(self.kind, OperatorKind::Change)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_enters_insert() {
        assert!(Operator::new(OperatorKind::Change).enters_insert());
        assert!(!Operator::new(OperatorKind::Delete).enters_insert());
    }
}
