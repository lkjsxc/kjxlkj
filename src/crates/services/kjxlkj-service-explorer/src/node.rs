//! Tree node types.

use std::path::PathBuf;

/// Unique node identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

impl NodeId {
    /// Create a new node ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// File tree node.
#[derive(Debug, Clone)]
pub struct TreeNode {
    /// Unique node ID.
    pub id: NodeId,
    /// File or directory path.
    pub path: PathBuf,
    /// Display name.
    pub name: String,
    /// True if directory.
    pub is_dir: bool,
    /// Children (directories only, when expanded).
    pub children: Vec<NodeId>,
    /// Parent node ID.
    pub parent: Option<NodeId>,
    /// Depth in tree.
    pub depth: usize,
}

impl TreeNode {
    /// Create a new tree node.
    pub fn new(id: NodeId, path: PathBuf, is_dir: bool, parent: Option<NodeId>, depth: usize) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());

        Self {
            id,
            path,
            name,
            is_dir,
            children: Vec::new(),
            parent,
            depth,
        }
    }
}

/// Clipboard state for cut/copy operations.
#[derive(Debug, Clone)]
pub struct ClipboardState {
    /// Node IDs in clipboard.
    pub nodes: Vec<NodeId>,
    /// True if cut, false if copy.
    pub is_cut: bool,
}
