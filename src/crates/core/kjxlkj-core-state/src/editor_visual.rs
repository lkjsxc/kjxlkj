//! Visual mode operator execution.
//!
//! Computes the selection range from visual anchor + cursor,
//! then applies the operator on that range.

use kjxlkj_core_edit::Cursor;
use kjxlkj_core_types::{ContentKind, Mode, Operator, RangeType, VisualKind};

use crate::editor::EditorState;

impl EditorState {
    /// Apply an operator on the current visual selection.
    pub(crate) fn apply_visual_operator(&mut self, op: Operator) {
        let anchor = match self.visual_anchor {
            Some(a) => a,
            None => return,
        };
        let kind = match self.mode {
            Mode::Visual(k) => k,
            _ => VisualKind::Char,
        };
        let win = match self.windows.get(&self.focus.focused) {
            Some(w) => w,
            None => return,
        };
        let cursor = win.cursor;
        let buf_id = match win.content {
            ContentKind::Buffer(id) => id,
            _ => return,
        };
        let (sl, sc, el, ec) = ordered(anchor, cursor);
        match kind {
            VisualKind::Char => {
                self.apply_visual_char_op(op, buf_id, sl, sc, el, ec);
            }
            VisualKind::Line => {
                self.apply_visual_line_op(op, buf_id, sl, el);
            }
            VisualKind::Block => {
                // Block operations are a stub for now.
                self.apply_visual_char_op(op, buf_id, sl, sc, el, ec);
            }
        }
        self.visual_anchor = None;
    }

    fn apply_visual_char_op(
        &mut self, op: Operator,
        buf_id: kjxlkj_core_types::BufferId,
        sl: usize, sc: usize, el: usize, ec: usize,
    ) {
        let end_exclusive = ec + 1;
        if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
            self.apply_range_case_op(op, sl, sc, el, end_exclusive);
            let w = self.focused_window_mut();
            w.cursor.line = sl;
            w.cursor.col = sc;
            return;
        }
        let text = if let Some(buf) = self.buffers.get(&buf_id) {
            buf.text_range(sl, sc, el, end_exclusive)
        } else {
            String::new()
        };
        let scope = if sl != el {
            RangeType::Linewise
        } else {
            RangeType::Characterwise
        };
        match op {
            Operator::Yank => {
                self.registers.record_yank(text, scope);
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
            }
            Operator::Delete | Operator::Change => {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let _ = buf.delete(sl, sc, el, end_exclusive);
                }
                self.registers.record_delete(text, scope);
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
            }
            _ => {
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
            }
        }
    }

    fn apply_visual_line_op(
        &mut self, op: Operator,
        buf_id: kjxlkj_core_types::BufferId,
        sl: usize, el: usize,
    ) {
        if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
            for line in sl..=el {
                if let Some(buf) = self.buffers.get(&buf_id) {
                    let gc = buf.line_grapheme_count(line);
                    if gc > 0 {
                        self.apply_range_case_op(op, line, 0, line, gc);
                    }
                }
            }
            let w = self.focused_window_mut();
            w.cursor.line = sl;
            w.cursor.col = 0;
            return;
        }
        // Collect text for all lines.
        let mut text = String::new();
        if let Some(buf) = self.buffers.get(&buf_id) {
            for line in sl..=el {
                if let Some(l) = buf.line(line) { text.push_str(&l); }
            }
        }
        match op {
            Operator::Yank => {
                self.registers.record_yank(text, RangeType::Linewise);
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = 0;
            }
            Operator::Delete | Operator::Change => {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    for _ in sl..=el {
                        if sl < buf.line_count() {
                            let gc = buf.line_grapheme_count(sl);
                            let _ = buf.delete(sl, 0, sl, gc);
                            if buf.line_count() > 1 && sl < buf.line_count() {
                                let _ = buf.delete(sl, 0, sl, 1);
                            }
                        }
                    }
                }
                self.registers.record_delete(text, RangeType::Linewise);
                let new_line = sl.min(
                    self.buffers.get(&buf_id)
                        .map(|b| b.line_count().saturating_sub(1))
                        .unwrap_or(0)
                );
                let w = self.focused_window_mut();
                w.cursor.line = new_line;
                w.cursor.col = 0;
            }
            _ => {
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = 0;
            }
        }
    }

    /// Swap visual anchor and cursor.
    pub(crate) fn swap_visual_anchor(&mut self) {
        if let Some(anchor) = self.visual_anchor.take() {
            let win = match self.windows.get_mut(&self.focus.focused) {
                Some(w) => w,
                None => return,
            };
            let old_cursor = win.cursor;
            win.cursor = anchor;
            self.visual_anchor = Some(old_cursor);
        }
    }
}

fn ordered(a: Cursor, b: Cursor) -> (usize, usize, usize, usize) {
    if (a.line, a.col) <= (b.line, b.col) {
        (a.line, a.col, b.line, b.col)
    } else {
        (b.line, b.col, a.line, a.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_sorts_correctly() {
        let a = Cursor::new(2, 3);
        let b = Cursor::new(0, 1);
        assert_eq!(ordered(a, b), (0, 1, 2, 3));
        assert_eq!(ordered(b, a), (0, 1, 2, 3));
    }

    #[test]
    fn ordered_same_line() {
        let a = Cursor::new(1, 5);
        let b = Cursor::new(1, 2);
        assert_eq!(ordered(a, b), (1, 2, 1, 5));
    }
}
