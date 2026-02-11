//! Operator and extended edit operations.
//!
//! Implements operator-line, operator-motion, and extended
//! delete/change/substitute operations.

use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{ContentKind, Motion, Operator};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn apply_operator_line(
        &mut self,
        op: Operator,
    ) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let gc = buf.line_grapheme_count(line);
                match op {
                    Operator::Delete | Operator::Change => {
                        let _ =
                            buf.delete(line, 0, line, gc);
                        if buf.line_count() > 1 {
                            let _ = buf.delete(
                                line, 0, line, 1,
                            );
                        }
                    }
                    Operator::Yank => {
                        // TODO: yank register
                    }
                    _ => {}
                }
            }
        }
    }

    pub(crate) fn apply_operator_motion(
        &mut self,
        op: Operator,
        motion: Motion,
        count: usize,
    ) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let mut cursor = win.cursor;
            if let Some(buf) = self.buffers.get(&buf_id) {
                for _ in 0..count {
                    cursor =
                        apply_motion(&cursor, &motion, buf);
                }
            }
            let start_line = win.cursor.line;
            let start_col = win.cursor.col;
            let end_line = cursor.line;
            let end_col = cursor.col;
            let (sl, sc, el, ec) =
                if (start_line, start_col)
                    <= (end_line, end_col)
                {
                    (start_line, start_col, end_line, end_col)
                } else {
                    (end_line, end_col, start_line, start_col)
                };
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                match op {
                    Operator::Delete | Operator::Change => {
                        let _ =
                            buf.delete(sl, sc, el, ec);
                    }
                    Operator::Yank => {}
                    _ => {}
                }
                let w = self.focused_window_mut();
                w.cursor.line = sl;
                w.cursor.col = sc;
            }
        }
    }

    pub(crate) fn delete_current_line_content(
        &mut self,
    ) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let gc = buf.line_grapheme_count(line);
                if gc > 0 {
                    let _ =
                        buf.delete(line, 0, line, gc);
                }
                let w = self.focused_window_mut();
                w.cursor.col = 0;
            }
        }
    }

    pub(crate) fn delete_to_eol(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if let Some(buf) =
                self.buffers.get_mut(&buf_id)
            {
                let gc = buf.line_grapheme_count(line);
                if col < gc {
                    let _ =
                        buf.delete(line, col, line, gc);
                }
            }
        }
    }

    pub(crate) fn delete_word_backward(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let cursor = win.cursor;
            if let Some(buf) = self.buffers.get(&buf_id) {
                let new_cur = apply_motion(
                    &cursor,
                    &Motion::WordBackward,
                    buf,
                );
                if let Some(buf) =
                    self.buffers.get_mut(&buf_id)
                {
                    let _ = buf.delete(
                        new_cur.line,
                        new_cur.col,
                        cursor.line,
                        cursor.col,
                    );
                    let w = self.focused_window_mut();
                    w.cursor = new_cur;
                }
            }
        }
    }

    pub(crate) fn delete_to_line_start(&mut self) {
        let win =
            self.windows.get(&self.focus.focused).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            let line = win.cursor.line;
            let col = win.cursor.col;
            if col > 0 {
                if let Some(buf) =
                    self.buffers.get_mut(&buf_id)
                {
                    let _ =
                        buf.delete(line, 0, line, col);
                    let w = self.focused_window_mut();
                    w.cursor.col = 0;
                }
            }
        }
    }
}
