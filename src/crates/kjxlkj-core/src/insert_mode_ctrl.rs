//! Insert mode Ctrl-key handlers.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{EditorAction, KeyCode, KeyEvent, Mode, Position};

/// Handle Ctrl-key combinations in insert mode.
pub fn handle_insert_ctrl(state: &mut EditorState, key: &KeyEvent) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('w') => {
            let pos = state.active_window().cursor;
            let line = state.active_buffer().line(pos.line).unwrap_or_default();
            let new_col = word_back_pos(&line, pos.col);
            let start = Position::new(pos.line, new_col);
            state.active_buffer_mut().delete_range(start, pos);
            state.active_window_mut().cursor.col = new_col;
            None
        }
        KeyCode::Char('u') => {
            let pos = state.active_window().cursor;
            let start = Position::new(pos.line, 0);
            state.active_buffer_mut().delete_range(start, pos);
            state.active_window_mut().cursor.col = 0;
            None
        }
        KeyCode::Char('t') => {
            let line = state.active_window().cursor.line;
            let pos = Position::new(line, 0);
            let indent = " ".repeat(state.options.shiftwidth);
            state.active_buffer_mut().insert_text(pos, &indent);
            state.active_window_mut().cursor.col += indent.len();
            None
        }
        KeyCode::Char('d') => {
            let line = state.active_window().cursor.line;
            let text = state.active_buffer().line(line).unwrap_or_default();
            let sw = state.options.shiftwidth;
            let spaces: usize = text.chars().take(sw).take_while(|c| *c == ' ').count();
            if spaces > 0 {
                let start = Position::new(line, 0);
                let end = Position::new(line, spaces);
                state.active_buffer_mut().delete_range(start, end);
                state.active_window_mut().cursor.col =
                    state.active_window().cursor.col.saturating_sub(spaces);
            }
            None
        }
        KeyCode::Char('o') => Some(EditorAction::ChangeMode(Mode::Normal)),
        KeyCode::Char('r') => None,
        KeyCode::Char('h') => {
            let pos = state.active_window().cursor;
            if pos.col > 0 {
                let start = Position::new(pos.line, pos.col - 1);
                state.active_buffer_mut().delete_range(start, pos);
                state.active_window_mut().cursor.col -= 1;
            }
            None
        }
        _ => None,
    }
}

/// Find the column position after deleting one word backward.
pub fn word_back_pos(line: &str, col: usize) -> usize {
    let chars: Vec<char> = line.chars().collect();
    let mut i = col;
    while i > 0 && chars.get(i - 1).is_some_and(|c| c.is_whitespace()) {
        i -= 1;
    }
    while i > 0 && chars.get(i - 1).is_some_and(|c| !c.is_whitespace()) {
        i -= 1;
    }
    i
}
