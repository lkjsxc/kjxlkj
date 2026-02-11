//! Boundary condition and safety tests.
//! Exercises deterministic replay, empty buffer resilience,
//! ex command boundary cases, and force-quit behavior.

use crate::editor::EditorState;
use kjxlkj_core_types::{Action, BufferId, ContentKind, Key, KeyModifiers, Mode};

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
fn deterministic_replay_insert_delete() {
    let run = || {
        let mut s = ed();
        s.handle_key(&Key::Char('i'), &m());
        for c in "abcdef".chars() { s.handle_key(&Key::Char(c), &m()); }
        s.handle_key(&Key::Escape, &m());
        s.handle_key(&Key::Char('x'), &m());
        s.handle_key(&Key::Char('x'), &m());
        let buf = s.buffers.get(&BufferId(0)).unwrap();
        buf.line(0).unwrap_or_default().to_string()
    };
    let r1 = run();
    let r2 = run();
    assert_eq!(r1, r2, "replay must be deterministic");
}

#[test]
fn delete_on_empty_buffer_is_safe() {
    let mut s = ed();
    for _ in 0..20 { s.handle_key(&Key::Char('x'), &m()); }
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn motion_on_empty_buffer_is_safe() {
    let mut s = ed();
    for key in ['h', 'j', 'k', 'l', 'w', 'b', 'e', '0', '$'] {
        s.handle_key(&Key::Char(key), &m());
    }
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn ex_unknown_command_is_noop() {
    let mut s = ed();
    s.handle_key(&Key::Char(':'), &m());
    for c in "nonexistent_cmd".chars() { s.handle_key(&Key::Char(c), &m()); }
    s.handle_key(&Key::Enter, &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn sequential_ex_commands() {
    let mut s = ed();
    for cmd in ["bn", "bp", "bf", "bl"] {
        s.handle_key(&Key::Char(':'), &m());
        for c in cmd.chars() { s.handle_key(&Key::Char(c), &m()); }
        s.handle_key(&Key::Enter, &m());
        assert_eq!(s.mode, Mode::Normal);
    }
}

#[test]
fn ctrl_6_without_alternate() {
    let mut s = ed();
    s.handle_key(&Key::Char('6'), &ctrl());
    assert_eq!(cur_buf(&s), BufferId(0));
}

#[test]
fn force_quit_sets_flag() {
    let mut s = ed();
    s.apply_action(Action::ForceQuit);
    assert!(s.quit_requested);
}
