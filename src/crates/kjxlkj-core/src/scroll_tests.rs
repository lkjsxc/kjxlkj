use super::*;
use kjxlkj_core_types::Position;

#[test]
fn ctrl_d_scrolls_down() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(100));
    let key = KeyEvent::ctrl('d');
    handle_scroll_key(&mut state, &key);
    assert!(state.active_window().cursor.line > 0);
}

#[test]
fn ctrl_u_scrolls_up() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(100));
    state.active_window_mut().cursor.line = 50;
    let key = KeyEvent::ctrl('u');
    handle_scroll_key(&mut state, &key);
    assert!(state.active_window().cursor.line < 50);
}

#[test]
fn ctrl_e_scrolls_one() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(50));
    let key = KeyEvent::ctrl('e');
    handle_scroll_key(&mut state, &key);
    assert_eq!(state.viewport.top_line, 1);
}

#[test]
fn z_center() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(100));
    state.active_window_mut().cursor.line = 50;
    handle_z_scroll(&mut state, &KeyEvent::char('z'));
    assert!(state.viewport.top_line > 0);
}

#[test]
fn z_top() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(100));
    state.active_window_mut().cursor.line = 30;
    handle_z_scroll(&mut state, &KeyEvent::char('t'));
    assert_eq!(state.viewport.top_line, 30);
}

#[test]
fn z_bottom() {
    let mut state = EditorState::new();
    state
        .active_buffer_mut()
        .insert_text(Position::ZERO, &"line\n".repeat(100));
    state.active_window_mut().cursor.line = 30;
    handle_z_scroll(&mut state, &KeyEvent::char('b'));
    assert!(state.viewport.top_line < 30);
}

#[test]
fn ctrl_v_visual_block() {
    let mut state = EditorState::new();
    let key = KeyEvent::ctrl('v');
    let action = handle_scroll_key(&mut state, &key);
    assert_eq!(
        action,
        Some(EditorAction::ChangeMode(kjxlkj_core_types::Mode::VisualBlock))
    );
}
