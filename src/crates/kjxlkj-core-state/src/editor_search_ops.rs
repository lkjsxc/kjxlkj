//! Search operations for EditorState.

use kjxlkj_core_types::Mode;

use crate::EditorState;

impl EditorState {
    /// Handle search forward command.
    pub(crate) fn do_search_forward(&mut self, pattern: String) {
        let content = self.active_buffer().map(|b| b.content.clone());
        if let Some(content) = content {
            self.search_state.search(&pattern, &content, true);
            let cursor_pos = self.cursor_pos();
            let cursor = kjxlkj_core_edit::CursorPosition::new(cursor_pos.0, cursor_pos.1);
            if let Some(pos) = self.search_state.next_match(&cursor) {
                if let Some(w) = self.focused_window_mut() {
                    w.cursor.line = pos.line;
                    w.cursor.grapheme_offset = pos.grapheme_offset;
                    w.viewport.follow_cursor(pos.line, 3, 0);
                }
            }
        }
        self.mode = Mode::Normal;
        self.command_state = None;
    }

    /// Handle search backward command.
    pub(crate) fn do_search_backward(&mut self, pattern: String) {
        let content = self.active_buffer().map(|b| b.content.clone());
        if let Some(content) = content {
            self.search_state.search(&pattern, &content, false);
            let cursor_pos = self.cursor_pos();
            let cursor = kjxlkj_core_edit::CursorPosition::new(cursor_pos.0, cursor_pos.1);
            if let Some(pos) = self.search_state.prev_match(&cursor) {
                if let Some(w) = self.focused_window_mut() {
                    w.cursor.line = pos.line;
                    w.cursor.grapheme_offset = pos.grapheme_offset;
                    w.viewport.follow_cursor(pos.line, 3, 0);
                }
            }
        }
        self.mode = Mode::Normal;
        self.command_state = None;
    }

    /// Jump to next search match.
    pub(crate) fn do_next_match(&mut self) {
        let cursor_pos = self.cursor_pos();
        let cursor = kjxlkj_core_edit::CursorPosition::new(cursor_pos.0, cursor_pos.1);
        if let Some(pos) = self.search_state.next_match(&cursor) {
            if let Some(w) = self.focused_window_mut() {
                w.cursor.line = pos.line;
                w.cursor.grapheme_offset = pos.grapheme_offset;
                w.viewport.follow_cursor(pos.line, 3, 0);
            }
        }
    }

    /// Jump to previous search match.
    pub(crate) fn do_prev_match(&mut self) {
        let cursor_pos = self.cursor_pos();
        let cursor = kjxlkj_core_edit::CursorPosition::new(cursor_pos.0, cursor_pos.1);
        if let Some(pos) = self.search_state.prev_match(&cursor) {
            if let Some(w) = self.focused_window_mut() {
                w.cursor.line = pos.line;
                w.cursor.grapheme_offset = pos.grapheme_offset;
                w.viewport.follow_cursor(pos.line, 3, 0);
            }
        }
    }
}
