//! Undo tree implementation.

use crate::undo_node::UndoNode;
use kjxlkj_core_edit::Transaction;
use serde::{Deserialize, Serialize};

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

    /// Returns all branch IDs at current node.
    pub fn branches(&self) -> &[usize] {
        &self.nodes[self.current].children
    }

    /// Switches to a specific branch by child index.
    pub fn switch_branch(&mut self, index: usize) -> Option<&Transaction> {
        let children = &self.nodes[self.current].children;
        if index < children.len() {
            let child_id = children[index];
            self.current = child_id;
            Some(&self.nodes[child_id].transaction)
        } else {
            None
        }
    }

    /// Switches to the previous (older) branch.
    pub fn prev_branch(&mut self) -> Option<&Transaction> {
        let current_id = self.current;
        if let Some(parent_id) = self.nodes[current_id].parent {
            let siblings = &self.nodes[parent_id].children;
            if let Some(idx) = siblings.iter().position(|&id| id == current_id) {
                if idx > 0 {
                    let new_id = siblings[idx - 1];
                    self.current = new_id;
                    return Some(&self.nodes[new_id].transaction);
                }
            }
        }
        None
    }

    /// Switches to the next (newer) branch.
    pub fn next_branch(&mut self) -> Option<&Transaction> {
        let current_id = self.current;
        if let Some(parent_id) = self.nodes[current_id].parent {
            let siblings = &self.nodes[parent_id].children;
            if let Some(idx) = siblings.iter().position(|&id| id == current_id) {
                if idx + 1 < siblings.len() {
                    let new_id = siblings[idx + 1];
                    self.current = new_id;
                    return Some(&self.nodes[new_id].transaction);
                }
            }
        }
        None
    }

    /// Returns path from root to current node.
    pub fn path_to_current(&self) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = self.current;
        while let Some(parent) = self.nodes[current].parent {
            path.push(current);
            current = parent;
        }
        path.push(0); // Root
        path.reverse();
        path
    }

    /// Goes to a specific node by ID.
    pub fn goto_node(&mut self, id: usize) -> Option<&Transaction> {
        if id < self.nodes.len() {
            self.current = id;
            Some(&self.nodes[id].transaction)
        } else {
            None
        }
    }
}

impl Default for UndoTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undo_tree_new() {
        let tree = UndoTree::new();
        assert_eq!(tree.node_count(), 1);
        assert!(!tree.can_undo());
    }

    #[test]
    fn test_push_and_undo() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        assert!(tree.can_undo());
        tree.undo();
        assert!(!tree.can_undo());
    }

    #[test]
    fn test_branching() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.undo();
        tree.push(Transaction::default());
        assert_eq!(tree.branches().len(), 0);
        tree.undo();
        assert_eq!(tree.branches().len(), 2);
    }

    #[test]
    fn test_switch_branch() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.undo();
        tree.push(Transaction::default());
        tree.undo();
        tree.switch_branch(0);
        assert_eq!(tree.current, 1);
    }

    #[test]
    fn test_path_to_current() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.push(Transaction::default());
        let path = tree.path_to_current();
        assert_eq!(path, vec![0, 1, 2]);
    }

    #[test]
    fn test_goto_node() {
        let mut tree = UndoTree::new();
        tree.push(Transaction::default());
        tree.push(Transaction::default());
        tree.goto_node(1);
        assert_eq!(tree.current, 1);
    }
}
