//! Transaction type for grouping edits.

use kjxlkj_core_edit::EditOp;
use kjxlkj_core_types::{BufferVersion, Cursor};

/// A transaction groups multiple edits into a single undo step.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Edit operations in this transaction.
    pub edits: Vec<EditOp>,
    /// Cursor position before the transaction.
    pub cursor_before: Cursor,
    /// Cursor position after the transaction.
    pub cursor_after: Cursor,
    /// Buffer version before.
    pub version_before: BufferVersion,
    /// Buffer version after.
    pub version_after: BufferVersion,
}

impl Transaction {
    /// Create a new empty transaction.
    pub fn new(cursor: Cursor, version: BufferVersion) -> Self {
        Self {
            edits: Vec::new(),
            cursor_before: cursor,
            cursor_after: cursor,
            version_before: version,
            version_after: version,
        }
    }

    /// Add an edit to the transaction.
    pub fn push(&mut self, edit: EditOp) {
        self.edits.push(edit);
    }

    /// Set the cursor position after the transaction.
    pub fn set_cursor_after(&mut self, cursor: Cursor) {
        self.cursor_after = cursor;
    }

    /// Set the version after the transaction.
    pub fn set_version_after(&mut self, version: BufferVersion) {
        self.version_after = version;
    }

    /// Check if the transaction has any edits.
    pub fn has_edits(&self) -> bool {
        !self.edits.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::CharOffset;

    #[test]
    fn empty_transaction() {
        let tx = Transaction::new(Cursor::origin(), BufferVersion::default());
        assert!(!tx.has_edits());
    }

    #[test]
    fn transaction_with_edits() {
        let mut tx = Transaction::new(Cursor::origin(), BufferVersion::default());
        tx.push(EditOp::insert(CharOffset::new(0), "test"));
        assert!(tx.has_edits());
        assert_eq!(tx.edits.len(), 1);
    }
}
