//! Operator execution: double operators, operator+motion,
//! char-level edits (join, toggle case, replace, increment).

use kjxlkj_core_edit::{execute_motion, CursorPosition};
use kjxlkj_core_types::{Motion, Operator};

use crate::EditorState;

impl EditorState {
    /// Execute dd, cc, yy etc.
    pub(crate) fn do_double_operator(&mut self, op: Operator, count: u32) {
        let (line, _col) = self.cursor_pos();
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let mut enter_insert = false;
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let end_line = (line + count as usize).min(buf.line_count());
            let start = buf.content.line_start_offset(line);
            let end = if end_line >= buf.line_count() {
                buf.content.len_chars()
            } else {
                buf.content.line_start_offset(end_line)
            };
            match op {
                Operator::Delete => {
                    buf.content.delete_range(start, end);
                    buf.modified = true;
                }
                Operator::Yank => {}
                Operator::Change => {
                    buf.content.delete_range(start, end);
                    buf.modified = true;
                    enter_insert = true;
                }
                _ => {}
            }
        }
        if enter_insert {
            self.mode = kjxlkj_core_types::Mode::Insert;
        }
        let max_line = self
            .active_buffer()
            .map(|b| b.line_count().saturating_sub(1))
            .unwrap_or(0);
        if let Some(w) = self.focused_window_mut() {
            if w.cursor.line > max_line {
                w.cursor.line = max_line;
            }
            w.cursor.grapheme_offset = 0;
        }
    }

    /// Execute operator + motion.
    pub(crate) fn do_operator_motion_action(&mut self, op: Operator, motion: Motion, count: u32) {
        let (line, col) = self.cursor_pos();
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let (end_line, end_col) = {
            let buf = match self.buffers.get(&bid) {
                Some(b) => b,
                None => return,
            };
            let mut cursor = CursorPosition::new(line, col);
            let result = execute_motion(&mut cursor, &motion, count, &buf.content);
            (result.line, result.grapheme_offset)
        };
        let mut enter_insert = false;
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let start = buf.content.line_grapheme_to_offset(line, col);
            let end = buf.content.line_grapheme_to_offset(end_line, end_col);
            let (s, e) = if start <= end {
                (start, end + 1)
            } else {
                (end, start + 1)
            };
            let e = e.min(buf.content.len_chars());
            match op {
                Operator::Delete => {
                    buf.content.delete_range(s, e);
                    buf.modified = true;
                }
                Operator::Change => {
                    buf.content.delete_range(s, e);
                    buf.modified = true;
                    enter_insert = true;
                }
                Operator::Yank => {}
                _ => {}
            }
        }
        if enter_insert {
            self.mode = kjxlkj_core_types::Mode::Insert;
        }
        let max = self
            .active_buffer()
            .map(|b| b.line_count().saturating_sub(1))
            .unwrap_or(0);
        if let Some(w) = self.focused_window_mut() {
            if w.cursor.line > max {
                w.cursor.line = max;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;

    #[test]
    fn double_operator_delete() {
        let mut state = EditorState::new(80, 24);
        // Set up a buffer with multiple lines.
        state.mode = kjxlkj_core_types::Mode::Insert;
        state.insert_char('a');
        state.insert_char('\n');
        state.insert_char('b');
        state.insert_char('\n');
        state.insert_char('c');
        state.mode = kjxlkj_core_types::Mode::Normal;
        // Move cursor to line 0.
        if let Some(w) = state.focused_window_mut() {
            w.cursor.line = 0;
            w.cursor.grapheme_offset = 0;
        }
        let initial = state.active_buffer().unwrap().line_count();
        state.do_double_operator(kjxlkj_core_types::Operator::Delete, 1);
        let after = state.active_buffer().unwrap().line_count();
        assert!(after < initial, "expected {after} < {initial}",);
    }
}
