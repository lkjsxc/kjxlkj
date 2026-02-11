//! Operator and extended edit operations.
//!
//! See /docs/spec/editing/operators/operators.md.

use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{ContentKind, Motion, Operator, RangeType};

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
                let text = buf.line(line).unwrap_or_default();
                match op {
                    Operator::Yank => {
                        self.registers.record_yank(text, RangeType::Linewise);
                    }
                    Operator::Delete | Operator::Change => {
                        let gc = buf.line_grapheme_count(line);
                        let _ = buf.delete(line, 0, line, gc);
                        if buf.line_count() > 1 {
                            let _ = buf.delete(line, 0, line, 1);
                        }
                        self.registers.record_delete(text, RangeType::Linewise);
                    }
                    _ => {}
                }
            }
        }
    }

    pub(crate) fn apply_operator_motion(
        &mut self, op: Operator, motion: Motion, count: usize,
    ) {
        // Text object dispatch.
        match motion {
            Motion::TextObjInner(ch) => { self.apply_operator_text_obj(op, ch, true); return; }
            Motion::TextObjAround(ch) => { self.apply_operator_text_obj(op, ch, false); return; }
            _ => {}
        }
        let win = self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let start = win.cursor;
            let mut end = start;
            if let Some(buf) = self.buffers.get(&buf_id) {
                for _ in 0..count { end = apply_motion(&end, &motion, buf); }
            }
            let (sl, sc, el, ec) = ordered_range(start.line, start.col, end.line, end.col);
            if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
                self.apply_range_case_op(op, sl, sc, el, ec);
                let w = self.focused_window_mut();
                w.cursor.line = sl; w.cursor.col = sc;
                return;
            }
            let text = if let Some(buf) = self.buffers.get(&buf_id) {
                buf.text_range(sl, sc, el, ec)
            } else { String::new() };
            let scope = if sl != el { RangeType::Linewise } else { RangeType::Characterwise };
            match op {
                Operator::Yank => {
                    self.registers.record_yank(text, scope);
                    let w = self.focused_window_mut();
                    w.cursor.line = sl; w.cursor.col = sc;
                }
                Operator::Delete | Operator::Change => {
                    if let Some(buf) = self.buffers.get_mut(&buf_id) {
                        let _ = buf.delete(sl, sc, el, ec);
                    }
                    self.registers.record_delete(text, scope);
                    let w = self.focused_window_mut();
                    w.cursor.line = sl; w.cursor.col = sc;
                }
                _ => {
                    let w = self.focused_window_mut();
                    w.cursor.line = sl; w.cursor.col = sc;
                }
            }
        }
    }

    fn apply_operator_text_obj(&mut self, op: Operator, ch: char, inner: bool) {
        let win = self.windows.get(&self.focus.focused).unwrap();
        let buf_id = match win.content { ContentKind::Buffer(id) => id, _ => return };
        let cursor = win.cursor;
        let range = if let Some(buf) = self.buffers.get(&buf_id) {
            kjxlkj_core_edit::text_obj_range(&cursor.into(), buf, ch, inner)
        } else { return };
        let (sc, ec) = match range { Some((s, e)) => (s, e), None => return };
        let (sl, scol, el, ecol) = (sc.line, sc.col, ec.line, ec.col);
        if matches!(op, Operator::Uppercase | Operator::Lowercase | Operator::ToggleCase) {
            self.apply_range_case_op(op, sl, scol, el, ecol + 1);
            let w = self.focused_window_mut();
            w.cursor.line = sl; w.cursor.col = scol;
            return;
        }
        let text = if let Some(buf) = self.buffers.get(&buf_id) {
            buf.text_range(sl, scol, el, ecol + 1)
        } else { String::new() };
        let scope = if sl != el { RangeType::Linewise } else { RangeType::Characterwise };
        match op {
            Operator::Yank => {
                self.registers.record_yank(text, scope);
                let w = self.focused_window_mut();
                w.cursor.line = sl; w.cursor.col = scol;
            }
            Operator::Delete | Operator::Change => {
                if let Some(buf) = self.buffers.get_mut(&buf_id) {
                    let _ = buf.delete(sl, scol, el, ecol + 1);
                }
                self.registers.record_delete(text, scope);
                let w = self.focused_window_mut();
                w.cursor.line = sl; w.cursor.col = scol;
            }
            _ => {
                let w = self.focused_window_mut();
                w.cursor.line = sl; w.cursor.col = scol;
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

    /// Insert the contents of register `reg` at cursor position.
    pub(crate) fn insert_register_contents(&mut self, reg: char) {
        let text = match self.registers.get(reg) {
            Some(e) => e.text.clone(),
            None => return,
        };
        for ch in text.chars() { self.insert_char(ch); }
    }
}

fn ordered_range(
    sl: usize, sc: usize, el: usize, ec: usize,
) -> (usize, usize, usize, usize) {
    if (sl, sc) <= (el, ec) { (sl, sc, el, ec) } else { (el, ec, sl, sc) }
}
