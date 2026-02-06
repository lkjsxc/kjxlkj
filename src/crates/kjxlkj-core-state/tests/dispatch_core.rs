//! Tests for dispatch core logic.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{
    InsertPosition, Intent, Mode, MotionKind, OperatorKind,
    ScrollKind, Size,
};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

#[test]
fn insert_char() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('x'));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with('x'));
}

#[test]
fn motion_down() {
    let mut s = setup("line1\nline2\nline3");
    dispatch_intent(
        &mut s,
        Intent::Motion(MotionKind::Down, 1),
    );
    assert_eq!(s.cursor().line, 1);
}

#[test]
fn dd_deletes_line() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(
        &mut s,
        Intent::LineOperator(OperatorKind::Delete, 1),
    );
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with('b'));
}

#[test]
fn quit_command() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
    assert!(s.should_quit);
}

#[test]
fn enter_insert_mode() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::EnterInsert(InsertPosition::BeforeCursor),
    );
    assert_eq!(s.current_mode(), Mode::Insert);
}

#[test]
fn open_line_below() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::OpenLine(true));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().line, 1);
}

#[test]
fn toggle_case() {
    let mut s = setup("Hello");
    dispatch_intent(&mut s, Intent::ToggleCase);
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).starts_with('h'));
}

#[test]
fn scroll_half_page_down() {
    let mut s = setup(
        &(0..50)
            .map(|i| format!("line{}", i))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    dispatch_intent(
        &mut s,
        Intent::Scroll(ScrollKind::HalfPageDown),
    );
    let win = s.active_window_state().unwrap();
    assert!(win.cursor_line > 0);
}
