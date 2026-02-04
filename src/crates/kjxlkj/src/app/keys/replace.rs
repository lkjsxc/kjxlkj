//! Replace mode key processing.

use kjxlkj_core::{EditorState, Mode, Position};
use kjxlkj_input::{Key, KeyCode};

/// Process a key in replace mode.
pub fn process_replace_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => state.set_mode(Mode::Normal),
        KeyCode::Backspace => {
            if state.cursor.col() > 0 {
                state.cursor.position.col -= 1;
            }
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            let pos = state.cursor.position;
            let line_len = state.buffer.line_len(pos.line);
            if pos.col < line_len {
                let next_pos = Position::new(pos.line, pos.col + 1);
                state.buffer.delete_range(pos, next_pos);
            }
            state.buffer.insert(pos, &c.to_string());
            state.cursor.position.col += 1;
        }
        _ => {}
    }
}
