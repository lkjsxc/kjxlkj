use super::*;
use kjxlkj_core_types::Position;

#[test]
fn normal_h_moves_left() {
    let mut state = EditorState::new();
    state.active_window_mut().cursor.col = 5;
    handle_normal_key(&mut state, KeyEvent::char('h'));
    assert_eq!(state.active_window().cursor.col, 4);
}

#[test]
fn normal_j_moves_down() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, "a\nb\nc\n");
    handle_normal_key(&mut state, KeyEvent::char('j'));
    assert_eq!(state.active_window().cursor.line, 1);
}

#[test]
fn normal_i_enters_insert() {
    let mut state = EditorState::new();
    let action = handle_normal_key(&mut state, KeyEvent::char('i'));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Insert)));
}

#[test]
fn normal_colon_enters_command() {
    let mut state = EditorState::new();
    let action = handle_normal_key(&mut state, KeyEvent::char(':'));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Command)));
    assert!(state.command_line.active);
}
