//! Insert mode key handling.

use kjxlkj_core_types::{Intent, Key, KeyCode};

use crate::ModeState;

/// Handle a key in insert mode.
pub fn handle_insert_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    let mut intents = Vec::new();

    match key.code {
        KeyCode::Esc => {
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        KeyCode::Char(c) => {
            intents.push(Intent::InsertText(c.to_string()));
        }

        KeyCode::Enter => {
            intents.push(Intent::InsertNewline);
        }

        KeyCode::Backspace => {
            intents.push(Intent::DeleteBackward);
        }

        KeyCode::Delete => {
            intents.push(Intent::DeleteForward);
        }

        KeyCode::Tab => {
            // Insert spaces for tab (default 4)
            intents.push(Intent::InsertText("    ".to_string()));
        }

        KeyCode::Left => {
            intents.push(Intent::CursorMove(
                kjxlkj_core_types::CursorDirection::Left,
            ));
        }

        KeyCode::Right => {
            intents.push(Intent::CursorMove(
                kjxlkj_core_types::CursorDirection::Right,
            ));
        }

        KeyCode::Up => {
            intents.push(Intent::CursorMove(kjxlkj_core_types::CursorDirection::Up));
        }

        KeyCode::Down => {
            intents.push(Intent::CursorMove(
                kjxlkj_core_types::CursorDirection::Down,
            ));
        }

        KeyCode::Home => {
            intents.push(Intent::CursorLineStart);
        }

        KeyCode::End => {
            intents.push(Intent::CursorLineEnd);
        }

        _ => {}
    }

    intents
}

/// Handle a key in replace mode.
pub fn handle_replace_key(state: &mut ModeState, key: Key) -> Vec<Intent> {
    let mut intents = Vec::new();

    match key.code {
        KeyCode::Esc => {
            intents.push(Intent::ExitToNormal);
            state.exit_to_normal();
        }

        KeyCode::Char(c) => {
            // Delete char under cursor and insert new one
            intents.push(Intent::DeleteChar);
            intents.push(Intent::InsertText(c.to_string()));
        }

        KeyCode::Backspace => {
            intents.push(Intent::DeleteBackward);
        }

        _ => {
            // Fall back to insert behavior for other keys
            return handle_insert_key(state, key);
        }
    }

    intents
}
