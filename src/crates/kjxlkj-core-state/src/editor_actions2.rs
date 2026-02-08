//! Editor actions: undo, put, operators, file ops.

use kjxlkj_core_edit::{execute_motion, CursorPosition};
use kjxlkj_core_types::{BufferId, Motion, Operator};

use crate::{BufferState, EditorState, WindowContent, WindowState};

impl EditorState {
    pub(crate) fn do_undo(&mut self) {
        if let Some(buf) = self.active_buffer_mut() {
            buf.undo();
        }
    }

    pub(crate) fn do_redo(&mut self) {
        if let Some(buf) = self.active_buffer_mut() {
            buf.redo();
        }
    }

    pub(crate) fn do_put(&mut self, _before: bool) {
        // Stub: paste from register.
    }

    /// Execute dd, cc, yy etc.
    pub(crate) fn do_double_operator(
        &mut self,
        op: Operator,
        count: u32,
    ) {
        let (line, _col) = self.cursor_pos();
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let mut enter_insert = false;
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let end_line = (line + count as usize)
                .min(buf.line_count());
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
        // Clamp cursor after delete.
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
    pub(crate) fn do_operator_motion_action(
        &mut self,
        op: Operator,
        motion: Motion,
        count: u32,
    ) {
        let (line, col) = self.cursor_pos();
        // Compute motion endpoint.
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
            let result = execute_motion(
                &mut cursor, &motion, count, &buf.content,
            );
            (result.line, result.grapheme_offset)
        };
        // Compute char range.
        let mut enter_insert = false;
        if let Some(buf) = self.buffers.get_mut(&bid) {
            let start = buf
                .content
                .line_grapheme_to_offset(line, col);
            let end = buf
                .content
                .line_grapheme_to_offset(end_line, end_col);
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
        // Clamp cursor.
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

    pub(crate) fn do_join(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            if line + 1 < buf.line_count() {
                let end = buf.content.line_end_offset(line);
                let next_start = end + 1;
                if next_start <= buf.content.len_chars() {
                    buf.content.delete_range(end, next_start);
                    buf.content.insert_char(end, ' ');
                    buf.modified = true;
                }
            }
        }
    }

    pub(crate) fn do_toggle_case(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off =
                buf.content.line_grapheme_to_offset(line, col);
            if let Some(ch) = buf.content.char_at(off) {
                let toggled = if ch.is_uppercase() {
                    ch.to_lowercase().next().unwrap_or(ch)
                } else {
                    ch.to_uppercase().next().unwrap_or(ch)
                };
                buf.content.delete_range(off, off + 1);
                buf.content.insert_char(off, toggled);
                buf.modified = true;
            }
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
    }

    pub(crate) fn do_replace_char(&mut self, ch: char) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off =
                buf.content.line_grapheme_to_offset(line, col);
            if off < buf.content.len_chars() {
                buf.content.delete_range(off, off + 1);
                buf.content.insert_char(off, ch);
                buf.modified = true;
            }
        }
    }

    pub(crate) fn do_increment(&mut self, _n: i64) {
        // Stub: increment number under cursor.
    }

    pub(crate) fn do_write(&mut self) {
        if let Some(buf) = self.active_buffer_mut() {
            buf.modified = false;
        }
    }

    pub(crate) fn do_open_file(
        &mut self,
        path: &std::path::Path,
    ) {
        let buf_id = self.alloc_buffer_id();
        let buf = BufferState::new_with_path(
            buf_id,
            path.to_path_buf(),
        );
        self.buffers.insert(buf_id, buf);
        if let Some(w) = self.focused_window_mut() {
            w.content = WindowContent::Buffer(buf_id);
            w.cursor.line = 0;
            w.cursor.grapheme_offset = 0;
        }
    }

    pub(crate) fn do_next_buffer(&mut self) {
        let ids: Vec<BufferId> =
            self.buffers.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let current = self.active_buffer_id();
        let idx = current
            .and_then(|c| ids.iter().position(|&i| i == c))
            .unwrap_or(0);
        let next = ids[(idx + 1) % ids.len()];
        if let Some(w) = self.focused_window_mut() {
            w.content = WindowContent::Buffer(next);
        }
    }

    pub(crate) fn do_prev_buffer(&mut self) {
        let ids: Vec<BufferId> =
            self.buffers.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let current = self.active_buffer_id();
        let idx = current
            .and_then(|c| ids.iter().position(|&i| i == c))
            .unwrap_or(0);
        let prev = if idx == 0 {
            ids[ids.len() - 1]
        } else {
            ids[idx - 1]
        };
        if let Some(w) = self.focused_window_mut() {
            w.content = WindowContent::Buffer(prev);
        }
    }

    pub(crate) fn do_delete_buffer(&mut self) {
        if let Some(buf_id) = self.active_buffer_id() {
            self.buffers.remove(&buf_id);
            if self.buffers.is_empty() {
                let id = self.alloc_buffer_id();
                let buf = BufferState::new(id);
                self.buffers.insert(id, buf);
                if let Some(w) = self.focused_window_mut() {
                    w.content = WindowContent::Buffer(id);
                }
            }
        }
    }

    pub(crate) fn do_split_horizontal(&mut self) {
        if let Some(bid) = self.active_buffer_id() {
            let win_id = self.alloc_window_id();
            let (cols, rows) = self.terminal_size;
            let mut win =
                WindowState::new_buffer(win_id, bid);
            win.viewport
                .set_size(cols, rows.saturating_sub(2) / 2);
            self.windows.insert(win_id, win);
        }
    }

    pub(crate) fn do_split_vertical(&mut self) {
        if let Some(bid) = self.active_buffer_id() {
            let win_id = self.alloc_window_id();
            let (cols, rows) = self.terminal_size;
            let mut win =
                WindowState::new_buffer(win_id, bid);
            win.viewport
                .set_size(cols / 2, rows.saturating_sub(2));
            self.windows.insert(win_id, win);
        }
    }
}
