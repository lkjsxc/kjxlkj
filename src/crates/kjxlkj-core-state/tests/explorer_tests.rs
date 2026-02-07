//! Tests for ExplorerTree: expand, collapse, visible_nodes, format_node.

use kjxlkj_core_state::explorer::{
    format_node, visible_nodes, ExplorerTree, GitBadge, TreeNode, TreeNodeKind,
};

fn sample_tree() -> ExplorerTree {
    let mut tree = ExplorerTree::new("/project".into());
    tree.nodes = vec![
        TreeNode {
            id: 0,
            name: "root".into(),
            path: "/project".into(),
            kind: TreeNodeKind::Directory,
            depth: 0,
            children: vec![1, 2, 3],
        },
        TreeNode {
            id: 1,
            name: "src".into(),
            path: "/project/src".into(),
            kind: TreeNodeKind::Directory,
            depth: 1,
            children: vec![4],
        },
        TreeNode {
            id: 2,
            name: "Cargo.toml".into(),
            path: "/project/Cargo.toml".into(),
            kind: TreeNodeKind::File,
            depth: 1,
            children: vec![],
        },
        TreeNode {
            id: 3,
            name: ".gitignore".into(),
            path: "/project/.gitignore".into(),
            kind: TreeNodeKind::File,
            depth: 1,
            children: vec![],
        },
        TreeNode {
            id: 4,
            name: "main.rs".into(),
            path: "/project/src/main.rs".into(),
            kind: TreeNodeKind::File,
            depth: 2,
            children: vec![],
        },
    ];
    tree
}

#[test]
fn new_tree_empty() {
    let tree = ExplorerTree::new("/tmp".into());
    assert_eq!(tree.root, "/tmp");
    assert!(tree.nodes.is_empty());
}

#[test]
fn toggle_expand() {
    let mut tree = sample_tree();
    assert!(!tree.is_expanded(0));
    tree.toggle_expand(0);
    assert!(tree.is_expanded(0));
    tree.toggle_expand(0);
    assert!(!tree.is_expanded(0));
}

#[test]
fn visible_nodes_collapsed() {
    let tree = sample_tree();
    let vis = visible_nodes(&tree);
    // Only root is visible (children of unexpanded dir not shown)
    assert_eq!(vis.len(), 1);
    assert_eq!(vis[0].name, "root");
}

#[test]
fn visible_nodes_expanded_root() {
    let mut tree = sample_tree();
    tree.toggle_expand(0);
    let vis = visible_nodes(&tree);
    // root + src + Cargo.toml (but NOT .gitignore since show_hidden=false)
    assert!(vis.len() >= 3);
    assert!(vis.iter().any(|n| n.name == "Cargo.toml"));
}

#[test]
fn visible_nodes_hides_dotfiles() {
    let mut tree = sample_tree();
    tree.toggle_expand(0);
    let vis = visible_nodes(&tree);
    assert!(!vis.iter().any(|n| n.name == ".gitignore"));
}

#[test]
fn visible_nodes_show_hidden() {
    let mut tree = sample_tree();
    tree.show_hidden = true;
    tree.toggle_expand(0);
    let vis = visible_nodes(&tree);
    assert!(vis.iter().any(|n| n.name == ".gitignore"));
}

#[test]
fn format_node_file() {
    let node = TreeNode {
        id: 0,
        name: "main.rs".into(),
        path: "src/main.rs".into(),
        kind: TreeNodeKind::File,
        depth: 1,
        children: vec![],
    };
    let s = format_node(&node, false);
    assert!(s.contains("main.rs"));
    assert!(s.starts_with("  ")); // depth=1 indent
}

#[test]
fn format_node_dir_expanded() {
    let node = TreeNode {
        id: 0,
        name: "src".into(),
        path: "src".into(),
        kind: TreeNodeKind::Directory,
        depth: 0,
        children: vec![],
    };
    assert!(format_node(&node, true).contains('▼'));
    assert!(format_node(&node, false).contains('▶'));
}

#[test]
fn format_node_symlink() {
    let node = TreeNode {
        id: 0,
        name: "link".into(),
        path: "link".into(),
        kind: TreeNodeKind::Symlink,
        depth: 0,
        children: vec![],
    };
    assert!(format_node(&node, false).contains('⤷'));
}

#[test]
fn git_badge_variants() {
    let badges = [
        GitBadge::Modified,
        GitBadge::Added,
        GitBadge::Deleted,
        GitBadge::Untracked,
        GitBadge::Ignored,
        GitBadge::Conflict,
        GitBadge::Clean,
    ];
    assert_eq!(badges.len(), 7);
}
