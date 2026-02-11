//! File explorer service.
//!
//! Manages tree state, filesystem ops, reveal, and refresh.
//! See /docs/spec/features/navigation/file_explorer.md.

mod explorer_nav;
mod explorer_tree;

pub use explorer_nav::ExplorerAction;
pub use explorer_tree::{ExplorerNode, NodeId};

use std::collections::HashSet;
use std::path::PathBuf;

/// Explorer instance state per spec State Model.
#[derive(Debug, Clone)]
pub struct ExplorerState {
    /// Workspace root (absolute).
    pub root_path: PathBuf,
    /// Tree root node (None if not yet loaded).
    pub root: Option<ExplorerNode>,
    /// Set of expanded directory node IDs.
    pub expansion_set: HashSet<NodeId>,
    /// Currently selected row index in visible_rows.
    pub selected_index: usize,
    /// Cached flattened visible rows.
    cached_rows: Vec<VisibleRow>,
    /// Next node ID counter.
    next_node_id: u64,
}

/// A single visible row in the flattened explorer view.
#[derive(Debug, Clone)]
pub struct VisibleRow {
    pub node_id: NodeId,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
    pub path: PathBuf,
}

impl ExplorerState {
    /// Create a new explorer state rooted at `root_path`.
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            root: None,
            expansion_set: HashSet::new(),
            selected_index: 0,
            cached_rows: Vec::new(),
            next_node_id: 0,
        }
    }

    /// Allocate a new unique node ID.
    pub fn alloc_node_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    /// Build a synthetic tree for testing / initial load.
    pub fn set_root(&mut self, root: ExplorerNode) {
        self.root = Some(root);
        self.rebuild_visible_rows();
    }

    /// Get the flattened visible rows.
    pub fn visible_rows(&self) -> &[VisibleRow] {
        &self.cached_rows
    }

    /// Number of visible rows.
    pub fn row_count(&self) -> usize {
        self.cached_rows.len()
    }

    /// Get the selected row (if any).
    pub fn selected_row(&self) -> Option<&VisibleRow> {
        self.cached_rows.get(self.selected_index)
    }

    /// Rebuild cached visible rows from tree + expansion set.
    pub fn rebuild_visible_rows(&mut self) {
        self.cached_rows.clear();
        if let Some(ref root) = self.root {
            Self::flatten(root, &self.expansion_set, &mut self.cached_rows);
        }
        self.clamp_selection();
    }

    fn flatten(
        node: &ExplorerNode,
        expansion: &HashSet<NodeId>,
        out: &mut Vec<VisibleRow>,
    ) {
        let expanded = node.is_dir && expansion.contains(&node.id);
        out.push(VisibleRow {
            node_id: node.id,
            name: node.name.clone(),
            is_dir: node.is_dir,
            depth: node.depth,
            expanded,
            path: node.path.clone(),
        });
        if expanded {
            for child in &node.children {
                Self::flatten(child, expansion, out);
            }
        }
    }

    /// Clamp selected_index to valid range.
    fn clamp_selection(&mut self) {
        if self.cached_rows.is_empty() {
            self.selected_index = 0;
        } else if self.selected_index >= self.cached_rows.len() {
            self.selected_index = self.cached_rows.len() - 1;
        }
    }
}

#[cfg(test)]
mod explorer_state_tests;

