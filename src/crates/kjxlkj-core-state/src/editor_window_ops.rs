//! Editor window navigation and write-all operations.

use kjxlkj_core_types::{Direction, WindowId};

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
        let old = self.focused_window;
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
        self.prev_window = Some(old);
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


}
