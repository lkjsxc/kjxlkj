//! Operator and extended edit operations.
//!
//! See /docs/spec/editing/operators/operators.md.

use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{ContentKind, Motion, Operator};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn apply_operator_line(&mut self, op: Operator) {
        if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
            self.apply_case_operator_line(op);
            return;
        }
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let gc = buf.line_grapheme_count(line);
                if matches!(op, Operator::Delete | Operator::Change) {
                    let _ = buf.delete(line, 0, line, gc);
                    if buf.line_count() > 1 {
                        let _ = buf.delete(line, 0, line, 1);
                    }
                }
                // Yank/Indent/Dedent/Reindent/Format: TODO
            }
        }
    }

    pub(crate) fn apply_operator_motion(
        &mut self, op: Operator, motion: Motion, count: usize,
    ) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let start = win.cursor;
            let mut end = start;
            if let Some(buf) = self.buffers.get(&buf_id) {
                for _ in 0..count {
                    end = apply_motion(&end, &motion, buf);
                }
            }
            let (sl, sc, el, ec) = ordered_range(
                start.line, start.col, end.line, end.col,
            );
            if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
                self.apply_range_case_op(op, sl, sc, el, ec);
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
                return;
            }
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                if matches!(op, Operator::Delete | Operator::Change) {
                    let _ = buf.delete(sl, sc, el, ec);
                }
                // Yank/Indent/Dedent/Reindent/Format: TODO
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
            }
        }
    }

    pub(crate) fn delete_current_line_content(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let gc = buf.line_grapheme_count(line);
                if gc > 0 { let _ = buf.delete(line, 0, line, gc); }
                let w = self.focused_window_mut();
                w.cursor.col = 0;
            }
        }
    }

    pub(crate) fn delete_to_eol(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                let gc = buf.line_grapheme_count(line);
                if col < gc { let _ = buf.delete(line, col, line, gc); }
            }
        }
    }

    pub(crate) fn delete_word_backward(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let cursor = win.cursor;
            if let Some(buf) = self.buffers.get(&buf_id) {
                let nc = apply_motion(&cursor, &Motion::WordBackward, buf);
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let _ = buf.delete(nc.line, nc.col, cursor.line, cursor.col);
                    let w = self.focused_window_mut();
                    w.cursor = nc;
                }
            }
        }
    }

    pub(crate) fn delete_to_line_start(&mut self) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let (line, col) = (win.cursor.line, win.cursor.col);
            if col > 0 {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let _ = buf.delete(line, 0, line, col);
                    let w = self.focused_window_mut();
                    w.cursor.col = 0;
                }
            }
        }
    }
}

fn ordered_range(
    sl: usize, sc: usize, el: usize, ec: usize,
) -> (usize, usize, usize, usize) {
    if (sl, sc) <= (el, ec) { (sl, sc, el, ec) } else { (el, ec, sl, sc) }
}
