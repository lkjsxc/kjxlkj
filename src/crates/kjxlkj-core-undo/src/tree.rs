//! Undo tree for branching history.

use kjxlkj_core_edit::EditOperation;
use kjxlkj_core_types::position::Position;
use std::collections::HashMap;

/// A node in the undo tree.
#[derive(Debug, Clone)]
pub struct UndoNode {
    /// Unique node ID.
    pub id: usize,
    /// Parent node ID (None for root).
    pub parent: Option<usize>,
    /// Child node IDs.
    pub children: Vec<usize>,
    /// The operation that created this state.
    pub operation: Option<EditOperation>,
    /// Cursor position at this state.
    pub cursor: Position,
    /// Timestamp when this node was created.
    pub timestamp: u64,
}

/// Undo tree supporting branching history.
#[derive(Debug)]
pub struct UndoTree {
    /// All nodes in the tree.
    nodes: HashMap<usize, UndoNode>,
    /// Current node ID.
    current: usize,
    /// Next node ID.
    next_id: usize,
    /// Maximum nodes to keep.
    max_nodes: usize,
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}

impl UndoTree {
    /// Creates a new undo tree.
    pub fn new() -> Self {
        let root = UndoNode {
            id: 0,
            parent: None,
            children: Vec::new(),
            operation: None,
            cursor: Position::origin(),
            timestamp: 0,
        };
        let mut nodes = HashMap::new();
        nodes.insert(0, root);

        Self {
            nodes,
            current: 0,
            next_id: 1,
            max_nodes: 1000,
        }
    }

    /// Adds a new change to the tree.
    pub fn push(&mut self, operation: EditOperation, cursor: Position, timestamp: u64) {
        let id = self.next_id;
        self.next_id += 1;

        let node = UndoNode {
            id,
            parent: Some(self.current),
            children: Vec::new(),
            operation: Some(operation),
            cursor,
            timestamp,
        };

        // Add child to current node
        if let Some(current) = self.nodes.get_mut(&self.current) {
            current.children.push(id);
        }

        self.nodes.insert(id, node);
        self.current = id;

        // Prune if necessary
        self.prune_if_needed();
    }

    /// Undoes the current change.
    pub fn undo(&mut self) -> Option<&UndoNode> {
        let current = self.nodes.get(&self.current)?;
        let parent_id = current.parent?;
        self.current = parent_id;
        self.nodes.get(&self.current)
    }

    /// Redoes a change (follows the most recent branch).
    pub fn redo(&mut self) -> Option<&UndoNode> {
        let current = self.nodes.get(&self.current)?;
        let child_id = current.children.last()?;
        self.current = *child_id;
        self.nodes.get(&self.current)
    }

    /// Returns the current node.
    pub fn current(&self) -> Option<&UndoNode> {
        self.nodes.get(&self.current)
    }

    /// Returns true if undo is available.
    pub fn can_undo(&self) -> bool {
        self.nodes
            .get(&self.current)
            .and_then(|n| n.parent)
            .is_some()
    }

    /// Returns true if redo is available.
    pub fn can_redo(&self) -> bool {
        self.nodes
            .get(&self.current)
            .map(|n| !n.children.is_empty())
            .unwrap_or(false)
    }

    fn prune_if_needed(&mut self) {
        // Simple pruning: if we exceed max nodes, we could prune old branches
        // For now, just limit growth by not allowing more than max_nodes
        if self.nodes.len() > self.max_nodes {
            // TODO: Implement proper pruning strategy
        }
    }
}
