//! Editing dispatch: insert, delete, open, join, replace, toggle, indent.

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

pub(crate) fn dispatch_open_line(state: &mut EditorState, below: bool) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if below {
        if let Some(buf) = state.buffers.get_mut(&bid) {
            let insert_line = line + 1;
            let pos = if insert_line >= buf.text.line_count() {
                let last = buf.text.line_count() - 1;
                let len = buf.text.line_len(last);
                Position::new(last, len)
            } else {
                Position::new(insert_line, 0)
            };
            buf.text.insert_text(pos, "\n");
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_line = line + 1;
        win.cursor_col = 0;
    } else {
        let pos = Position::new(line, 0);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.insert_text(pos, "\n");
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_col = 0;
    }
    state.mode.transition(Mode::Insert);
}

pub(crate) fn dispatch_join_lines(
    state: &mut EditorState,
    spaces: bool,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..count {
            if line + 1 >= buf.text.line_count() {
                break;
            }
            let curr_len = buf.text.line_len(line);
            let r = Range::new(
                Position::new(line, curr_len),
                Position::new(line + 1, 0),
            );
            buf.text.delete_range(r);
            let joined = buf.text.line_to_string(line);
            let trimmed_start = joined[curr_len..]
                .chars()
                .take_while(|c| c.is_whitespace())
                .count();
            if trimmed_start > 0 {
                buf.text.delete_range(Range::new(
                    Position::new(line, curr_len),
                    Position::new(line, curr_len + trimmed_start),
                ));
            }
            if spaces && curr_len > 0 {
                buf.text
                    .insert_char(Position::new(line, curr_len), ' ');
            }
            buf.modified = true;
        }
    }
}

pub(crate) fn dispatch_replace_char(
    state: &mut EditorState,
    c: char,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    let end = Position::new(win.cursor_line, win.cursor_col + 1);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.delete_range(Range::new(pos, end));
        buf.text.insert_char(pos, c);
        buf.modified = true;
    }
}

pub(crate) fn dispatch_replace_insert(
    state: &mut EditorState,
    c: char,
) {
    dispatch_replace_char(state, c);
    if let Some(win) = state.active_window_mut() {
        win.cursor_col += 1;
    }
}

pub(crate) fn dispatch_toggle_case(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        if let Some(c) = buf.text.char_at(pos) {
            if c.is_alphabetic() {
                let toggled: char = if c.is_uppercase() {
                    c.to_lowercase().next().unwrap()
                } else {
                    c.to_uppercase().next().unwrap()
                };
                let end = Position::new(
                    win.cursor_line,
                    win.cursor_col + 1,
                );
                buf.text.delete_range(Range::new(pos, end));
                buf.text.insert_char(pos, toggled);
                buf.modified = true;
            }
        }
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_col += 1;
    }
}

pub(crate) fn dispatch_indent(
    state: &mut EditorState,
    indent: bool,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..count {
            if indent {
                let pos = Position::new(line, 0);
                buf.text.insert_text(pos, "    ");
            } else {
                let text = buf.text.line_to_string(line);
                let spaces: usize =
                    text.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    buf.text.delete_range(Range::new(
                        Position::new(line, 0),
                        Position::new(line, spaces),
                    ));
                }
            }
        }
        buf.modified = true;
    }
}
