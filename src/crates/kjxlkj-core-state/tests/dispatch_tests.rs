//! Integration tests for action dispatch.
//!
//! These verify that actions dispatched through
//! EditorState::dispatch produce the expected
//! state mutations, exercising real user paths.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Action, InsertPosition, Mode, Motion, Operator,
    VisualKind,
};

fn editor() -> EditorState {
    EditorState::new(80, 24)
}

#[test]
fn dispatch_quit_sets_flag() {
    let mut ed = editor();
    ed.dispatch(Action::Quit);
    assert!(ed.should_quit);
}

#[test]
fn dispatch_force_quit() {
    let mut ed = editor();
    ed.dispatch(Action::ForceQuit);
    assert!(ed.should_quit);
}

#[test]
fn dispatch_nop_no_change() {
    let mut ed = editor();
    let mode = ed.mode.clone();
    ed.dispatch(Action::Nop);
    assert_eq!(ed.mode, mode);
    assert!(!ed.should_quit);
}

#[test]
fn dispatch_enter_insert() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    assert_eq!(ed.mode, Mode::Insert);
}

#[test]
fn dispatch_return_to_normal() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    assert_eq!(ed.mode, Mode::Insert);
    ed.dispatch(Action::ReturnToNormal);
    assert_eq!(ed.mode, Mode::Normal);
}

#[test]
fn dispatch_insert_char() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('H'));
    ed.dispatch(Action::InsertChar('i'));
    let buf = ed.active_buffer().unwrap();
    let line = buf.content.line_content(0);
    assert!(line.starts_with("Hi"));
}

#[test]
fn dispatch_backspace() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('A'));
    ed.dispatch(Action::InsertChar('B'));
    ed.dispatch(Action::DeleteCharBackward);
    let buf = ed.active_buffer().unwrap();
    let line = buf.content.line_content(0);
    assert!(line.starts_with('A'));
    assert!(!line.contains('B'));
}

#[test]
fn dispatch_enter_visual() {
    let mut ed = editor();
    ed.dispatch(Action::EnterVisual(
        VisualKind::Char,
    ));
    assert!(
        matches!(ed.mode, Mode::Visual(VisualKind::Char))
    );
}

#[test]
fn dispatch_enter_command() {
    let mut ed = editor();
    ed.dispatch(Action::EnterCommand(
        kjxlkj_core_types::ActionCommandKind::Ex,
    ));
    assert!(matches!(ed.mode, Mode::Command(_)));
}

#[test]
fn dispatch_write_clears_modified() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('x'));
    assert!(ed.active_buffer().unwrap().modified);
    ed.dispatch(Action::Write);
    assert!(!ed.active_buffer().unwrap().modified);
}

#[test]
fn dispatch_write_quit() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('x'));
    ed.dispatch(Action::WriteQuit);
    assert!(!ed.active_buffer().unwrap().modified);
    assert!(ed.should_quit);
}

#[test]
fn dispatch_undo_redo() {
    let mut ed = editor();
    ed.dispatch(Action::Undo);
    ed.dispatch(Action::Redo);
    // Should not panic on empty undo tree.
}

#[test]
fn dispatch_resize() {
    let mut ed = editor();
    ed.dispatch(Action::Resize(120, 40));
    assert_eq!(ed.terminal_size, (120, 40));
}

#[test]
fn dispatch_replace_char() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('a'));
    ed.dispatch(Action::ReturnToNormal);
    // Cursor moves back 1 on ReturnToNormal, so at 0.
    ed.dispatch(Action::ReplaceChar('Z'));
    let buf = ed.active_buffer().unwrap();
    let line = buf.content.line_content(0);
    assert!(
        line.starts_with('Z'),
        "expected 'Z' start, got: {:?}",
        line
    );
}

#[test]
fn dispatch_open_line_below() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::NewLineBelow,
    ));
    assert_eq!(ed.mode, Mode::Insert);
    let w = ed.focused_window().unwrap();
    assert_eq!(w.cursor.line, 1);
}

#[test]
fn dispatch_split_horizontal() {
    let mut ed = editor();
    let before = ed.windows.len();
    ed.dispatch(Action::SplitHorizontal);
    assert_eq!(ed.windows.len(), before + 1);
}

#[test]
fn dispatch_next_prev_buffer() {
    let mut ed = editor();
    // Only one buffer — should not panic.
    ed.dispatch(Action::NextBuffer);
    ed.dispatch(Action::PrevBuffer);
}

#[test]
fn dispatch_delete_buffer_creates_scratch() {
    let mut ed = editor();
    ed.dispatch(Action::DeleteBuffer);
    // Should create a new scratch buffer.
    assert!(!ed.buffers.is_empty());
}

#[test]
fn dispatch_double_operator_delete() {
    let mut ed = editor();
    // Insert some content first.
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('a'));
    ed.dispatch(Action::InsertChar('\n'));
    ed.dispatch(Action::InsertChar('b'));
    ed.dispatch(Action::InsertChar('\n'));
    ed.dispatch(Action::InsertChar('c'));
    ed.dispatch(Action::ReturnToNormal);
    // Go to first line.
    ed.dispatch(Action::MoveCursor(
        Motion::GotoLine(0),
        1,
    ));
    // dd on line 1.
    ed.dispatch(Action::DoubleOperator(
        Operator::Delete,
        1,
    ));
    let buf = ed.active_buffer().unwrap();
    // Should have fewer lines now.
    assert!(buf.line_count() < 4);
}

#[test]
fn dispatch_enter_replace_mode() {
    let mut ed = editor();
    ed.dispatch(Action::EnterReplace);
    assert_eq!(ed.mode, Mode::Replace);
}

#[test]
fn dispatch_toggle_case() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('a'));
    ed.dispatch(Action::ReturnToNormal);
    // Cursor is now at offset 0 after clamping.
    ed.dispatch(Action::ToggleCaseChar);
    let buf = ed.active_buffer().unwrap();
    let line = buf.content.line_content(0);
    assert!(
        line.starts_with('A'),
        "expected 'A' start, got: {:?}",
        line
    );
}

#[test]
fn dispatch_join_lines() {
    let mut ed = editor();
    ed.dispatch(Action::EnterInsert(
        InsertPosition::BeforeCursor,
    ));
    ed.dispatch(Action::InsertChar('a'));
    ed.dispatch(Action::InsertChar('\n'));
    ed.dispatch(Action::InsertChar('b'));
    ed.dispatch(Action::ReturnToNormal);
    ed.dispatch(Action::MoveCursor(
        Motion::GotoLine(0),
        1,
    ));
    let before = ed.active_buffer().unwrap().line_count();
    ed.dispatch(Action::JoinLines);
    let after = ed.active_buffer().unwrap().line_count();
    assert!(after < before);
}
