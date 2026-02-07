//! Normal mode: supplementary key handlers (short operators, macros, etc.)

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{
    Direction, EditorAction, KeyCode, KeyEvent, Mode, Motion, Operator, Position,
};

use crate::normal_mode::first_non_blank_col;

/// Handle keys not covered by the primary normal-mode dispatch.
pub fn handle_remaining(state: &mut EditorState, key: &KeyEvent) -> Option<EditorAction> {
    match &key.code {
        KeyCode::Char('D') => Some(EditorAction::DeleteRange(to_eol_range(state))),
        KeyCode::Char('C') => {
            let _ = to_eol_range(state);
            Some(EditorAction::ChangeMode(Mode::Insert))
        }
        KeyCode::Char('S') | KeyCode::Char('s') => Some(EditorAction::ChangeMode(Mode::Insert)),
        KeyCode::Char('x') | KeyCode::Delete => Some(EditorAction::DeleteChar),
        KeyCode::Char('X') => Some(EditorAction::DeleteBack),
        KeyCode::Char('p') => Some(EditorAction::Paste(Direction::Forward)),
        KeyCode::Char('P') => Some(EditorAction::Paste(Direction::Backward)),
        KeyCode::Char('u') => Some(EditorAction::Undo),
        KeyCode::Char('.') => Some(EditorAction::Repeat),
        KeyCode::Char('J') => Some(EditorAction::JoinLine(false)),
        KeyCode::Char('~') => Some(EditorAction::ToggleCase),
        KeyCode::Char('r') => None,
        KeyCode::Char('q') => handle_macro_toggle(state),
        KeyCode::Char('@') => None,
        KeyCode::Char('m') => None,
        KeyCode::Char('`') | KeyCode::Char('\'') => None,
        KeyCode::Char('"') => None,
        KeyCode::Char('f') | KeyCode::Char('F') | KeyCode::Char('t') | KeyCode::Char('T') => {
            None
        }
        KeyCode::Char(';') => Some(EditorAction::Noop),
        KeyCode::Char(',') => Some(EditorAction::Noop),
        KeyCode::Char('+') | KeyCode::Enter => {
            state.active_window_mut().cursor.line += 1;
            let line = state.active_window().cursor.line;
            let fnb = first_non_blank_col(state, line);
            state.active_window_mut().cursor.col = fnb;
            None
        }
        KeyCode::Char('-') => {
            let cur = state.active_window().cursor.line;
            state.active_window_mut().cursor.line = cur.saturating_sub(1);
            let line = state.active_window().cursor.line;
            let fnb = first_non_blank_col(state, line);
            state.active_window_mut().cursor.col = fnb;
            None
        }
        KeyCode::Char(' ') => None,
        KeyCode::Char('Z') => None,
        _ => None,
    }
}

fn handle_macro_toggle(state: &mut EditorState) -> Option<EditorAction> {
    if state.macro_state.is_recording() {
        state.macro_state.stop_recording();
        state.set_message("recording stopped");
    }
    None
}

fn handle_open_below(state: &mut EditorState) -> Option<EditorAction> {
    let line = state.active_window().cursor.line;
    let len = state.active_buffer().line_len(line);
    let pos = Position::new(line, len);
    state.active_buffer_mut().insert_char(pos, '\n');
    state.active_window_mut().cursor = Position::new(line + 1, 0);
    Some(EditorAction::ChangeMode(Mode::Insert))
}

fn handle_open_above(state: &mut EditorState) -> Option<EditorAction> {
    let line = state.active_window().cursor.line;
    let pos = Position::new(line, 0);
    state.active_buffer_mut().insert_char(pos, '\n');
    state.active_window_mut().cursor = Position::new(line, 0);
    Some(EditorAction::ChangeMode(Mode::Insert))
}

pub fn current_line_range(state: &EditorState) -> kjxlkj_core_types::Range {
    let line = state.active_window().cursor.line;
    kjxlkj_core_types::Range::new(
        Position::new(line, 0),
        Position::new(line + 1, 0),
    )
}

pub fn to_eol_range(state: &EditorState) -> kjxlkj_core_types::Range {
    let pos = state.active_window().cursor;
    let len = state.active_buffer().line_len(pos.line);
    kjxlkj_core_types::Range::new(pos, Position::new(pos.line, len))
}

/// Open a new line below cursor and enter insert mode.
pub fn open_below(state: &mut EditorState) -> Option<EditorAction> {
    handle_open_below(state)
}

/// Open a new line above cursor and enter insert mode.
pub fn open_above(state: &mut EditorState) -> Option<EditorAction> {
    handle_open_above(state)
}

pub fn apply_motion(state: &mut EditorState, motion: &Motion, count: usize) {
    for _ in 0..count {
        match motion {
            Motion::Left => {
                let c = state.active_window().cursor.col;
                state.active_window_mut().cursor.col = c.saturating_sub(1);
            }
            Motion::Right => state.active_window_mut().cursor.col += 1,
            Motion::Up => {
                let l = state.active_window().cursor.line;
                state.active_window_mut().cursor.line = l.saturating_sub(1);
            }
            Motion::Down => state.active_window_mut().cursor.line += 1,
            Motion::LineStart => state.active_window_mut().cursor.col = 0,
            Motion::LineEnd => {}
            Motion::FileStart => {
                state.active_window_mut().cursor.line = 0;
                state.active_window_mut().cursor.col = 0;
            }
            Motion::FileEnd => {
                let max = state.active_buffer().line_count().saturating_sub(1);
                state.active_window_mut().cursor.line = max;
                return;
            }
            Motion::GoToLine(n) => {
                state.active_window_mut().cursor.line = *n;
                return;
            }
            _ => {}
        }
    }
}

pub fn apply_operator_motion(
    _state: &mut EditorState, _op: Operator, _motion: Motion, _count: usize,
) {}

pub fn handle_operator(state: &mut EditorState, op: Operator) -> Option<EditorAction> {
    if state.mode.pending_operator() == Some(&op) {
        state.mode.clear_pending_operator();
        return match op {
            Operator::Delete => Some(EditorAction::DeleteRange(current_line_range(state))),
            _ => None,
        };
    }
    state.mode.set_pending_operator(op);
    None
}

#[cfg(test)]
#[path = "normal_mode_ops_tests.rs"]
mod tests;
