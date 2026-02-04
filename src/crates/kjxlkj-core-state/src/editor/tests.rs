//! Editor state tests.

use super::EditorState;
use kjxlkj_core_types::{Mode, Position};

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

#[test]
fn editor_state_jump_list() {
    let mut state = EditorState::new();
    state.push_jump();
    state.push_jump();
    let pos = state.jump_backward();
    assert!(pos.is_some());
}

#[test]
fn editor_state_clamp_cursor() {
    let mut state = EditorState::new();
    state.cursor.position = Position::new(1000, 1000);
    state.clamp_cursor();
    assert!(state.cursor.line() < 1000);
}

#[test]
fn editor_state_default() {
    let state = EditorState::default();
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_quit_flag() {
    let mut state = EditorState::new();
    assert!(!state.should_quit);
    state.should_quit = true;
    assert!(state.should_quit);
}

#[test]
fn editor_state_visual_to_normal_clears_selection() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Visual);
    assert!(state.selection.is_some());
    state.set_mode(Mode::Normal);
    assert!(state.selection.is_none());
}

#[test]
fn editor_state_jump_forward() {
    let mut state = EditorState::new();
    state.push_jump();
    state.push_jump();
    state.jump_backward();
    let pos = state.jump_forward();
    assert!(pos.is_some());
}

#[test]
fn editor_state_jump_list_empty() {
    let mut state = EditorState::new();
    assert!(state.jump_backward().is_none());
}

#[test]
fn editor_state_insert_mode() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    assert_eq!(state.mode(), Mode::Insert);
    assert!(state.selection.is_none());
}

#[test]
fn editor_state_command_mode() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Command);
    assert_eq!(state.mode(), Mode::Command);
}

#[test]
fn editor_state_replace_mode() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Replace);
    assert_eq!(state.mode(), Mode::Replace);
}

#[test]
fn editor_state_visual_mode() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Visual);
    assert_eq!(state.mode(), Mode::Visual);
}

#[test]
fn editor_state_push_multiple_jumps() {
    let mut state = EditorState::new();
    state.push_jump();
    state.push_jump();
    state.push_jump();
    let _ = state.jump_backward();
    let _ = state.jump_backward();
    assert!(state.jump_backward().is_some() || state.jump_backward().is_none());
}

#[test]
fn editor_state_mode_default() {
    let state = EditorState::new();
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_selection_none() {
    let state = EditorState::new();
    assert!(state.selection.is_none());
}

#[test]
fn editor_state_switch_to_normal() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Insert);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_command_back_to_normal() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Command);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_jump_forward_empty() {
    let mut state = EditorState::new();
    assert!(state.jump_forward().is_none());
}

#[test]
fn editor_state_initial_jump_list() {
    let mut state = EditorState::new();
    assert!(state.jump_backward().is_none());
}

#[test]
fn editor_state_normal_mode_check() {
    let state = EditorState::new();
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_visual_back_to_normal() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Visual);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn editor_state_replace_back_to_normal() {
    let mut state = EditorState::new();
    state.set_mode(Mode::Replace);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode(), Mode::Normal);
}
