//! Navigation dispatch: motion, scroll, undo, redo.

use crate::EditorState;
use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{MotionKind, Position, Range};

pub(crate) fn dispatch_motion(
    state: &mut EditorState,
    kind: MotionKind,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let bid = match state.windows.get(&wid) {
        Some(w) => w.buffer_id,
        None => return,
    };
    let buf = match state.buffers.get(&bid) {
        Some(b) => b,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let pos = Position::new(win.cursor_line, win.cursor_col);
    // H/M/L motions need viewport info
    let new_pos = match kind {
        MotionKind::ScreenTop => {
            let line = win.top_line;
            let col = first_non_blank_col(buf, line);
            Position::new(line, col)
        }
        MotionKind::ScreenMiddle => {
            let line = win.top_line + win.height / 2;
            let max = buf.text.line_count().saturating_sub(1);
            let line = line.min(max);
            let col = first_non_blank_col(buf, line);
            Position::new(line, col)
        }
        MotionKind::ScreenBottom => {
            let line = win.top_line + win.height.saturating_sub(1);
            let max = buf.text.line_count().saturating_sub(1);
            let line = line.min(max);
            let col = first_non_blank_col(buf, line);
            Position::new(line, col)
        }
        _ => apply_motion(&buf.text, pos, kind, count),
    };
    let win = state.windows.get_mut(&wid).unwrap();
    win.set_cursor(new_pos);
    win.ensure_cursor_visible();
}

/// Helper: get first non-blank column on a line.
fn first_non_blank_col(
    buf: &crate::BufferState,
    line: usize,
) -> usize {
    if line >= buf.text.line_count() {
        return 0;
    }
    let text = buf.text.line_to_string(line);
    text.chars()
        .position(|c| !c.is_whitespace())
        .unwrap_or(0)
}

pub(crate) fn dispatch_scroll(
    state: &mut EditorState,
    kind: kjxlkj_core_types::ScrollKind,
) {
    use kjxlkj_core_types::ScrollKind;
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get_mut(&wid).unwrap();
    let half = win.height / 2;
    match kind {
        ScrollKind::HalfPageDown => {
            win.cursor_line += half;
            win.top_line += half;
        }
        ScrollKind::HalfPageUp => {
            win.cursor_line = win.cursor_line.saturating_sub(half);
            win.top_line = win.top_line.saturating_sub(half);
        }
        ScrollKind::FullPageDown => {
            win.cursor_line += win.height;
            win.top_line += win.height;
        }
        ScrollKind::FullPageUp => {
            win.cursor_line =
                win.cursor_line.saturating_sub(win.height);
            win.top_line = win.top_line.saturating_sub(win.height);
        }
        ScrollKind::LineDown => {
            win.top_line += 1;
            if win.cursor_line < win.top_line {
                win.cursor_line = win.top_line;
            }
        }
        ScrollKind::LineUp => {
            win.top_line = win.top_line.saturating_sub(1);
            if win.cursor_line >= win.top_line + win.height {
                win.cursor_line = win.top_line + win.height - 1;
            }
        }
        ScrollKind::CursorCenter
        | ScrollKind::CursorCenterFirstNonBlank => {
            win.top_line = win.cursor_line.saturating_sub(half);
        }
        ScrollKind::CursorTop
        | ScrollKind::CursorTopFirstNonBlank => {
            win.top_line = win.cursor_line;
        }
        ScrollKind::CursorBottom
        | ScrollKind::CursorBottomFirstNonBlank => {
            win.top_line = win
                .cursor_line
                .saturating_sub(win.height.saturating_sub(1));
        }
    }
    // Clamp cursor to buffer
    if let Some(buf) = state.buffers.get(&win.buffer_id) {
        let max_line = buf.text.line_count().saturating_sub(1);
        if win.cursor_line > max_line {
            win.cursor_line = max_line;
        }
    }
}

pub(crate) fn dispatch_undo(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        if let Some(entry) = buf.undo.undo() {
            let reverse =
                String::from_utf8_lossy(&entry.reverse).to_string();
            if let Some((text, rest)) = reverse.split_once('|') {
                let parts: Vec<&str> = rest.splitn(3, '|').collect();
                if parts.len() == 3 {
                    let line: usize = parts[0].parse().unwrap_or(0);
                    let col: usize = parts[1].parse().unwrap_or(0);
                    let len: usize = parts[2].parse().unwrap_or(0);
                    let pos = Position::new(line, col);
                    let end_idx = buf.text.pos_to_char_idx(pos) + len;
                    let end_pos = buf.text.char_idx_to_pos(end_idx);
                    buf.text.delete_range(Range::new(pos, end_pos));
                    if !text.is_empty() {
                        buf.text.insert_text(pos, text);
                    }
                    let win = state.windows.get_mut(&wid).unwrap();
                    win.set_cursor(pos);
                    win.ensure_cursor_visible();
                    return;
                }
            }
        }
        state.message = Some("already at oldest change".into());
    }
}

pub(crate) fn dispatch_redo(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        if let Some(entry) = buf.undo.redo() {
            let forward =
                String::from_utf8_lossy(&entry.forward).to_string();
            if let Some((text, rest)) = forward.split_once('|') {
                let parts: Vec<&str> = rest.splitn(3, '|').collect();
                if parts.len() == 3 {
                    let line: usize = parts[0].parse().unwrap_or(0);
                    let col: usize = parts[1].parse().unwrap_or(0);
                    let len: usize = parts[2].parse().unwrap_or(0);
                    let pos = Position::new(line, col);
                    let end_idx = buf.text.pos_to_char_idx(pos) + len;
                    let end_pos = buf.text.char_idx_to_pos(end_idx);
                    buf.text.delete_range(Range::new(pos, end_pos));
                    if !text.is_empty() {
                        buf.text.insert_text(pos, text);
                    }
                    let win = state.windows.get_mut(&wid).unwrap();
                    win.set_cursor(pos);
                    win.ensure_cursor_visible();
                    return;
                }
            }
        }
        state.message = Some("already at newest change".into());
    }
}

/// Push current position onto the jump list.
pub(crate) fn push_jump(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    // Don't push duplicates
    if state.jump_list.last() == Some(&(bid, pos)) {
        return;
    }
    // Truncate any forward entries
    state.jump_list.truncate(state.jump_list_idx);
    state.jump_list.push((bid, pos));
    state.jump_list_idx = state.jump_list.len();
    // Limit jump list size
    if state.jump_list.len() > 100 {
        state.jump_list.remove(0);
        state.jump_list_idx = state.jump_list.len();
    }
}

pub(crate) fn dispatch_jump_back(state: &mut EditorState) {
    if state.jump_list_idx == 0 || state.jump_list.is_empty() {
        state.message = Some("At bottom of jump list".into());
        return;
    }
    // Save current position before jumping
    if state.jump_list_idx == state.jump_list.len() {
        push_jump(state);
        state.jump_list_idx -= 1; // back up past current pos
    }
    state.jump_list_idx = state.jump_list_idx.saturating_sub(1);
    let (bid, pos) = state.jump_list[state.jump_list_idx];
    // Switch to the target buffer/position
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

pub(crate) fn dispatch_jump_forward(state: &mut EditorState) {
    if state.jump_list_idx + 1 >= state.jump_list.len() {
        state.message = Some("At top of jump list".into());
        return;
    }
    state.jump_list_idx += 1;
    let (bid, pos) = state.jump_list[state.jump_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

/// Push a change position onto the change list.
pub(crate) fn push_change(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    state.change_list.push((bid, pos));
    state.change_list_idx = state.change_list.len();
    if state.change_list.len() > 100 {
        state.change_list.remove(0);
        state.change_list_idx = state.change_list.len();
    }
}

pub(crate) fn dispatch_change_older(state: &mut EditorState) {
    if state.change_list_idx == 0 || state.change_list.is_empty() {
        state.message = Some("At oldest change".into());
        return;
    }
    state.change_list_idx -= 1;
    let (bid, pos) = state.change_list[state.change_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}

pub(crate) fn dispatch_change_newer(state: &mut EditorState) {
    if state.change_list_idx + 1 >= state.change_list.len() {
        state.message = Some("At newest change".into());
        return;
    }
    state.change_list_idx += 1;
    let (bid, pos) = state.change_list[state.change_list_idx];
    if let Some(wid) = state.active_window {
        if let Some(win) = state.windows.get_mut(&wid) {
            win.buffer_id = bid;
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    }
}
