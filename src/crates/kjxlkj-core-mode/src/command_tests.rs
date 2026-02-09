//! Tests for CommandModeState.

use crate::command::CommandModeState;
use kjxlkj_core_types::{Action, ActionCommandKind, Key, KeyCode};

#[test]
fn prompt_characters() {
    let ex = CommandModeState::new(ActionCommandKind::Ex);
    assert_eq!(ex.prompt_char(), ':');
    let fwd = CommandModeState::new(ActionCommandKind::SearchForward);
    assert_eq!(fwd.prompt_char(), '/');
}

#[test]
fn insert_and_execute() {
    let mut s = CommandModeState::new(ActionCommandKind::Ex);
    s.process_key(&Key::char('q'));
    assert_eq!(s.content(), "q");
    let action = s.process_key(&Key::enter());
    assert!(matches!(action, Some(Action::ExecuteCommand(c)) if c == "q"));
}

#[test]
fn backspace_empty_cancels() {
    let mut s = CommandModeState::new(ActionCommandKind::Ex);
    let action = s.process_key(&Key::new(KeyCode::Backspace));
    assert!(matches!(action, Some(Action::ReturnToNormal)));
}

#[test]
fn escape_cancels() {
    let mut s = CommandModeState::new(ActionCommandKind::Ex);
    s.process_key(&Key::char('f'));
    let action = s.process_key(&Key::esc());
    assert!(matches!(action, Some(Action::ReturnToNormal)));
    assert!(s.buffer.is_empty());
}
