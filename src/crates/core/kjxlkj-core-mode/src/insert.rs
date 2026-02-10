//! Insert mode handler.

use crate::{HandleResult, ModeAction, ModeState};
use kjxlkj_core_types::{Key, KeyEvent, Modifiers, SpecialKey};

/// Dispatch in insert mode.
///
/// IME composition is handled first: while `state.is_composing()` is true,
/// normal character keys are accumulated as preedit text. Only Escape and
/// certain control keys break out of composition. This prevents leader
/// mappings from triggering during IME candidate selection.
pub fn dispatch_insert(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    // IME composition handling: when composing, block leader mappings
    // and route characters to IME preedit accumulation.
    if state.is_composing() {
        return dispatch_ime_composing(state, key);
    }

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

/// Handle key events while IME is composing.
///
/// During IME composition:
/// - Escape cancels composition and returns to normal mode
/// - Enter commits the current preedit text
/// - Backspace removes from preedit
/// - Character keys are accumulated in preedit
/// - Space during composition is treated as IME navigation, not leader trigger
fn dispatch_ime_composing(state: &mut ModeState, key: &KeyEvent) -> HandleResult {
    match (&key.key, &key.modifiers) {
        // Escape: cancel IME and return to normal
        (Key::Special(SpecialKey::Escape), _) => {
            state.ime_cancel();
            state.enter_normal();
            HandleResult::Consumed(vec![ModeAction::ReturnNormal])
        }
        // Enter: commit preedit text
        (Key::Special(SpecialKey::Enter), _) => {
            if let Some(text) = state.ime_commit() {
                HandleResult::Consumed(vec![ModeAction::InsertText(text)])
            } else {
                HandleResult::Consumed(vec![ModeAction::InsertText("\n".to_string())])
            }
        }
        // Backspace: remove from preedit or delete
        (Key::Special(SpecialKey::Backspace), _) => {
            if !state.ime_preedit.is_empty() {
                let mut chars: Vec<char> = state.ime_preedit.chars().collect();
                chars.pop();
                state.ime_preedit = chars.into_iter().collect();
                HandleResult::Consumed(vec![])
            } else {
                // Empty preedit: cancel and delete
                state.ime_cancel();
                HandleResult::Consumed(vec![ModeAction::DeleteAtCursor(
                    kjxlkj_core_edit::Direction::Backward,
                )])
            }
        }
        // Space during composition: consumed by IME, not leader trigger (JP-03)
        (Key::Char(' '), Modifiers { ctrl: false, alt: false, .. }) => {
            // In real IME, space might select a candidate. We just append it.
            state.ime_update(&format!("{} ", state.ime_preedit));
            HandleResult::Consumed(vec![])
        }
        // Character keys: accumulate in preedit
        (Key::Char(c), Modifiers { ctrl: false, alt: false, .. }) => {
            state.ime_update(&format!("{}{}", state.ime_preedit, c));
            HandleResult::Consumed(vec![])
        }
        // Other keys are ignored during composition
        _ => HandleResult::Consumed(vec![]),
    }
}
