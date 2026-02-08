//! Tests for VisualModeState.

use crate::visual::VisualModeState;
use kjxlkj_core_types::{
    Action, Key, Motion, VisualKind,
};

#[test]
fn escape_exits_visual() {
    let mut s = VisualModeState::new(
        VisualKind::Char,
        (0, 0),
    );
    let action = s.process_key(&Key::esc());
    assert!(matches!(
        action,
        Some(Action::ReturnToNormal)
    ));
}

#[test]
fn motion_in_visual() {
    let mut s = VisualModeState::new(
        VisualKind::Char,
        (0, 0),
    );
    let action = s.process_key(&Key::char('j'));
    assert!(matches!(
        action,
        Some(Action::MoveCursor(Motion::Down, 1))
    ));
}

#[test]
fn delete_selection() {
    let mut s = VisualModeState::new(
        VisualKind::Line,
        (0, 0),
    );
    let action = s.process_key(&Key::char('d'));
    assert!(matches!(
        action,
        Some(Action::Delete(_, _))
    ));
}
