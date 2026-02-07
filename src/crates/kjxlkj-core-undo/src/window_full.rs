//! Window state types: options, snapshots, and close guards.

use kjxlkj_core_types::types::{BufferId, Position, WindowId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sign column display mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignColumn {
    Auto,
    Yes,
    No,
    Number,
}

/// Per-window display options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowOptions {
    pub number: bool,
    pub relative_number: bool,
    pub wrap: bool,
    pub sign_column: SignColumn,
    pub scrolloff: usize,
    pub sidescrolloff: usize,
    pub cursor_line: bool,
    pub cursor_column: bool,
    pub list_mode: bool,
    pub spell: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            number: false,
            relative_number: false,
            wrap: true,
            sign_column: SignColumn::Auto,
            scrolloff: 0,
            sidescrolloff: 0,
            cursor_line: false,
            cursor_column: false,
            list_mode: false,
            spell: false,
        }
    }
}

/// Whether a window may be closed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseGuard {
    Allow,
    NeedsSave,
    LastWindow,
}

/// Check if a window can be closed given buffer and session state.
pub fn can_close(modified: bool, is_last: bool) -> CloseGuard {
    if modified {
        CloseGuard::NeedsSave
    } else if is_last {
        CloseGuard::LastWindow
    } else {
        CloseGuard::Allow
    }
}

/// Snapshot of a window's state at a point in time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowSnapshot {
    pub id: WindowId,
    pub buffer_id: BufferId,
    pub options: WindowOptions,
    pub cursor: Position,
    pub viewport_top: usize,
}

/// Store of per-window option overrides.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowOptionStore {
    overrides: HashMap<WindowId, WindowOptions>,
}

impl WindowOptionStore {
    /// Get options for a window, falling back to defaults.
    pub fn get(&self, id: WindowId) -> WindowOptions {
        self.overrides.get(&id).cloned().unwrap_or_default()
    }

    /// Set options for a window.
    pub fn set(&mut self, id: WindowId, options: WindowOptions) {
        self.overrides.insert(id, options);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options() {
        let opts = WindowOptions::default();
        assert!(opts.wrap);
        assert!(!opts.number);
        assert_eq!(opts.sign_column, SignColumn::Auto);
    }

    #[test]
    fn close_guard_modified() {
        assert_eq!(can_close(true, false), CloseGuard::NeedsSave);
    }

    #[test]
    fn close_guard_last_window() {
        assert_eq!(can_close(false, true), CloseGuard::LastWindow);
    }

    #[test]
    fn close_guard_allow() {
        assert_eq!(can_close(false, false), CloseGuard::Allow);
    }

    #[test]
    fn option_store_defaults() {
        let store = WindowOptionStore::default();
        let opts = store.get(WindowId(1));
        assert_eq!(opts, WindowOptions::default());
    }

    #[test]
    fn option_store_override() {
        let mut store = WindowOptionStore::default();
        let opts = WindowOptions { number: true, scrolloff: 5, ..Default::default() };
        store.set(WindowId(42), opts.clone());
        assert_eq!(store.get(WindowId(42)), opts);
    }
}
