//! Editing dispatch: delete operations in insert mode.

use crate::EditorState;
use kjxlkj_core_types::{Position, Range};

pub(crate) fn dispatch_delete_char_before(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (line, col) = (win.cursor_line, win.cursor_col);
    if col == 0 && line == 0 { return; }
    if col > 0 {
        let range = Range::new(Position::new(line, col - 1), Position::new(line, col));
        if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.delete_range(range); buf.modified = true; }
        state.windows.get_mut(&wid).unwrap().cursor_col -= 1;
    } else {
        if let Some(buf) = state.buffers.get(&bid) {
            let prev_len = buf.text.line_len(line - 1);
            let range = Range::new(Position::new(line - 1, prev_len), Position::new(line, 0));
            if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.delete_range(range); buf.modified = true; }
            let win = state.windows.get_mut(&wid).unwrap();
            win.cursor_line -= 1;
            win.cursor_col = prev_len;
        }
    }
}

pub(crate) fn dispatch_delete_char_at(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let range = Range::new(
        Position::new(win.cursor_line, win.cursor_col),
        Position::new(win.cursor_line, win.cursor_col + 1),
    );
    if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.delete_range(range); buf.modified = true; }
}

/// Delete word before cursor (Ctrl-w in insert mode).
pub(crate) fn dispatch_delete_word_before(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (line, col) = (win.cursor_line, win.cursor_col);
    if col == 0 { return; }
    let start = if let Some(buf) = state.buffers.get(&bid) {
        let chars: Vec<char> = buf.text.line_to_string(line).chars().collect();
        let mut s = col;
        while s > 0 && chars[s - 1].is_whitespace() { s -= 1; }
        while s > 0 && !chars[s - 1].is_whitespace() { s -= 1; }
        s
    } else { return };
    let range = Range::new(Position::new(line, start), Position::new(line, col));
    if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.delete_range(range); buf.modified = true; }
    if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col = start; }
}

/// Delete to beginning of line (Ctrl-u in insert mode).
pub(crate) fn dispatch_delete_to_line_start(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let (line, col) = (win.cursor_line, win.cursor_col);
    if col == 0 { return; }
    let range = Range::new(Position::new(line, 0), Position::new(line, col));
    if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.delete_range(range); buf.modified = true; }
    if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col = 0; }
}
