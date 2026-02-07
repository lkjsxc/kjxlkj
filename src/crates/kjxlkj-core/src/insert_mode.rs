//! Insert mode key handling.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{EditorAction, KeyCode, KeyEvent, Mode, Modifiers, Position};

use crate::insert_mode_ctrl::handle_insert_ctrl;

/// Handle a key event in Insert mode.
pub fn handle_insert_key(state: &mut EditorState, key: KeyEvent) -> Option<EditorAction> {
    if state.macro_state.is_recording() {
        state.macro_state.record_key(key.clone());
    }
    if key.modifiers.contains(Modifiers::CTRL) {
        return handle_insert_ctrl(state, &key);
    }
    match &key.code {
        KeyCode::Escape => {
            let col = state.active_window().cursor.col;
            if col > 0 {
                state.active_window_mut().cursor.col = col - 1;
            }
            Some(EditorAction::ChangeMode(Mode::Normal))
        }
        KeyCode::Char(ch) => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, *ch);
            state.active_window_mut().cursor.col += 1;
            None
        }
        KeyCode::Enter => {
            let pos = state.active_window().cursor;
            let indent = auto_indent_level(state, pos.line);
            state.active_buffer_mut().insert_char(pos, '\n');
            let new_line = pos.line + 1;
            if !indent.is_empty() {
                let ins_pos = Position::new(new_line, 0);
                state.active_buffer_mut().insert_text(ins_pos, &indent);
            }
            state.active_window_mut().cursor = Position::new(new_line, indent.len());
            state.active_window_mut().ensure_cursor_visible();
            None
        }
        KeyCode::Backspace => {
            let pos = state.active_window().cursor;
            if pos.col > 0 {
                let start = Position::new(pos.line, pos.col - 1);
                state.active_buffer_mut().delete_range(start, pos);
                state.active_window_mut().cursor.col -= 1;
            } else if pos.line > 0 {
                let prev_len = state.active_buffer().line_len(pos.line - 1);
                let start = Position::new(pos.line - 1, prev_len);
                let end = Position::new(pos.line, 0);
                state.active_buffer_mut().delete_range(start, end);
                state.active_window_mut().cursor = Position::new(pos.line - 1, prev_len);
            }
            None
        }
        KeyCode::Delete => {
            let pos = state.active_window().cursor;
            let line_len = state.active_buffer().line_len(pos.line);
            if pos.col < line_len {
                let end = Position::new(pos.line, pos.col + 1);
                state.active_buffer_mut().delete_range(pos, end);
            }
            None
        }
        KeyCode::Tab => {
            let pos = state.active_window().cursor;
            let tab_str = if state.options.expandtab {
                " ".repeat(state.options.tabstop)
            } else {
                "\t".to_string()
            };
            state.active_buffer_mut().insert_text(pos, &tab_str);
            state.active_window_mut().cursor.col += tab_str.len();
            None
        }
        KeyCode::Left => {
            let col = state.active_window().cursor.col;
            state.active_window_mut().cursor.col = col.saturating_sub(1);
            None
        }
        KeyCode::Right => {
            state.active_window_mut().cursor.col += 1;
            None
        }
        KeyCode::Up => {
            let line = state.active_window().cursor.line;
            state.active_window_mut().cursor.line = line.saturating_sub(1);
            None
        }
        KeyCode::Down => {
            state.active_window_mut().cursor.line += 1;
            None
        }
        KeyCode::Home => {
            state.active_window_mut().cursor.col = 0;
            None
        }
        KeyCode::End => {
            let line = state.active_window().cursor.line;
            let len = state.active_buffer().line_len(line);
            state.active_window_mut().cursor.col = len;
            None
        }
        _ => None,
    }
}

/// Compute auto-indent: copy leading whitespace from current line.
fn auto_indent_level(state: &EditorState, line: usize) -> String {
    let text = state.active_buffer().line(line).unwrap_or_default();
    text.chars()
        .take_while(|c| c.is_ascii_whitespace())
        .collect()
}

#[cfg(test)]
#[path = "insert_mode_tests.rs"]
mod tests;
