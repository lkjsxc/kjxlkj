use crate::{EditorAction, EditorState};
use kjxlkj_core_mode::Mode;

#[test]
fn a_at_eol_enters_insert_after_last_grapheme() {
    let mut state = EditorState::new("abc".to_string(), 2);
    let result = state.apply(EditorAction::NormalModeKey('a'));
    assert_eq!(result.resolved_action, "EnterInsertAfterCursor");
    assert_eq!(state.cursor(), 3);
    assert_eq!(state.mode(), Mode::Insert);
}

#[test]
fn i_at_eol_differs_from_a_at_eol() {
    let mut i_state = EditorState::new("abc".to_string(), 2);
    let mut a_state = EditorState::new("abc".to_string(), 2);
    i_state.apply(EditorAction::NormalModeKey('i'));
    a_state.apply(EditorAction::NormalModeKey('a'));
    assert_eq!(i_state.cursor(), 2);
    assert_eq!(a_state.cursor(), 3);
}

#[test]
fn upper_a_enters_insert_at_true_eol() {
    let mut state = EditorState::new("abc".to_string(), 0);
    let result = state.apply(EditorAction::NormalModeKey('A'));
    assert_eq!(result.resolved_action, "EnterInsertAtEol");
    assert_eq!(state.cursor(), 3);
}

#[test]
fn i_on_terminal_window_enters_terminal_insert_mode() {
    let mut state = EditorState::new("abc".to_string(), 0);
    state.apply(EditorAction::WindowCommand('T'));
    let result = state.apply(EditorAction::NormalModeKey('i'));
    assert_eq!(result.resolved_action, "EnterTerminalInsert");
    assert_eq!(state.mode(), Mode::TerminalInsert);
}

#[test]
fn terminal_exit_to_normal_is_applied() {
    let mut state = EditorState::new("abc".to_string(), 0);
    state.apply(EditorAction::WindowCommand('T'));
    state.apply(EditorAction::NormalModeKey('i'));
    let result = state.apply(EditorAction::TerminalExitToNormal);
    assert_eq!(result.resolved_action, "TerminalExitToNormal");
    assert_eq!(state.mode(), Mode::Normal);
}
