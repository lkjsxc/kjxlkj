//! Edit operations.

use kjxlkj_core_types::{BufferVersion, CharOffset};

/// Result of an edit operation.
#[derive(Debug, Clone)]
pub struct EditResult {
    /// New buffer version after edit.
    pub version: BufferVersion,
    /// Text that was deleted (if any).
    pub deleted_text: Option<String>,
    /// New cursor position (char offset).
    pub cursor_offset: CharOffset,
}

/// Edit operation types.
#[derive(Debug, Clone)]
pub enum EditOp {
    /// Insert text at position.
    Insert {
        offset: CharOffset,
        text: String,
    },
    /// Delete text range.
    Delete {
        start: CharOffset,
        end: CharOffset,
    },
    /// Replace text range with new text.
    Replace {
        start: CharOffset,
        end: CharOffset,
        text: String,
    },
    /// Batch of operations (for transactions).
    Batch(Vec<EditOp>),
}

impl EditOp {
    /// Create an insert operation.
    pub fn insert(offset: CharOffset, text: impl Into<String>) -> Self {
        EditOp::Insert {
            offset,
            text: text.into(),
        }
    }

    /// Create a delete operation.
    pub fn delete(start: CharOffset, end: CharOffset) -> Self {
        EditOp::Delete { start, end }
    }

    /// Create a replace operation.
    pub fn replace(start: CharOffset, end: CharOffset, text: impl Into<String>) -> Self {
        EditOp::Replace {
            start,
            end,
            text: text.into(),
        }
    }

    /// Create a batch operation.
    pub fn batch(ops: Vec<EditOp>) -> Self {
        EditOp::Batch(ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_insert_op() {
        let op = EditOp::insert(CharOffset::new(0), "hello");
        match op {
            EditOp::Insert { offset, text } => {
                assert_eq!(offset.as_usize(), 0);
                assert_eq!(text, "hello");
            }
            _ => panic!("wrong op type"),
        }
    }

    #[test]
    fn create_delete_op() {
        let op = EditOp::delete(CharOffset::new(0), CharOffset::new(5));
        match op {
            EditOp::Delete { start, end } => {
                assert_eq!(start.as_usize(), 0);
                assert_eq!(end.as_usize(), 5);
            }
            _ => panic!("wrong op type"),
        }
    }
}
