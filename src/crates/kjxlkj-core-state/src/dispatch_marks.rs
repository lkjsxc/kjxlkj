//! Mark dispatch: set marks, jump to mark (exact/line).

use crate::EditorState;
use kjxlkj_core_types::Position;

/// Set a mark at current cursor position.
pub(crate) fn dispatch_set_mark(state: &mut EditorState, c: char) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    state.marks.insert(c, (bid, pos));
}

/// Jump to mark (exact position with `).
pub(crate) fn dispatch_jump_to_mark(state: &mut EditorState, c: char) {
    if let Some((_bid, pos)) = state.marks.get(&c) {
        let pos = *pos;
        if let Some(win) = state.active_window_mut() {
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    } else {
        state.message = Some(format!("Mark '{}' not set", c));
    }
}

/// Jump to mark line (first non-blank with ').
pub(crate) fn dispatch_jump_to_mark_line(state: &mut EditorState, c: char) {
    if let Some((_bid, pos)) = state.marks.get(&c) {
        let line = pos.line;
        if let Some(wid) = state.active_window {
            if let Some(win) = state.windows.get(&wid) {
                let bid = win.buffer_id;
                if let Some(buf) = state.buffers.get(&bid) {
                    let text = buf.text.line_to_string(line);
                    let col = text.chars().position(|ch| !ch.is_whitespace()).unwrap_or(0);
                    let win = state.windows.get_mut(&wid).unwrap();
                    win.set_cursor(Position::new(line, col));
                    win.ensure_cursor_visible();
                }
            }
        }
    } else {
        state.message = Some(format!("Mark '{}' not set", c));
    }
}
