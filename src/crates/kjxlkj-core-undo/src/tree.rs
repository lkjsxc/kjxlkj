//! Undo tree implementation.

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

/// Undo tree for persistent undo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoTree {
    /// All nodes.
    nodes: Vec<UndoNode>,
    /// Current node ID.
    current: usize,
    /// Next node ID.
    next_id: usize,
}

impl UndoTree {
    /// Creates a new undo tree.
    pub fn new() -> Self {
        let root = UndoNode::new(0, Transaction::default());
        Self {
            nodes: vec![root],
            current: 0,
            next_id: 1,
        }
    }

    /// Adds a new change.
    pub fn push(&mut self, transaction: Transaction) {
        let id = self.next_id;
        self.next_id += 1;

        let mut node = UndoNode::new(id, transaction);
        node.parent = Some(self.current);

        self.nodes[self.current].children.push(id);
        self.nodes.push(node);
        self.current = id;
    }

    /// Undoes to parent.
    pub fn undo(&mut self) -> Option<&Transaction> {
        let current_node = &self.nodes[self.current];
        if let Some(parent_id) = current_node.parent {
            let tx = &self.nodes[self.current].transaction;
            self.current = parent_id;
            Some(tx)
        } else {
            None
        }
    }

    /// Redoes to most recent child.
    pub fn redo(&mut self) -> Option<&Transaction> {
        let current_node = &self.nodes[self.current];
        if let Some(&child_id) = current_node.children.last() {
            self.current = child_id;
            Some(&self.nodes[child_id].transaction)
        } else {
            None
        }
    }

    /// Returns the current node.
    pub fn current_node(&self) -> &UndoNode {
        &self.nodes[self.current]
    }

    /// Returns true if undo is available.
    pub fn can_undo(&self) -> bool {
        self.nodes[self.current].parent.is_some()
    }

    /// Returns true if redo is available.
    pub fn can_redo(&self) -> bool {
        !self.nodes[self.current].children.is_empty()
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}
