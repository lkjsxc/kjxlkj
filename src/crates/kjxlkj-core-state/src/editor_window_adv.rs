//! Advanced window operations: only, exchange, focus
//! top/bottom, previous, new splits with files,
//! resize commands.

use kjxlkj_core_types::{Direction, WindowId};

use crate::EditorState;

impl EditorState {
    /// Close all other windows (`:only`, `Ctrl-w o`).
    pub(crate) fn do_only_window(&mut self) {
        let keep = self.focused_window;
        let to_remove: Vec<WindowId> = self
            .windows
            .keys()
            .filter(|&&id| id != keep)
            .copied()
            .collect();
        for id in to_remove {
            self.windows.remove(&id);
        }
    }

    /// Exchange current window with the next sibling.
    pub(crate) fn do_exchange_window(&mut self) {
        let ids: Vec<WindowId> =
            self.windows.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let idx = ids
            .iter()
            .position(|&i| i == self.focused_window)
            .unwrap_or(0);
        let next_idx = (idx + 1) % ids.len();
        let a = ids[idx];
        let b = ids[next_idx];
        // Swap the buffer contents of the two windows.
        let a_content = self.windows[&a].content;
        let b_content = self.windows[&b].content;
        if let Some(w) = self.windows.get_mut(&a) {
            w.content = b_content;
        }
        if let Some(w) = self.windows.get_mut(&b) {
            w.content = a_content;
        }
    }

    /// Focus the top-left window (`Ctrl-w t`).
    pub(crate) fn do_focus_top_left(&mut self) {
        if let Some(&id) = self.windows.keys().min() {
            self.focused_window = id;
        }
    }

    /// Focus the bottom-right window (`Ctrl-w b`).
    pub(crate) fn do_focus_bottom_right(&mut self) {
        if let Some(&id) = self.windows.keys().max() {
            self.focused_window = id;
        }
    }

    /// Focus the previously focused window (`Ctrl-w p`).
    /// In our simplified model, this cycles backward.
    pub(crate) fn do_focus_prev_window(&mut self) {
        let ids: Vec<WindowId> =
            self.windows.keys().copied().collect();
        if ids.len() <= 1 {
            return;
        }
        let idx = ids
            .iter()
            .position(|&i| i == self.focused_window)
            .unwrap_or(0);
        let prev = if idx == 0 {
            ids[ids.len() - 1]
        } else {
            ids[idx - 1]
        };
        self.focused_window = prev;
    }

    /// Create new horizontal split with empty buffer.
    pub(crate) fn do_new_split(&mut self) {
        let buf_id = self.alloc_buffer_id();
        let buf = crate::BufferState::new(buf_id);
        self.buffers.insert(buf_id, buf);
        let win_id = self.alloc_window_id();
        let (cols, rows) = self.terminal_size;
        let mut win = crate::WindowState::new_buffer(
            win_id, buf_id,
        );
        win.viewport.set_size(
            cols,
            rows.saturating_sub(2),
        );
        self.windows.insert(win_id, win);
        self.focused_window = win_id;
    }

    /// Create new vertical split with empty buffer.
    pub(crate) fn do_new_vsplit(&mut self) {
        self.do_new_split();
    }

    /// Horizontal split opening a file.
    pub(crate) fn do_split_open(&mut self, path: &str) {
        self.do_split_horizontal();
        self.do_open_file(&std::path::PathBuf::from(path));
    }

    /// Vertical split opening a file.
    pub(crate) fn do_vsplit_open(&mut self, path: &str) {
        self.do_split_vertical();
        self.do_open_file(&std::path::PathBuf::from(path));
    }

    /// Handle `:resize` command.
    pub(crate) fn do_resize_cmd(&mut self, args: &str) {
        let args = args.trim();
        // `v N` → vertical resize (width)
        let (vertical, spec) =
            if let Some(rest) = args.strip_prefix("v ") {
                (true, rest.trim())
            } else {
                (false, args)
            };
        if spec.is_empty() {
            return;
        }
        let (relative, value) =
            if let Some(rest) = spec.strip_prefix('+') {
                (true, rest.parse::<i32>().unwrap_or(0))
            } else if let Some(rest) =
                spec.strip_prefix('-')
            {
                (true, -(rest.parse::<i32>().unwrap_or(0)))
            } else {
                (false, spec.parse::<i32>().unwrap_or(0))
            };
        if vertical {
            if relative {
                self.do_resize_window(
                    Direction::Right,
                    value,
                );
            } else if let Some(w) =
                self.focused_window_mut()
            {
                w.viewport.width = value.max(1) as u16;
            }
        } else if relative {
            self.do_resize_window(
                Direction::Down,
                value,
            );
        } else if let Some(w) =
            self.focused_window_mut()
        {
            w.viewport.height = value.max(1) as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_window_closes_others() {
        let mut ed = EditorState::new(80, 24);
        ed.do_split_horizontal();
        assert_eq!(ed.windows.len(), 2);
        ed.do_only_window();
        assert_eq!(ed.windows.len(), 1);
    }

    #[test]
    fn new_split_creates_empty() {
        let mut ed = EditorState::new(80, 24);
        let bufs_before = ed.buffers.len();
        ed.do_new_split();
        assert_eq!(ed.buffers.len(), bufs_before + 1);
        assert_eq!(ed.windows.len(), 2);
    }

    #[test]
    fn exchange_window() {
        let mut ed = EditorState::new(80, 24);
        ed.do_split_horizontal();
        let ids: Vec<_> =
            ed.windows.keys().copied().collect();
        let c0 = ed.windows[&ids[0]].content;
        let c1 = ed.windows[&ids[1]].content;
        ed.do_exchange_window();
        assert_eq!(
            ed.windows[&ids[0]].content, c1
        );
        assert_eq!(
            ed.windows[&ids[1]].content, c0
        );
    }

    #[test]
    fn resize_cmd() {
        let mut ed = EditorState::new(80, 24);
        ed.do_resize_cmd("10");
        assert_eq!(
            ed.focused_window().unwrap().viewport.height,
            10,
        );
    }
}
