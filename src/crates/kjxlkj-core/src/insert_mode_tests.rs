use super::*;
use crate::insert_mode_ctrl::word_back_pos;

#[test]
fn escape_returns_to_normal() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    let action = handle_insert_key(&mut state, KeyEvent::plain(KeyCode::Escape));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Normal)));
}

#[test]
fn char_insertion() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    handle_insert_key(&mut state, KeyEvent::char('A'));
    assert_eq!(state.active_buffer().line(0), Some("A".to_string()));
    assert_eq!(state.active_window().cursor.col, 1);
}

#[test]
fn backspace_deletes() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    state.active_buffer_mut().insert_text(Position::ZERO, "AB");
    state.active_window_mut().cursor.col = 2;
    handle_insert_key(&mut state, KeyEvent::plain(KeyCode::Backspace));
    assert_eq!(state.active_buffer().line(0), Some("A".to_string()));
    assert_eq!(state.active_window().cursor.col, 1);
}

#[test]
fn enter_with_indent() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "  hello");
    state.active_window_mut().cursor = Position::new(0, 7);
    handle_insert_key(&mut state, KeyEvent::plain(KeyCode::Enter));
    assert_eq!(state.active_window().cursor.line, 1);
    assert_eq!(state.active_window().cursor.col, 2);
}

#[test]
fn word_back_pos_basic() {
    assert_eq!(word_back_pos("hello world", 11), 6);
    assert_eq!(word_back_pos("hello world", 5), 0);
}

#[test]
fn ctrl_u_deletes_to_start() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "hello");
    state.active_window_mut().cursor.col = 5;
    handle_insert_key(&mut state, KeyEvent::ctrl('u'));
    assert_eq!(state.active_buffer().line(0), Some("".to_string()));
}

#[test]
fn arrow_keys_move() {
    let mut state = EditorState::new();
    state.mode.transition(Mode::Insert);
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "ab\ncd");
    state.active_window_mut().cursor = Position::new(0, 1);
    handle_insert_key(&mut state, KeyEvent::plain(KeyCode::Right));
    assert_eq!(state.active_window().cursor.col, 2);
    handle_insert_key(&mut state, KeyEvent::plain(KeyCode::Left));
    assert_eq!(state.active_window().cursor.col, 1);
}
