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
