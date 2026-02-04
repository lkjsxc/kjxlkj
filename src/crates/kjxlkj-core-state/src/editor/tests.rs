//! Editor state tests.

use super::EditorState;
use kjxlkj_core_types::Mode;

#[test]
fn editor_state_creation() {
    let state = EditorState::new();
    assert_eq!(state.mode(), Mode::Normal);
    assert!(!state.should_quit);
}

#[test]
fn editor_state_mode_change() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    assert_eq!(state.mode(), Mode::Insert);
}

#[test]
fn editor_state_visual_creates_selection() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Visual);
    assert!(state.selection.is_some());
}

#[test]
fn editor_state_snapshot() {
    let state = EditorState::new();
    let snap = state.snapshot();
    assert_eq!(snap.mode, Mode::Normal);
}
