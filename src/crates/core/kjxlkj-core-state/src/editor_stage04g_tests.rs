//! Wave-039 integration tests: fold commands, z-prefix dispatch, stress.

use kjxlkj_core_types::{BufferId, Key, KeyModifiers, Mode};
use crate::editor::EditorState;

fn k(c: char) -> Key { Key::Char(c) }
fn no() -> KeyModifiers { KeyModifiers::default() }
fn cur_line(s: &EditorState) -> usize { s.windows.get(&s.focus.focused).unwrap().cursor.line }
fn set_text(s: &mut EditorState, text: &str) {
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, text).unwrap();
}
fn folds2(s: &mut EditorState) { s.fold_state.compute_indent_folds(&["a {", "    b", "}", "c {", "    d", "}"]); }
fn folds1(s: &mut EditorState) { s.fold_state.compute_indent_folds(&["fn a() {", "    x", "}"]); }

#[test]
fn z_prefix_fold_open_dispatches() {
    let mut s = EditorState::new(80, 24);
    set_text(&mut s, "fn main() {\n    body\n}\n");
    s.fold_state.compute_indent_folds(&["fn main() {", "    body", "}"]);
    s.fold_state.close(0);
    assert!(s.fold_state.closed_set().contains(&0));
    s.handle_key(&k('z'), &no()); s.handle_key(&k('o'), &no());
    assert!(!s.fold_state.closed_set().contains(&0));
}

#[test]
fn z_prefix_fold_close_dispatches() {
    let mut s = EditorState::new(80, 24);
    s.fold_state.compute_indent_folds(&["fn main() {", "    body", "}"]);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('c'), &no());
    assert!(s.fold_state.closed_set().contains(&0));
}

#[test]
fn z_prefix_fold_toggle_dispatches() {
    let mut s = EditorState::new(80, 24);
    s.fold_state.compute_indent_folds(&["fn main() {", "    body", "}"]);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no());
    assert!(s.fold_state.closed_set().contains(&0));
    s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no());
    assert!(!s.fold_state.closed_set().contains(&0));
}

#[test]
fn z_big_r_opens_all_folds() {
    let mut s = EditorState::new(80, 24);
    folds2(&mut s); s.fold_state.close_all();
    assert!(!s.fold_state.closed_set().is_empty());
    s.handle_key(&k('z'), &no()); s.handle_key(&k('R'), &no());
    assert!(s.fold_state.closed_set().is_empty());
}

#[test]
fn z_big_m_closes_all_folds() {
    let mut s = EditorState::new(80, 24);
    folds2(&mut s);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('M'), &no());
    assert!(!s.fold_state.closed_set().is_empty());
}

#[test]
fn zj_navigates_to_next_closed_fold() {
    let mut s = EditorState::new(80, 24);
    set_text(&mut s, "a {\n    b\n}\nc {\n    d\n}\n");
    folds2(&mut s); s.fold_state.close_all();
    s.handle_key(&k('z'), &no()); s.handle_key(&k('j'), &no());
    assert_eq!(cur_line(&s), 3);
}

#[test]
fn zk_navigates_to_prev_closed_fold() {
    let mut s = EditorState::new(80, 24);
    set_text(&mut s, "a {\n    b\n}\nc {\n    d\n}\n");
    folds2(&mut s); s.fold_state.close_all();
    s.windows.get_mut(&s.focus.focused).unwrap().cursor.line = 5;
    s.handle_key(&k('z'), &no()); s.handle_key(&k('k'), &no());
    assert_eq!(cur_line(&s), 3);
}

#[test]
fn zj_noop_when_no_next_closed_fold() {
    let mut s = EditorState::new(80, 24);
    folds1(&mut s);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('j'), &no());
    assert_eq!(cur_line(&s), 0);
}

#[test]
fn fold_is_hidden_after_close() {
    let mut s = EditorState::new(80, 24);
    s.fold_state.compute_indent_folds(&["fn main() {", "    body", "}"]);
    s.fold_state.close(0);
    assert!(s.fold_state.is_hidden(1));
    assert!(!s.fold_state.is_hidden(0));
    assert!(!s.fold_state.is_hidden(2));
}

#[test]
fn rapid_fold_toggle_100x() {
    let mut s = EditorState::new(80, 24);
    folds1(&mut s);
    for _ in 0..100 { s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no()); }
    assert!(!s.fold_state.closed_set().contains(&0)); // even toggles → open
}

#[test]
fn fold_open_on_non_fold_line_is_noop() {
    let mut s = EditorState::new(80, 24);
    folds1(&mut s);
    s.windows.get_mut(&s.focus.focused).unwrap().cursor.line = 2;
    s.handle_key(&k('z'), &no()); s.handle_key(&k('o'), &no());
    assert!(s.fold_state.closed_set().is_empty());
}

#[test]
fn fold_with_empty_buffer_is_safe() {
    let mut s = EditorState::new(80, 24);
    s.fold_state.compute_indent_folds(&[]);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no());
    assert!(s.fold_state.closed_set().is_empty());
}

#[test]
fn fold_and_macro_interaction() {
    let mut s = EditorState::new(80, 24);
    folds1(&mut s);
    s.handle_key(&k('q'), &no()); s.handle_key(&k('a'), &no()); // record into 'a'
    s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no()); // fold toggle
    s.handle_key(&k('q'), &no()); // stop
    assert!(s.fold_state.closed_set().contains(&0));
    s.handle_key(&Key::Char('@'), &no()); s.handle_key(&k('a'), &no()); // play
    assert!(!s.fold_state.closed_set().contains(&0));
}

#[test]
fn fold_and_mark_interaction() {
    let mut s = EditorState::new(80, 24);
    set_text(&mut s, "fn a() {\n    body\n}\n");
    s.fold_state.compute_indent_folds(&["fn a() {", "    body", "}"]);
    s.handle_key(&k('m'), &no()); s.handle_key(&k('a'), &no()); // set mark
    s.handle_key(&k('z'), &no()); s.handle_key(&k('c'), &no()); // close fold
    assert!(s.fold_state.closed_set().contains(&0));
    s.handle_key(&k('j'), &no()); s.handle_key(&k('j'), &no());
    s.handle_key(&k('\''), &no()); s.handle_key(&k('a'), &no()); // goto mark
    assert_eq!(cur_line(&s), 0);
}

#[test]
fn fold_reduce_and_more_cycle() {
    let mut s = EditorState::new(80, 24);
    folds1(&mut s);
    let lv = s.fold_state.fold_level;
    s.handle_key(&k('z'), &no()); s.handle_key(&k('m'), &no()); // zm
    assert_eq!(s.fold_state.fold_level, lv - 1);
    s.handle_key(&k('z'), &no()); s.handle_key(&k('r'), &no()); // zr
    assert_eq!(s.fold_state.fold_level, lv);
}

#[test]
fn fold_combined_stress_20x() {
    let mut s = EditorState::new(80, 24);
    folds2(&mut s);
    for _ in 0..20 {
        s.handle_key(&k('z'), &no()); s.handle_key(&k('M'), &no());
        s.handle_key(&k('z'), &no()); s.handle_key(&k('R'), &no());
        s.handle_key(&k('z'), &no()); s.handle_key(&k('a'), &no());
    }
    assert_eq!(s.mode, Mode::Normal);
}
