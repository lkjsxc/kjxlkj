//! Editor window navigation and write-all operations.

use kjxlkj_core_types::{Direction, Mode, WindowId};

use crate::EditorState;

impl EditorState {
    /// Write all modified buffers.
    pub(crate) fn do_write_all(&mut self) {
        for buf in self.buffers.values_mut() {
            if buf.modified {
                buf.modified = false;
            }
        }
    }

    /// Focus the next window in a direction.
    /// Since windows don't have spatial positions in our
    /// simplified model, we cycle through them.
    pub(crate) fn do_focus_window(
        &mut self,
        direction: Direction,
    ) {
        let ids: Vec<WindowId> =
            self.windows.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let idx = ids
            .iter()
            .position(|&i| i == self.focused_window)
            .unwrap_or(0);
        let next = match direction {
            Direction::Left | Direction::Up => {
                if idx == 0 {
                    ids[ids.len() - 1]
                } else {
                    ids[idx - 1]
                }
            }
            Direction::Right | Direction::Down => {
                ids[(idx + 1) % ids.len()]
            }
        };
        self.focused_window = next;
    }

    /// Cycle to the next window.
    pub(crate) fn do_cycle_window(&mut self) {
        let ids: Vec<WindowId> =
            self.windows.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let idx = ids
            .iter()
            .position(|&i| i == self.focused_window)
            .unwrap_or(0);
        let next = ids[(idx + 1) % ids.len()];
        self.focused_window = next;
    }

    /// Close the current window.
    pub(crate) fn do_close_window(&mut self) {
        if self.windows.len() <= 1 {
            self.should_quit = true;
            return;
        }
        self.windows.remove(&self.focused_window);
        if let Some(&id) =
            self.windows.keys().next()
        {
            self.focused_window = id;
        }
    }

    /// Handle replace mode character overwrite.
    pub(crate) fn do_replace_char_at_cursor(
        &mut self,
        ch: char,
    ) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off = buf
                .content
                .line_grapheme_to_offset(line, col);
            if off < buf.content.len_chars() {
                buf.content
                    .delete_range(off, off + 1);
                buf.content.insert_char(off, ch);
            } else {
                buf.content.insert_char(off, ch);
            }
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
    }

    /// Handle search forward command.
    pub(crate) fn do_search_forward(
        &mut self,
        pattern: String,
    ) {
        let content = self
            .active_buffer()
            .map(|b| b.content.clone());
        if let Some(content) = content {
            self.search_state
                .search(&pattern, &content, true);
            let cursor_pos = self.cursor_pos();
            let cursor =
                kjxlkj_core_edit::CursorPosition::new(
                    cursor_pos.0,
                    cursor_pos.1,
                );
            if let Some(pos) =
                self.search_state.next_match(&cursor)
            {
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.cursor.line = pos.line;
                    w.cursor.grapheme_offset =
                        pos.grapheme_offset;
                    w.viewport
                        .follow_cursor(pos.line, 3, 0);
                }
            }
        }
        self.mode = Mode::Normal;
        self.command_state = None;
    }

    /// Handle search backward command.
    pub(crate) fn do_search_backward(
        &mut self,
        pattern: String,
    ) {
        let content = self
            .active_buffer()
            .map(|b| b.content.clone());
        if let Some(content) = content {
            self.search_state
                .search(&pattern, &content, false);
            let cursor_pos = self.cursor_pos();
            let cursor =
                kjxlkj_core_edit::CursorPosition::new(
                    cursor_pos.0,
                    cursor_pos.1,
                );
            if let Some(pos) =
                self.search_state.prev_match(&cursor)
            {
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.cursor.line = pos.line;
                    w.cursor.grapheme_offset =
                        pos.grapheme_offset;
                    w.viewport
                        .follow_cursor(pos.line, 3, 0);
                }
            }
        }
        self.mode = Mode::Normal;
        self.command_state = None;
    }

    /// Jump to next search match.
    pub(crate) fn do_next_match(&mut self) {
        let cursor_pos = self.cursor_pos();
        let cursor =
            kjxlkj_core_edit::CursorPosition::new(
                cursor_pos.0,
                cursor_pos.1,
            );
        if let Some(pos) =
            self.search_state.next_match(&cursor)
        {
            if let Some(w) = self.focused_window_mut()
            {
                w.cursor.line = pos.line;
                w.cursor.grapheme_offset =
                    pos.grapheme_offset;
                w.viewport
                    .follow_cursor(pos.line, 3, 0);
            }
        }
    }

    /// Jump to previous search match.
    pub(crate) fn do_prev_match(&mut self) {
        let cursor_pos = self.cursor_pos();
        let cursor =
            kjxlkj_core_edit::CursorPosition::new(
                cursor_pos.0,
                cursor_pos.1,
            );
        if let Some(pos) =
            self.search_state.prev_match(&cursor)
        {
            if let Some(w) = self.focused_window_mut()
            {
                w.cursor.line = pos.line;
                w.cursor.grapheme_offset =
                    pos.grapheme_offset;
                w.viewport
                    .follow_cursor(pos.line, 3, 0);
            }
        }
    }
}
