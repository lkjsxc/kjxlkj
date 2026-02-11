//! Integration tests for Stage 04 wave-034 features.
//!
//! Covers: explorer state lifecycle, key dispatch through handle_key(),
//! explorer + wincmd interaction, terminal state model.

use crate::editor::EditorState;
use kjxlkj_core_types::{ContentKind, Key, KeyModifiers};
use kjxlkj_service_explorer::{ExplorerNode, NodeId};
use std::path::PathBuf;

fn key(c: char) -> (Key, KeyModifiers) { (Key::Char(c), KeyModifiers::default()) }
fn press(s: &mut EditorState, k: &Key, m: &KeyModifiers) { s.handle_key(k, m); }
fn wincmd(s: &mut EditorState, c: char) {
    let m = KeyModifiers { ctrl: true, ..Default::default() };
    press(s, &Key::Char('w'), &m);
    let (ck, cm) = key(c);
    press(s, &ck, &cm);
}

fn setup_explorer(s: &mut EditorState) {
    s.open_explorer();
    let eid = s.focused_explorer_id().unwrap();
    let estate = s.explorer_states.get_mut(&eid).unwrap();
    let ids: Vec<NodeId> = (0..5).map(|_| estate.alloc_node_id()).collect();
    let mut src = ExplorerNode::dir(ids[1], "src".into(), 1, PathBuf::from("/proj/src"));
    src.children.push(ExplorerNode::file(ids[3], "main.rs".into(), 2, PathBuf::from("/proj/src/main.rs")));
    src.children.push(ExplorerNode::file(ids[4], "lib.rs".into(), 2, PathBuf::from("/proj/src/lib.rs")));
    let root = ExplorerNode {
        id: ids[0], name: "proj".into(), is_dir: true, depth: 0,
        path: PathBuf::from("/proj"),
        children: vec![
            src,
            ExplorerNode::file(ids[2], "README.md".into(), 1, PathBuf::from("/proj/README.md")),
        ],
    };
    estate.set_root(root);
    estate.expansion_set.insert(ids[0]);
    estate.rebuild_visible_rows();
}

#[test] fn explorer_states_populated_on_open() {
    let mut s = EditorState::new(80, 24);
    assert!(s.explorer_states.is_empty());
    s.open_explorer();
    assert_eq!(s.explorer_states.len(), 1);
}

#[test] fn explorer_states_cleaned_on_close() {
    let mut s = EditorState::new(80, 24);
    s.open_explorer();
    s.close_explorer();
    assert!(s.explorer_states.is_empty());
}

#[test] fn explorer_reopen_reuses_window() {
    let mut s = EditorState::new(80, 24);
    s.open_explorer();
    let first = s.focused_explorer_id().unwrap();
    wincmd(&mut s, 'w');
    s.open_explorer();
    assert_eq!(s.focused_explorer_id().unwrap(), first);
    assert_eq!(s.explorer_states.len(), 1);
}

#[test] fn handle_key_j_moves_selection() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let eid = s.focused_explorer_id().unwrap();
    assert_eq!(s.explorer_states[&eid].selected_index, 0);
    let (k, m) = key('j');
    press(&mut s, &k, &m);
    assert_eq!(s.explorer_states[&eid].selected_index, 1);
    press(&mut s, &k, &m);
    assert_eq!(s.explorer_states[&eid].selected_index, 2);
}

#[test] fn handle_key_k_moves_up() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let eid = s.focused_explorer_id().unwrap();
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm); press(&mut s, &jk, &jm);
    let (kk, km) = key('k');
    press(&mut s, &kk, &km);
    assert_eq!(s.explorer_states[&eid].selected_index, 1);
}

#[test] fn handle_key_l_expands_dir() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let eid = s.focused_explorer_id().unwrap();
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm); // select "src"
    let (lk, lm) = key('l');
    press(&mut s, &lk, &lm); // expand
    assert_eq!(s.explorer_states[&eid].row_count(), 5); // proj,src,main,lib,README
}

#[test] fn handle_key_h_collapses_dir() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let eid = s.focused_explorer_id().unwrap();
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm);
    let (lk, lm) = key('l');
    press(&mut s, &lk, &lm);
    assert_eq!(s.explorer_states[&eid].row_count(), 5);
    let (hk, hm) = key('h');
    press(&mut s, &hk, &hm);
    assert_eq!(s.explorer_states[&eid].row_count(), 3);
}

#[test] fn handle_key_q_closes_explorer() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    assert_eq!(s.layout.window_ids().len(), 2);
    let (qk, qm) = key('q');
    press(&mut s, &qk, &qm);
    assert_eq!(s.layout.window_ids().len(), 1);
    assert!(s.explorer_states.is_empty());
}

#[test] fn handle_key_enter_expands_like_l() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let eid = s.focused_explorer_id().unwrap();
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm);
    press(&mut s, &Key::Enter, &KeyModifiers::default());
    assert_eq!(s.explorer_states[&eid].row_count(), 5);
}

#[test] fn wincmd_works_from_explorer() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let exp_wid = s.focus.focused;
    wincmd(&mut s, 'w');
    assert_ne!(s.focus.focused, exp_wid);
    wincmd(&mut s, 'w');
    assert_eq!(s.focus.focused, exp_wid);
}

#[test] fn buffer_keys_unaffected_without_explorer() {
    let mut s = EditorState::new(80, 24);
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm);
    let ws = s.windows.get(&s.focus.focused).unwrap();
    assert!(matches!(ws.content, ContentKind::Buffer(_)));
}

#[test] fn terminal_state_lifecycle() {
    use kjxlkj_core_types::TerminalId;
    use kjxlkj_service_terminal::TerminalState;
    let mut ts = TerminalState::new(TerminalId(1), "/bin/bash".into(), 80, 24);
    assert_eq!(ts.shell, "/bin/bash");
    assert!(!ts.exited); assert_eq!(ts.exit_code, None);
    ts.resize(120, 40);
    assert_eq!(ts.cols, 120); assert_eq!(ts.rows, 40);
    ts.set_exited(0);
    assert!(ts.exited); assert_eq!(ts.exit_code, Some(0));
}

#[test] fn terminal_state_default_size() {
    use kjxlkj_core_types::TerminalId;
    use kjxlkj_service_terminal::TerminalState;
    let ts = TerminalState::new(TerminalId(42), "/bin/zsh".into(), 80, 24);
    assert_eq!(ts.cols, 80); assert_eq!(ts.rows, 24);
}
