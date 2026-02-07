//! Branching undo tree data model.

use crate::undo_tree::TextChange;
use kjxlkj_core_types::types::Position;
use serde::{Deserialize, Serialize};

/// Opaque node identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub usize);

/// A change entry storing both forward and reverse operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeEntry {
    pub forward: Vec<TextChange>,
    pub reverse: Vec<TextChange>,
    pub cursor_before: Position,
    pub cursor_after: Position,
}

/// A node in the branching undo tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchNode {
    pub id: NodeId,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
    pub entry: ChangeEntry,
}

/// Branching undo tree supporting multiple redo paths.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingUndoTree {
    nodes: Vec<BranchNode>,
    current: NodeId,
}

impl BranchingUndoTree {
    /// Create a new tree with a sentinel root node.
    pub fn new() -> Self {
        let root = BranchNode {
            id: NodeId(0),
            parent: None,
            children: Vec::new(),
            entry: ChangeEntry {
                forward: Vec::new(),
                reverse: Vec::new(),
                cursor_before: Position::ZERO,
                cursor_after: Position::ZERO,
            },
        };
        Self {
            nodes: vec![root],
            current: NodeId(0),
        }
    }

    /// Append a new change as a child of the current node, return its id.
    pub fn push(&mut self, entry: ChangeEntry) -> NodeId {
        let id = NodeId(self.nodes.len());
        let node = BranchNode {
            id,
            parent: Some(self.current),
            children: Vec::new(),
            entry,
        };
        self.nodes.push(node);
        self.nodes[self.current.0].children.push(id);
        self.current = id;
        id
    }

    /// Move to the parent node, returning the current entry (to reverse).
    pub fn undo(&mut self) -> Option<&ChangeEntry> {
        let cur = &self.nodes[self.current.0];
        let parent = cur.parent?;
        let entry_idx = self.current.0;
        self.current = parent;
        Some(&self.nodes[entry_idx].entry)
    }

    /// Redo along the given branch index, returning the entry to apply.
    pub fn redo(&mut self, branch: usize) -> Option<&ChangeEntry> {
        let children = &self.nodes[self.current.0].children;
        let &child_id = children.get(branch)?;
        self.current = child_id;
        Some(&self.nodes[child_id.0].entry)
    }

    /// Total number of nodes (including the sentinel root).
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Number of redo branches available at the current node.
    pub fn branches_at_current(&self) -> usize {
        self.nodes[self.current.0].children.len()
    }
}

impl Default for BranchingUndoTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::undo_tree::{ChangeKind, TextChange};

    fn change_entry(label: &str) -> ChangeEntry {
        ChangeEntry {
            forward: vec![TextChange {
                kind: ChangeKind::Insert,
                position: Position::new(0, 0),
                text: label.to_string(),
            }],
            reverse: vec![TextChange {
                kind: ChangeKind::Delete,
                position: Position::new(0, 0),
                text: label.to_string(),
            }],
            cursor_before: Position::ZERO,
            cursor_after: Position::new(0, label.len()),
        }
    }

    #[test]
    fn push_and_undo() {
        let mut tree = BranchingUndoTree::new();
        tree.push(change_entry("a"));
        tree.push(change_entry("b"));
        assert_eq!(tree.node_count(), 3); // root + 2

        let e = tree.undo().unwrap();
        assert_eq!(e.forward[0].text, "b");
    }

    #[test]
    fn branching_redo() {
        let mut tree = BranchingUndoTree::new();
        tree.push(change_entry("a"));
        tree.undo();
        // Create a second branch
        tree.push(change_entry("b"));
        tree.undo();
        assert_eq!(tree.branches_at_current(), 2);

        let e = tree.redo(1).unwrap();
        assert_eq!(e.forward[0].text, "b");
    }

    #[test]
    fn undo_at_root_returns_none() {
        let mut tree = BranchingUndoTree::new();
        assert!(tree.undo().is_none());
    }

    #[test]
    fn redo_bad_branch_returns_none() {
        let mut tree = BranchingUndoTree::new();
        tree.push(change_entry("a"));
        assert!(tree.redo(5).is_none());
    }
}
