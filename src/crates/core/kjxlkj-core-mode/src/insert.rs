//! Insert mode handler.

use crate::{HandleResult, ModeAction, ModeState};
use kjxlkj_core_types::{Key, KeyEvent, Modifiers, SpecialKey};

/// Dispatch in insert mode.
pub fn dispatch_insert(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        (Key::Special(SpecialKey::Escape), _) => {
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        (Key::Char('o'), Modifiers { ctrl: true, .. }) => {
            state.enter_insert_normal();
            HandleResult::Consumed(vec![])
        }
        (Key::Char(c), Modifiers { ctrl: false, alt: false, .. }) => {
            HandleResult::Consumed(vec![ModeAction::InsertText(c.to_string())])
        }
        (Key::Special(SpecialKey::Enter), _) => {
            HandleResult::Consumed(vec![ModeAction::InsertText("\n".to_string())])
        }
        (Key::Special(SpecialKey::Tab), _) => {
            HandleResult::Consumed(vec![ModeAction::InsertText("\t".to_string())])
        }
        (Key::Special(SpecialKey::Backspace), _) => {
            HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(
                kjxlkj_core_edit::Direction::Backward,
            )])
        }
        (Key::Special(SpecialKey::Delete), _) => {
            HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(
                kjxlkj_core_edit::Direction::Forward,
            )])
        }
        _ => HandleResult::Ignored,
    }
}
