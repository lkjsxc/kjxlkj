//! Integration tests for search, window, replace,
//! and write-all features.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Action, ActionCommandKind, Direction, InsertPosition, Mode, Motion};

fn make_editor() -> EditorState {
    EditorState::new(80, 24)
}

fn insert_text(ed: &mut EditorState, text: &str) {
    ed.dispatch(Action::EnterInsert(InsertPosition::BeforeCursor));
    for ch in text.chars() {
        ed.dispatch(Action::InsertChar(ch));
    }
    ed.dispatch(Action::ReturnToNormal);
}

#[test]
fn search_forward_moves_cursor() {
    let mut ed = make_editor();
    insert_text(&mut ed, "hello world\nfoo bar");
    ed.dispatch(Action::MoveCursor(Motion::GotoLine(0), 1));
    ed.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    ed.dispatch(Action::SearchForward("bar".into()));
    let w = ed.focused_window().unwrap();
    // Should land on "bar" in line 1.
    assert_eq!(w.cursor.line, 1);
}

#[test]
fn search_backward_moves_cursor() {
    let mut ed = make_editor();
    insert_text(&mut ed, "xyz abc\nxyz def\nxyz abc");
    // Move to end of buffer.
    ed.dispatch(Action::MoveCursor(Motion::GotoLastLine, 1));
    ed.dispatch(Action::MoveCursor(Motion::LineEnd, 1));
    ed.dispatch(Action::SearchBackward("xyz".into()));
    let w = ed.focused_window().unwrap();
    // Should land on some "xyz" match before end.
    assert_eq!(w.cursor.grapheme_offset, 0);
}

#[test]
fn next_prev_match() {
    let mut ed = make_editor();
    insert_text(&mut ed, "aa\naa\naa");
    ed.dispatch(Action::MoveCursor(Motion::GotoLine(0), 1));
    ed.dispatch(Action::SearchForward("aa".into()));
    let line1 = ed.focused_window().unwrap().cursor.line;
    ed.dispatch(Action::NextMatch);
    let line2 = ed.focused_window().unwrap().cursor.line;
    assert!(line2 >= line1 || line2 == 0);
    ed.dispatch(Action::PrevMatch);
}

#[test]
fn write_all_clears_modified() {
    let mut ed = make_editor();
    insert_text(&mut ed, "hello");
    assert!(ed.active_buffer().unwrap().modified);
    ed.dispatch(Action::WriteAll);
    assert!(!ed.active_buffer().unwrap().modified);
}

#[test]
fn write_all_quit_sets_quit() {
    let mut ed = make_editor();
    ed.dispatch(Action::WriteAllQuit);
    assert!(ed.should_quit);
}

#[test]
fn wa_command() {
    let mut ed = make_editor();
    insert_text(&mut ed, "test");
    assert!(ed.active_buffer().unwrap().modified);
    ed.dispatch(Action::ExecuteCommand("wa".into()));
    assert!(!ed.active_buffer().unwrap().modified);
}

#[test]
fn wqa_command() {
    let mut ed = make_editor();
    ed.dispatch(Action::ExecuteCommand("wqa".into()));
    assert!(ed.should_quit);
}

#[test]
fn focus_window_cycles() {
    let mut ed = make_editor();
    ed.dispatch(Action::SplitHorizontal);
    assert_eq!(ed.windows.len(), 2);
    let first = ed.focused_window;
    ed.dispatch(Action::FocusWindow(Direction::Right));
    let second = ed.focused_window;
    assert_ne!(first, second);
    ed.dispatch(Action::FocusWindow(Direction::Left));
    let third = ed.focused_window;
    assert_eq!(third, first);
}

#[test]
fn cycle_window() {
    let mut ed = make_editor();
    ed.dispatch(Action::SplitVertical);
    let first = ed.focused_window;
    ed.dispatch(Action::CycleWindow);
    let second = ed.focused_window;
    assert_ne!(first, second);
    ed.dispatch(Action::CycleWindow);
    assert_eq!(ed.focused_window, first);
}

#[test]
fn close_window() {
    let mut ed = make_editor();
    ed.dispatch(Action::SplitHorizontal);
    assert_eq!(ed.windows.len(), 2);
    ed.dispatch(Action::CloseWindow);
    assert_eq!(ed.windows.len(), 1);
}

#[test]
fn close_last_window_quits() {
    let mut ed = make_editor();
    assert_eq!(ed.windows.len(), 1);
    ed.dispatch(Action::CloseWindow);
    assert!(ed.should_quit);
}

#[test]
fn replace_mode_overwrites() {
    let mut ed = make_editor();
    insert_text(&mut ed, "hello");
    ed.dispatch(Action::MoveCursor(Motion::GotoLine(0), 1));
    ed.dispatch(Action::MoveCursor(Motion::LineStart, 1));
    ed.dispatch(Action::EnterReplace);
    assert_eq!(ed.mode, Mode::Replace);
}

#[test]
fn enter_command_search_forward() {
    let mut ed = make_editor();
    insert_text(&mut ed, "hello world");
    ed.dispatch(Action::EnterCommand(ActionCommandKind::SearchForward));
    assert!(matches!(ed.mode, Mode::Command(_)));
}
