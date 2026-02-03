//! Keymap definitions.

use kjxlkj_core_types::{Intent, Key, Mode};

use crate::{
    handle_command_key, handle_insert_key, handle_normal_key, handle_replace_key,
    handle_visual_key, ModeState,
};

/// Process a key event and return intents.
pub fn process_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    match state.mode {
        Mode::Normal => handle_normal_key(state, key),
        Mode::Insert => handle_insert_key(state, key),
        Mode::Replace => handle_replace_key(state, key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => handle_visual_key(state, key),
        Mode::Command => handle_command_key(state, key),
    }
}
