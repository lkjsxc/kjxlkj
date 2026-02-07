//! Action dispatch logic for the core processing loop.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Direction, EditorAction, Position};

/// Apply an EditorAction to editor state.
pub(crate) fn dispatch_action(state: &mut EditorState, action: EditorAction) {
    match action {
        EditorAction::CursorMove(dir, n) => move_cursor(state, dir, n),
        EditorAction::CursorTo(pos) => {
            state.active_window_mut().cursor = pos;
            clamp_and_follow(state);
        }
        EditorAction::InsertChar(ch) => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, ch);
            state.active_window_mut().cursor.col += 1;
        }
        EditorAction::InsertNewline => {
            let pos = state.active_window().cursor;
            state.active_buffer_mut().insert_char(pos, '\n');
            state.active_window_mut().cursor.line += 1;
            state.active_window_mut().cursor.col = 0;
        }
        EditorAction::DeleteChar => {
            let pos = state.active_window().cursor;
            let end = Position::new(pos.line, pos.col + 1);
            state.active_buffer_mut().delete_range(pos, end);
        }
        EditorAction::DeleteBack => {
            let pos = state.active_window().cursor;
            if pos.col > 0 {
                let start = Position::new(pos.line, pos.col - 1);
                state.active_buffer_mut().delete_range(start, pos);
                state.active_window_mut().cursor.col -= 1;
            }
        }
        EditorAction::ChangeMode(m) => state.mode.transition(m),
        EditorAction::Quit => state.should_quit = true,
        EditorAction::ForceQuit => state.should_quit = true,
        EditorAction::Undo | EditorAction::Redo => { /* delegated to undo system */ }
        EditorAction::Scroll(delta) => {
            let max = state.active_buffer().line_count().saturating_sub(1);
            kjxlkj_core_state::viewport::scroll(&mut state.viewport, delta, max);
        }
        EditorAction::Noop => {}
        _ => { /* remaining actions handled inline by mode handlers */ }
    }
    clamp_and_follow(state);
}

fn move_cursor(state: &mut EditorState, dir: Direction, n: usize) {
    let win = state.active_window_mut();
    match dir {
        Direction::Forward => win.cursor.col = win.cursor.col.saturating_add(n),
        Direction::Backward => win.cursor.col = win.cursor.col.saturating_sub(n),
    }
}

pub(crate) fn clamp_and_follow(state: &mut EditorState) {
    let max_line = state.active_buffer().line_count().saturating_sub(1);
    state.active_window_mut().cursor.line = state.active_window().cursor.line.min(max_line);
    let cur_line = state.active_window().cursor.line;
    let line_len = state.active_buffer().line_len(cur_line);
    let max_col = if line_len > 0 {
        line_len.saturating_sub(1)
    } else {
        0
    };
    state.active_window_mut().cursor.col = state.active_window().cursor.col.min(max_col);
    state.active_window_mut().ensure_cursor_visible();
}
