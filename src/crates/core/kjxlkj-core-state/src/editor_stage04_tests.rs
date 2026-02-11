//! Integration tests for Stage 04 wave-033 features.
//!
//! Covers: boundary focus (t/b), resize dispatch (+/-/>/<),
//! equalize (=), maximize (_/|), explorer open/close.

use crate::editor::EditorState;
use kjxlkj_core_types::{ContentKind, Key, KeyModifiers, Mode};

fn ctrl(c: char) -> (Key, KeyModifiers) {
    (Key::Char(c), KeyModifiers { ctrl: true, ..Default::default() })
}
fn key(c: char) -> (Key, KeyModifiers) {
    (Key::Char(c), KeyModifiers::default())
}
fn press(s: &mut EditorState, k: &Key, m: &KeyModifiers) { s.handle_key(k, m); }
fn wincmd(s: &mut EditorState, c: char) {
    let (wk, wm) = ctrl('w');
    press(s, &wk, &wm);
    let (ck, cm) = key(c);
    press(s, &ck, &cm);
}

#[test]
fn ctrl_w_t_focuses_top_left() {
    let mut s = EditorState::new(80, 24);
    let first = s.focus.focused;
    wincmd(&mut s, 'v');
    let second = s.focus.focused;
    assert_ne!(first, second);
    wincmd(&mut s, 't');
    assert_eq!(s.focus.focused, first);
}

#[test]
fn ctrl_w_b_focuses_bottom_right() {
    let mut s = EditorState::new(80, 24);
    let first = s.focus.focused;
    wincmd(&mut s, 'v');
    let second = s.focus.focused;
    wincmd(&mut s, 't');
    assert_eq!(s.focus.focused, first);
    wincmd(&mut s, 'b');
    assert_eq!(s.focus.focused, second);
}

#[test]
fn ctrl_w_t_b_single_window() {
    let mut s = EditorState::new(80, 24);
    let only = s.focus.focused;
    wincmd(&mut s, 't');
    assert_eq!(s.focus.focused, only);
    wincmd(&mut s, 'b');
    assert_eq!(s.focus.focused, only);
}

#[test]
fn ctrl_w_equalize_does_not_crash() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, 's');
    wincmd(&mut s, '=');
    assert_eq!(s.layout.window_ids().len(), 3);
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn ctrl_w_resize_noop_does_not_crash() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, '+');
    wincmd(&mut s, '-');
    wincmd(&mut s, '>');
    wincmd(&mut s, '<');
    assert_eq!(s.layout.window_ids().len(), 2);
}

#[test]
fn ctrl_w_max_height_noop() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, '_');
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn ctrl_w_max_width_noop() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, '|');
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn open_explorer_creates_explorer_window() {
    let mut s = EditorState::new(80, 24);
    assert_eq!(s.layout.window_ids().len(), 1);
    s.open_explorer();
    let ids = s.layout.window_ids();
    assert_eq!(ids.len(), 2);
    let has_explorer = s.windows.values()
        .any(|ws| matches!(ws.content, ContentKind::Explorer(_)));
    assert!(has_explorer);
}

#[test]
fn open_explorer_focuses_existing() {
    let mut s = EditorState::new(80, 24);
    s.open_explorer();
    let explorer_wid = s.focus.focused;
    // Switch away
    wincmd(&mut s, 'w');
    assert_ne!(s.focus.focused, explorer_wid);
    // Open again — should focus the existing one
    s.open_explorer();
    assert_eq!(s.focus.focused, explorer_wid);
    assert_eq!(s.layout.window_ids().len(), 2);
}

#[test]
fn close_explorer_removes_explorer_window() {
    let mut s = EditorState::new(80, 24);
    s.open_explorer();
    assert_eq!(s.layout.window_ids().len(), 2);
    s.close_explorer();
    assert_eq!(s.layout.window_ids().len(), 1);
    let has_explorer = s.windows.values()
        .any(|ws| matches!(ws.content, ContentKind::Explorer(_)));
    assert!(!has_explorer);
}

#[test]
fn close_explorer_noop_when_none() {
    let mut s = EditorState::new(80, 24);
    s.close_explorer();
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn close_explorer_does_not_close_last_window() {
    let mut s = EditorState::new(80, 24);
    // Close the buffer window, open explorer as only window — not possible
    // since open_explorer splits. Close the buffer half, leaving explorer only.
    s.open_explorer();
    let explorer_wid = s.focus.focused;
    // Focus the other (buffer) window and close it
    wincmd(&mut s, 'w');
    wincmd(&mut s, 'c');
    assert_eq!(s.layout.window_ids().len(), 1);
    assert_eq!(s.focus.focused, explorer_wid);
    // Now close_explorer should not close the last window
    s.close_explorer();
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn explorer_close_ex_command() {
    let mut s = EditorState::new(80, 24);
    s.open_explorer();
    assert_eq!(s.layout.window_ids().len(), 2);
    // Enter command mode and type :ExplorerClose<Enter>
    let (ck, cm) = key(':');
    press(&mut s, &ck, &cm);
    for ch in "ExplorerClose".chars() {
        let (k, m) = key(ch);
        press(&mut s, &k, &m);
    }
    let enter = (Key::Enter, KeyModifiers::default());
    press(&mut s, &enter.0, &enter.1);
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn boundary_focus_with_three_windows() {
    let mut s = EditorState::new(120, 40);
    let w1 = s.focus.focused;
    wincmd(&mut s, 'v'); // w1 | w2
    let w2 = s.focus.focused;
    wincmd(&mut s, 's'); // w1 | (w2 / w3)
    let w3 = s.focus.focused;
    // w3 is bottom-right
    wincmd(&mut s, 'b');
    // bottom-right should be w2 or w3 depending on geometry;
    // with nested split, w3 is bottom-right
    let br = s.focus.focused;
    assert!(br == w2 || br == w3);
    // top-left should be w1
    wincmd(&mut s, 't');
    assert_eq!(s.focus.focused, w1);
}
