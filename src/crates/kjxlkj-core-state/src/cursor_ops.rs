/// Cursor movement operations for EditorState.
use kjxlkj_core_types::{BufferId, ContentSource, CursorPosition};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn current_buffer_id(&self) -> BufferId {
        match &self.windows.focused().content {
            ContentSource::Buffer(id) => *id,
            ContentSource::Terminal(_) => self.buffers.current_id(),
        }
    }

    pub(crate) fn move_cursor_up(&mut self, n: usize) {
        let cursor = &mut self.windows.focused_mut().cursor;
        cursor.line = cursor.line.saturating_sub(n);
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub(crate) fn move_cursor_down(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        let line_count = self
            .buffers
            .get(buf_id)
            .map(|b| b.line_count())
            .unwrap_or(1);
        let cursor = &mut self.windows.focused_mut().cursor;
        cursor.line = (cursor.line + n).min(line_count.saturating_sub(1));
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub(crate) fn move_cursor_left(&mut self, n: usize) {
        let cursor = &mut self.windows.focused_mut().cursor;
        cursor.grapheme = cursor.grapheme.saturating_sub(n);
    }

    pub(crate) fn move_cursor_right(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        let max_g = self.line_grapheme_count(buf_id);
        let cursor = &mut self.windows.focused_mut().cursor;
        cursor.grapheme = (cursor.grapheme + n).min(max_g.saturating_sub(1));
    }

    pub(crate) fn move_to_line_start(&mut self) {
        self.windows.focused_mut().cursor.grapheme = 0;
    }

    pub(crate) fn move_to_first_non_blank(&mut self) {
        let buf_id = self.current_buffer_id();
        let line = self.windows.focused().cursor.line;
        if let Some(buf) = self.buffers.get(buf_id) {
            let line_slice = buf.content.line(line);
            let s: std::borrow::Cow<str> = line_slice.into();
            let mut g = 0;
            for ch in s.chars() {
                if ch != ' ' && ch != '\t' {
                    break;
                }
                g += 1;
            }
            self.windows.focused_mut().cursor.grapheme = g;
        }
    }

    pub(crate) fn move_to_line_end(&mut self) {
        let buf_id = self.current_buffer_id();
        let max_g = self.line_grapheme_count(buf_id);
        self.windows.focused_mut().cursor.grapheme = max_g.saturating_sub(1);
    }

    pub(crate) fn move_to_line_end_insert(&mut self) {
        let buf_id = self.current_buffer_id();
        let max_g = self.line_grapheme_count(buf_id);
        self.windows.focused_mut().cursor.grapheme = max_g;
    }

    pub(crate) fn move_word_forward(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let pos = self.windows.focused().cursor;
            let (new_pos, _) = kjxlkj_core_edit::resolve_motion(
                &kjxlkj_core_edit::Motion::WordForward(n),
                pos,
                &buf.content,
                self.viewport_height(),
            );
            self.windows.focused_mut().cursor = new_pos;
        }
    }

    pub(crate) fn move_word_backward(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let pos = self.windows.focused().cursor;
            let (new_pos, _) = kjxlkj_core_edit::resolve_motion(
                &kjxlkj_core_edit::Motion::WordBackward(n),
                pos,
                &buf.content,
                self.viewport_height(),
            );
            self.windows.focused_mut().cursor = new_pos;
        }
    }

    pub(crate) fn move_word_end_forward(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get(buf_id) {
            let pos = self.windows.focused().cursor;
            let (new_pos, _) = kjxlkj_core_edit::resolve_motion(
                &kjxlkj_core_edit::Motion::WordEndForward(n),
                pos,
                &buf.content,
                self.viewport_height(),
            );
            self.windows.focused_mut().cursor = new_pos;
        }
    }

    pub(crate) fn move_to_top(&mut self) {
        self.windows.focused_mut().cursor = CursorPosition::new(0, 0);
        self.ensure_cursor_visible();
    }

    pub(crate) fn move_to_bottom(&mut self) {
        let buf_id = self.current_buffer_id();
        let line_count = self
            .buffers
            .get(buf_id)
            .map(|b| b.line_count())
            .unwrap_or(1);
        let last_line = line_count.saturating_sub(1);
        self.windows.focused_mut().cursor = CursorPosition::new(last_line, 0);
        self.ensure_cursor_visible();
    }

    pub(crate) fn move_to_line(&mut self, n: usize) {
        let buf_id = self.current_buffer_id();
        let line_count = self
            .buffers
            .get(buf_id)
            .map(|b| b.line_count())
            .unwrap_or(1);
        let line = n.min(line_count.saturating_sub(1));
        self.windows.focused_mut().cursor = CursorPosition::new(line, 0);
        self.ensure_cursor_visible();
    }

    /// Push a position onto the changelist.
    pub(crate) fn push_changelist(&mut self, line: usize, col: usize) {
        let bid = self.current_buffer_id().0 as usize;
        self.changelist.push((bid, line, col));
        self.changelist_idx = self.changelist.len();
    }

    /// Navigate to older changelist entry (g;).
    pub(crate) fn changelist_older(&mut self) {
        if self.changelist.is_empty() || self.changelist_idx == 0 {
            self.notify_error("E662: At start of changelist");
            return;
        }
        self.changelist_idx -= 1;
        let (_, line, col) = self.changelist[self.changelist_idx];
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }

    /// Navigate to newer changelist entry (g,).
    pub(crate) fn changelist_newer(&mut self) {
        if self.changelist_idx >= self.changelist.len().saturating_sub(1) {
            self.notify_error("E663: At end of changelist");
            return;
        }
        self.changelist_idx += 1;
        let (_, line, col) = self.changelist[self.changelist_idx];
        self.windows.focused_mut().cursor = CursorPosition::new(line, col);
        self.ensure_cursor_visible();
    }
}
