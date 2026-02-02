//! Tab page support.
//!
//! Tab pages contain window layouts.

use std::collections::HashMap;

/// Tab page identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(pub usize);

impl TabId {
    /// Returns the numeric id.
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

/// A tab page.
#[derive(Debug, Clone)]
pub struct TabPage {
    /// Tab identifier.
    pub id: TabId,
    /// Windows in this tab.
    pub windows: Vec<usize>,
    /// Current window index.
    pub current: usize,
    /// Tab-local variables.
    pub variables: HashMap<String, String>,
}

impl TabPage {
    /// Creates a new tab page.
    pub fn new(id: TabId) -> Self {
        Self {
            id,
            windows: Vec::new(),
            current: 0,
            variables: HashMap::new(),
        }
    }

    /// Adds a window.
    pub fn add_window(&mut self, window_id: usize) {
        self.windows.push(window_id);
    }

    /// Removes a window.
    pub fn remove_window(&mut self, window_id: usize) -> bool {
        if let Some(pos) = self.windows.iter().position(|&w| w == window_id) {
            self.windows.remove(pos);
            if self.current >= self.windows.len() && !self.windows.is_empty() {
                self.current = self.windows.len() - 1;
            }
            true
        } else {
            false
        }
    }

    /// Returns the current window.
    pub fn current_window(&self) -> Option<usize> {
        self.windows.get(self.current).copied()
    }

    /// Returns window count.
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Sets a tab variable.
    pub fn set_var(&mut self, name: &str, value: &str) {
        self.variables.insert(name.to_string(), value.to_string());
    }

    /// Gets a tab variable.
    pub fn get_var(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }
}

/// Tab page manager.
#[derive(Debug)]
pub struct TabManager {
    /// All tabs.
    tabs: Vec<TabPage>,
    /// Current tab index.
    current: usize,
    /// Next tab id.
    next_id: usize,
}

impl TabManager {
    /// Creates a new tab manager.
    pub fn new() -> Self {
        let mut mgr = Self {
            tabs: Vec::new(),
            current: 0,
            next_id: 1,
        };
        // Start with one tab
        mgr.new_tab();
        mgr
    }

    /// Creates a new tab.
    pub fn new_tab(&mut self) -> TabId {
        let id = TabId(self.next_id);
        self.next_id += 1;
        self.tabs.push(TabPage::new(id));
        id
    }

    /// Closes a tab.
    pub fn close_tab(&mut self, id: TabId) -> bool {
        if self.tabs.len() <= 1 {
            return false; // Must keep at least one tab
        }
        if let Some(pos) = self.tabs.iter().position(|t| t.id == id) {
            self.tabs.remove(pos);
            if self.current >= self.tabs.len() {
                self.current = self.tabs.len() - 1;
            }
            true
        } else {
            false
        }
    }

    /// Returns the current tab.
    pub fn current(&self) -> Option<&TabPage> {
        self.tabs.get(self.current)
    }

    /// Returns the current tab mutably.
    pub fn current_mut(&mut self) -> Option<&mut TabPage> {
        self.tabs.get_mut(self.current)
    }

    /// Goes to next tab.
    pub fn advance_next(&mut self) {
        if !self.tabs.is_empty() {
            self.current = (self.current + 1) % self.tabs.len();
        }
    }

    /// Goes to previous tab.
    pub fn prev(&mut self) {
        if !self.tabs.is_empty() {
            self.current = self.current.checked_sub(1).unwrap_or(self.tabs.len() - 1);
        }
    }

    /// Goes to tab by number (1-indexed).
    pub fn goto(&mut self, num: usize) -> bool {
        if num > 0 && num <= self.tabs.len() {
            self.current = num - 1;
            true
        } else {
            false
        }
    }

    /// Returns tab count.
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Returns all tabs.
    pub fn all(&self) -> &[TabPage] {
        &self.tabs
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_id() {
        let id = TabId(1);
        assert_eq!(id.as_usize(), 1);
    }

    #[test]
    fn test_tab_page_new() {
        let tab = TabPage::new(TabId(1));
        assert_eq!(tab.window_count(), 0);
    }

    #[test]
    fn test_tab_page_windows() {
        let mut tab = TabPage::new(TabId(1));
        tab.add_window(1);
        tab.add_window(2);
        assert_eq!(tab.window_count(), 2);
        tab.remove_window(1);
        assert_eq!(tab.window_count(), 1);
    }

    #[test]
    fn test_tab_manager_new() {
        let mgr = TabManager::new();
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_tab_manager_navigation() {
        let mut mgr = TabManager::new();
        mgr.new_tab();
        mgr.new_tab();
        assert_eq!(mgr.len(), 3);
        mgr.advance_next();
        mgr.prev();
        assert!(mgr.goto(2));
    }

    #[test]
    fn test_tab_manager_close() {
        let mut mgr = TabManager::new();
        let id = mgr.new_tab();
        assert_eq!(mgr.len(), 2);
        mgr.close_tab(id);
        assert_eq!(mgr.len(), 1);
    }
}
