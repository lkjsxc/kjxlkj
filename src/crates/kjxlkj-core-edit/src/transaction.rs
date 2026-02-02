//! Transaction types for undo/redo.

use crate::Edit;
use kjxlkj_core_types::BufferVersion;
use serde::{Deserialize, Serialize};

/// A transaction grouping multiple edits for undo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Edits in this transaction.
    pub edits: Vec<Edit>,
    /// Version before the transaction.
    pub version_before: BufferVersion,
    /// Version after the transaction.
    pub version_after: BufferVersion,
    /// Cursor position before.
    pub cursor_before: Option<kjxlkj_core_types::Position>,
    /// Cursor position after.
    pub cursor_after: Option<kjxlkj_core_types::Position>,
}

impl Transaction {
    /// Creates a new empty transaction.
    pub fn new(version: BufferVersion) -> Self {
        Self {
            edits: Vec::new(),
            version_before: version,
            version_after: version,
            cursor_before: None,
            cursor_after: None,
        }
    }

    /// Adds an edit to the transaction.
    pub fn push(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    /// Returns true if the transaction is empty.
    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    /// Sets the cursor positions.
    pub fn with_cursors(
        mut self,
        before: kjxlkj_core_types::Position,
        after: kjxlkj_core_types::Position,
    ) -> Self {
        self.cursor_before = Some(before);
        self.cursor_after = Some(after);
        self
    }

    /// Sets the version after.
    pub fn with_version_after(mut self, version: BufferVersion) -> Self {
        self.version_after = version;
        self
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::new(BufferVersion::initial())
    }
}
