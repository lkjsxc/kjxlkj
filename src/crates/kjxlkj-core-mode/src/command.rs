//! Command mode key dispatch.

use kjxlkj_core_types::{Key, KeyAction, KeyCode, KeyModifiers, Mode};

use crate::dispatch::DispatchResult;

/// Handle a key in Command mode (Ex or Search).
pub fn dispatch_command(key: &Key) -> DispatchResult {
    match &key.code {
        KeyCode::Esc => DispatchResult::ModeChange(Mode::Normal),
        KeyCode::Enter => {
            // Command execution handled by core state
            DispatchResult::Action(KeyAction::Noop)
        }
        KeyCode::Char(c) => {
            if key.modifiers.is_empty() || key.modifiers.contains(KeyModifiers::SHIFT) {
                DispatchResult::Action(KeyAction::InsertChar(*c))
            } else {
                DispatchResult::Noop
            }
        }
        KeyCode::Backspace => DispatchResult::Action(KeyAction::DeleteCharBackward),
        _ => DispatchResult::Noop,
    }
}
