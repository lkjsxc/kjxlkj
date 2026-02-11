//! Integration tests for Stage 04 wave-035 features.
//!
//! Covers: wincmd W/H/J/K/L/r/R/x dispatch, terminal window creation,
//! focus cycle reverse, explorer v/s split-open keys.

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

// --- Wincmd W (reverse cycle) ---

#[test] fn wincmd_big_w_cycles_backwards() {
    let mut s = EditorState::new(80, 24);
    s.split_vertical(); // now 2 windows
    s.split_vertical(); // now 3 windows
    let ids = s.layout.window_ids();
    assert_eq!(ids.len(), 3);
    // Focus is on last split (ids[2]).
    assert_eq!(s.focus.focused, ids[2]);
    wincmd(&mut s, 'W');
    assert_eq!(s.focus.focused, ids[1]);
    wincmd(&mut s, 'W');
    assert_eq!(s.focus.focused, ids[0]);
    wincmd(&mut s, 'W'); // wraps
    assert_eq!(s.focus.focused, ids[2]);
}

#[test] fn wincmd_big_w_single_window_noop() {
    let mut s = EditorState::new(80, 24);
    let focused = s.focus.focused;
    wincmd(&mut s, 'W');
    assert_eq!(s.focus.focused, focused);
}

// --- Wincmd H/J/K/L (move-to-edge placeholders; verify no crash) ---

#[test] fn wincmd_big_h_j_k_l_no_crash() {
    let mut s = EditorState::new(80, 24);
    s.split_vertical();
    for c in ['H', 'J', 'K', 'L'] { wincmd(&mut s, c); }
    assert_eq!(s.layout.window_ids().len(), 2);
}

// --- Wincmd r/R (rotate placeholders; verify no crash) ---

#[test] fn wincmd_r_big_r_no_crash() {
    let mut s = EditorState::new(80, 24);
    s.split_vertical();
    wincmd(&mut s, 'r');
    wincmd(&mut s, 'R');
    assert_eq!(s.layout.window_ids().len(), 2);
}

// --- Wincmd x (exchange placeholder; verify no crash) ---

#[test] fn wincmd_x_no_crash() {
    let mut s = EditorState::new(80, 24);
    s.split_vertical();
    let f = s.focus.focused;
    wincmd(&mut s, 'x');
    // Placeholder — focus unchanged.
    assert_eq!(s.focus.focused, f);
}

// --- Terminal window creation ---

#[test] fn terminal_open_creates_window() {
    let mut s = EditorState::new(80, 24);
    assert_eq!(s.layout.window_ids().len(), 1);
    s.open_terminal();
    assert_eq!(s.layout.window_ids().len(), 2);
    let tw = s.windows.get(&s.focus.focused).unwrap();
    assert!(matches!(tw.content, ContentKind::Terminal(_)));
}

#[test] fn terminal_open_focuses_new_window() {
    let mut s = EditorState::new(80, 24);
    let original = s.focus.focused;
    s.open_terminal();
    assert_ne!(s.focus.focused, original);
}

#[test] fn terminal_window_wincmd_navigable() {
    let mut s = EditorState::new(80, 24);
    s.open_terminal();
    let term_wid = s.focus.focused;
    wincmd(&mut s, 'w');
    assert_ne!(s.focus.focused, term_wid); // back to buffer
    wincmd(&mut s, 'w');
    assert_eq!(s.focus.focused, term_wid); // back to terminal
}

// --- Focus cycle reverse ---

#[test] fn focus_cycle_reverse_wraps() {
    let mut s = EditorState::new(80, 24);
    s.split_vertical();
    let ids = s.layout.window_ids();
    s.focus.set_focus(ids[0]); // focus first
    s.focus_cycle_reverse(); // should wrap to last
    assert_eq!(s.focus.focused, ids[1]);
}

#[test] fn focus_cycle_reverse_single_noop() {
    let mut s = EditorState::new(80, 24);
    let f = s.focus.focused;
    s.focus_cycle_reverse();
    assert_eq!(s.focus.focused, f);
}

// --- Explorer v/s split-open keys ---

#[test] fn explorer_v_opens_vertical_split() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    // Select README.md (index 2, a file).
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm); // select "src"
    press(&mut s, &jk, &jm); // select "README.md"
    let before = s.layout.window_ids().len();
    let (vk, vm) = key('v');
    press(&mut s, &vk, &vm); // open in vsplit
    // Should have created a new window.
    assert!(s.layout.window_ids().len() > before);
    // Focus should be on the new buffer window, not explorer.
    let fw = s.windows.get(&s.focus.focused).unwrap();
    assert!(matches!(fw.content, ContentKind::Buffer(_)));
}

#[test] fn explorer_s_opens_horizontal_split() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let (jk, jm) = key('j');
    press(&mut s, &jk, &jm);
    press(&mut s, &jk, &jm); // select "README.md"
    let before = s.layout.window_ids().len();
    let (sk, sm) = key('s');
    press(&mut s, &sk, &sm);
    assert!(s.layout.window_ids().len() > before);
    let fw = s.windows.get(&s.focus.focused).unwrap();
    assert!(matches!(fw.content, ContentKind::Buffer(_)));
}

#[test] fn explorer_v_on_dir_is_noop() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    // Selected index 0 = "proj" (root dir).
    let before = s.layout.window_ids().len();
    let (vk, vm) = key('v');
    press(&mut s, &vk, &vm);
    // No new split created for directory.
    assert_eq!(s.layout.window_ids().len(), before);
}

#[test] fn explorer_s_on_dir_is_noop() {
    let mut s = EditorState::new(80, 24);
    setup_explorer(&mut s);
    let before = s.layout.window_ids().len();
    let (sk, sm) = key('s');
    press(&mut s, &sk, &sm);
    assert_eq!(s.layout.window_ids().len(), before);
}
