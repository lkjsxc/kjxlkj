//! Tests for ExplorerState: flatten, expand, collapse, clamp.

use super::*;
use std::path::PathBuf;

fn sample_tree() -> (ExplorerState, ExplorerNode) {
    let mut st = ExplorerState::new(PathBuf::from("/project"));
    let root_id = st.alloc_node_id();
    let child1_id = st.alloc_node_id();
    let child2_id = st.alloc_node_id();
    let grandchild_id = st.alloc_node_id();
    let root = ExplorerNode {
        id: root_id, name: "project".into(), is_dir: true,
        depth: 0, path: PathBuf::from("/project"),
        children: vec![
            ExplorerNode {
                id: child1_id, name: "src".into(), is_dir: true,
                depth: 1, path: PathBuf::from("/project/src"),
                children: vec![ExplorerNode {
                    id: grandchild_id, name: "main.rs".into(),
                    is_dir: false, depth: 2,
                    path: PathBuf::from("/project/src/main.rs"),
                    children: vec![],
                }],
            },
            ExplorerNode {
                id: child2_id, name: "README.md".into(),
                is_dir: false, depth: 1,
                path: PathBuf::from("/project/README.md"),
                children: vec![],
            },
        ],
    };
    (st, root)
}

#[test] fn collapsed_shows_root_only() {
    let (mut st, root) = sample_tree();
    st.set_root(root);
    assert_eq!(st.row_count(), 1);
    assert_eq!(st.visible_rows()[0].name, "project");
}

#[test] fn expand_root_shows_children() {
    let (mut st, root) = sample_tree();
    let root_id = root.id;
    st.set_root(root);
    st.expansion_set.insert(root_id);
    st.rebuild_visible_rows();
    assert_eq!(st.row_count(), 3);
}

#[test] fn expand_nested_shows_grandchild() {
    let (mut st, root) = sample_tree();
    let root_id = root.id;
    let src_id = root.children[0].id;
    st.set_root(root);
    st.expansion_set.insert(root_id);
    st.expansion_set.insert(src_id);
    st.rebuild_visible_rows();
    assert_eq!(st.row_count(), 4);
}

#[test] fn clamp_selection_on_collapse() {
    let (mut st, root) = sample_tree();
    let root_id = root.id;
    st.set_root(root);
    st.expansion_set.insert(root_id);
    st.rebuild_visible_rows();
    st.selected_index = 2;
    st.expansion_set.clear();
    st.rebuild_visible_rows();
    assert_eq!(st.selected_index, 0);
}

#[test] fn selected_row_returns_correct() {
    let (mut st, root) = sample_tree();
    let root_id = root.id;
    st.set_root(root);
    st.expansion_set.insert(root_id);
    st.rebuild_visible_rows();
    let r = st.selected_row().unwrap();
    assert_eq!(r.name, "project");
    st.selected_index = 1;
    let r = st.selected_row().unwrap();
    assert_eq!(r.name, "src");
}
