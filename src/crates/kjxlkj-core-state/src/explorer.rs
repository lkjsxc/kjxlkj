//! File explorer tree model.

use std::collections::HashSet;

/// The explorer tree model.
#[derive(Debug, Clone)]
pub struct ExplorerTree {
    pub nodes: Vec<TreeNode>,
    pub root: String,
    pub expanded: HashSet<usize>,
    pub show_hidden: bool,
    pub filter: Option<String>,
}

/// A single node in the file tree.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub id: usize,
    pub name: String,
    pub path: String,
    pub kind: TreeNodeKind,
    pub depth: usize,
    pub children: Vec<usize>,
}

/// Kind of tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeNodeKind {
    File,
    Directory,
    Symlink,
}

/// Git status badge for tree nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitBadge {
    Modified,
    Added,
    Deleted,
    Untracked,
    Ignored,
    Conflict,
    Clean,
}

impl ExplorerTree {
    pub fn new(root: String) -> Self {
        Self {
            nodes: Vec::new(),
            root,
            expanded: HashSet::new(),
            show_hidden: false,
            filter: None,
        }
    }

    /// Toggle expansion of a directory node.
    pub fn toggle_expand(&mut self, id: usize) {
        if self.expanded.contains(&id) {
            self.expanded.remove(&id);
        } else {
            self.expanded.insert(id);
        }
    }

    /// Check if a node is expanded.
    pub fn is_expanded(&self, id: usize) -> bool {
        self.expanded.contains(&id)
    }
}

/// Collect visible nodes based on expansion state.
pub fn visible_nodes(tree: &ExplorerTree) -> Vec<&TreeNode> {
    let mut result = Vec::new();
    if tree.nodes.is_empty() {
        return result;
    }
    collect_visible(&tree.nodes, 0, tree, &mut result);
    result
}

fn collect_visible<'a>(
    nodes: &'a [TreeNode],
    id: usize,
    tree: &ExplorerTree,
    out: &mut Vec<&'a TreeNode>,
) {
    if id >= nodes.len() {
        return;
    }
    let node = &nodes[id];
    if !tree.show_hidden && node.name.starts_with('.') && node.depth > 0 {
        return;
    }
    if let Some(ref filter) = tree.filter {
        if node.kind == TreeNodeKind::File && !node.name.contains(filter.as_str()) {
            return;
        }
    }
    out.push(node);
    if node.kind == TreeNodeKind::Directory && tree.is_expanded(id) {
        for &child_id in &node.children {
            collect_visible(nodes, child_id, tree, out);
        }
    }
}

/// Format a node for display with indentation and icon.
pub fn format_node(node: &TreeNode, expanded: bool) -> String {
    let indent = "  ".repeat(node.depth);
    let icon = match node.kind {
        TreeNodeKind::Directory => {
            if expanded { "▼ " } else { "▶ " }
        }
        TreeNodeKind::File => "  ",
        TreeNodeKind::Symlink => "⤷ ",
    };
    format!("{indent}{icon}{}", node.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_file_node() {
        let node = TreeNode {
            id: 0, name: "main.rs".into(), path: "src/main.rs".into(),
            kind: TreeNodeKind::File, depth: 1, children: vec![],
        };
        let s = format_node(&node, false);
        assert!(s.contains("main.rs"));
        assert!(s.starts_with("  "));
    }

    #[test]
    fn format_dir_expanded() {
        let node = TreeNode {
            id: 0, name: "src".into(), path: "src".into(),
            kind: TreeNodeKind::Directory, depth: 0, children: vec![],
        };
        assert!(format_node(&node, true).contains('▼'));
        assert!(format_node(&node, false).contains('▶'));
    }

    #[test]
    fn toggle_expand() {
        let mut tree = ExplorerTree::new("/root".into());
        tree.toggle_expand(0);
        assert!(tree.is_expanded(0));
        tree.toggle_expand(0);
        assert!(!tree.is_expanded(0));
    }
}
