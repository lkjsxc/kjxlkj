//! Window-type operations: terminal and explorer window management.

use kjxlkj_core_ui::WindowContent;

use crate::editor::EditorState;
use crate::window_tree::Window;

impl EditorState {
    /// Open a terminal in the current window (`:terminal`, `<leader>t`).
    ///
    /// Creates a new terminal id, inserts a terminal window as a horizontal
    /// split from the current window, and transitions mode to TerminalInsert.
    pub fn do_terminal_open(&mut self) {
        let tid = self.next_terminal_id();
        let wid = self.windows.next_window_id();
        let win = Window::new_terminal(wid, tid);
        self.windows.active_tab_mut().split_horizontal(win);
        self.mode = kjxlkj_core_types::Mode::TerminalInsert;
    }

    /// Open a terminal in a split (`<leader>th` / `<leader>tv`).
    pub fn do_terminal_split(&mut self, vertical: bool) {
        let tid = self.next_terminal_id();
        let wid = self.windows.next_window_id();
        let win = Window::new_terminal(wid, tid);
        let tab = self.windows.active_tab_mut();
        if vertical {
            tab.split_vertical(win);
        } else {
            tab.split_horizontal(win);
        }
        self.mode = kjxlkj_core_types::Mode::TerminalInsert;
    }

    /// Toggle the explorer window (`<leader>e`, `:Explorer`).
    ///
    /// If an explorer window already exists in the active tab, close it.
    /// Otherwise, create one as a vertical split on the left.
    pub fn do_explorer_toggle(&mut self) {
        let tab = self.windows.active_tab_mut();
        // Check if an explorer window already exists
        let explorer_idx = tab
            .windows
            .iter()
            .position(|w| matches!(w.content, WindowContent::Explorer));
        if let Some(idx) = explorer_idx {
            // Close explorer window
            if tab.windows.len() > 1 {
                tab.windows.remove(idx);
                if tab.active_window >= tab.windows.len() {
                    tab.active_window = tab.windows.len() - 1;
                }
                tab.rebuild_layout_pub();
            }
        } else {
            // Open explorer as vertical split
            let wid = self.windows.next_window_id();
            let win = Window::new_explorer(wid);
            self.windows.active_tab_mut().split_vertical(win);
        }
    }

    /// Reveal the current file in the explorer (`<leader>E`).
    ///
    /// If no explorer window exists, create one. Then set the explorer
    /// cursor to the current buffer's file path.
    pub fn do_explorer_reveal(&mut self) {
        let tab = self.windows.active_tab_mut();
        let has_explorer = tab
            .windows
            .iter()
            .any(|w| matches!(w.content, WindowContent::Explorer));
        if !has_explorer {
            let wid = self.windows.next_window_id();
            let win = Window::new_explorer(wid);
            self.windows.active_tab_mut().split_vertical(win);
        }
        // TODO: set explorer cursor to current buffer path
    }
}
