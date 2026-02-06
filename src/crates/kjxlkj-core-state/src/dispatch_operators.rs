//! Operator dispatch: operators, text objects, yank, paste, delete-to-end.

use crate::{BufferState, EditorState};
use kjxlkj_core_edit::{
    apply_operator, compute_motion_range, find_text_object,
};
use kjxlkj_core_types::{
    Mode, MotionKind, OperatorKind, Position, Range, TextObjectKind,
};

/// Push an undo entry into the buffer's undo tree.
pub(crate) fn push_undo_entry(
    buf: &mut BufferState,
    old_text: &str,
    new_text: &str,
    pos: Position,
    _extra: usize,
) {
    use kjxlkj_core_undo::UndoEntry;
    let forward = format!(
        "{}|{}|{}|{}",
        new_text,
        pos.line,
        pos.col,
        old_text.chars().count()
    );
    let reverse = format!(
        "{}|{}|{}|{}",
        old_text,
        pos.line,
        pos.col,
        new_text.chars().count()
    );
    buf.undo.push(UndoEntry {
        forward: forward.into_bytes(),
        reverse: reverse.into_bytes(),
        timestamp: std::time::Instant::now(),
    });
}

pub(crate) fn dispatch_operator(
    state: &mut EditorState,
    op: OperatorKind,
    motion: MotionKind,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);

    // In visual mode, use the visual selection range instead.
    let is_visual = state.mode.current().is_visual();
    let range = if is_visual {
        if let Some(r) = state.visual_range() {
            // Extend end to include the character at cursor
            Range::new(r.start, Position::new(r.end.line, r.end.col + 1))
        } else {
            compute_motion_range(&state.buffers[&bid].text, pos, motion, count)
        }
    } else {
        if let Some(buf) = state.buffers.get(&bid) {
            compute_motion_range(&buf.text, pos, motion, count)
        } else {
            return;
        }
    };

    // Use selected register if set, otherwise default behavior.
    let selected_reg = state.registers.take_selected();

    if let Some(buf) = state.buffers.get_mut(&bid) {
        let result = apply_operator(&mut buf.text, op, range, false);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            if let Some(sel) = selected_reg {
                let content = if false {
                    kjxlkj_core_types::RegisterContent::linewise(&reg.text)
                } else {
                    kjxlkj_core_types::RegisterContent::charwise(&reg.text)
                };
                state.registers.set(sel, content);
            } else if op == OperatorKind::Yank {
                state.registers.yank(&reg.text, false);
            } else {
                state.registers.delete(&reg.text, false);
            }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        win.visual_anchor = None;
        if result.enter_insert {
            state.mode.transition(Mode::Insert);
        } else if is_visual {
            state.mode.transition(Mode::Normal);
        }
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_line_operator(
    state: &mut EditorState,
    op: OperatorKind,
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
        let end_line =
            (line + count).min(buf.text.line_count()).saturating_sub(1);
        let range = Range::new(
            Position::new(line, 0),
            Position::new(end_line, buf.text.line_len(end_line)),
        );
        let result = apply_operator(&mut buf.text, op, range, true);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            match op {
                OperatorKind::Yank => {
                    state.registers.yank(&reg.text, true)
                }
                _ => state.registers.delete(&reg.text, true),
            }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert {
            state.mode.transition(Mode::Insert);
        }
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_operator_text_object(
    state: &mut EditorState,
    op: OperatorKind,
    kind: TextObjectKind,
    inner: bool,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let range = match find_text_object(&buf.text, pos, kind, inner) {
            Some(r) => r,
            None => return,
        };
        let result = apply_operator(&mut buf.text, op, range, false);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            if op == OperatorKind::Yank {
                state.registers.yank(&reg.text, false);
            } else {
                state.registers.delete(&reg.text, false);
            }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert {
            state.mode.transition(Mode::Insert);
        }
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_yank_line(
    state: &mut EditorState,
    count: usize,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    if let Some(buf) = state.buffers.get(&win.buffer_id) {
        let start = win.cursor_line;
        let end = (start + count).min(buf.text.line_count());
        let start_pos = Position::new(start, 0);
        let end_pos = if end >= buf.text.line_count() {
            let last = buf.text.line_count() - 1;
            Position::new(last, buf.text.line_len(last))
        } else {
            Position::new(end, 0)
        };
        let text = buf.text.text_in_range(start_pos, end_pos);
        state.registers.yank(&text, true);
    }
}

pub(crate) fn dispatch_delete_to_end(state: &mut EditorState) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let line_len = buf.text.line_len(line);
        if col < line_len {
            let start = Position::new(line, col);
            let end = Position::new(line, line_len);
            let text = buf.text.text_in_range(start, end);
            buf.text.delete_range(Range::new(start, end));
            buf.modified = true;
            state.registers.delete(&text, false);
        }
    }
    if let Some(win) = state.windows.get_mut(&wid) {
        if let Some(buf) = state.buffers.get(&bid) {
            let new_len = buf.text.line_len(win.cursor_line);
            if win.cursor_col >= new_len && new_len > 0 {
                win.cursor_col = new_len.saturating_sub(1);
            }
        }
    }
}

pub(crate) fn dispatch_paste(
    state: &mut EditorState,
    paste_pos: kjxlkj_core_types::PastePosition,
) {
    let text = match state.registers.unnamed_text() {
        Some(t) => t.to_string(),
        None => return,
    };
    let linewise = state.registers.unnamed_type()
        == Some(kjxlkj_core_types::RegisterType::Linewise);
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    use kjxlkj_core_types::PastePosition;
    if linewise {
        let line = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => {
                win.cursor_line + 1
            }
            PastePosition::Before | PastePosition::BeforeCursorEnd => {
                win.cursor_line
            }
        };
        let pos = Position::new(line, 0);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            let insert_text = if text.ends_with('\n') {
                text.clone()
            } else {
                format!("{}\n", text)
            };
            buf.text.insert_text(pos, &insert_text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_line = line;
        win.cursor_col = 0;
    } else {
        let col = match paste_pos {
            PastePosition::After | PastePosition::AfterCursorEnd => {
                win.cursor_col + 1
            }
            PastePosition::Before | PastePosition::BeforeCursorEnd => {
                win.cursor_col
            }
        };
        let pos = Position::new(win.cursor_line, col);
        if let Some(buf) = state.buffers.get_mut(&bid) {
            buf.text.insert_text(pos, &text);
            buf.modified = true;
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.cursor_col = col + text.len().saturating_sub(1);
    }
}
