//! Visual mode handler.

use super::Action;
use kjxlkj_core::edit::*;
use kjxlkj_core::types::{Mode, Position, Range};
use kjxlkj_core::EditorState;
use kjxlkj_input::{Key, KeyCode};

/// Handle visual mode keys.
pub fn handle_visual(state: &mut EditorState, key: Key) -> Action {
    match key.code {
        KeyCode::Esc => {
            state.cursor.clear_selection();
            state.mode.to_normal();
        }
        // Motions extend selection
        KeyCode::Char('h') | KeyCode::Left => {
            state.cursor.pos = move_left(&state.buffer, state.cursor.pos, 1);
        }
        KeyCode::Char('l') | KeyCode::Right => {
            state.cursor.pos = move_right(&state.buffer, state.cursor.pos, 1);
        }
        KeyCode::Char('j') | KeyCode::Down => {
            state.cursor.pos = move_down(&state.buffer, state.cursor.pos, 1);
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.cursor.pos = move_up(&state.buffer, state.cursor.pos, 1);
        }
        KeyCode::Char('w') => {
            state.cursor.pos = move_word_start(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('b') => {
            state.cursor.pos = move_word_back(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('e') => {
            state.cursor.pos = move_word_end(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('0') => {
            state.cursor.pos = move_line_start(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('^') => {
            state.cursor.pos = move_first_non_blank(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('$') => {
            state.cursor.pos = move_line_end(&state.buffer, state.cursor.pos);
        }
        KeyCode::Char('g') => {
            // Handle gg
            state.cursor.pos = move_file_start(&state.buffer);
        }
        KeyCode::Char('G') => {
            state.cursor.pos = move_file_end(&state.buffer);
        }
        // Operators
        KeyCode::Char('d') | KeyCode::Char('x') => {
            if let Some(range) = get_selection_range(state) {
                let deleted = delete_range(
                    &mut state.buffer,
                    range,
                    &mut state.history,
                    state.cursor.pos,
                );
                let linewise = state.mode.mode() == Mode::VisualLine;
                state.yank(deleted, linewise);
                state.cursor.pos = range.normalized().start;
                state.cursor.clear_selection();
                state.mode.to_normal();
                state.modified = true;
            }
        }
        KeyCode::Char('y') => {
            if let Some(range) = get_selection_range(state) {
                let yanked = yank_range(&state.buffer, range);
                let linewise = state.mode.mode() == Mode::VisualLine;
                state.yank(yanked, linewise);
                state.cursor.pos = range.normalized().start;
                state.cursor.clear_selection();
                state.mode.to_normal();
            }
        }
        KeyCode::Char('c') | KeyCode::Char('s') => {
            if let Some(range) = get_selection_range(state) {
                let deleted = delete_range(
                    &mut state.buffer,
                    range,
                    &mut state.history,
                    state.cursor.pos,
                );
                let linewise = state.mode.mode() == Mode::VisualLine;
                state.yank(deleted, linewise);
                state.cursor.pos = range.normalized().start;
                state.cursor.clear_selection();
                state.mode.transition(Mode::Insert);
                state.modified = true;
            }
        }
        KeyCode::Char('>') => {
            if let Some(range) = get_selection_range(state) {
                let range = range.normalized();
                indent_lines(
                    &mut state.buffer,
                    range.start.line,
                    range.end.line,
                    &mut state.history,
                );
                state.cursor.clear_selection();
                state.mode.to_normal();
                state.modified = true;
            }
        }
        KeyCode::Char('<') => {
            if let Some(range) = get_selection_range(state) {
                let range = range.normalized();
                outdent_lines(
                    &mut state.buffer,
                    range.start.line,
                    range.end.line,
                    &mut state.history,
                );
                state.cursor.clear_selection();
                state.mode.to_normal();
                state.modified = true;
            }
        }
        KeyCode::Char('o') => {
            // Swap cursor and anchor
            if let Some(anchor) = state.cursor.anchor {
                state.cursor.anchor = Some(state.cursor.pos);
                state.cursor.pos = anchor;
            }
        }
        _ => {}
    }
    Action::Continue
}

fn get_selection_range(state: &EditorState) -> Option<Range> {
    let anchor = state.cursor.anchor?;
    let pos = state.cursor.pos;

    match state.mode.mode() {
        Mode::Visual => Some(Range::new(anchor, pos)),
        Mode::VisualLine => {
            let (start_line, end_line) = if anchor.line <= pos.line {
                (anchor.line, pos.line)
            } else {
                (pos.line, anchor.line)
            };
            Some(Range::new(
                Position::new(start_line, 0),
                Position::new(end_line, state.buffer.line_len(end_line)),
            ))
        }
        Mode::VisualBlock => {
            // Block selection - for now treat as char selection
            Some(Range::new(anchor, pos))
        }
        _ => None,
    }
}
