//! Tab page types.
//!
//! Types for tab page identification and structure.

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
