//! Explorer state model.

use crate::node::{ClipboardState, NodeId, TreeNode};
use std::collections::HashSet;
use std::path::PathBuf;

/// Explorer state.
#[derive(Debug)]
pub struct ExplorerState {
    /// Root path.
    pub root: PathBuf,
    /// All nodes by ID.
    nodes: std::collections::HashMap<NodeId, TreeNode>,
    /// Expanded directory IDs.
    expanded: HashSet<NodeId>,
    /// Visible node IDs (flattened tree).
    visible: Vec<NodeId>,
    /// Selected node index in visible list.
    selected: usize,
    /// Next node ID.
    next_id: u64,
    /// Root node ID.
    root_node: Option<NodeId>,
    /// Clipboard state.
    pub clipboard: Option<ClipboardState>,
}

impl Default for ExplorerState {
    fn default() -> Self {
        Self {
            root: PathBuf::new(),
            nodes: std::collections::HashMap::new(),
            expanded: HashSet::new(),
            visible: Vec::new(),
            selected: 0,
            next_id: 0,
            root_node: None,
            clipboard: None,
        }
    }
}

impl ExplorerState {
    /// Create a new explorer state with root path.
    pub fn new(root: PathBuf) -> Self {
        let mut state = Self {
            root: root.clone(),
            ..Default::default()
        };

        let root_id = state.allocate_id();
        let root_node = TreeNode::new(root_id, root, true, None, 0);
        state.nodes.insert(root_id, root_node);
        state.root_node = Some(root_id);
        state.expanded.insert(root_id);

        state
    }

    fn allocate_id(&mut self) -> NodeId {
        let id = NodeId::new(self.next_id);
        self.next_id += 1;
        id
    }

    /// Get node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&TreeNode> {
        self.nodes.get(&id)
    }

    /// Get selected node.
    pub fn selected_node(&self) -> Option<&TreeNode> {
        self.visible.get(self.selected).and_then(|id| self.nodes.get(id))
    }

    /// Get selected node ID.
    pub fn selected_id(&self) -> Option<NodeId> {
        self.visible.get(self.selected).copied()
    }

    /// Get selected index.
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get visible node count.
    pub fn visible_count(&self) -> usize {
        self.visible.len()
    }

    /// Get visible nodes.
    pub fn visible_nodes(&self) -> impl Iterator<Item = &TreeNode> {
        self.visible.iter().filter_map(|id| self.nodes.get(id))
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        if self.selected + 1 < self.visible.len() {
            self.selected += 1;
        }
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Check if a directory is expanded.
    pub fn is_expanded(&self, id: NodeId) -> bool {
        self.expanded.contains(&id)
    }

    /// Toggle directory expansion.
    pub fn toggle_expand(&mut self, id: NodeId) {
        if self.expanded.contains(&id) {
            self.expanded.remove(&id);
        } else {
            self.expanded.insert(id);
        }
    }

    /// Expand a directory.
    pub fn expand(&mut self, id: NodeId) {
        self.expanded.insert(id);
    }

    /// Collapse a directory.
    pub fn collapse(&mut self, id: NodeId) {
        self.expanded.remove(&id);
    }

    /// Add a child node to a parent.
    pub fn add_child(&mut self, parent_id: NodeId, path: PathBuf, is_dir: bool) -> NodeId {
        let depth = self.nodes.get(&parent_id).map(|n| n.depth + 1).unwrap_or(0);
        let child_id = self.allocate_id();
        let child = TreeNode::new(child_id, path, is_dir, Some(parent_id), depth);
        self.nodes.insert(child_id, child);

        if let Some(parent) = self.nodes.get_mut(&parent_id) {
            parent.children.push(child_id);
        }

        child_id
    }

    /// Rebuild visible list from expanded tree.
    pub fn rebuild_visible(&mut self) {
        self.visible.clear();
        if let Some(root_id) = self.root_node {
            self.collect_visible(root_id);
        }
        if self.visible.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.visible.len() {
            self.selected = self.visible.len() - 1;
        }
    }

    fn collect_visible(&mut self, id: NodeId) {
        self.visible.push(id);

        if self.expanded.contains(&id) {
            if let Some(node) = self.nodes.get(&id) {
                let children = node.children.clone();
                for child_id in children {
                    self.collect_visible(child_id);
                }
            }
        }
    }

    /// Clear all nodes except root.
    pub fn clear_children(&mut self) {
        if let Some(root_id) = self.root_node {
            let to_remove: Vec<NodeId> = self.nodes.keys()
                .filter(|&&id| id != root_id)
                .copied()
                .collect();
            for id in to_remove {
                self.nodes.remove(&id);
            }
            if let Some(root) = self.nodes.get_mut(&root_id) {
                root.children.clear();
            }
        }
    }
}
