use super::*;
use kjxlkj_core_types::KeyCode;

fn setup_cmd() -> EditorState {
    let mut state = EditorState::new();
    state.command_line.activate(":");
    state.mode.transition(Mode::Command);
    state
}

#[test]
fn insert_chars() {
    let mut state = setup_cmd();
    handle_command_key(&mut state, KeyEvent::char('q'));
    assert_eq!(state.command_line.content, "q");
    assert_eq!(state.command_line.cursor_pos, 1);
}

#[test]
fn backspace_deletes() {
    let mut state = setup_cmd();
    handle_command_key(&mut state, KeyEvent::char('a'));
    handle_command_key(&mut state, KeyEvent::plain(KeyCode::Backspace));
    assert_eq!(state.command_line.content, "");
}

#[test]
fn escape_cancels() {
    let mut state = setup_cmd();
    let action = handle_command_key(&mut state, KeyEvent::plain(KeyCode::Escape));
    assert_eq!(action, Some(EditorAction::ChangeMode(Mode::Normal)));
    assert!(!state.command_line.active);
}

#[test]
fn enter_executes() {
    let mut state = setup_cmd();
    handle_command_key(&mut state, KeyEvent::char('q'));
    handle_command_key(&mut state, KeyEvent::char('!'));
    let _action = handle_command_key(&mut state, KeyEvent::plain(KeyCode::Enter));
    assert!(state.should_quit);
}

#[test]
fn search_forward() {
    let mut state = EditorState::new();
    state.command_line.activate("/");
    state.mode.transition(Mode::Command);
    handle_command_key(&mut state, KeyEvent::char('f'));
    handle_command_key(&mut state, KeyEvent::char('o'));
    let action = handle_command_key(&mut state, KeyEvent::plain(KeyCode::Enter));
    assert!(matches!(
        action,
        Some(EditorAction::Search(_, Direction::Forward))
    ));
}

#[test]
fn ctrl_w_deletes_word() {
    let mut state = setup_cmd();
    for ch in "hello world".chars() {
        handle_command_key(&mut state, KeyEvent::char(ch));
    }
    handle_command_key(&mut state, KeyEvent::ctrl('w'));
    assert_eq!(state.command_line.content, "hello ");
}

#[test]
fn cursor_movement() {
    let mut state = setup_cmd();
    handle_command_key(&mut state, KeyEvent::char('a'));
    handle_command_key(&mut state, KeyEvent::char('b'));
    handle_command_key(&mut state, KeyEvent::plain(KeyCode::Left));
    assert_eq!(state.command_line.cursor_pos, 1);
    handle_command_key(&mut state, KeyEvent::plain(KeyCode::Home));
    assert_eq!(state.command_line.cursor_pos, 0);
    handle_command_key(&mut state, KeyEvent::plain(KeyCode::End));
    assert_eq!(state.command_line.cursor_pos, 2);
}

#[test]
fn word_back_fn() {
    assert_eq!(word_back("hello world", 11), 6);
    assert_eq!(word_back("word", 4), 0);
}
