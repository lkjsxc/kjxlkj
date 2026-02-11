//! Integration tests for Ctrl-w window commands.
//!
//! See /docs/spec/features/window/wincmd.md.

use crate::editor::EditorState;
use kjxlkj_core_types::{Key, KeyModifiers, WindowId};

fn ctrl(c: char) -> (Key, KeyModifiers) {
    (Key::Char(c), KeyModifiers { ctrl: true, ..Default::default() })
}

fn key(c: char) -> (Key, KeyModifiers) {
    (Key::Char(c), KeyModifiers::default())
}

fn press(s: &mut EditorState, k: &Key, m: &KeyModifiers) {
    s.handle_key(k, m);
}

/// Helper: sends Ctrl-w then a second char.
fn wincmd(s: &mut EditorState, c: char) {
    let (wk, wm) = ctrl('w');
    press(s, &wk, &wm);
    let (ck, cm) = key(c);
    press(s, &ck, &cm);
}

#[test]
fn ctrl_w_v_splits_vertically() {
    let mut s = EditorState::new(80, 24);
    assert_eq!(s.layout.window_ids().len(), 1);
    wincmd(&mut s, 'v');
    assert_eq!(s.layout.window_ids().len(), 2);
}

#[test]
fn ctrl_w_s_splits_horizontally() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 's');
    assert_eq!(s.layout.window_ids().len(), 2);
}

#[test]
fn ctrl_w_c_closes_split() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    assert_eq!(s.layout.window_ids().len(), 2);
    wincmd(&mut s, 'c');
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn ctrl_w_c_does_not_close_last_window() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'c');
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn ctrl_w_o_closes_all_other_windows() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, 'v');
    assert_eq!(s.layout.window_ids().len(), 3);
    wincmd(&mut s, 'o');
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn ctrl_w_w_cycles_focus() {
    let mut s = EditorState::new(80, 24);
    let first = s.focus.focused;
    wincmd(&mut s, 'v');
    let second = s.focus.focused;
    assert_ne!(first, second);
    wincmd(&mut s, 'w');
    assert_eq!(s.focus.focused, first);
    wincmd(&mut s, 'w');
    assert_eq!(s.focus.focused, second);
}

#[test]
fn ctrl_w_p_toggles_previous() {
    let mut s = EditorState::new(80, 24);
    let first = s.focus.focused;
    wincmd(&mut s, 'v');
    let second = s.focus.focused;
    wincmd(&mut s, 'p');
    assert_eq!(s.focus.focused, first);
    wincmd(&mut s, 'p');
    assert_eq!(s.focus.focused, second);
}

#[test]
fn ctrl_w_h_l_directional_focus() {
    let mut s = EditorState::new(80, 24);
    let left_win = s.focus.focused;
    wincmd(&mut s, 'v');
    let right_win = s.focus.focused;
    // Focus is on right, go left
    wincmd(&mut s, 'h');
    assert_eq!(s.focus.focused, left_win);
    // Go right again
    wincmd(&mut s, 'l');
    assert_eq!(s.focus.focused, right_win);
}

#[test]
fn ctrl_w_j_k_directional_focus() {
    let mut s = EditorState::new(80, 24);
    let top_win = s.focus.focused;
    wincmd(&mut s, 's');
    let bottom_win = s.focus.focused;
    // Focus is on bottom, go up
    wincmd(&mut s, 'k');
    assert_eq!(s.focus.focused, top_win);
    // Go down again
    wincmd(&mut s, 'j');
    assert_eq!(s.focus.focused, bottom_win);
}

#[test]
fn ctrl_w_n_splits_horizontal() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'n');
    assert_eq!(s.layout.window_ids().len(), 2);
}

#[test]
fn ctrl_w_q_closes_like_c() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    assert_eq!(s.layout.window_ids().len(), 2);
    wincmd(&mut s, 'q');
    assert_eq!(s.layout.window_ids().len(), 1);
}

#[test]
fn wincmd_focus_stays_in_normal_mode() {
    let mut s = EditorState::new(80, 24);
    wincmd(&mut s, 'v');
    wincmd(&mut s, 'h');
    assert_eq!(s.mode, kjxlkj_core_types::Mode::Normal);
}
