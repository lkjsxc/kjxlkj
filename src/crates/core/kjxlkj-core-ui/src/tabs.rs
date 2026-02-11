//! Tab page model. See /docs/spec/features/window/tabs.md.
//!
//! Each tab owns a LayoutTree and tracks its active window.

use crate::layout::{LayoutNode, LayoutTree};
use crate::focus::FocusState;
use kjxlkj_core_types::{ContentKind, WindowId};

/// A single tab page with its own split tree.
#[derive(Debug, Clone)]
pub struct TabPage {
    pub id: TabId,
    pub layout: LayoutTree,
    pub active_window: WindowId,
    pub label: String,
    pub modified: bool,
}

/// Tab identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(pub u64);

impl TabPage {
    pub fn new(id: TabId, layout: LayoutTree, active: WindowId) -> Self {
        Self { id, layout, active_window: active, label: String::new(), modified: false }
    }
    /// All window IDs in this tab.
    pub fn window_ids(&self) -> Vec<WindowId> { self.layout.window_ids() }
    /// Set the label from buffer name.
    pub fn set_label(&mut self, name: &str) { self.label = name.to_string(); }
}

/// Ordered list of tab pages with exactly one active.
#[derive(Debug, Clone)]
pub struct TabList {
    pub tabs: Vec<TabPage>,
    pub active: usize,
    next_id: u64,
}

impl TabList {
    /// Create with a single initial tab.
    pub fn new(layout: LayoutTree, active_window: WindowId) -> Self {
        let tab = TabPage::new(TabId(0), layout, active_window);
        Self { tabs: vec![tab], active: 0, next_id: 1 }
    }
    fn alloc_id(&mut self) -> TabId { let id = TabId(self.next_id); self.next_id += 1; id }

    /// Create a new tab after current and focus it. Returns the new TabId.
    pub fn tab_new(&mut self, layout: LayoutTree, active_window: WindowId) -> TabId {
        let id = self.alloc_id();
        let tab = TabPage::new(id, layout, active_window);
        self.tabs.insert(self.active + 1, tab);
        self.active += 1;
        id
    }
    /// Close current tab. Returns false if it's the last tab.
    pub fn tab_close(&mut self) -> bool {
        if self.tabs.len() <= 1 { return false; }
        self.tabs.remove(self.active);
        if self.active >= self.tabs.len() { self.active = self.tabs.len() - 1; }
        true
    }
    /// Close all tabs except current.
    pub fn tab_only(&mut self) {
        let current = self.tabs.remove(self.active);
        self.tabs = vec![current];
        self.active = 0;
    }
    /// Focus next tab (wrapping).
    pub fn tab_next(&mut self) { self.active = (self.active + 1) % self.tabs.len(); }
    /// Focus previous tab (wrapping).
    pub fn tab_prev(&mut self) {
        self.active = if self.active == 0 { self.tabs.len() - 1 } else { self.active - 1 };
    }
    /// Focus tab by 1-indexed number. Returns false if out of range.
    pub fn tab_goto(&mut self, n: usize) -> bool {
        if n == 0 || n > self.tabs.len() { return false; }
        self.active = n - 1; true
    }
    /// Focus first tab.
    pub fn tab_first(&mut self) { self.active = 0; }
    /// Focus last tab.
    pub fn tab_last(&mut self) { self.active = self.tabs.len() - 1; }

    /// Move current tab to 0-indexed position. Clamps to bounds.
    pub fn tab_move(&mut self, pos: usize) {
        let pos = pos.min(self.tabs.len().saturating_sub(1));
        if pos == self.active { return; }
        let tab = self.tabs.remove(self.active);
        self.tabs.insert(pos, tab);
        self.active = pos;
    }
    /// Move current tab by relative offset (positive=right, negative=left).
    pub fn tab_move_relative(&mut self, offset: i32) {
        let new = (self.active as i32 + offset)
            .max(0).min(self.tabs.len() as i32 - 1) as usize;
        self.tab_move(new);
    }
    /// Get the active tab.
    pub fn current(&self) -> &TabPage { &self.tabs[self.active] }
    /// Get the active tab mutably.
    pub fn current_mut(&mut self) -> &mut TabPage { &mut self.tabs[self.active] }
    /// Number of tabs.
    pub fn count(&self) -> usize { self.tabs.len() }
    /// Active 1-indexed tab number.
    pub fn active_number(&self) -> usize { self.active + 1 }
    /// Whether tabline should show (more than 1 tab or always_show).
    pub fn should_show_tabline(&self, always_show: bool) -> bool {
        self.tabs.len() > 1 || always_show
    }
}

#[cfg(test)]
#[path = "tabs_tests.rs"]
mod tests;
