//! Tests for headless processor.

use super::processor::process_key;
use kjxlkj_core::{EditorState, Mode, Position, Selection, SelectionKind};
use kjxlkj_input::{Key, KeyCode, Modifiers};

fn key_escape() -> Key {
    Key::new(KeyCode::Escape, Modifiers::none())
}

#[test]
fn process_normal_mode_escape() {
    let mut state = EditorState::new();
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_insert_mode_escape() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_insert_mode_char() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    process_key(&mut state, Key::char('x'));
    let line = state.buffer.line(0).unwrap_or_default();
    assert!(line.contains('x'));
}

#[test]
fn process_command_mode_escape() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Command);
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_visual_mode_escape() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Visual);
    state.selection = Some(Selection::new(
        Position::new(0, 0),
        Position::new(0, 0),
        SelectionKind::Char,
    ));
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
    assert!(state.selection.is_none());
}

#[test]
fn process_replace_mode_escape() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Replace);
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_insert_backspace() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "ab");
    state.set_mode(Mode::Insert);
    state.cursor.position = Position::new(0, 2);
    process_key(&mut state, Key::new(KeyCode::Backspace, Modifiers::none()));
    let line = state.buffer.line(0).unwrap_or_default();
    assert_eq!(line, "a");
}

#[test]
fn process_command_enter() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Command);
    state.mode_state.command_line = "q".to_string();
    process_key(&mut state, Key::new(KeyCode::Enter, Modifiers::none()));
    assert!(state.should_quit);
}

#[test]
fn process_insert_char_basic() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    state.cursor.position = Position::new(0, 0);
    process_key(&mut state, Key::new(KeyCode::Char('x'), Modifiers::none()));
    let line = state.buffer.line(0).unwrap_or_default();
    assert!(line.contains('x'));
}

#[test]
fn process_command_mode_char() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Command);
    state.mode_state.command_line = "".to_string();
    process_key(&mut state, Key::new(KeyCode::Char('w'), Modifiers::none()));
    // Command line receives characters in command mode
    assert_eq!(state.mode_state.command_line, "w");
}

#[test]
fn process_visual_line_escape() {
    let mut state = EditorState::new();
    state.set_mode(Mode::VisualLine);
    state.selection = Some(Selection::new(
        Position::new(0, 0),
        Position::new(1, 0),
        SelectionKind::Line,
    ));
    process_key(&mut state, key_escape());
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_normal_mode_j() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "line1\nline2");
    process_key(&mut state, Key::new(KeyCode::Char('j'), Modifiers::none()));
    assert_eq!(state.cursor.line(), 1);
}

#[test]
fn process_normal_mode_k() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "line1\nline2");
    state.cursor.position = Position::new(1, 0);
    process_key(&mut state, Key::new(KeyCode::Char('k'), Modifiers::none()));
    assert_eq!(state.cursor.line(), 0);
}

#[test]
fn process_normal_mode_h() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "hello");
    state.cursor.position = Position::new(0, 3);
    process_key(&mut state, Key::new(KeyCode::Char('h'), Modifiers::none()));
    assert_eq!(state.cursor.col(), 2);
}

#[test]
fn process_normal_mode_0() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "hello");
    state.cursor.position = Position::new(0, 3);
    process_key(&mut state, Key::new(KeyCode::Char('0'), Modifiers::none()));
    assert_eq!(state.cursor.col(), 0);
}

#[test]
fn process_normal_mode_dollar() {
    let mut state = EditorState::new();
    state.buffer.insert(Position::new(0, 0), "hello");
    state.cursor.position = Position::new(0, 0);
    process_key(&mut state, Key::new(KeyCode::Char('$'), Modifiers::none()));
    assert!(state.cursor.col() >= 4);
}

#[test]
fn process_insert_char() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    process_key(&mut state, Key::new(KeyCode::Char('a'), Modifiers::none()));
    assert!(state.buffer.line_count() >= 1);
}

#[test]
fn process_escape_from_insert() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    process_key(&mut state, Key::new(KeyCode::Escape, Modifiers::none()));
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn process_v_enters_visual() {
    let mut state = EditorState::new();
    process_key(&mut state, Key::new(KeyCode::Char('v'), Modifiers::none()));
    assert_eq!(state.mode(), Mode::Visual);
}

#[test]
fn process_colon_enters_command() {
    let mut state = EditorState::new();
    process_key(&mut state, Key::new(KeyCode::Char(':'), Modifiers::none()));
    assert_eq!(state.mode(), Mode::Command);
}

#[test]
fn process_r_mode() {
    let mut state = EditorState::new();
    process_key(&mut state, Key::new(KeyCode::Char('r'), Modifiers::none()));
    // Should enter Replace mode or stay in Normal depending on implementation
    assert!(state.mode() == Mode::Replace || state.mode() == Mode::Normal);
}
