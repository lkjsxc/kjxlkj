//! Key processing for different modes.

mod normal;
mod insert;
mod visual;
mod command;
mod replace;

use kjxlkj_core::{EditorState, Mode};
use kjxlkj_input::Key;

pub use normal::process_normal_key;
pub use insert::process_insert_key;
pub use visual::process_visual_key;
pub use command::process_command_key;
pub use replace::process_replace_key;

/// Process a key event based on current mode.
pub fn process_key(state: &mut EditorState, key: Key) {
    state.clear_status();

    match state.mode() {
        Mode::Normal => process_normal_key(state, key),
        Mode::Insert => process_insert_key(state, key),
        Mode::Command => process_command_key(state, key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => process_visual_key(state, key),
        Mode::Replace => process_replace_key(state, key),
    }

    state.clamp_cursor();
    state.ensure_cursor_visible();
}
