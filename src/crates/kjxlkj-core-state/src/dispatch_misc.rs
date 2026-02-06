//! Miscellaneous dispatch: marks, find-char repeat, case operators,
//! visual swap, select register, dot repeat, increment.

use crate::EditorState;
use kjxlkj_core_types::{
    CaseOp, FindCharKind, MotionKind, Position, Range,
};

// ── Marks ────────────────────────────────────────────────────

/// Set a mark at current cursor position.
pub(crate) fn dispatch_set_mark(
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
    state.marks.insert(c, (bid, pos));
}

/// Jump to mark (exact position with `).
pub(crate) fn dispatch_jump_to_mark(
    state: &mut EditorState,
    c: char,
) {
    if let Some((_bid, pos)) = state.marks.get(&c) {
        let pos = *pos;
        if let Some(win) = state.active_window_mut() {
            win.set_cursor(pos);
            win.ensure_cursor_visible();
        }
    } else {
        state.message = Some(format!("Mark '{}' not set", c));
    }
}

/// Jump to mark line (first non-blank with ').
pub(crate) fn dispatch_jump_to_mark_line(
    state: &mut EditorState,
    c: char,
) {
    if let Some((_bid, pos)) = state.marks.get(&c) {
        let line = pos.line;
        if let Some(wid) = state.active_window {
            if let Some(win) = state.windows.get(&wid) {
                let bid = win.buffer_id;
                if let Some(buf) = state.buffers.get(&bid) {
                    let text = buf.text.line_to_string(line);
                    let col = text
                        .chars()
                        .position(|ch| !ch.is_whitespace())
                        .unwrap_or(0);
                    let win =
                        state.windows.get_mut(&wid).unwrap();
                    win.set_cursor(Position::new(line, col));
                    win.ensure_cursor_visible();
                }
            }
        }
    } else {
        state.message = Some(format!("Mark '{}' not set", c));
    }
}

// ── Find-char repeat ────────────────────────────────────────

/// Dispatch a find-char intent (f/t/F/T).
pub(crate) fn dispatch_find_char(
    state: &mut EditorState,
    c: char,
    kind: FindCharKind,
) {
    state.last_find_char = Some((c, kind));
    do_find_char(state, c, kind);
}

/// Repeat last find-char (;).
pub(crate) fn dispatch_repeat_find_char(
    state: &mut EditorState,
) {
    if let Some((c, kind)) = state.last_find_char {
        do_find_char(state, c, kind);
    }
}

/// Repeat last find-char reversed (,).
pub(crate) fn dispatch_repeat_find_char_reverse(
    state: &mut EditorState,
) {
    if let Some((c, kind)) = state.last_find_char {
        let rev = match kind {
            FindCharKind::Forward => FindCharKind::Backward,
            FindCharKind::Backward => FindCharKind::Forward,
            FindCharKind::TillForward => FindCharKind::TillBackward,
            FindCharKind::TillBackward => FindCharKind::TillForward,
        };
        do_find_char(state, c, rev);
    }
}

fn do_find_char(
    state: &mut EditorState,
    c: char,
    kind: FindCharKind,
) {
    let motion = match kind {
        FindCharKind::Forward => MotionKind::FindCharForward(c),
        FindCharKind::Backward => MotionKind::FindCharBackward(c),
        FindCharKind::TillForward => MotionKind::TillCharForward(c),
        FindCharKind::TillBackward => {
            MotionKind::TillCharBackward(c)
        }
    };
    crate::dispatch_navigation::dispatch_motion(state, motion, 1);
}

// ── Case operators ──────────────────────────────────────────

/// Case operator over a motion (g~/gU/gu + motion).
pub(crate) fn dispatch_case_operator(
    state: &mut EditorState,
    op: CaseOp,
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
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let target = kjxlkj_core_edit::apply_motion(
            &buf.text, pos, motion, count,
        );
        let (start, end) = if target < pos {
            (target, pos)
        } else {
            (pos, Position::new(target.line, target.col + 1))
        };
        apply_case_in_range(buf, Range::new(start, end), op);
    }
}

/// Case operator on entire line (g~~/gUU/guu).
pub(crate) fn dispatch_case_operator_line(
    state: &mut EditorState,
    op: CaseOp,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let len = buf.text.line_len(line);
        let range = Range::new(
            Position::new(line, 0),
            Position::new(line, len),
        );
        apply_case_in_range(buf, range, op);
    }
}

fn apply_case_in_range(
    buf: &mut crate::BufferState,
    range: Range,
    op: CaseOp,
) {
    let text = buf.text.text_in_range(range.start, range.end);
    let converted: String = text
        .chars()
        .map(|c| match op {
            CaseOp::Toggle => {
                if c.is_uppercase() {
                    c.to_lowercase().next().unwrap_or(c)
                } else {
                    c.to_uppercase().next().unwrap_or(c)
                }
            }
            CaseOp::Upper => c.to_uppercase().next().unwrap_or(c),
            CaseOp::Lower => c.to_lowercase().next().unwrap_or(c),
        })
        .collect();
    buf.text.delete_range(range);
    buf.text.insert_text(range.start, &converted);
    buf.modified = true;
}

// ── Visual swap end ─────────────────────────────────────────

/// Swap cursor/anchor in visual mode (o).
pub(crate) fn dispatch_visual_swap_end(
    state: &mut EditorState,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = match state.windows.get_mut(&wid) {
        Some(w) => w,
        None => return,
    };
    if let Some(anchor) = win.visual_anchor {
        let cursor = Position::new(win.cursor_line, win.cursor_col);
        win.visual_anchor = Some(cursor);
        win.cursor_line = anchor.line;
        win.cursor_col = anchor.col;
    }
}

// ── Select register ─────────────────────────────────────────

/// Select a register for the next operation.
pub(crate) fn dispatch_select_register(
    state: &mut EditorState,
    reg: kjxlkj_core_types::RegisterName,
) {
    state.registers.select(reg);
}

// ── Increment/decrement ─────────────────────────────────────

/// Increment or decrement the number under cursor.
pub(crate) fn dispatch_increment_number(
    state: &mut EditorState,
    delta: i64,
) {
    let wid = match state.active_window {
        Some(w) => w,
        None => return,
    };
    let win = state.windows.get(&wid).unwrap();
    let bid = win.buffer_id;
    let line = win.cursor_line;
    let col = win.cursor_col;
    if let Some(buf) = state.buffers.get_mut(&bid) {
        let text = buf.text.line_to_string(line);
        let chars: Vec<char> = text.chars().collect();
        // Find digit sequence containing or after cursor
        let mut start = col;
        // Scan forward to find a digit
        while start < chars.len() && !chars[start].is_ascii_digit()
        {
            start += 1;
        }
        if start >= chars.len() {
            return;
        }
        // Check for negative sign
        let negative =
            start > 0 && chars[start - 1] == '-';
        if negative {
            start -= 1;
        }
        let mut end = if negative { start + 1 } else { start };
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }
        let num_str: String = chars[start..end].iter().collect();
        if let Ok(n) = num_str.parse::<i64>() {
            let new_n = n + delta;
            let new_str = new_n.to_string();
            let range = Range::new(
                Position::new(line, start),
                Position::new(line, end),
            );
            buf.text.delete_range(range);
            buf.text.insert_text(
                Position::new(line, start),
                &new_str,
            );
            buf.modified = true;
            // Place cursor at end of number
            let win =
                state.windows.get_mut(&wid).unwrap();
            win.cursor_col = start + new_str.len() - 1;
        }
    }
}
