//! Operator definitions.

/// Operator kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorKind {
    /// Delete operator (d).
    Delete,
    /// Change operator (c).
    Change,
    /// Yank operator (y).
    Yank,
    /// Indent right (>).
    IndentRight,
    /// Indent left (<).
    IndentLeft,
    /// Format (=).
    Format,
    /// Toggle case (~).
    ToggleCase,
    /// Uppercase (gU).
    Uppercase,
    /// Lowercase (gu).
    Lowercase,
}

/// Operator with context.
#[derive(Debug, Clone)]
pub struct Operator {
    /// Operator kind.
    pub kind: OperatorKind,
    /// Repeat count.
    pub count: usize,
    /// Register to use.
    pub register: Option<char>,
}

impl Operator {
    /// Create a new operator.
    pub fn new(kind: OperatorKind) -> Self {
        Self {
            kind,
            count: 1,
            register: None,
        }
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = count;
        self
    }

    /// Set the register.
    pub fn with_register(mut self, reg: char) -> Self {
        self.register = Some(reg);
        self
    }

    /// Check if this operator deletes text.
    pub fn deletes_text(&self) -> bool {
        matches!(self.kind, OperatorKind::Delete | OperatorKind::Change)
    }

    /// Check if this operator yanks text.
    pub fn yanks_text(&self) -> bool {
        matches!(
            self.kind,
            OperatorKind::Delete | OperatorKind::Change | OperatorKind::Yank
        )
    }

    /// Check if this operator should enter insert mode after.
    pub fn enters_insert(&self) -> bool {
        matches!(self.kind, OperatorKind::Change)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delete_operator() {
        let op = Operator::new(OperatorKind::Delete);
        assert!(op.deletes_text());
        assert!(op.yanks_text());
        assert!(!op.enters_insert());
    }

    #[test]
    fn change_operator() {
        let op = Operator::new(OperatorKind::Change);
        assert!(op.deletes_text());
        assert!(op.yanks_text());
        assert!(op.enters_insert());
    }

    #[test]
    fn yank_operator() {
        let op = Operator::new(OperatorKind::Yank);
        assert!(!op.deletes_text());
        assert!(op.yanks_text());
    }
}
