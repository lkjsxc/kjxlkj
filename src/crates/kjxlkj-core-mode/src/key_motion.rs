//! Key-to-motion mapping.

use kjxlkj_core_types::{Key, KeyCode, KeyModifiers, MotionAction};

/// Convert a key to a motion action.
pub fn key_to_motion(key: &Key) -> Option<MotionAction> {
    if key.modifiers.is_empty() {
        match &key.code {
            KeyCode::Char('h') | KeyCode::Left => Some(MotionAction::Left),
            KeyCode::Char('l') | KeyCode::Right => Some(MotionAction::Right),
            KeyCode::Char('j') | KeyCode::Down => Some(MotionAction::Down),
            KeyCode::Char('k') | KeyCode::Up => Some(MotionAction::Up),
            KeyCode::Char('0') => Some(MotionAction::LineStart),
            KeyCode::Char('$') => Some(MotionAction::LineEnd),
            KeyCode::Char('^') => Some(MotionAction::FirstNonBlank),
            KeyCode::Char('w') => Some(MotionAction::WordForward),
            KeyCode::Char('b') => Some(MotionAction::WordBackward),
            KeyCode::Char('e') => Some(MotionAction::WordEndForward),
            KeyCode::Char('W') => Some(MotionAction::BigWordForward),
            KeyCode::Char('B') => Some(MotionAction::BigWordBackward),
            KeyCode::Char('G') => Some(MotionAction::GoToLastLine),
            _ => None,
        }
    } else if key.modifiers.contains(KeyModifiers::CTRL) {
        match &key.code {
            KeyCode::Char('f') => Some(MotionAction::PageDown),
            KeyCode::Char('b') => Some(MotionAction::PageUp),
            KeyCode::Char('d') => Some(MotionAction::HalfPageDown),
            KeyCode::Char('u') => Some(MotionAction::HalfPageUp),
            _ => None,
        }
    } else {
        None
    }
}
