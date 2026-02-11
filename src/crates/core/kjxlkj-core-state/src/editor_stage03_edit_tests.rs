//! Stage 03 exit tests — editing operators, text objects, visual, increments.

use crate::editor::EditorState;
use kjxlkj_core_text::Buffer;
use kjxlkj_core_types::{
    Action, BufferId, ContentKind, Key, KeyModifiers, Mode, VisualKind,
};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }

fn cur_buf(s: &EditorState) -> BufferId {
    let wid = s.focus.focused;
    match s.windows.get(&wid).unwrap().content {
        ContentKind::Buffer(id) => id,
        _ => panic!("no buffer"),
    }
}

fn add_buf(s: &mut EditorState) -> BufferId {
    let id = BufferId(s.next_id());
    s.buffers.insert(id, Buffer::new_scratch(id));
    id
}

fn buf_text(s: &EditorState, id: BufferId) -> String {
    s.buffers.get(&id).unwrap().line(0).unwrap_or_default().to_string()
}

/// diw removes inner word.
#[test]
fn text_object_diw() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &m());
    for c in "hello world".chars() {
        s.handle_key(&Key::Char(c), &m());
    }
    s.handle_key(&Key::Escape, &m());
    s.handle_key(&Key::Char('0'), &m());
    s.handle_key(&Key::Char('d'), &m());
    s.handle_key(&Key::Char('i'), &m());
    s.handle_key(&Key::Char('w'), &m());
    let text = buf_text(&s, BufferId(0));
    assert!(!text.starts_with("hello"));
}

/// Ctrl-a increments number.
#[test]
fn ctrl_a_increments() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &m());
    for c in "count: 42".chars() {
        s.handle_key(&Key::Char(c), &m());
    }
    s.handle_key(&Key::Escape, &m());
    s.handle_key(&Key::Char('0'), &m());
    s.apply_action(Action::IncrementNumber);
    let text = buf_text(&s, BufferId(0));
    assert!(text.contains("43"));
}

/// :bfirst / :blast navigate to first and last buffer.
#[test]
fn bfirst_blast_navigation() {
    let mut s = ed();
    let _id1 = add_buf(&mut s);
    let _id2 = add_buf(&mut s);
    s.apply_action(Action::FirstBuffer);
    let first = cur_buf(&s);
    s.apply_action(Action::LastBuffer);
    let last = cur_buf(&s);
    assert_ne!(first, last);
}

/// Visual mode yank records in register.
#[test]
fn visual_yank_records_register() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &m());
    for c in "test data".chars() {
        s.handle_key(&Key::Char(c), &m());
    }
    s.handle_key(&Key::Escape, &m());
    s.handle_key(&Key::Char('0'), &m());
    s.handle_key(&Key::Char('v'), &m());
    assert!(matches!(s.mode, Mode::Visual(VisualKind::Char)));
    s.handle_key(&Key::Char('e'), &m());
    s.handle_key(&Key::Char('y'), &m());
    assert_eq!(s.mode, Mode::Normal);
    assert!(s.registers.get('"').is_some());
}

/// Bracket match (%) moves cursor.
#[test]
fn bracket_match_percent() {
    let mut s = ed();
    s.handle_key(&Key::Char('i'), &m());
    for c in "(hello)".chars() {
        s.handle_key(&Key::Char(c), &m());
    }
    s.handle_key(&Key::Escape, &m());
    s.handle_key(&Key::Char('0'), &m());
    let wid = s.focus.focused;
    let before = s.windows.get(&wid).unwrap().cursor.col;
    s.handle_key(&Key::Char('%'), &m());
    let after = s.windows.get(&wid).unwrap().cursor.col;
    assert_ne!(before, after);
}
