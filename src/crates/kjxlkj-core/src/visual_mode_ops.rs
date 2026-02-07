//! Visual mode helpers: cursor movement and selection range computation.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Motion;

/// Move cursor in visual mode (extending the selection).
pub fn move_visual_cursor(state: &mut EditorState, motion: Motion, count: usize) {
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
            Motion::LineEnd => { /* handled by clamp */ }
            Motion::FileEnd => {
                let max = state.active_buffer().line_count().saturating_sub(1);
                state.active_window_mut().cursor.line = max;
            }
            _ => {}
        }
    }
    // Update visual selection cursor
    let cursor_pos = state.active_window().cursor;
    if let Some(ref mut sel) = state.visual {
        sel.cursor = cursor_pos;
    }
}

/// Compute the range of the current visual selection.
pub fn visual_range(state: &EditorState) -> kjxlkj_core_types::Range {
    if let Some(ref sel) = state.visual {
        kjxlkj_core_types::Range::new(sel.anchor, sel.cursor).normalized()
    } else {
        let pos = state.active_window().cursor;
        kjxlkj_core_types::Range::new(pos, pos)
    }
}
