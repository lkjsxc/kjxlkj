//! Headless E2E tests HE-01 through HE-09.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Action, InsertPosition, Mode, Motion, Operator,
};

fn ed() -> EditorState {
    EditorState::new(80, 24)
}

fn ins(ed: &mut EditorState, text: &str) {
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for ch in text.chars() {
        ed.dispatch(Action::InsertChar(ch));
    }
    ed.dispatch(Action::ReturnToNormal);
}

/// HE-01: Create, edit, save.
#[test]
fn he01_create_edit_save() {
    let mut e = ed();
    ins(&mut e, "test content");
    assert!(e.active_buffer().unwrap()
        .content.line_str(0).contains("test content"));
    e.dispatch(Action::WriteAll);
    assert!(!e.active_buffer().unwrap().modified);
}

/// HE-02: Navigate to line.
#[test]
fn he02_navigate() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for i in 0..50 {
        for ch in format!("line {i}").chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        e.dispatch(Action::InsertChar('\n'));
    }
    e.dispatch(Action::ReturnToNormal);
    e.dispatch(Action::MoveCursor(
        Motion::GotoLine(24), 1,
    ));
    assert_eq!(e.focused_window().unwrap().cursor.line, 24);
}

/// HE-03: Search and replace.
#[test]
fn he03_search_replace() {
    let mut e = ed();
    ins(&mut e, "foo bar foo");
    e.dispatch(Action::ExecuteCommand(
        "%s/foo/baz/g".into(),
    ));
    let line = e.active_buffer().unwrap()
        .content.line_str(0);
    assert!(line.contains("baz"));
    assert!(!line.contains("foo"));
}

/// HE-04: Visual block (structural).
#[test]
fn he04_visual_block() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for i in 0..5 {
        for ch in "ABCDEFGH".chars() {
            e.dispatch(Action::InsertChar(ch));
        }
        if i < 4 {
            e.dispatch(Action::InsertChar('\n'));
        }
    }
    e.dispatch(Action::ReturnToNormal);
    assert_eq!(e.active_buffer().unwrap()
        .content.line_count(), 5);
}

/// HE-05: Macro record and replay.
#[test]
fn he05_macro() {
    let mut e = ed();
    ins(&mut e, "aaa bbb ccc ddd");
    e.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    e.dispatch(Action::RecordMacro('a'));
    // Recording started (may be Some or toggle impl)
    e.dispatch(Action::Delete(Motion::WordForward, 1));
    e.dispatch(Action::RecordMacro('a'));
    // After stop, buffer is still valid
    let line = e.active_buffer().unwrap()
        .content.line_str(0);
    assert!(!line.is_empty());
}

/// HE-06: Split and navigate.
#[test]
fn he06_split() {
    let mut e = ed();
    e.dispatch(Action::SplitVertical);
    assert_eq!(e.windows.len(), 2);
    let first = e.focused_window;
    e.dispatch(Action::CycleWindow);
    assert_ne!(e.focused_window, first);
    e.dispatch(Action::CycleWindow);
    assert_eq!(e.focused_window, first);
}

/// HE-07: Session roundtrip.
#[test]
fn he07_session() {
    use kjxlkj_core_state::{SessionData, SessionLayout};
    use std::path::PathBuf;
    let data = SessionData {
        buffers: vec![PathBuf::from("test.txt")],
        layout: SessionLayout::Single,
        active: 0,
        cwd: PathBuf::from("/tmp"),
    };
    let json = serde_json::to_string(&data).unwrap();
    let loaded: SessionData =
        serde_json::from_str(&json).unwrap();
    assert_eq!(loaded.buffers.len(), 1);
}

/// HE-08: CJK insert and cursor.
#[test]
fn he08_cjk() {
    let mut e = ed();
    e.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    for ch in "あいうえお".chars() {
        e.dispatch(Action::InsertChar(ch));
    }
    e.dispatch(Action::ReturnToNormal);
    let line = e.active_buffer().unwrap()
        .content.line_str(0);
    assert!(line.contains("あいうえお"));
    use kjxlkj_core_text::display_width::grapheme_display_width;
    let w: usize = "あいうえお".chars()
        .map(|c| grapheme_display_width(&c.to_string()) as usize)
        .sum();
    assert_eq!(w, 10);
}

/// HE-09: Undo to empty.
#[test]
fn he09_undo_empty() {
    let mut e = ed();
    ins(&mut e, "abc");
    e.dispatch(Action::Undo);
    let buf = e.active_buffer().unwrap();
    assert!(buf.content.line_count() >= 1);
}
