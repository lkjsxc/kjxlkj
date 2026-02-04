//! Insert mode key processing.

use kjxlkj_core::{EditorState, Intent, Mode, Position};
use kjxlkj_input::{Key, KeyCode};

use crate::app::apply_intent;

/// Process a key in insert mode.
pub fn process_insert_key(state: &mut EditorState, key: Key) {
    match key.code {
        KeyCode::Escape => {
            if state.cursor.col() > 0 {
                state.cursor.position.col -= 1;
            }
            state.set_mode(Mode::Normal);
        }
        KeyCode::Backspace | KeyCode::Char('h') if key.mods.ctrl => {
            process_insert_backspace(state);
        }
        KeyCode::Enter | KeyCode::Char('j') if key.mods.ctrl => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, "\n");
            state.cursor.position = Position::new(pos.line + 1, 0);
        }
        KeyCode::Char(c) if !key.mods.ctrl => {
            let pos = state.cursor.position;
            state.buffer.insert(pos, &c.to_string());
            state.cursor.position.col += 1;
        }
        KeyCode::Char('w') if key.mods.ctrl => {
            apply_intent(state, Intent::DeleteWordBefore);
        }
        KeyCode::Char('u') if key.mods.ctrl => {
            apply_intent(state, Intent::DeleteToLineStart);
        }
        _ => {}
    }
}

fn process_insert_backspace(state: &mut EditorState) {
    if state.cursor.col() > 0 {
        let pos = state.cursor.position;
        let prev_pos = Position::new(pos.line, pos.col - 1);
        state.buffer.delete_range(prev_pos, pos);
        state.cursor.position = prev_pos;
    } else if state.cursor.line() > 0 {
        let prev_line = state.cursor.line() - 1;
        let prev_len = state.buffer.line_len(prev_line);
        let line_start = Position::new(state.cursor.line(), 0);
        let prev_end = Position::new(prev_line, prev_len);
        state.buffer.delete_range(prev_end, line_start);
        state.cursor.position = prev_end;
    }
}
