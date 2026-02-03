//! Visual mode key handling.

use kjxlkj_core_types::{CursorDirection, Intent, Key, KeyCode, Mode};

use crate::ModeState;

/// Handle a key in visual mode.
pub fn handle_visual_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    let mut intents = Vec::new();

    match key.code {
        KeyCode::Esc => {
            intents.push(Intent::ClearSelection);
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        // Movement extends selection
        KeyCode::Char('h') | KeyCode::Left => {
            intents.push(Intent::ExtendSelection(CursorDirection::Left));
        }
        KeyCode::Char('j') | KeyCode::Down => {
            intents.push(Intent::ExtendSelection(CursorDirection::Down));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            intents.push(Intent::ExtendSelection(CursorDirection::Up));
        }
        KeyCode::Char('l') | KeyCode::Right => {
            intents.push(Intent::ExtendSelection(CursorDirection::Right));
        }

        // Word motions
        KeyCode::Char('w') => {
            intents.push(Intent::ExtendSelection(CursorDirection::WordForward));
        }
        KeyCode::Char('b') => {
            intents.push(Intent::ExtendSelection(CursorDirection::WordBackward));
        }
        KeyCode::Char('e') => {
            intents.push(Intent::ExtendSelection(CursorDirection::WordEndForward));
        }

        // Operators on selection
        KeyCode::Char('d') | KeyCode::Char('x') => {
            // Delete selection
            intents.push(Intent::DeleteRange(Default::default())); // Range filled by core
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }
        KeyCode::Char('y') => {
            // Yank selection
            intents.push(Intent::YankRange(Default::default()));
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        // Mode switching
        KeyCode::Char('v') => {
            if state.mode == Mode::Visual {
                intents.push(Intent::ClearSelection);
                intents.push(Intent::ExitToNormal);
                state.exit_to_normal();
            } else {
                state.mode = Mode::Visual;
            }
        }
        KeyCode::Char('V') => {
            if state.mode == Mode::VisualLine {
                intents.push(Intent::ClearSelection);
                intents.push(Intent::ExitToNormal);
                state.exit_to_normal();
            } else {
                state.mode = Mode::VisualLine;
            }
        }

        _ => {}
    }

    intents
}
