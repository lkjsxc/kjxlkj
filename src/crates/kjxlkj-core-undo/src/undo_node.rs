//! Undo node type.

use kjxlkj_core_edit::Transaction;
use serde::{Deserialize, Serialize};

/// Node in the undo tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoNode {
    /// Node ID.
    pub id: usize,
    /// Parent node ID.
    pub parent: Option<usize>,
    /// Child node IDs.
    pub children: Vec<usize>,
    /// Transaction at this node.
    pub transaction: Transaction,
    /// Timestamp.
    pub timestamp: u64,
}

impl UndoNode {
    /// Creates a new node.
    pub fn new(id: usize, transaction: Transaction) -> Self {
        Self {
            id,
            parent: None,
            children: Vec::new(),
            transaction,
            timestamp: 0,
        }
    }

    /// Sets the parent.
    pub fn with_parent(mut self, parent: usize) -> Self {
        self.parent = Some(parent);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_node_new() {
        let node = UndoNode::new(1, Transaction::default());
        assert_eq!(node.id, 1);
        assert!(node.parent.is_none());
        assert!(node.children.is_empty());
    }

    #[test]
    fn test_undo_node_with_parent() {
        let node = UndoNode::new(2, Transaction::default()).with_parent(1);
        assert_eq!(node.parent, Some(1));
    }
}
