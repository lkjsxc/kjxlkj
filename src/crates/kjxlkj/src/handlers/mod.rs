//! Input handlers for all modes.

mod command;
mod insert;
mod normal;
mod visual;

use kjxlkj_core::EditorState;
use kjxlkj_input::Key;
use kjxlkj_services::Services;

/// Action result from handling a key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Continue running.
    Continue,
    /// Quit the editor.
    Quit,
}

/// Handle a key press.
pub fn handle_key(state: &mut EditorState, key: Key, services: &Services) -> Action {
    // Clear status on new input (unless in command mode)
    if state.mode.mode() != kjxlkj_core::Mode::Command {
        state.status.clear();
    }

    match state.mode.mode() {
        kjxlkj_core::Mode::Normal => normal::handle_normal(state, key, services),
        kjxlkj_core::Mode::Insert => insert::handle_insert(state, key),
        kjxlkj_core::Mode::Command => command::handle_command(state, key, services),
        kjxlkj_core::Mode::Visual
        | kjxlkj_core::Mode::VisualLine
        | kjxlkj_core::Mode::VisualBlock => visual::handle_visual(state, key),
        kjxlkj_core::Mode::Replace => insert::handle_replace(state, key),
    }
}
