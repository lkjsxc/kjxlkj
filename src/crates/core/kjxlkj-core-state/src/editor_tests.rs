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
