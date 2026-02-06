//! Insert-mode dispatch: enter insert, insert char/newline, insert from register.

use crate::EditorState;
use kjxlkj_core_types::{InsertPosition, Mode, Position};

pub(crate) fn dispatch_enter_insert(state: &mut EditorState, pos: InsertPosition) {
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            match pos {
                InsertPosition::AfterCursor => { win.cursor_col += 1; }
                InsertPosition::EndOfLine => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        win.cursor_col = buf.text.line_len(win.cursor_line);
                    }
                }
                InsertPosition::FirstNonBlank => {
                    if let Some(buf) = state.buffers.get(&win.buffer_id) {
                        let line = buf.text.line_to_string(win.cursor_line);
                        win.cursor_col = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
                    }
                }
                InsertPosition::BeforeCursor => {}
            }
        }
    }
    state.mode.transition(Mode::Insert);
}

pub(crate) fn dispatch_insert_char(state: &mut EditorState, c: char) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = match state.windows.get(&wid) { Some(w) => w, None => return };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    // Autopair: skip over closing bracket if it matches
    if state.options.autopairs {
        if let Some(buf) = state.buffers.get(&bid) {
            let line = buf.text.line_to_string(win.cursor_line);
            let next_char = line.chars().nth(win.cursor_col);
            if is_close_pair(c) && next_char == Some(c) {
                if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col += 1; }
                return;
            }
        }
    }
    if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.insert_char(pos, c); buf.modified = true; }
    if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col += 1; }
    // Autopair: insert matching close after open
    if state.options.autopairs {
        if let Some(close) = pair_for(c) {
            let win = state.windows.get(&wid).unwrap();
            let close_pos = Position::new(win.cursor_line, win.cursor_col);
            if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.insert_char(close_pos, close); }
        }
    }
}

fn pair_for(c: char) -> Option<char> {
    match c { '(' => Some(')'), '[' => Some(']'), '{' => Some('}'), _ => None }
}

fn is_close_pair(c: char) -> bool { matches!(c, ')' | ']' | '}') }

pub(crate) fn dispatch_insert_newline(state: &mut EditorState) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = match state.windows.get(&wid) { Some(w) => w, None => return };
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    let indent = if state.options.autoindent {
        if let Some(buf) = state.buffers.get(&bid) {
            buf.text.line_to_string(win.cursor_line).chars().take_while(|c| c.is_whitespace() && *c != '\n').collect()
        } else { String::new() }
    } else { String::new() };
    if let Some(buf) = state.buffers.get_mut(&bid) {
        buf.text.insert_char(pos, '\n');
        if !indent.is_empty() { buf.text.insert_text(Position::new(pos.line + 1, 0), &indent); }
        buf.modified = true;
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        win.cursor_line += 1;
        win.cursor_col = indent.len();
        win.ensure_cursor_visible();
    }
}

/// Insert contents of a register at cursor (Ctrl-r {reg}).
pub(crate) fn dispatch_insert_from_register(state: &mut EditorState, reg: char) {
    use kjxlkj_core_types::RegisterName;
    let name = match RegisterName::from_char(reg) { Some(n) => n, None => return };
    let text = match state.registers.get(name) { Some(entry) => entry.text.clone(), None => return };
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) { buf.text.insert_text(pos, &text); buf.modified = true; }
    if let Some(win) = state.windows.get_mut(&wid) { win.cursor_col += text.len(); }
}

/// Insert a digraph character at cursor (Ctrl-K c1 c2).
pub(crate) fn dispatch_insert_digraph(state: &mut EditorState, c1: char, c2: char) {
    if let Some(ch) = kjxlkj_core_types::digraph_lookup(c1, c2) {
        dispatch_insert_char(state, ch);
    } else {
        state.message = Some(format!("Unknown digraph: {}{}", c1, c2));
    }
}
