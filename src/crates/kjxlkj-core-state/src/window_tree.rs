//! Window and tab management.
//!
//! TabPage logic lives in tab_page.rs.

use kjxlkj_core_types::{BufferId, TerminalId, WindowId};
use kjxlkj_core_ui::WindowContent;

pub use crate::tab_page::TabPage;

/// A window in the editor.
pub struct Window {
    pub id: WindowId,
    pub content: WindowContent,
    pub cursor_line: usize,
    pub cursor_offset: usize,
    pub top_line: usize,
    pub left_col: usize,
    pub wrap: bool,
    pub line_numbers: bool,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
}

impl Window {
    pub fn new_buffer(id: WindowId, buffer_id: BufferId) -> Self {
        Self {
            id,
            content: WindowContent::Buffer(buffer_id),
            cursor_line: 0,
            cursor_offset: 0,
            top_line: 0,
            left_col: 0,
            wrap: true,
            line_numbers: true,
            scrolloff: 5,
            sidescrolloff: 3,
        }
    }

    pub fn new_terminal(id: WindowId, terminal_id: TerminalId) -> Self {
        Self {
            id,
            content: WindowContent::Terminal(terminal_id),
            cursor_line: 0,
            cursor_offset: 0,
            top_line: 0,
            left_col: 0,
            wrap: false,
            line_numbers: false,
            scrolloff: 0,
            sidescrolloff: 0,
        }
    }

    pub fn new_explorer(id: WindowId) -> Self {
        Self {
            id,
            content: WindowContent::Explorer,
            cursor_line: 0,
            cursor_offset: 0,
            top_line: 0,
            left_col: 0,
            wrap: false,
            line_numbers: false,
            scrolloff: 0,
            sidescrolloff: 0,
        }
    }
}

/// Manages tabs.
pub struct WindowTree {
    pub tabs: Vec<TabPage>,
    pub active_tab: usize,
    next_window_id: u64,
}

impl WindowTree {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
            next_window_id: 1,
        }
    }

    /// Allocate a new window id.
    pub fn next_window_id(&mut self) -> WindowId {
        let id = WindowId(self.next_window_id);
        self.next_window_id += 1;
        id
    }

    /// Get the active tab.
    pub fn active_tab(&self) -> &TabPage {
        &self.tabs[self.active_tab]
    }

    /// Get the active tab mutably.
    pub fn active_tab_mut(&mut self) -> &mut TabPage {
        &mut self.tabs[self.active_tab]
    }

    /// Add a new tab.
    pub fn add_tab(&mut self, tab: TabPage) {
        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    /// Close the active tab.
    pub fn close_active_tab(&mut self) -> bool {
        if self.tabs.len() <= 1 {
            return true;
        }
        self.tabs.remove(self.active_tab);
        if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
        false
    }

    /// Go to next tab.
    pub fn next_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = (self.active_tab + 1) % self.tabs.len();
        }
    }

    /// Go to previous tab.
    pub fn prev_tab(&mut self) {
        if !self.tabs.is_empty() {
            self.active_tab = if self.active_tab == 0 {
                self.tabs.len() - 1
            } else {
                self.active_tab - 1
            };
        }
    }
}

impl Default for WindowTree {
    fn default() -> Self {
        Self::new()
    }
}
