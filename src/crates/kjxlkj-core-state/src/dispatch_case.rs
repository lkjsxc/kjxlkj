//! Case operator dispatch: toggle, upper, lower case over motion/line.

use crate::{BufferState, EditorState};
use kjxlkj_core_types::{CaseOp, MotionKind, Position, Range};

/// Case operator over a motion (g~/gU/gu + motion).
pub(crate) fn dispatch_case_operator(state: &mut EditorState, op: CaseOp, motion: MotionKind, count: usize) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let pos = Position::new(win.cursor_line, win.cursor_col);
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let target = kjxlkj_core_edit::apply_motion(&buf.text, pos, motion, count);
        let (start, end) = if target < pos { (target, pos) } else { (pos, Position::new(target.line, target.col + 1)) };
        apply_case_in_range(buf, Range::new(start, end), op);
    }
}

/// Case operator on entire line (g~~/gUU/guu).
pub(crate) fn dispatch_case_operator_line(state: &mut EditorState, op: CaseOp) {
    let wid = match state.active_window { Some(w) => w, None => return };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let len = buf.text.line_len(line);
        apply_case_in_range(buf, Range::new(Position::new(line, 0), Position::new(line, len)), op);
    }
}

fn apply_case_in_range(buf: &mut BufferState, range: Range, op: CaseOp) {
    let text = buf.text.text_in_range(range.start, range.end);
    let converted: String = text.chars().map(|c| match op {
        CaseOp::Toggle => if c.is_uppercase() { c.to_lowercase().next().unwrap_or(c) } else { c.to_uppercase().next().unwrap_or(c) },
        CaseOp::Upper => c.to_uppercase().next().unwrap_or(c),
        CaseOp::Lower => c.to_lowercase().next().unwrap_or(c),
    }).collect();
    buf.text.delete_range(range);
    buf.text.insert_text(range.start, &converted);
    buf.modified = true;
}
