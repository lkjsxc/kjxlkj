//! Editing dispatch: open line, join, replace, toggle case, indent.

use crate::EditorState;
use kjxlkj_core_types::{Mode, Position, Range};

pub(crate) fn dispatch_open_line(
    state: &mut EditorState,
    below: bool,
) {
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
    let is_visual = state.mode.current().is_visual();
    let (line, join_count) = if is_visual {
        if let Some(r) = state.visual_range() {
            (r.start.line, r.end.line - r.start.line)
        } else {
            let win = state.windows.get(&wid).unwrap();
            (win.cursor_line, count)
        }
    } else {
        let win = state.windows.get(&wid).unwrap();
        (win.cursor_line, count)
    };
    let bid = state.windows.get(&wid).unwrap().buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..join_count {
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
    if is_visual {
        if let Some(w) = state.windows.get_mut(&wid) { w.visual_anchor = None; }
        state.mode.transition(Mode::Normal);
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

pub(crate) fn dispatch_replace_insert(state: &mut EditorState, c: char) {
    dispatch_replace_char(state, c);
    if let Some(win) = state.active_window_mut() { win.cursor_col += 1; }
}

fn toggle_char(c: char) -> char {
    if c.is_uppercase() { c.to_lowercase().next().unwrap() } else { c.to_uppercase().next().unwrap() }
}

pub(crate) fn dispatch_toggle_case(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if state.mode.current().is_visual() {
        if let Some(range) = state.visual_range() {
            if let Some(buf) = state.buffers.get_mut(&bid) {
                for ln in range.start.line..=range.end.line {
                    let sc = if ln == range.start.line { range.start.col } else { 0 };
                    let ec = if ln == range.end.line { range.end.col + 1 } else { buf.text.line_len(ln) };
                    let chars: Vec<char> = buf.text.line_to_string(ln).chars().collect();
                    for col in sc..ec.min(chars.len()) {
                        if chars[col].is_alphabetic() {
                            let p = Position::new(ln, col);
                            buf.text.delete_range(Range::new(p, Position::new(ln, col + 1)));
                            buf.text.insert_char(p, toggle_char(chars[col]));
                        }
                    }
                }
                buf.modified = true;
            }
        }
        if let Some(w) = state.windows.get_mut(&wid) { w.visual_anchor = None; }
        state.mode.transition(Mode::Normal);
    } else {
        let pos = Position::new(win.cursor_line, win.cursor_col);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            if let Some(c) = buf.text.char_at(pos) {
                if c.is_alphabetic() {
                    buf.text.delete_range(Range::new(pos, Position::new(pos.line, pos.col + 1)));
                    buf.text.insert_char(pos, toggle_char(c));
                    buf.modified = true;
                }
            }
        }
        if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col += 1; }
    }
}

pub(crate) fn dispatch_indent(state: &mut EditorState, indent: bool, count: usize) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        for _ in 0..count {
            if indent {
                buf.text.insert_text(Position::new(line, 0), "    ");
            } else {
                let text = buf.text.line_to_string(line);
                let spaces: usize = text.chars().take(4).take_while(|c| *c == ' ').count();
                if spaces > 0 {
                    buf.text.delete_range(Range::new(Position::new(line, 0), Position::new(line, spaces)));
                }
            }
        }
        buf.modified = true;
    }
}
