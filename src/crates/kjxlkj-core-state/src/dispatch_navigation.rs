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
            win.center_cursor();
        }
        ScrollKind::CursorTop
        | ScrollKind::CursorTopFirstNonBlank => {
            win.cursor_to_top();
        }
        ScrollKind::CursorBottom
        | ScrollKind::CursorBottomFirstNonBlank => {
            win.cursor_to_bottom();
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
