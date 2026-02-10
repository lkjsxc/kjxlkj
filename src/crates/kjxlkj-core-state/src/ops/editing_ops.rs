//! Editing operations dispatched from editor state.

use kjxlkj_core_edit::{
    apply_motion, apply_operator, delete_char_backward, delete_char_forward, insert_char_at,
    insert_newline_above, insert_newline_below, join_lines, replace_char_at, CursorPosition,
    RegisterType,
};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Mode, MotionAction};

use super::cursor_ops;
use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn do_insert_append(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(|buf, cur| cursor_ops::cursor_append(cur, buf));
        self.mode = Mode::Insert;
    }
    pub(crate) fn do_insert_append_eol(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(|buf, cur| cursor_ops::cursor_append_eol(cur, buf));
        self.mode = Mode::Insert;
    }
    pub(crate) fn do_insert_first_nonblank(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(|buf, cur| {
            cursor_ops::cursor_insert_first_nonblank(cur, buf);
        });
        self.mode = Mode::Insert;
    }
    pub(crate) fn do_insert_char(&mut self, c: char) {
        self.with_active_buffer(|buf, cur| insert_char_at(buf, cur, c));
    }
    pub(crate) fn do_delete_forward(&mut self) {
        self.with_active_buffer(delete_char_forward);
    }
    pub(crate) fn do_delete_backward(&mut self) {
        self.with_active_buffer(delete_char_backward);
    }
    pub(crate) fn do_replace_char(&mut self, c: char) {
        let mut old = None;
        self.with_active_buffer(|buf, cur| {
            old = replace_char_at(buf, cur, c);
        });
        self.replace_stack.push(old);
    }
    pub(crate) fn do_replace_backspace(&mut self) {
        if let Some(orig) = self.replace_stack.pop() {
            self.with_active_buffer(|buf, cur| {
                if cur.grapheme_offset > 0 {
                    cur.grapheme_offset -= 1;
                    if let Some(ch) = orig {
                        delete_char_forward(buf, cur);
                        insert_char_at(buf, cur, ch);
                        cur.grapheme_offset -= 1;
                    }
                }
            });
        }
    }
    pub(crate) fn do_motion(&mut self, motion: MotionAction) {
        let count = self.dispatcher.take_count();
        self.with_active_buffer(|buf, cur| apply_motion(&motion, cur, buf, count));
        self.scroll_active_window();
    }
    pub(crate) fn do_operator(
        &mut self,
        op: kjxlkj_core_types::Operator,
        motion: MotionAction,
        count: usize,
    ) {
        self.save_undo_checkpoint();
        let reg_target = self.pending_register.take();
        let mut reg_content = String::new();
        self.with_active_buffer(|buf, cur| {
            apply_operator(&op, &motion, count, cur, buf, &mut reg_content);
        });
        if !reg_content.is_empty() {
            let is_lw = reg_content.ends_with('\n');
            let rtype = if is_lw {
                RegisterType::Linewise
            } else {
                RegisterType::Charwise
            };
            match op {
                kjxlkj_core_types::Operator::Yank => {
                    self.registers.store_yank(reg_target, reg_content, rtype);
                }
                kjxlkj_core_types::Operator::Delete | kjxlkj_core_types::Operator::Change => {
                    self.registers
                        .store_delete(reg_target, reg_content, rtype, is_lw);
                }
                _ => {}
            }
        }
        if op == kjxlkj_core_types::Operator::Change {
            self.mode = Mode::Insert;
        } else {
            self.mode = Mode::Normal;
        }
    }
    pub(crate) fn do_open_line_below(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(insert_newline_below);
        self.mode = Mode::Insert;
    }
    pub(crate) fn do_open_line_above(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(insert_newline_above);
        self.mode = Mode::Insert;
    }
    pub(crate) fn do_join_lines(&mut self) {
        self.save_undo_checkpoint();
        self.with_active_buffer(join_lines);
    }
    pub(crate) fn do_window_split_h(&mut self) {
        if let Some(id) = self.active_buffer_id() {
            let wid = self.windows.next_window_id();
            let w = crate::window_tree::Window::new_buffer(wid, id);
            self.windows.active_tab_mut().split_horizontal(w);
        }
    }
    pub(crate) fn do_window_split_v(&mut self) {
        if let Some(id) = self.active_buffer_id() {
            let wid = self.windows.next_window_id();
            let w = crate::window_tree::Window::new_buffer(wid, id);
            self.windows.active_tab_mut().split_vertical(w);
        }
    }
    pub(crate) fn do_window_close(&mut self) {
        let empty = self.windows.active_tab_mut().close_active();
        if empty && self.windows.close_active_tab() {
            self.try_quit(false);
        }
    }
    pub(crate) fn do_viewport_center(&mut self) {
        let h = self.terminal_size.1.saturating_sub(2) as usize;
        let win = self.windows.active_tab_mut().active_mut();
        cursor_ops::viewport_center(win.cursor_line, &mut win.top_line, h);
    }
    pub(crate) fn do_viewport_top(&mut self) {
        let win = self.windows.active_tab_mut().active_mut();
        cursor_ops::viewport_top(win.cursor_line, &mut win.top_line);
    }
    pub(crate) fn do_viewport_bottom(&mut self) {
        let h = self.terminal_size.1.saturating_sub(2) as usize;
        let win = self.windows.active_tab_mut().active_mut();
        cursor_ops::viewport_bottom(win.cursor_line, &mut win.top_line, h);
    }
    pub(crate) fn scroll_active_window(&mut self) {
        let h = self.terminal_size.1.saturating_sub(2) as usize;
        let w = self.terminal_size.0 as usize;
        let win = self.windows.active_tab_mut().active_mut();
        let (scrolloff, cursor_line) = (win.scrolloff, win.cursor_line);
        cursor_ops::scroll_to_cursor(
            &CursorPosition::new(cursor_line, 0),
            &mut win.top_line,
            h,
            scrolloff,
        );
        if !win.wrap {
            let (so, cc) = (win.sidescrolloff, win.cursor_offset);
            cursor_ops::scroll_horizontal(cc, &mut win.left_col, w, so);
        }
    }
    pub(crate) fn with_active_buffer<F>(&mut self, f: F)
    where
        F: FnOnce(&mut TextBuffer, &mut CursorPosition),
    {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let win = self.windows.active_tab_mut().active_mut();
        let mut cur = CursorPosition::new(win.cursor_line, win.cursor_offset);
        if let Some(buf) = self.buffers.get_mut(&buf_id) {
            f(buf, &mut cur);
        }
        win.cursor_line = cur.line;
        win.cursor_offset = cur.grapheme_offset;
    }
}
