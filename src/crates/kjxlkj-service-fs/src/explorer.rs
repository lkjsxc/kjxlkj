//! Tree rendering: flatten explorer tree into display rows with icons.

use crate::{EntryKind, ExplorerState, TreeNode};

/// A rendered row in the explorer view.
#[derive(Debug, Clone)]
pub struct ExplorerRow {
    pub text: String,
    pub depth: usize,
    pub kind: EntryKind,
    pub expanded: bool,
    pub is_selected: bool,
    pub node_index: usize,
}

/// Actions the explorer can perform.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplorerAction {
    ToggleExpand(usize),
    OpenFile(std::path::PathBuf),
    SelectUp,
    SelectDown,
    Close,
    Refresh,
    CreateFile(String),
}

/// Render the tree into flat rows for display.
pub fn render_tree(state: &ExplorerState) -> Vec<ExplorerRow> {
    let mut rows = Vec::new();
    for node in &state.tree { flatten_node(node, &mut rows, 0, state.selected); }
    if let Some(row) = rows.get_mut(state.selected) { row.is_selected = true; }
    rows
}

fn flatten_node(node: &TreeNode, rows: &mut Vec<ExplorerRow>, depth: usize, selected: usize) {
    let prefix = if matches!(node.entry.kind, EntryKind::Directory) {
        if node.expanded { "▾ " } else { "▸ " }
    } else { "  " };
    let indent = "  ".repeat(depth);
    let icon = icon_for_kind(&node.entry.kind, &node.entry.name);
    let text = format!("{}{}{}{}", indent, prefix, icon, node.entry.name);
    let idx = rows.len();
    rows.push(ExplorerRow {
        text, depth, kind: node.entry.kind,
        expanded: node.expanded, is_selected: idx == selected,
        node_index: idx,
    });
    if node.expanded {
        for child in &node.children { flatten_node(child, rows, depth + 1, selected); }
    }
}

fn icon_for_kind(kind: &EntryKind, name: &str) -> &'static str {
    match kind {
        EntryKind::Directory => "📁 ", EntryKind::Symlink => "🔗 ",
        EntryKind::File => match name.rsplit('.').next() {
            Some("rs") => "🦀 ", Some("py") => "🐍 ",
            Some("js" | "ts") => "📜 ", Some("md") => "📝 ",
            _ => "📄 ",
        },
    }
}

/// Handle a keypress in the explorer, returning an action.
pub fn handle_explorer_key(state: &mut ExplorerState, key: char) -> Option<ExplorerAction> {
    let row_count = count_visible_rows(&state.tree);
    match key {
        'j' | 'J' => { state.select_next(row_count); Some(ExplorerAction::SelectDown) }
        'k' | 'K' => { state.select_prev(); Some(ExplorerAction::SelectUp) }
        'q' => { state.visible = false; Some(ExplorerAction::Close) }
        'R' | 'r' => Some(ExplorerAction::Refresh),
        '\n' | 'l' => {
            let rows = render_tree(state);
            if let Some(row) = rows.get(state.selected) {
                if row.kind == EntryKind::Directory {
                    Some(ExplorerAction::ToggleExpand(state.selected))
                } else {
                    find_path_at(&state.tree, state.selected)
                        .map(ExplorerAction::OpenFile)
                }
            } else { None }
        }
        'h' => {
            // Collapse current dir or go to parent
            Some(ExplorerAction::ToggleExpand(state.selected))
        }
        _ => None,
    }
}

/// Count visible rows in the tree.
fn count_visible_rows(nodes: &[TreeNode]) -> usize {
    let mut count = 0;
    for node in nodes {
        count += 1;
        if node.expanded { count += count_visible_rows(&node.children); }
    }
    count
}

/// Find the path of the node at a given flat index.
fn find_path_at(nodes: &[TreeNode], target: usize) -> Option<std::path::PathBuf> {
    let mut idx = 0;
    find_path_recursive(nodes, target, &mut idx)
}

fn find_path_recursive(nodes: &[TreeNode], target: usize, idx: &mut usize) -> Option<std::path::PathBuf> {
    for node in nodes {
        if *idx == target { return Some(node.entry.path.clone()); }
        *idx += 1;
        if node.expanded {
            if let Some(p) = find_path_recursive(&node.children, target, idx) { return Some(p); }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DirEntry;

    fn make_node(name: &str, kind: EntryKind, expanded: bool) -> TreeNode {
        TreeNode { entry: DirEntry { path: name.into(), name: name.into(), kind, size: 0, hidden: false },
            children: Vec::new(), expanded, depth: 0 }
    }

    #[test]
    fn render_empty_tree() {
        let state = ExplorerState::new();
        let rows = render_tree(&state);
        assert!(rows.is_empty());
    }

    #[test]
    fn render_flat_files() {
        let mut state = ExplorerState::new();
        state.tree = vec![make_node("a.rs", EntryKind::File, false), make_node("b.py", EntryKind::File, false)];
        let rows = render_tree(&state);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].text.contains("a.rs"));
        assert!(rows[1].text.contains("b.py"));
        assert!(rows[0].is_selected);
    }

    #[test]
    fn render_expanded_dir() {
        let mut state = ExplorerState::new();
        let mut dir = make_node("src", EntryKind::Directory, true);
        dir.children.push(make_node("main.rs", EntryKind::File, false));
        dir.children.push(make_node("lib.rs", EntryKind::File, false));
        state.tree = vec![dir];
        let rows = render_tree(&state);
        assert_eq!(rows.len(), 3); // dir + 2 children
        assert!(rows[0].text.contains("▾"));
        assert!(rows[1].text.contains("main.rs"));
    }

    #[test]
    fn render_collapsed_dir_hides_children() {
        let mut state = ExplorerState::new();
        let mut dir = make_node("src", EntryKind::Directory, false);
        dir.children.push(make_node("main.rs", EntryKind::File, false));
        state.tree = vec![dir];
        let rows = render_tree(&state);
        assert_eq!(rows.len(), 1);
        assert!(rows[0].text.contains("▸"));
    }

    #[test]
    fn handle_key_navigation() {
        let mut state = ExplorerState::new();
        state.tree = vec![make_node("a", EntryKind::File, false), make_node("b", EntryKind::File, false)];
        let action = handle_explorer_key(&mut state, 'j');
        assert_eq!(action, Some(ExplorerAction::SelectDown));
        assert_eq!(state.selected, 1);
        let action = handle_explorer_key(&mut state, 'k');
        assert_eq!(action, Some(ExplorerAction::SelectUp));
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn handle_key_quit() {
        let mut state = ExplorerState::new();
        state.visible = true;
        let action = handle_explorer_key(&mut state, 'q');
        assert_eq!(action, Some(ExplorerAction::Close));
        assert!(!state.visible);
    }

    #[test]
    fn find_path_at_index() {
        let nodes = vec![make_node("a.rs", EntryKind::File, false), make_node("b.rs", EntryKind::File, false)];
        let p = find_path_at(&nodes, 1);
        assert_eq!(p, Some(std::path::PathBuf::from("b.rs")));
    }
}
