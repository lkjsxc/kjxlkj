//! Normal mode scroll commands: Ctrl-d/u/f/b/e/y and z-prefixed scrolling.

use kjxlkj_core_state::viewport;
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{EditorAction, KeyCode, KeyEvent, Modifiers};

/// Handle Ctrl-key scroll commands from normal mode.
pub fn handle_scroll_key(
    state: &mut EditorState,
    key: &KeyEvent,
) -> Option<EditorAction> {
    if !key.modifiers.contains(Modifiers::CTRL) {
        return None;
    }
    let half = (state.active_window().height / 2).max(1) as i64;
    let full = state.active_window().height.max(1) as i64;

    match &key.code {
        KeyCode::Char('d') => {
            scroll_and_move(state, half);
            None
        }
        KeyCode::Char('u') => {
            scroll_and_move(state, -half);
            None
        }
        KeyCode::Char('f') | KeyCode::PageDown => {
            scroll_and_move(state, full);
            None
        }
        KeyCode::Char('b') | KeyCode::PageUp => {
            scroll_and_move(state, -full);
            None
        }
        KeyCode::Char('e') => {
            scroll_lines(state, 1);
            None
        }
        KeyCode::Char('y') => {
            scroll_lines(state, -1);
            None
        }
        KeyCode::Char('v') => {
            // Ctrl-v: visual block
            Some(EditorAction::ChangeMode(kjxlkj_core_types::Mode::VisualBlock))
        }
        _ => None,
    }
}

/// Handle z-prefixed scroll positioning from normal mode.
pub fn handle_z_scroll(
    state: &mut EditorState,
    key: &KeyEvent,
) -> Option<EditorAction> {
    let cursor_line = state.active_window().cursor.line;
    match &key.code {
        KeyCode::Char('z') => {
            viewport::center_on_line(&mut state.viewport, cursor_line);
            None
        }
        KeyCode::Char('t') => {
            viewport::cursor_to_top(&mut state.viewport, cursor_line);
            None
        }
        KeyCode::Char('b') => {
            viewport::cursor_to_bottom(&mut state.viewport, cursor_line);
            None
        }
        KeyCode::Char('.') => {
            viewport::center_on_line(&mut state.viewport, cursor_line);
            let fnb = crate::normal_mode::first_non_blank_col(state, cursor_line);
            state.active_window_mut().cursor.col = fnb;
            None
        }
        KeyCode::Char('-') => {
            viewport::cursor_to_bottom(&mut state.viewport, cursor_line);
            let fnb = crate::normal_mode::first_non_blank_col(state, cursor_line);
            state.active_window_mut().cursor.col = fnb;
            None
        }
        KeyCode::Enter => {
            viewport::cursor_to_top(&mut state.viewport, cursor_line);
            let fnb = crate::normal_mode::first_non_blank_col(state, cursor_line);
            state.active_window_mut().cursor.col = fnb;
            None
        }
        _ => None,
    }
}

/// Scroll the viewport and move the cursor by `delta` lines.
fn scroll_and_move(state: &mut EditorState, delta: i64) {
    let max = state.active_buffer().line_count().saturating_sub(1);
    viewport::scroll(&mut state.viewport, delta, max);
    let new_line = if delta > 0 {
        state
            .active_window()
            .cursor
            .line
            .saturating_add(delta as usize)
            .min(max)
    } else {
        state
            .active_window()
            .cursor
            .line
            .saturating_sub((-delta) as usize)
    };
    state.active_window_mut().cursor.line = new_line;
}

/// Scroll viewport only (cursor stays put, clamped to visible area).
fn scroll_lines(state: &mut EditorState, delta: i64) {
    let max = state.active_buffer().line_count().saturating_sub(1);
    viewport::scroll(&mut state.viewport, delta, max);
    state.active_window_mut().ensure_cursor_visible();
}

#[cfg(test)]
#[path = "scroll_tests.rs"]
mod tests;
