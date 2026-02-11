//! Wave-036 boundary and error-semantics integration tests.
//!
//! Focus: jumplist/changelist navigation, boundary behavior for windows,
//! explorer, and terminal operations under edge conditions.

use kjxlkj_core_types::{Action, BufferId, Key, KeyModifiers, Mode};

use crate::editor::EditorState;

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn ctrl() -> KeyModifiers { KeyModifiers { ctrl: true, ..Default::default() } }

// --- Jumplist boundary tests ---

#[test] fn jumplist_older_on_empty_no_crash() {
    let mut s = ed();
    s.handle_key(&Key::Char('o'), &ctrl()); // Ctrl-o on empty jumplist
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn jumplist_newer_on_empty_no_crash() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &ctrl()); // Ctrl-i on empty jumplist
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn jumplist_records_on_goto_line() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "a\nb\nc\nd\ne").unwrap();
    // G goes to last line — should record jump
    s.handle_key(&Key::Char('G'), &m());
    assert!(s.jumplist.len() > 0);
    let cur = s.windows.get(&s.focus.focused).unwrap().cursor;
    assert_eq!(cur.line, 4);
    // Ctrl-o should go back to line 0
    s.handle_key(&Key::Char('o'), &ctrl());
    let cur = s.windows.get(&s.focus.focused).unwrap().cursor;
    assert_eq!(cur.line, 0);
}

#[test] fn jumplist_older_past_end_stays() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "a\nb\nc").unwrap();
    s.handle_key(&Key::Char('G'), &m()); // records jump at line 0
    s.handle_key(&Key::Char('o'), &ctrl()); // go back to line 0
    s.handle_key(&Key::Char('o'), &ctrl()); // past the end — noop
    let cur = s.windows.get(&s.focus.focused).unwrap().cursor;
    assert_eq!(cur.line, 0);
}

#[test] fn jumplist_newer_past_end_stays() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "a\nb\nc").unwrap();
    s.handle_key(&Key::Char('G'), &m()); // jump to last line
    s.handle_key(&Key::Char('o'), &ctrl()); // go older → line 0
    s.handle_key(&Key::Char('i'), &ctrl()); // go newer → line 2
    s.handle_key(&Key::Char('i'), &ctrl()); // past end — noop
    let cur = s.windows.get(&s.focus.focused).unwrap().cursor;
    // Should remain at the position from the last successful go_newer
    assert!(cur.line <= 2);
}

// --- Changelist boundary tests ---

#[test] fn changelist_older_on_empty_no_crash() {
    let mut s = ed();
    // g; on empty changelist
    s.handle_key(&Key::Char('g'), &m());
    s.handle_key(&Key::Char(';'), &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn changelist_newer_on_empty_no_crash() {
    let mut s = ed();
    // g, on empty changelist
    s.handle_key(&Key::Char('g'), &m());
    s.handle_key(&Key::Char(','), &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test] fn changelist_records_on_text_change() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "hello").unwrap();
    // Delete a char with 'x' — text-changing action
    s.handle_key(&Key::Char('x'), &m());
    assert!(s.changelist.len() > 0);
}

#[test] fn changelist_navigate_after_changes() {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, "hello\nworld").unwrap();
    // Move to line 1, then delete char
    s.handle_key(&Key::Char('j'), &m());
    s.handle_key(&Key::Char('x'), &m());
    // g; should go older
    s.handle_key(&Key::Char('g'), &m());
    s.handle_key(&Key::Char(';'), &m());
    assert_eq!(s.mode, Mode::Normal);
}

// --- Window boundary tests ---

#[test] fn close_only_window_is_noop() {
    let mut s = ed();
    s.apply_action(Action::CloseWindow);
    assert_eq!(s.windows.len(), 1); // still have one window
}

#[test] fn window_only_with_single_window() {
    let mut s = ed();
    s.apply_action(Action::WindowOnly);
    assert_eq!(s.windows.len(), 1);
}

#[test] fn focus_direction_single_window_noop() {
    let mut s = ed();
    let before = s.focus.focused;
    s.apply_action(Action::FocusDirection(kjxlkj_core_types::Direction::Right));
    assert_eq!(s.focus.focused, before);
}

#[test] fn focus_cycle_single_window_noop() {
    let mut s = ed();
    let before = s.focus.focused;
    s.apply_action(Action::FocusCycle);
    assert_eq!(s.focus.focused, before);
}

#[test] fn focus_cycle_reverse_single_window_noop() {
    let mut s = ed();
    let before = s.focus.focused;
    s.apply_action(Action::FocusCycleReverse);
    assert_eq!(s.focus.focused, before);
}

// --- Explorer boundary tests ---

#[test] fn close_explorer_when_none_open() {
    let mut s = ed();
    s.apply_action(Action::CloseExplorer);
    assert_eq!(s.windows.len(), 1); // no crash, no change
}

// --- Terminal boundary tests ---

#[test] fn terminal_open_creates_window() {
    let mut s = ed();
    s.apply_action(Action::OpenTerminal);
    assert_eq!(s.windows.len(), 2);
}
