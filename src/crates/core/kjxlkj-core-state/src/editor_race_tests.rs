//! Race and boundary condition tests.
//! Exercises rapid mode switching, command boundary cases,
//! window operations under stress, and deterministic replay.

use crate::editor::EditorState;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    Action, BufferId, CommandKind, ContentKind, Key, KeyModifiers, Mode,
};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn ctrl() -> KeyModifiers { KeyModifiers { ctrl: true, ..Default::default() } }
fn cur_buf(s: &EditorState) -> BufferId {
    let wid = s.focus.focused;
    match s.windows.get(&wid).unwrap().content {
        ContentKind::Buffer(id) => id, _ => panic!("no buffer"),
    }
}

#[test]
fn rapid_mode_toggle_100_cycles() {
    let mut s = ed();
    for _ in 0..100 {
        s.handle_key(&Key::Char('i'), &m()); // Normal → Insert
        assert_eq!(s.mode, Mode::Insert);
        s.handle_key(&Key::Escape, &m()); // Insert → Normal
        assert_eq!(s.mode, Mode::Normal);
    }
}

#[test]
fn rapid_visual_toggle_100_cycles() {
    let mut s = ed();
    for _ in 0..100 {
        s.handle_key(&Key::Char('v'), &m()); // Normal → Visual
        assert!(matches!(s.mode, Mode::Visual(_)));
        s.handle_key(&Key::Escape, &m()); // Visual → Normal
        assert_eq!(s.mode, Mode::Normal);
    }
}

#[test]
fn command_mode_enter_exit_100_cycles() {
    let mut s = ed();
    for _ in 0..100 {
        s.handle_key(&Key::Char(':'), &m()); // Normal → Command
        assert_eq!(s.mode, Mode::Command(CommandKind::Ex));
        s.handle_key(&Key::Escape, &m()); // Command → Normal
        assert_eq!(s.mode, Mode::Normal);
    }
}

#[test]
fn insert_escape_preserves_text() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &m());
    let text = "hello world ";
    for c in text.chars() {
        s.handle_key(&Key::Char(c), &m());
    }
    s.handle_key(&Key::Escape, &m());
    let buf = s.buffers.get(&BufferId(0)).unwrap();
    let line = buf.line(0).unwrap();
    assert!(line.starts_with("hello world"));
}

#[test]
fn split_close_cycle_10_times() {
    let mut s = ed();
    for _ in 0..10 {
        s.apply_action(Action::SplitVertical);
        let count_after_split = s.windows.len();
        assert!(count_after_split >= 2);
        s.apply_action(Action::CloseWindow);
    }
    // Should always have at least 1 window.
    assert!(!s.windows.is_empty());
}

#[test]
fn buffer_create_delete_cycle_20() {
    let mut s = ed();
    for i in 0..20 {
        let id = BufferId(s.next_id());
        s.buffers.insert(id, Buffer::new_scratch(id));
        assert!(s.buffers.contains_key(&id), "iter {i}: buffer should exist");
        s.apply_action(Action::SwitchBuffer(id));
        assert_eq!(cur_buf(&s), id, "iter {i}: should be on new buffer");
        s.apply_action(Action::DeleteBuffer);
        assert!(!s.buffers.contains_key(&id), "iter {i}: buffer should be gone");
    }
    assert_eq!(s.buffers.len(), 1); // Only the original buffer remains.
}

#[test]
fn alternate_buffer_stress() {
    let mut s = ed();
    let id1 = BufferId(s.next_id());
    s.buffers.insert(id1, Buffer::new_scratch(id1));
    for _ in 0..50 {
        s.apply_action(Action::SwitchAlternate);
        let cur = cur_buf(&s);
        assert!(cur == BufferId(0) || cur == id1);
    }
}

#[test]
fn resize_boundary_1x1() {
    let mut s = ed();
    s.apply_action(Action::Resize(1, 1));
    assert_eq!(s.terminal_size, (1, 1));
    // Insert and escape should still work at 1x1.
    s.handle_key(&Key::Char('i'), &m());
    assert_eq!(s.mode, Mode::Insert);
    s.handle_key(&Key::Escape, &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn resize_boundary_large() {
    let mut s = ed();
    s.apply_action(Action::Resize(500, 200));
    assert_eq!(s.terminal_size, (500, 200));
    s.handle_key(&Key::Char('i'), &m());
    s.handle_key(&Key::Char('x'), &m());
    s.handle_key(&Key::Escape, &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn resize_churn_50_cycles() {
    let mut s = ed();
    for i in 0u16..50 {
        let cols = (i % 100) + 1;
        let rows = (i % 50) + 1;
        s.apply_action(Action::Resize(cols, rows));
        assert_eq!(s.terminal_size, (cols, rows));
    }
}
