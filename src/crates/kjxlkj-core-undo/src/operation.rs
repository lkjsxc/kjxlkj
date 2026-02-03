//! Edit operations for undo/redo.

use kjxlkj_core_types::LineCol;

/// A single edit operation that can be undone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditOperation {
    /// Insert text at position.
    Insert { pos: LineCol, text: String },
    /// Delete text at position.
    Delete { pos: LineCol, text: String },
}

impl EditOperation {
    /// Returns the inverse operation for undo.
    pub fn inverse(&self) -> Self {
        match self {
            EditOperation::Insert { pos, text } => EditOperation::Delete {
                pos: *pos,
                text: text.clone(),
            },
            EditOperation::Delete { pos, text } => EditOperation::Insert {
                pos: *pos,
                text: text.clone(),
            },
        }
    }
}

/// A group of operations that form a single undo step.
#[derive(Debug, Clone, Default)]
pub struct UndoGroup {
    operations: Vec<EditOperation>,
}

impl UndoGroup {
    /// Creates a new empty undo group.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Adds an operation to the group.
    pub fn push(&mut self, op: EditOperation) {
        self.operations.push(op);
    }

    /// Returns true if the group is empty.
    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }

    /// Returns the operations in the group.
    pub fn operations(&self) -> &[EditOperation] {
        &self.operations
    }

    /// Returns the inverse group for undo.
    pub fn inverse(&self) -> Self {
        Self {
            operations: self
                .operations
                .iter()
                .rev()
                .map(|op| op.inverse())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inverse_insert() {
        let op = EditOperation::Insert {
            pos: LineCol::new(0, 0),
            text: "hello".to_string(),
        };
        let inv = op.inverse();
        assert!(matches!(inv, EditOperation::Delete { .. }));
    }

    #[test]
    fn group_inverse() {
        let mut group = UndoGroup::new();
        group.push(EditOperation::Insert {
            pos: LineCol::new(0, 0),
            text: "a".to_string(),
        });
        group.push(EditOperation::Insert {
            pos: LineCol::new(0, 1),
            text: "b".to_string(),
        });
        let inv = group.inverse();
        assert_eq!(inv.operations().len(), 2);
    }
}
