//! Tests for mode state aggregation.

use crate::{ModeState, NormalModeState};
use kjxlkj_core_types::Mode;

#[test]
fn mode_state_default() {
    let state = ModeState::new();
    assert_eq!(state.mode, Mode::Normal);
}

#[test]
fn mode_state_set_mode() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Insert);
    assert_eq!(state.mode, Mode::Insert);
}

#[test]
fn mode_state_reset() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Insert);
    state.command_line = "test".to_string();
    state.reset();
    assert_eq!(state.mode, Mode::Normal);
    assert!(state.command_line.is_empty());
}

#[test]
fn normal_to_insert_transition() {
    let mut state = ModeState::new();
    assert_eq!(state.mode, Mode::Normal);
    state.set_mode(Mode::Insert);
    assert_eq!(state.mode, Mode::Insert);
}

#[test]
fn normal_to_visual_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Visual);
    assert_eq!(state.mode, Mode::Visual);
}

#[test]
fn normal_to_command_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Command);
    assert_eq!(state.mode, Mode::Command);
}

#[test]
fn normal_to_replace_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Replace);
    assert_eq!(state.mode, Mode::Replace);
}

#[test]
fn insert_to_normal_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Insert);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode, Mode::Normal);
}

#[test]
fn visual_to_normal_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Visual);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode, Mode::Normal);
}

#[test]
fn command_to_normal_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Command);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode, Mode::Normal);
}

#[test]
fn normal_resets_normal_state() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Insert);
    state.set_mode(Mode::Normal);
    assert_eq!(state.normal.get_count(), 1);
}

#[test]
fn search_pattern_default_empty() {
    let state = ModeState::new();
    assert!(state.search_pattern.is_empty());
}

#[test]
fn search_pattern_set() {
    let mut state = ModeState::new();
    state.search_pattern = "test".to_string();
    assert_eq!(state.search_pattern, "test");
}

#[test]
fn search_forward_default() {
    let state = ModeState::new();
    assert!(!state.search_forward);
}

#[test]
fn search_forward_toggle() {
    let mut state = ModeState::new();
    state.search_forward = true;
    assert!(state.search_forward);
    state.search_forward = false;
    assert!(!state.search_forward);
}

#[test]
fn recording_macro_default_none() {
    let state = ModeState::new();
    assert!(state.recording_macro.is_none());
}

#[test]
fn recording_macro_set() {
    let mut state = ModeState::new();
    state.recording_macro = Some('a');
    assert_eq!(state.recording_macro, Some('a'));
}

#[test]
fn recording_macro_clear() {
    let mut state = ModeState::new();
    state.recording_macro = Some('q');
    state.recording_macro = None;
    assert!(state.recording_macro.is_none());
}

#[test]
fn reset_clears_command_line() {
    let mut state = ModeState::new();
    state.command_line = "some command".to_string();
    state.reset();
    assert!(state.command_line.is_empty());
}

#[test]
fn reset_preserves_search_pattern() {
    let mut state = ModeState::new();
    state.search_pattern = "pattern".to_string();
    state.reset();
    assert_eq!(state.search_pattern, "pattern");
}

#[test]
fn replace_to_normal_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Replace);
    state.set_mode(Mode::Normal);
    assert_eq!(state.mode, Mode::Normal);
}

#[test]
fn visual_to_insert_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Visual);
    state.set_mode(Mode::Insert);
    assert_eq!(state.mode, Mode::Insert);
}

#[test]
fn mode_state_clone() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Insert);
    state.command_line = "test".to_string();
    let cloned = state.clone();
    assert_eq!(cloned.mode, Mode::Insert);
    assert_eq!(cloned.command_line, "test");
}

#[test]
fn visual_line_mode_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::VisualLine);
    assert_eq!(state.mode, Mode::VisualLine);
}

#[test]
fn command_mode_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Command);
    assert_eq!(state.mode, Mode::Command);
}

#[test]
fn replace_mode_transition() {
    let mut state = ModeState::new();
    state.set_mode(Mode::Replace);
    assert_eq!(state.mode, Mode::Replace);
}
