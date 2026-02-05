//! File explorer model for tree navigation.
//!
//! Implements the explorer state model as specified in
//! `/docs/spec/features/navigation/file_explorer.md`.

use std::path::{Path, PathBuf};

/// Node state in the explorer tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeState {
    /// Node is collapsed (default for directories).
    #[default]
    Collapsed,
    /// Node is expanded (showing children).
    Expanded,
    /// Node is loading children.
    Loading,
}

/// Node type in the explorer tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeKind {
    /// Directory node.
    Directory,
    /// File node.
    #[default]
    File,
    /// Symbolic link.
    Symlink,
}

/// A node in the file explorer tree.
#[derive(Debug, Clone)]
pub struct ExplorerNode {
    /// Full path to this node.
    pub path: PathBuf,
    /// Display name.
    pub name: String,
    /// Node kind (file/directory).
    pub kind: NodeKind,
    /// Node state (for directories).
    pub state: NodeState,
    /// Depth in tree (for indentation).
    pub depth: usize,
    /// Children (if expanded).
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    /// Create a new explorer node.
    pub fn new(path: PathBuf, kind: NodeKind, depth: usize) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());
        Self {
            path,
            name,
            kind,
            state: NodeState::default(),
            depth,
            children: Vec::new(),
        }
    }

    /// Create a directory node.
    pub fn directory(path: PathBuf, depth: usize) -> Self {
        Self::new(path, NodeKind::Directory, depth)
    }

    /// Create a file node.
    pub fn file(path: PathBuf, depth: usize) -> Self {
        Self::new(path, NodeKind::File, depth)
    }

    /// Check if this is a directory.
    pub fn is_dir(&self) -> bool {
        self.kind == NodeKind::Directory
    }

    /// Check if this is expanded.
    pub fn is_expanded(&self) -> bool {
        self.state == NodeState::Expanded
    }

    /// Expand this node (mark as expanded).
    pub fn expand(&mut self) {
        if self.is_dir() {
            self.state = NodeState::Expanded;
        }
    }

    /// Collapse this node.
    pub fn collapse(&mut self) {
        if self.is_dir() {
            self.state = NodeState::Collapsed;
        }
    }

    /// Toggle expand/collapse.
    pub fn toggle(&mut self) {
        if self.is_dir() {
            self.state = match self.state {
                NodeState::Collapsed => NodeState::Expanded,
                NodeState::Expanded => NodeState::Collapsed,
                NodeState::Loading => NodeState::Loading,
            };
        }
    }
}

/// Display row for rendering the explorer.
#[derive(Debug, Clone)]
pub struct ExplorerRow {
    /// Node path.
    pub path: PathBuf,
    /// Display text (with indentation).
    pub display: String,
    /// Node kind.
    pub kind: NodeKind,
    /// Expansion indicator for directories.
    pub indicator: Option<char>,
    /// Depth level.
    pub depth: usize,
    /// Is this row selected?
    pub selected: bool,
}

/// File explorer state.
#[derive(Debug, Default)]
pub struct Explorer {
    /// Root path being explored.
    root: Option<PathBuf>,
    /// Root node of the tree.
    tree: Option<ExplorerNode>,
    /// Currently selected path.
    selected: Option<PathBuf>,
    /// Visible (flattened) nodes for rendering.
    visible: Vec<PathBuf>,
    /// Current selection index in visible list.
    cursor: usize,
    /// Whether explorer is visible.
    visible_panel: bool,
}

impl Explorer {
    /// Create a new explorer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the root path.
    pub fn set_root(&mut self, path: PathBuf) {
        let node = ExplorerNode::directory(path.clone(), 0);
        self.root = Some(path.clone());
        self.tree = Some(node);
        self.selected = Some(path);
        self.cursor = 0;
        self.rebuild_visible();
    }

    /// Get the root path.
    pub fn root(&self) -> Option<&Path> {
        self.root.as_deref()
    }

    /// Get currently selected path.
    pub fn selected(&self) -> Option<&Path> {
        self.selected.as_deref()
    }

    /// Toggle explorer visibility.
    pub fn toggle_visible(&mut self) {
        self.visible_panel = !self.visible_panel;
    }

    /// Check if explorer is visible.
    pub fn is_visible(&self) -> bool {
        self.visible_panel
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        if !self.visible.is_empty() && self.cursor < self.visible.len() - 1 {
            self.cursor += 1;
            self.selected = Some(self.visible[self.cursor].clone());
        }
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.selected = Some(self.visible[self.cursor].clone());
        }
    }

    /// Get cursor position.
    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }

    /// Toggle expand/collapse on current selection.
    pub fn toggle_current(&mut self) {
        if let Some(ref sel) = self.selected {
            if let Some(ref mut tree) = self.tree {
                Self::toggle_node(tree, sel);
                self.rebuild_visible();
            }
        }
    }

    /// Toggle a node by path (recursive helper).
    fn toggle_node(node: &mut ExplorerNode, path: &Path) -> bool {
        if node.path == path {
            node.toggle();
            return true;
        }
        for child in &mut node.children {
            if Self::toggle_node(child, path) {
                return true;
            }
        }
        false
    }

    /// Add children to a node by path.
    pub fn add_children(&mut self, parent: &Path, children: Vec<ExplorerNode>) {
        if let Some(ref mut tree) = self.tree {
            Self::add_children_to(tree, parent, children);
            self.rebuild_visible();
        }
    }

    /// Add children helper (recursive).
    fn add_children_to(node: &mut ExplorerNode, parent: &Path, children: Vec<ExplorerNode>) {
        if node.path == parent {
            node.children = children;
            node.state = NodeState::Expanded;
            return;
        }
        for child in &mut node.children {
            Self::add_children_to(child, parent, children.clone());
        }
    }

    /// Rebuild the visible list from tree.
    fn rebuild_visible(&mut self) {
        self.visible.clear();
        if let Some(ref tree) = self.tree {
            Self::flatten_to_visible(tree, &mut self.visible);
        }
        // Clamp cursor
        if !self.visible.is_empty() && self.cursor >= self.visible.len() {
            self.cursor = self.visible.len() - 1;
        }
    }

    /// Flatten tree to visible list (recursive).
    fn flatten_to_visible(node: &ExplorerNode, out: &mut Vec<PathBuf>) {
        out.push(node.path.clone());
        if node.is_expanded() {
            for child in &node.children {
                Self::flatten_to_visible(child, out);
            }
        }
    }

    /// Get visible rows for rendering.
    pub fn rows(&self) -> Vec<ExplorerRow> {
        let selected = self.selected.as_ref();
        self.collect_rows_from_tree(selected)
    }

    /// Collect rows from tree.
    fn collect_rows_from_tree(&self, selected: Option<&PathBuf>) -> Vec<ExplorerRow> {
        let mut rows = Vec::new();
        if let Some(ref tree) = self.tree {
            Self::collect_rows(tree, selected, &mut rows);
        }
        rows
    }

    /// Collect rows recursively.
    fn collect_rows(node: &ExplorerNode, selected: Option<&PathBuf>, out: &mut Vec<ExplorerRow>) {
        let indicator = if node.is_dir() {
            Some(if node.is_expanded() { '▼' } else { '▶' })
        } else {
            None
        };
        let indent = "  ".repeat(node.depth);
        let icon = if node.is_dir() { "📁" } else { "📄" };
        let display = format!("{}{} {}", indent, icon, node.name);
        out.push(ExplorerRow {
            path: node.path.clone(),
            display,
            kind: node.kind,
            indicator,
            depth: node.depth,
            selected: selected == Some(&node.path),
        });
        if node.is_expanded() {
            for child in &node.children {
                Self::collect_rows(child, selected, out);
            }
        }
    }

    /// Get visible count.
    pub fn visible_count(&self) -> usize {
        self.visible.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_state_default() {
        assert_eq!(NodeState::default(), NodeState::Collapsed);
    }

    #[test]
    fn test_node_kind_default() {
        assert_eq!(NodeKind::default(), NodeKind::File);
    }

    #[test]
    fn test_explorer_node_new() {
        let node = ExplorerNode::new(PathBuf::from("/test"), NodeKind::Directory, 0);
        assert_eq!(node.name, "test");
        assert!(node.is_dir());
    }

    #[test]
    fn test_explorer_node_file() {
        let node = ExplorerNode::file(PathBuf::from("/test/file.rs"), 1);
        assert!(!node.is_dir());
        assert_eq!(node.depth, 1);
    }

    #[test]
    fn test_explorer_node_toggle() {
        let mut node = ExplorerNode::directory(PathBuf::from("/test"), 0);
        assert!(!node.is_expanded());
        node.toggle();
        assert!(node.is_expanded());
        node.toggle();
        assert!(!node.is_expanded());
    }

    #[test]
    fn test_explorer_new() {
        let explorer = Explorer::new();
        assert!(explorer.root().is_none());
        assert!(!explorer.is_visible());
    }

    #[test]
    fn test_explorer_set_root() {
        let mut explorer = Explorer::new();
        explorer.set_root(PathBuf::from("/project"));
        assert_eq!(explorer.root(), Some(Path::new("/project")));
    }

    #[test]
    fn test_explorer_toggle_visible() {
        let mut explorer = Explorer::new();
        assert!(!explorer.is_visible());
        explorer.toggle_visible();
        assert!(explorer.is_visible());
    }

    #[test]
    fn test_explorer_move_down() {
        let mut explorer = Explorer::new();
        explorer.set_root(PathBuf::from("/project"));
        assert_eq!(explorer.cursor_pos(), 0);
        // Only root visible, can't move down
        explorer.move_down();
        assert_eq!(explorer.cursor_pos(), 0);
    }

    #[test]
    fn test_explorer_rows() {
        let mut explorer = Explorer::new();
        explorer.set_root(PathBuf::from("/project"));
        let rows = explorer.rows();
        assert_eq!(rows.len(), 1);
        assert!(rows[0].selected);
    }
}
