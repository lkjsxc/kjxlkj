//! Integration tests for core-mode.

use crate::*;
use kjxlkj_core_types::Mode;

#[test]
fn mode_state_integration() {
    let mut state = ModeState::new();
    assert_eq!(state.mode(), Mode::Normal);

    // Enter insert mode
    let intent = state.process_key(Key::char('i'));
    assert!(matches!(intent.kind, IntentKind::EnterInsert { after: false }));
    assert_eq!(state.mode(), Mode::Insert);

    // Type a character
    let intent = state.process_key(Key::char('x'));
    assert!(matches!(intent.kind, IntentKind::InsertChar('x')));

    // Escape back to normal
    let intent = state.process_key(Key::escape());
    assert!(matches!(intent.kind, IntentKind::ExitToNormal));
    assert_eq!(state.mode(), Mode::Normal);
}

#[test]
fn visual_mode_entry() {
    let mut state = ModeState::new();

    let intent = state.process_key(Key::char('v'));
    assert!(matches!(intent.kind, IntentKind::EnterVisual));
    assert_eq!(state.mode(), Mode::Visual);
}

#[test]
fn command_mode_entry() {
    let mut state = ModeState::new();

    let intent = state.process_key(Key::char(':'));
    assert!(matches!(intent.kind, IntentKind::EnterCommand));
    assert_eq!(state.mode(), Mode::Command);
}
