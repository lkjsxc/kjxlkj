/// File explorer tree model — directory tree, expand/collapse, file operations.
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeKind { File, Directory, Symlink }

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub id: NodeId, pub name: String, pub path: PathBuf,
    pub kind: NodeKind, pub expanded: bool, pub depth: usize,
    pub children: Vec<NodeId>, pub parent: Option<NodeId>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GitBadge { Modified, Added, Deleted, Untracked, Ignored, Conflict, Clean }

#[derive(Debug)]
pub struct ExplorerTree {
    nodes: HashMap<NodeId, TreeNode>, root: Option<NodeId>,
    next_id: u64, show_hidden: bool, show_ignored: bool, filter: Option<String>,
}

impl ExplorerTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(), root: None,
            next_id: 1, show_hidden: false, show_ignored: false, filter: None,
        }
    }

    fn alloc_id(&mut self) -> NodeId { let id = NodeId(self.next_id); self.next_id += 1; id }

    pub fn set_root(&mut self, path: impl AsRef<Path>) -> NodeId {
        let p = path.as_ref();
        let id = self.alloc_id();
        let name = p.file_name().map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| p.to_string_lossy().into_owned());
        let node = TreeNode {
            id, name, path: p.to_path_buf(), kind: NodeKind::Directory,
            expanded: true, depth: 0, children: Vec::new(), parent: None,
        };
        self.nodes.insert(id, node);
        self.root = Some(id);
        id
    }

    pub fn add_child(&mut self, parent: NodeId, name: &str, kind: NodeKind) -> Option<NodeId> {
        let parent_node = self.nodes.get(&parent)?;
        let depth = parent_node.depth + 1;
        let path = parent_node.path.join(name);
        let id = self.alloc_id();
        let node = TreeNode {
            id, name: name.to_string(), path, kind,
            expanded: false, depth, children: Vec::new(), parent: Some(parent),
        };
        self.nodes.insert(id, node);
        self.nodes.get_mut(&parent)?.children.push(id);
        Some(id)
    }

    pub fn toggle(&mut self, id: NodeId) -> bool {
        if let Some(node) = self.nodes.get_mut(&id) {
            if node.kind == NodeKind::Directory {
                node.expanded = !node.expanded;
                return node.expanded;
            }
        }
        false
    }

    pub fn get(&self, id: NodeId) -> Option<&TreeNode> { self.nodes.get(&id) }
    pub fn root_id(&self) -> Option<NodeId> { self.root }
    pub fn toggle_hidden(&mut self) { self.show_hidden = !self.show_hidden; }
    pub fn set_filter(&mut self, filter: Option<String>) { self.filter = filter; }

    pub fn visible_nodes(&self) -> Vec<NodeId> {
        let mut result = Vec::new();
        if let Some(root) = self.root {
            self.collect_visible(root, &mut result);
        }
        result
    }

    fn collect_visible(&self, id: NodeId, out: &mut Vec<NodeId>) {
        let node = match self.nodes.get(&id) { Some(n) => n, None => return };
        if !self.show_hidden && node.name.starts_with('.') && node.parent.is_some() {
            return;
        }
        if let Some(ref f) = self.filter {
            if node.parent.is_some() && node.kind == NodeKind::File
                && !node.name.contains(f.as_str()) {
                return;
            }
        }
        out.push(id);
        if node.expanded {
            for &child in &node.children {
                self.collect_visible(child, out);
            }
        }
    }

    pub fn len(&self) -> usize { self.nodes.len() }
    pub fn is_empty(&self) -> bool { self.nodes.is_empty() }
}

pub fn format_node(node: &TreeNode) -> String {
    let indent = "  ".repeat(node.depth);
    let icon = match (&node.kind, node.expanded) {
        (NodeKind::Directory, true) => "▼ ",
        (NodeKind::Directory, false) => "▶ ",
        (NodeKind::File, _) => "  ",
        (NodeKind::Symlink, _) => "↗ ",
    };
    format!("{}{}{}", indent, icon, node.name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> ExplorerTree {
        let mut t = ExplorerTree::new();
        let root = t.set_root("/project");
        t.add_child(root, "src", NodeKind::Directory);
        t.add_child(root, "README.md", NodeKind::File);
        t.add_child(root, ".git", NodeKind::Directory);
        t
    }

    #[test]
    fn root_setup() {
        let t = sample_tree();
        assert_eq!(t.len(), 4);
        let root = t.get(t.root_id().unwrap()).unwrap();
        assert_eq!(root.name, "project"); assert!(root.expanded);
    }

    #[test]
    fn visible_hides_dotfiles() {
        let t = sample_tree();
        let names: Vec<_> = t.visible_nodes().iter().filter_map(|id| t.get(*id))
            .map(|n| n.name.as_str()).collect();
        assert!(!names.contains(&".git"));
    }

    #[test]
    fn toggle_hidden_shows_dotfiles() {
        let mut t = sample_tree();
        t.toggle_hidden();
        let names: Vec<_> = t.visible_nodes().iter().filter_map(|id| t.get(*id))
            .map(|n| n.name.as_str()).collect();
        assert!(names.contains(&".git"));
    }

    #[test]
    fn toggle_expand_collapse() {
        let mut t = sample_tree();
        let root = t.root_id().unwrap();
        let children = t.get(root).unwrap().children.clone();
        let src_id = children[0];
        assert!(!t.get(src_id).unwrap().expanded);
        t.toggle(src_id);
        assert!(t.get(src_id).unwrap().expanded);
        t.toggle(src_id);
        assert!(!t.get(src_id).unwrap().expanded);
    }

    #[test]
    fn filter_by_name() {
        let mut t = sample_tree();
        t.set_filter(Some("README".into()));
        let vis = t.visible_nodes();
        let names: Vec<_> = vis.iter().filter_map(|id| t.get(*id))
            .filter(|n| n.kind == NodeKind::File).map(|n| n.name.as_str()).collect();
        assert_eq!(names, vec!["README.md"]);
    }

    #[test]
    fn format_node_display() {
        let t = sample_tree();
        let root = t.get(t.root_id().unwrap()).unwrap();
        assert!(format_node(root).contains("▼ project"));
    }

    #[test]
    fn add_nested_child() {
        let mut t = ExplorerTree::new();
        let root = t.set_root("/p");
        let dir = t.add_child(root, "sub", NodeKind::Directory).unwrap();
        t.add_child(dir, "f.txt", NodeKind::File).unwrap();
        assert_eq!(t.len(), 3); assert_eq!(t.get(dir).unwrap().depth, 1);
    }
}
