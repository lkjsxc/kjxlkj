//! Undo/redo history management.

use kjxlkj_core_types::{BufferVersion, CursorPosition};

/// An edit operation that can be undone/redone.
#[derive(Debug, Clone)]
pub struct EditOp {
    /// Type of edit.
    pub kind: EditKind,
    /// Cursor position before edit.
    pub cursor_before: CursorPosition,
    /// Cursor position after edit.
    pub cursor_after: CursorPosition,
    /// Buffer version before edit.
    pub version_before: BufferVersion,
}

/// Type of edit operation.
#[derive(Debug, Clone)]
pub enum EditKind {
    /// Insert text.
    Insert {
        /// Position where text was inserted.
        pos: CursorPosition,
        /// Text that was inserted.
        text: String,
    },
    /// Delete text.
    Delete {
        /// Start position of deleted text.
        pos: CursorPosition,
        /// Text that was deleted.
        text: String,
    },
    /// Replace text.
    Replace {
        /// Start position.
        pos: CursorPosition,
        /// Old text that was replaced.
        old_text: String,
        /// New text that replaced it.
        new_text: String,
    },
}

/// Undo history for a buffer.
#[derive(Debug, Clone)]
pub struct UndoHistory {
    /// Stack of undoable operations.
    undo_stack: Vec<EditOp>,
    /// Stack of redoable operations.
    redo_stack: Vec<EditOp>,
    /// Maximum history size.
    max_size: usize,
    /// Current transaction (for grouping edits).
    current_transaction: Option<Vec<EditOp>>,
}

impl Default for UndoHistory {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoHistory {
    /// Create new undo history.
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size: 1000,
            current_transaction: None,
        }
    }

    /// Create history with custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_size,
            current_transaction: None,
        }
    }

    /// Push an edit operation.
    pub fn push(&mut self, op: EditOp) {
        self.redo_stack.clear();
        if let Some(ref mut transaction) = self.current_transaction {
            transaction.push(op);
        } else {
            self.undo_stack.push(op);
            if self.undo_stack.len() > self.max_size {
                self.undo_stack.remove(0);
            }
        }
    }

    /// Start a transaction (group multiple edits as one undo unit).
    pub fn begin_transaction(&mut self) {
        if self.current_transaction.is_none() {
            self.current_transaction = Some(Vec::new());
        }
    }

    /// End current transaction.
    pub fn end_transaction(&mut self) {
        if let Some(transaction) = self.current_transaction.take() {
            if !transaction.is_empty() {
                let first = &transaction[0];
                let last = transaction.last().unwrap();
                let merged = EditOp {
                    kind: EditKind::Delete {
                        pos: first.cursor_before,
                        text: String::new(),
                    },
                    cursor_before: first.cursor_before,
                    cursor_after: last.cursor_after,
                    version_before: first.version_before,
                };
                self.undo_stack.push(merged);
            }
        }
    }

    /// Check if undo is available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Pop operation for undo.
    pub fn pop_undo(&mut self) -> Option<EditOp> {
        let op = self.undo_stack.pop()?;
        self.redo_stack.push(op.clone());
        Some(op)
    }

    /// Pop operation for redo.
    pub fn pop_redo(&mut self) -> Option<EditOp> {
        let op = self.redo_stack.pop()?;
        self.undo_stack.push(op.clone());
        Some(op)
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current_transaction = None;
    }

    /// Get undo stack depth.
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get redo stack depth.
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }
}

/// Invert an edit operation for undo.
pub fn invert_edit(op: &EditOp) -> EditOp {
    let inverted_kind = match &op.kind {
        EditKind::Insert { pos, text } => EditKind::Delete {
            pos: *pos,
            text: text.clone(),
        },
        EditKind::Delete { pos, text } => EditKind::Insert {
            pos: *pos,
            text: text.clone(),
        },
        EditKind::Replace {
            pos,
            old_text,
            new_text,
        } => EditKind::Replace {
            pos: *pos,
            old_text: new_text.clone(),
            new_text: old_text.clone(),
        },
    };

    EditOp {
        kind: inverted_kind,
        cursor_before: op.cursor_after,
        cursor_after: op.cursor_before,
        version_before: op.version_before,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_undo() {
        let mut history = UndoHistory::new();
        let op = EditOp {
            kind: EditKind::Insert {
                pos: CursorPosition::new(0, 0),
                text: "hello".to_string(),
            },
            cursor_before: CursorPosition::new(0, 0),
            cursor_after: CursorPosition::new(0, 5),
            version_before: BufferVersion::new(0),
        };
        history.push(op);
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_undo_redo() {
        let mut history = UndoHistory::new();
        let op = EditOp {
            kind: EditKind::Insert {
                pos: CursorPosition::new(0, 0),
                text: "hello".to_string(),
            },
            cursor_before: CursorPosition::new(0, 0),
            cursor_after: CursorPosition::new(0, 5),
            version_before: BufferVersion::new(0),
        };
        history.push(op);

        let undone = history.pop_undo();
        assert!(undone.is_some());
        assert!(!history.can_undo());
        assert!(history.can_redo());

        let redone = history.pop_redo();
        assert!(redone.is_some());
        assert!(history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn test_invert_insert() {
        let op = EditOp {
            kind: EditKind::Insert {
                pos: CursorPosition::new(0, 0),
                text: "hello".to_string(),
            },
            cursor_before: CursorPosition::new(0, 0),
            cursor_after: CursorPosition::new(0, 5),
            version_before: BufferVersion::new(0),
        };
        let inverted = invert_edit(&op);
        assert!(matches!(inverted.kind, EditKind::Delete { .. }));
    }
}
