//! Floating window state management.

use crate::float_types::{FloatConfig, FloatWindow};
use std::collections::HashMap;

/// Floating window state.
#[derive(Debug, Clone, Default)]
pub struct FloatState {
    /// Windows by ID.
    windows: HashMap<usize, FloatWindow>,
    /// Next ID.
    next_id: usize,
}

impl FloatState {
    /// Creates new float state.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Opens a floating window.
    pub fn open(&mut self, buffer_id: usize, config: FloatConfig) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.windows
            .insert(id, FloatWindow::new(id, buffer_id, config));
        id
    }

    /// Closes a floating window.
    pub fn close(&mut self, id: usize) -> bool {
        self.windows.remove(&id).is_some()
    }

    /// Gets a floating window.
    pub fn get(&self, id: usize) -> Option<&FloatWindow> {
        self.windows.get(&id)
    }

    /// Updates a floating window configuration.
    pub fn update_config(&mut self, id: usize, config: FloatConfig) -> bool {
        if let Some(win) = self.windows.get_mut(&id) {
            win.config = config;
            true
        } else {
            false
        }
    }

    /// Brings a floating window to the front by raising its z-index.
    pub fn bring_to_front(&mut self, id: usize) -> bool {
        let max_z = self
            .windows
            .values()
            .map(|w| w.config.zindex)
            .max()
            .unwrap_or(0);

        if let Some(win) = self.windows.get_mut(&id) {
            win.config.zindex = max_z.saturating_add(1);
            true
        } else {
            false
        }
    }

    /// Returns focusable windows sorted by z-index.
    pub fn focusable_sorted(&self) -> Vec<&FloatWindow> {
        let mut windows: Vec<_> = self
            .windows
            .values()
            .filter(|w| w.config.focusable)
            .collect();
        windows.sort_by_key(|w| w.config.zindex);
        windows
    }

    /// Returns all windows sorted by zindex.
    pub fn all_sorted(&self) -> Vec<&FloatWindow> {
        let mut windows: Vec<_> = self.windows.values().collect();
        windows.sort_by_key(|w| w.config.zindex);
        windows
    }

    /// Returns the topmost window.
    pub fn topmost(&self) -> Option<&FloatWindow> {
        self.windows.values().max_by_key(|w| w.config.zindex)
    }

    /// Closes all windows.
    pub fn close_all(&mut self) {
        self.windows.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float_types::FloatBorder;

    #[test]
    fn test_float_config_builder() {
        let config = FloatConfig::new()
            .at(10, 20)
            .size(40, 10)
            .with_border(FloatBorder::Rounded)
            .focusable();

        assert_eq!(config.row, 10);
        assert_eq!(config.col, 20);
        assert_eq!(config.border, FloatBorder::Rounded);
        assert!(config.focusable);
    }

    #[test]
    fn test_float_state_open() {
        let mut state = FloatState::new();
        let id = state.open(1, FloatConfig::new());
        assert!(state.get(id).is_some());
    }

    #[test]
    fn test_float_state_close() {
        let mut state = FloatState::new();
        let id = state.open(1, FloatConfig::new());
        assert!(state.close(id));
        assert!(state.get(id).is_none());
    }

    #[test]
    fn test_float_state_topmost() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig { zindex: 10, ..Default::default() });
        state.open(2, FloatConfig { zindex: 50, ..Default::default() });
        let top = state.topmost().unwrap();
        assert_eq!(top.config.zindex, 50);
    }

    #[test]
    fn test_float_state_sorted() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig { zindex: 50, ..Default::default() });
        state.open(2, FloatConfig { zindex: 10, ..Default::default() });
        let sorted = state.all_sorted();
        assert_eq!(sorted[0].config.zindex, 10);
        assert_eq!(sorted[1].config.zindex, 50);
    }

    #[test]
    fn test_float_state_close_all() {
        let mut state = FloatState::new();
        state.open(1, FloatConfig::new());
        state.open(2, FloatConfig::new());
        state.close_all();
        assert!(state.topmost().is_none());
    }

    #[test]
    fn test_float_state_bring_to_front() {
        let mut state = FloatState::new();
        let a = state.open(1, FloatConfig { zindex: 10, ..Default::default() });
        let b = state.open(2, FloatConfig { zindex: 50, ..Default::default() });
        assert!(state.bring_to_front(a));
        assert!(state.get(a).unwrap().config.zindex > state.get(b).unwrap().config.zindex);
    }
}
