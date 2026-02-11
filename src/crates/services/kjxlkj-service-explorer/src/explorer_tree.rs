//! Explorer tree node model.
//!
//! See /docs/spec/features/navigation/file_explorer.md — State Model.

use std::path::PathBuf;

/// Unique deterministic node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

/// A node in the explorer file tree.
#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub id: NodeId,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub path: PathBuf,
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    /// Create a new file node (no children).
    pub fn file(id: NodeId, name: String, depth: usize, path: PathBuf) -> Self {
        Self { id, name, is_dir: false, depth, path, children: vec![] }
    }

    /// Create a new directory node.
    pub fn dir(id: NodeId, name: String, depth: usize, path: PathBuf) -> Self {
        Self { id, name, is_dir: true, depth, path, children: vec![] }
    }

    /// Find a node by ID in this subtree.
    pub fn find(&self, target: NodeId) -> Option<&ExplorerNode> {
        if self.id == target { return Some(self); }
        for child in &self.children {
            if let Some(found) = child.find(target) { return Some(found); }
        }
        None
    }

    /// Find parent node ID of target in this subtree.
    pub fn parent_of(&self, target: NodeId) -> Option<NodeId> {
        for child in &self.children {
            if child.id == target { return Some(self.id); }
            if let Some(p) = child.parent_of(target) { return Some(p); }
        }
        None
    }

    /// Sort children: directories first, then alphabetically.
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| {
            b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name))
        });
        for child in &mut self.children {
            child.sort_children();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ExplorerNode {
        let mut root = ExplorerNode::dir(NodeId(0), "root".into(), 0, PathBuf::from("/"));
        let src = ExplorerNode::dir(NodeId(1), "src".into(), 1, PathBuf::from("/src"));
        let readme = ExplorerNode::file(NodeId(2), "README.md".into(), 1, PathBuf::from("/README.md"));
        root.children = vec![readme, src];
        root
    }

    #[test]
    fn find_node_by_id() {
        let root = sample();
        assert!(root.find(NodeId(1)).is_some());
        assert!(root.find(NodeId(99)).is_none());
    }

    #[test]
    fn parent_of_child() {
        let root = sample();
        assert_eq!(root.parent_of(NodeId(1)), Some(NodeId(0)));
        assert_eq!(root.parent_of(NodeId(0)), None);
    }

    #[test]
    fn sort_dirs_first() {
        let mut root = sample();
        root.sort_children();
        assert!(root.children[0].is_dir); // src first
        assert!(!root.children[1].is_dir); // README.md second
    }
}
