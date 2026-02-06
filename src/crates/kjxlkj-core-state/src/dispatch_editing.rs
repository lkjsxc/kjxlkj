//! Editing dispatch: insert, delete, enter-insert.

use crate::EditorState;
use kjxlkj_core_types::{InsertPosition, Mode, Position, Range};

pub(crate) fn dispatch_enter_insert(
    state: &mut EditorState,
    pos: InsertPosition,
) {
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            match pos {
                InsertPosition::AfterCursor => {
                    win.cursor_col += 1;
                }
                InsertPosition::EndOfLine => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        let line_len = buf.text.line_len(win.cursor_line);
                        win.cursor_col = line_len;
                    }
                }
                InsertPosition::FirstNonBlank => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        let line = buf.text.line_to_string(win.cursor_line);
                        let col = line
                            .chars()
                            .position(|c| !c.is_whitespace())
                            .unwrap_or(0);
                        win.cursor_col = col;
                    }
                }
                InsertPosition::BeforeCursor => {}
            }
        }
    }
    state.mode.transition(Mode::Insert);
}

pub(crate) fn dispatch_insert_char(state: &mut EditorState, c: char) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get(&wid) {
        Some(w) => w,
        None => return,
    };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_char(pos, c);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col += 1;
    }
}

pub(crate) fn dispatch_insert_newline(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get(&wid) {
        Some(w) => w,
        None => return,
    };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_char(pos, '\n');
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_line += 1;
        win.cursor_col = 0;
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_delete_char_before(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if col == 0 && line == 0 {
        return;
    }
    if col > 0 {
        let range = Range::new(
            Position::new(line, col - 1),
            Position::new(line, col),
        );
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.delete_range(range);
            buf.modified = true;
        }
        state.windows.get_mut(&wid).unwrap().cursor_col -= 1;
    } else {
        // Join with previous line
        if let Some(buf) = state.buffers.get(&bid) {
            let prev_len = buf.text.line_len(line - 1);
            let range = Range::new(
                Position::new(line - 1, prev_len),
                Position::new(line, 0),
            );
            if let Some(buf) = state.buffers.get_mut(&bid) {
                buf.text.delete_range(range);
                buf.modified = true;
            }
            let win = state.windows.get_mut(&wid).unwrap();
            win.cursor_line -= 1;
            win.cursor_col = prev_len;
        }
    }
}

pub(crate) fn dispatch_delete_char_at(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let range = Range::new(
        Position::new(win.cursor_line, win.cursor_col),
        Position::new(win.cursor_line, win.cursor_col + 1),
    );
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(range);
        buf.modified = true;
    }
}

/// Delete word before cursor (Ctrl-w in insert mode).
pub(crate) fn dispatch_delete_word_before(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if col == 0 {
        return;
    }
    let start = if let Some(buf) = state.buffers.get(&bid) {
        let text = buf.text.line_to_string(line);
        let chars: Vec<char> = text.chars().collect();
        let mut s = col;
        // Skip trailing whitespace
        while s > 0 && chars[s - 1].is_whitespace() {
            s -= 1;
        }
        // Skip word chars
        while s > 0 && !chars[s - 1].is_whitespace() {
            s -= 1;
        }
        s
    } else {
        return;
    };
    let range = Range::new(
        Position::new(line, start),
        Position::new(line, col),
    );
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(range);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col = start;
    }
}

/// Delete to beginning of line (Ctrl-u in insert mode).
pub(crate) fn dispatch_delete_to_line_start(
    state: &mut EditorState,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if col == 0 {
        return;
    }
    let range = Range::new(
        Position::new(line, 0),
        Position::new(line, col),
    );
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(range);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col = 0;
    }
}

/// Insert contents of a register at cursor (Ctrl-r {reg}).
pub(crate) fn dispatch_insert_from_register(
    state: &mut EditorState,
    reg: char,
) {
    use kjxlkj_core_types::RegisterName;
    let name = match RegisterName::from_char(reg) {
        Some(n) => n,
        None => return,
    };
    let text = match state.registers.get(name) {
        Some(entry) => entry.text.clone(),
        None => return,
    };
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_text(pos, &text);
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col += text.len();
    }
}
