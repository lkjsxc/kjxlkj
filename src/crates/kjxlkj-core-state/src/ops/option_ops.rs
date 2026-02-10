//! Window option management and buffer switching.
//!
//! Extracted from cmdline_ops.rs to keep files under 200 lines.

use kjxlkj_core_ui::{Notification, WindowContent};

use crate::editor::EditorState;
use crate::window_tree::Window;

impl EditorState {
    pub(crate) fn do_split(&mut self, vertical: bool, _path: Option<String>) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let win_id = self.windows.next_window_id();
        let new_win = Window::new_buffer(win_id, buf_id);
        let tab = self.windows.active_tab_mut();
        if vertical {
            tab.split_vertical(new_win);
        } else {
            tab.split_horizontal(new_win);
        }
    }

    pub(crate) fn switch_buffer_next(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        if let Some(next) = self.buffers.next(&buf_id) {
            let win = self.windows.active_tab_mut().active_mut();
            win.content = WindowContent::Buffer(next);
            win.cursor_line = 0;
            win.cursor_offset = 0;
            win.top_line = 0;
        }
    }

    pub(crate) fn switch_buffer_prev(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        if let Some(prev) = self.buffers.prev(&buf_id) {
            let win = self.windows.active_tab_mut().active_mut();
            win.content = WindowContent::Buffer(prev);
            win.cursor_line = 0;
            win.cursor_offset = 0;
            win.top_line = 0;
        }
    }

    pub(crate) fn notify_info(&mut self, msg: &str) {
        self.notifications.push(Notification {
            message: msg.to_string(),
            level: kjxlkj_core_ui::NotificationLevel::Info,
        });
    }

    pub(crate) fn notify_error(&mut self, msg: &str) {
        self.notifications.push(Notification {
            message: msg.to_string(),
            level: kjxlkj_core_ui::NotificationLevel::Error,
        });
    }

    /// Apply a `:set` option to the active window.
    pub(crate) fn apply_set_option(&mut self, opt: &str) {
        let opt = opt.trim();
        match opt {
            "wrap" => {
                self.windows.active_tab_mut().active_mut().wrap = true;
            }
            "nowrap" => {
                self.windows.active_tab_mut().active_mut().wrap = false;
            }
            "number" => {
                self.windows.active_tab_mut().active_mut().line_numbers = true;
            }
            "nonumber" => {
                self.windows.active_tab_mut().active_mut().line_numbers = false;
            }
            _ => {
                if let Some((key, val)) = opt.split_once('=') {
                    self.apply_set_kv(key.trim(), val.trim());
                } else {
                    self.notify_error(&format!("Unknown option: {opt}"));
                }
            }
        }
    }

    fn apply_set_kv(&mut self, key: &str, val: &str) {
        match key {
            "scrolloff" => {
                if let Ok(n) = val.parse::<usize>() {
                    self.windows.active_tab_mut().active_mut().scrolloff = n;
                }
            }
            "sidescrolloff" => {
                if let Ok(n) = val.parse::<usize>() {
                    self.windows.active_tab_mut().active_mut().sidescrolloff = n;
                }
            }
            _ => {
                self.notify_error(&format!("Unknown option: {key}"));
            }
        }
    }
}
