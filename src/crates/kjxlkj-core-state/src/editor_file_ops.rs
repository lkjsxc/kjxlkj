//! Editor file and buffer management operations.

use kjxlkj_core_types::BufferId;

use crate::{
    BufferState, EditorState, WindowContent,
    WindowState,
};

impl EditorState {
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
            .and_then(|c| {
                ids.iter().position(|&i| i == c)
            })
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
            .and_then(|c| {
                ids.iter().position(|&i| i == c)
            })
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
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.content =
                        WindowContent::Buffer(id);
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
            win.viewport.set_size(
                cols,
                rows.saturating_sub(2) / 2,
            );
            self.windows.insert(win_id, win);
        }
    }

    pub(crate) fn do_split_vertical(&mut self) {
        if let Some(bid) = self.active_buffer_id() {
            let win_id = self.alloc_window_id();
            let (cols, rows) = self.terminal_size;
            let mut win =
                WindowState::new_buffer(win_id, bid);
            win.viewport.set_size(
                cols / 2,
                rows.saturating_sub(2),
            );
            self.windows.insert(win_id, win);
        }
    }
}
