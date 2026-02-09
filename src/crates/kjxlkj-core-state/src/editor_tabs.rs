//! Tab page management.

use std::path::PathBuf;

use kjxlkj_core_types::{TabId, WindowId};

use crate::editor::EditorState;
use crate::{BufferState, WindowState};

#[derive(Debug)]
pub struct TabPage {
    pub id: TabId,
    pub window_ids: Vec<WindowId>,
    pub active_window: WindowId,
}

impl TabPage {
    pub fn new(id: TabId, win_id: WindowId) -> Self {
        Self {
            id,
            window_ids: vec![win_id],
            active_window: win_id,
        }
    }
}

impl EditorState {
    pub fn do_tab_new(&mut self, path: Option<&str>) {
        let tab_id = TabId(self.next_tab_id);
        self.next_tab_id += 1;
        let win_id = self.alloc_window_id();
        let buf_id = self.alloc_buffer_id();
        let mut buf = if let Some(p) = path {
            BufferState::new_with_path(buf_id, PathBuf::from(p))
        } else {
            BufferState::new(buf_id)
        };
        buf.detect_file_type();
        let (cols, rows) = self.terminal_size;
        let mut win = WindowState::new_buffer(win_id, buf_id);
        win.viewport.set_size(cols, rows.saturating_sub(2));
        self.buffers.insert(buf_id, buf);
        self.windows.insert(win_id, win);
        let tab = TabPage::new(tab_id, win_id);
        let insert_pos = self.active_tab + 1;
        self.tabs.insert(insert_pos, tab);
        self.active_tab = insert_pos;
        self.focused_window = win_id;
    }

    pub fn do_tab_close(&mut self) {
        if self.tabs.len() <= 1 {
            return;
        }
        let tab = self.tabs.remove(self.active_tab);
        for wid in &tab.window_ids {
            self.windows.remove(wid);
        }
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_only(&mut self) {
        if self.tabs.len() <= 1 {
            return;
        }
        let current = self.tabs.remove(self.active_tab);
        for tab in &self.tabs {
            for wid in &tab.window_ids {
                self.windows.remove(wid);
            }
        }
        self.tabs.clear();
        self.tabs.push(current);
        self.active_tab = 0;
    }

    pub fn do_tab_next(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.active_tab = (self.active_tab + 1) % self.tabs.len();
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_prev(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        if self.active_tab == 0 {
            self.active_tab = self.tabs.len() - 1;
        } else {
            self.active_tab -= 1;
        }
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_first(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.active_tab = 0;
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_last(&mut self) {
        if self.tabs.is_empty() {
            return;
        }
        self.active_tab = self.tabs.len() - 1;
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_goto(&mut self, n: usize) {
        if n == 0 || n > self.tabs.len() {
            return;
        }
        self.active_tab = n - 1;
        let aw = self.tabs[self.active_tab].active_window;
        self.focused_window = aw;
    }

    pub fn do_tab_move(&mut self, args: &str) {
        if self.tabs.len() <= 1 {
            return;
        }
        let target = if args == "$" {
            self.tabs.len() - 1
        } else if let Some(rest) = args.strip_prefix('+') {
            let n: usize = rest.parse().unwrap_or(1);
            (self.active_tab + n).min(self.tabs.len() - 1)
        } else if let Some(rest) = args.strip_prefix('-') {
            let n: usize = rest.parse().unwrap_or(1);
            self.active_tab.saturating_sub(n)
        } else {
            args.parse::<usize>().unwrap_or(0)
        };
        let target = target.min(self.tabs.len() - 1);
        if target != self.active_tab {
            let tab = self.tabs.remove(self.active_tab);
            self.tabs.insert(target, tab);
            self.active_tab = target;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_new_and_close() {
        let mut ed = EditorState::new(80, 24);
        assert_eq!(ed.tabs.len(), 1);
        ed.do_tab_new(None);
        assert_eq!(ed.tabs.len(), 2);
        assert_eq!(ed.active_tab, 1);
        ed.do_tab_close();
        assert_eq!(ed.tabs.len(), 1);
        assert_eq!(ed.active_tab, 0);
    }

    #[test]
    fn tab_next_prev_wrap() {
        let mut ed = EditorState::new(80, 24);
        ed.do_tab_new(None);
        ed.do_tab_new(None);
        assert_eq!(ed.active_tab, 2);
        ed.do_tab_next();
        assert_eq!(ed.active_tab, 0);
        ed.do_tab_prev();
        assert_eq!(ed.active_tab, 2);
    }

    #[test]
    fn tab_only() {
        let mut ed = EditorState::new(80, 24);
        ed.do_tab_new(None);
        ed.do_tab_new(None);
        ed.do_tab_first();
        ed.do_tab_only();
        assert_eq!(ed.tabs.len(), 1);
    }
}
