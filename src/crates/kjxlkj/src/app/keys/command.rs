//! Command mode key processing.

use kjxlkj_core::{EditorState, Mode};
use kjxlkj_input::{Key, KeyCode};

/// Process a key in command mode.
pub fn process_command_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            state.mode_state.command_line.clear();
            state.set_mode(Mode::Normal);
        }
        KeyCode::Enter => {
            let cmd = state.mode_state.command_line.clone();
            state.mode_state.command_line.clear();
            state.set_mode(Mode::Normal);
            crate::app::execute_command(state, &cmd);
        }
        KeyCode::Backspace => {
            state.mode_state.command_line.pop();
            if state.mode_state.command_line.is_empty() {
                state.set_mode(Mode::Normal);
            }
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            state.mode_state.command_line.push(c);
        }
        _ => {}
    }
}
