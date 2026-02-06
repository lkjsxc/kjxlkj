//! Operator dispatch: operators, text objects.

use crate::{BufferState, EditorState};
use kjxlkj_core_edit::{apply_operator, compute_motion_range, find_text_object};
use kjxlkj_core_types::{Mode, MotionKind, OperatorKind, Position, Range, TextObjectKind};

/// Push an undo entry into the buffer's undo tree.
pub(crate) fn push_undo_entry(buf: &mut BufferState, old_text: &str, new_text: &str, pos: Position, _extra: usize) {
    use kjxlkj_core_undo::UndoEntry;
    let forward = format!("{}|{}|{}|{}", new_text, pos.line, pos.col, old_text.chars().count());
    let reverse = format!("{}|{}|{}|{}", old_text, pos.line, pos.col, new_text.chars().count());
    buf.undo.push(UndoEntry { forward: forward.into_bytes(), reverse: reverse.into_bytes(), timestamp: std::time::Instant::now() });
}

pub(crate) fn dispatch_operator(state: &mut EditorState, op: OperatorKind, motion: MotionKind, count: usize) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    let is_visual = state.mode.current().is_visual();
    let range = if is_visual {
        if let Some(r) = state.visual_range() {
            Range::new(r.start, Position::new(r.end.line, r.end.col + 1))
        } else {
            compute_motion_range(&state.buffers[&bid].text, pos, motion, count)
        }
    } else if let Some(buf) = state.buffers.get(&bid) {
        compute_motion_range(&buf.text, pos, motion, count)
    } else {
        return;
    };
    let selected_reg = state.registers.take_selected();
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let result = apply_operator(&mut buf.text, op, range, false);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            if let Some(sel) = selected_reg {
                let content = kjxlkj_core_types::RegisterContent::charwise(&reg.text);
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
        if result.enter_insert { state.mode.transition(Mode::Insert); }
        else if is_visual { state.mode.transition(Mode::Normal); }
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_line_operator(state: &mut EditorState, op: OperatorKind, count: usize) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let end_line = (line + count).min(buf.text.line_count()).saturating_sub(1);
        let range = Range::new(Position::new(line, 0), Position::new(end_line, buf.text.line_len(end_line)));
        let result = apply_operator(&mut buf.text, op, range, true);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            match op {
                OperatorKind::Yank => state.registers.yank(&reg.text, true),
                _ => state.registers.delete(&reg.text, true),
            }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert { state.mode.transition(Mode::Insert); }
        win.ensure_cursor_visible();
    }
}

pub(crate) fn dispatch_operator_text_object(state: &mut EditorState, op: OperatorKind, kind: TextObjectKind, inner: bool) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let range = match find_text_object(&buf.text, pos, kind, inner) { Some(r) => r, None => return };
        let result = apply_operator(&mut buf.text, op, range, false);
        buf.modified = true;
        if let Some(ref reg) = result.deleted_text {
            if op == OperatorKind::Yank { state.registers.yank(&reg.text, false); }
            else { state.registers.delete(&reg.text, false); }
        }
        let win = state.windows.get_mut(&wid).unwrap();
        win.set_cursor(result.new_cursor);
        if result.enter_insert { state.mode.transition(Mode::Insert); }
        win.ensure_cursor_visible();
    }
}
