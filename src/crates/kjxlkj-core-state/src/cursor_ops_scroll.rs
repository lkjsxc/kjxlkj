/// Scroll and viewport operations for EditorState.
use kjxlkj_core_types::BufferId;

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn page_up(&mut self) {
        let h = self.viewport_height();
        self.move_cursor_up(h);
    }

    pub(crate) fn page_down(&mut self) {
        let h = self.viewport_height();
        self.move_cursor_down(h);
    }

    pub(crate) fn half_page_up(&mut self) {
        let h = self.viewport_height() / 2;
        self.move_cursor_up(h.max(1));
    }

    pub(crate) fn half_page_down(&mut self) {
        let h = self.viewport_height() / 2;
        self.move_cursor_down(h.max(1));
    }

    pub(crate) fn viewport_height(&self) -> usize {
        (self.terminal_size.1 as usize).saturating_sub(2)
    }

    pub(crate) fn line_grapheme_count(&self, buf_id: BufferId) -> usize {
        let line = self.windows.focused().cursor.line;
        if let Some(buf) = self.buffers.get(buf_id) {
            if line < buf.content.len_lines() {
                let line_slice = buf.content.line(line);
                let s: std::borrow::Cow<str> = line_slice.into();
                let trimmed = s.trim_end_matches(&['\n', '\r'][..]);
                unicode_segmentation::UnicodeSegmentation::graphemes(trimmed, true).count()
            } else {
                0
            }
        } else {
            0
        }
    }

    pub(crate) fn clamp_cursor(&mut self) {
        let buf_id = self.current_buffer_id();
        let max_g = self.line_grapheme_count(buf_id);
        let cursor = &mut self.windows.focused_mut().cursor;
        if max_g == 0 {
            cursor.grapheme = 0;
        } else if cursor.grapheme >= max_g {
            cursor.grapheme = max_g.saturating_sub(1);
        }
    }

    pub(crate) fn ensure_cursor_visible(&mut self) {
        let cursor_line = self.windows.focused().cursor.line;
        let scrolloff = self.windows.focused().scrolloff;
        let top = self.windows.focused().top_line;
        let vh = self.viewport_height();

        if cursor_line < top + scrolloff {
            self.windows.focused_mut().top_line = cursor_line.saturating_sub(scrolloff);
        } else if cursor_line >= top + vh - scrolloff {
            self.windows.focused_mut().top_line = cursor_line + scrolloff + 1 - vh;
        }
    }

    pub(crate) fn scroll_center_cursor(&mut self) {
        let cursor_line = self.windows.focused().cursor.line;
        let vh = self.viewport_height();
        self.windows.focused_mut().top_line = cursor_line.saturating_sub(vh / 2);
    }

    pub(crate) fn scroll_cursor_top(&mut self) {
        let cursor_line = self.windows.focused().cursor.line;
        self.windows.focused_mut().top_line = cursor_line;
    }

    pub(crate) fn scroll_cursor_bottom(&mut self) {
        let cursor_line = self.windows.focused().cursor.line;
        let vh = self.viewport_height();
        self.windows.focused_mut().top_line = cursor_line.saturating_sub(vh.saturating_sub(1));
    }
}
