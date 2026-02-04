//! Insert mode handler.

use super::Action;
use kjxlkj_core::edit::{indent_lines, insert_text, outdent_lines};
use kjxlkj_core::types::Position;
use kjxlkj_core::EditorState;
use kjxlkj_input::{Key, KeyCode};

/// Handle insert mode keys.
pub fn handle_insert(state: &mut EditorState, key: Key) -> Action {
    match (key.code, key.mods.ctrl) {
        (KeyCode::Esc, _) => {
            // Move cursor back one if possible
            if state.cursor.pos.col > 0 {
                state.cursor.pos.col -= 1;
            }
            state.mode.to_normal();
        }
        (KeyCode::Char(c), false) => {
            insert_char(state, c);
        }
        (KeyCode::Enter, _) | (KeyCode::Char('j'), true) | (KeyCode::Char('m'), true) => {
            insert_newline(state);
        }
        (KeyCode::Backspace, _) | (KeyCode::Char('h'), true) => {
            delete_char_before(state);
        }
        (KeyCode::Char('w'), true) => {
            delete_word_before(state);
        }
        (KeyCode::Char('u'), true) => {
            delete_to_line_start(state);
        }
        (KeyCode::Char('t'), true) => {
            indent_current_line(state);
        }
        (KeyCode::Char('d'), true) => {
            outdent_current_line(state);
        }
        (KeyCode::Left, _) => {
            if state.cursor.pos.col > 0 {
                state.cursor.pos.col -= 1;
            }
        }
        (KeyCode::Right, _) => {
            let line_len = state.buffer.line_len(state.cursor.pos.line);
            if state.cursor.pos.col < line_len {
                state.cursor.pos.col += 1;
            }
        }
        (KeyCode::Up, _) => {
            if state.cursor.pos.line > 0 {
                state.cursor.pos.line -= 1;
            }
        }
        (KeyCode::Down, _) => {
            if state.cursor.pos.line + 1 < state.buffer.line_count() {
                state.cursor.pos.line += 1;
            }
        }
        _ => {}
    }
    Action::Continue
}

/// Handle replace mode keys.
pub fn handle_replace(state: &mut EditorState, key: Key) -> Action {
    match key.code {
        KeyCode::Esc => {
            state.mode.to_normal();
        }
        KeyCode::Char(c) => {
            replace_char(state, c);
        }
        KeyCode::Backspace => {
            if state.cursor.pos.col > 0 {
                state.cursor.pos.col -= 1;
            }
        }
        _ => {}
    }
    Action::Continue
}

fn insert_char(state: &mut EditorState, c: char) {
    let pos = state.cursor.pos;
    insert_text(
        &mut state.buffer,
        pos,
        &c.to_string(),
        &mut state.history,
        pos,
    );
    state.cursor.pos.col += 1;
    state.modified = true;
}

fn insert_newline(state: &mut EditorState) {
    let pos = state.cursor.pos;
    insert_text(&mut state.buffer, pos, "\n", &mut state.history, pos);
    state.cursor.pos.line += 1;
    state.cursor.pos.col = 0;
    state.modified = true;
}

fn delete_char_before(state: &mut EditorState) {
    if state.cursor.pos.col > 0 {
        let start = Position::new(state.cursor.pos.line, state.cursor.pos.col - 1);
        let end = state.cursor.pos;
        state.buffer.delete(start, end);
        state.cursor.pos.col -= 1;
        state.modified = true;
    } else if state.cursor.pos.line > 0 {
        // Join with previous line
        let prev_line = state.cursor.pos.line - 1;
        let prev_len = state.buffer.line_len(prev_line);
        let start = Position::new(prev_line, prev_len);
        let end = Position::new(state.cursor.pos.line, 0);
        state.buffer.delete(start, end);
        state.cursor.pos.line = prev_line;
        state.cursor.pos.col = prev_len;
        state.modified = true;
    }
}

fn delete_word_before(state: &mut EditorState) {
    if state.cursor.pos.col == 0 {
        return;
    }
    let line = state.buffer.line(state.cursor.pos.line).unwrap_or_default();
    let chars: Vec<char> = line.chars().collect();
    let mut col = state.cursor.pos.col;

    // Skip whitespace
    while col > 0
        && chars
            .get(col - 1)
            .map(|c| c.is_whitespace())
            .unwrap_or(false)
    {
        col -= 1;
    }
    // Skip word chars
    while col > 0
        && chars
            .get(col - 1)
            .map(|c| c.is_alphanumeric() || *c == '_')
            .unwrap_or(false)
    {
        col -= 1;
    }

    let start = Position::new(state.cursor.pos.line, col);
    let end = state.cursor.pos;
    state.buffer.delete(start, end);
    state.cursor.pos.col = col;
    state.modified = true;
}

fn delete_to_line_start(state: &mut EditorState) {
    if state.cursor.pos.col > 0 {
        let start = Position::new(state.cursor.pos.line, 0);
        let end = state.cursor.pos;
        state.buffer.delete(start, end);
        state.cursor.pos.col = 0;
        state.modified = true;
    }
}

fn indent_current_line(state: &mut EditorState) {
    let line = state.cursor.pos.line;
    indent_lines(&mut state.buffer, line, line, &mut state.history);
    state.cursor.pos.col += 4;
    state.modified = true;
}

fn outdent_current_line(state: &mut EditorState) {
    let line = state.cursor.pos.line;
    outdent_lines(&mut state.buffer, line, line, &mut state.history);
    state.cursor.pos.col = state.cursor.pos.col.saturating_sub(4);
    state.modified = true;
}

fn replace_char(state: &mut EditorState, c: char) {
    let pos = state.cursor.pos;
    let line_len = state.buffer.line_len(pos.line);

    if pos.col < line_len {
        // Replace existing character
        let end = Position::new(pos.line, pos.col + 1);
        state.buffer.delete(pos, end);
        state.buffer.insert(pos, &c.to_string());
        state.cursor.pos.col += 1;
    } else {
        // At end of line, insert
        state.buffer.insert(pos, &c.to_string());
        state.cursor.pos.col += 1;
    }
    state.modified = true;
}
