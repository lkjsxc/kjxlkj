use super::EditorState;

impl EditorState {
    pub(super) fn move_left(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        if win.cursor.col > 0 {
            win.cursor.col -= 1;
            return;
        }
        if win.cursor.line == 0 {
            return;
        }
        win.cursor.line -= 1;
        win.cursor.col = buf.text.line_len_chars_no_nl(win.cursor.line).unwrap_or(0);
    }

    pub(super) fn move_right(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        let max_col = buf.text.line_len_chars_no_nl(win.cursor.line).unwrap_or(0);
        if win.cursor.col < max_col {
            win.cursor.col += 1;
            return;
        }
        let next_line = win.cursor.line.saturating_add(1);
        if next_line >= buf.text.line_count() {
            return;
        }
        win.cursor.line = next_line;
        win.cursor.col = 0;
    }

    pub(super) fn move_right_in_line(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        let max_col = buf.text.line_len_chars_no_nl(win.cursor.line).unwrap_or(0);
        if win.cursor.col < max_col {
            win.cursor.col += 1;
        }
    }

    pub(super) fn move_to_line_end(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        win.cursor.col = buf.text.line_len_chars_no_nl(win.cursor.line).unwrap_or(0);
    }

    pub(super) fn move_up(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        if win.cursor.line == 0 {
            return;
        }
        win.cursor.line -= 1;
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }

    pub(super) fn move_down(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        let next = win.cursor.line.saturating_add(1);
        if next >= buf.text.line_count() {
            return;
        }
        win.cursor.line = next;
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }
}
