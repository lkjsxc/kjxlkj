/// Full window feature docs — window options, layout, focus, close guards.

use std::collections::HashMap;

/// Per-window options that may differ across splits.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub number: bool,
    pub relative_number: bool,
    pub wrap: bool,
    pub sign_column: SignColumn,
    pub fold_column: u8,
    pub scroll_off: u16,
    pub side_scroll_off: u16,
    pub cursor_line: bool,
    pub cursor_column: bool,
    pub spell: bool,
    pub list: bool,
    pub line_break: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self { number: true, relative_number: false, wrap: true, sign_column: SignColumn::Auto,
            fold_column: 0, scroll_off: 0, side_scroll_off: 0, cursor_line: false,
            cursor_column: false, spell: false, list: false, line_break: false }
    }
}

/// Sign column display mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignColumn { Auto, Yes, No, Number }

/// Window close guard — prevent closing last window with unsaved changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseGuard { Allow, NeedsSave, LastWindow }

pub fn check_close_guard(modified: bool, is_last: bool, force: bool) -> CloseGuard {
    if force { return CloseGuard::Allow; }
    if is_last && modified { return CloseGuard::NeedsSave; }
    if modified { return CloseGuard::NeedsSave; }
    CloseGuard::Allow
}

/// Window snapshot options for rendering.
#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    pub win_id: u64,
    pub buf_id: u64,
    pub options: WindowOptions,
    pub top_line: usize,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub width: u16,
    pub height: u16,
}

/// Window option overrides per window id.
#[derive(Debug, Default)]
pub struct WindowOptionStore { overrides: HashMap<u64, WindowOptions> }

impl WindowOptionStore {
    pub fn new() -> Self { Self::default() }

    pub fn set(&mut self, win_id: u64, opts: WindowOptions) { self.overrides.insert(win_id, opts); }

    pub fn get(&self, win_id: u64) -> WindowOptions {
        self.overrides.get(&win_id).cloned().unwrap_or_default()
    }

    pub fn remove(&mut self, win_id: u64) { self.overrides.remove(&win_id); }
    pub fn count(&self) -> usize { self.overrides.len() }
}

/// Format window status string.
pub fn window_status(win_id: u64, buf_name: &str, modified: bool) -> String {
    let mod_flag = if modified { "[+]" } else { "" };
    format!("[{}] {}{}", win_id, buf_name, mod_flag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_options() {
        let o = WindowOptions::default();
        assert!(o.number); assert!(o.wrap); assert!(!o.cursor_line);
    }

    #[test]
    fn close_guard_allow() { assert_eq!(check_close_guard(false, false, false), CloseGuard::Allow); }

    #[test]
    fn close_guard_needs_save() { assert_eq!(check_close_guard(true, false, false), CloseGuard::NeedsSave); }

    #[test]
    fn close_guard_force() { assert_eq!(check_close_guard(true, true, true), CloseGuard::Allow); }

    #[test]
    fn option_store() {
        let mut s = WindowOptionStore::new();
        let mut opts = WindowOptions::default(); opts.number = false;
        s.set(1, opts);
        assert!(!s.get(1).number);
        assert!(s.get(99).number); // default fallback
    }

    #[test]
    fn option_store_remove() {
        let mut s = WindowOptionStore::new();
        s.set(1, WindowOptions::default()); s.remove(1);
        assert_eq!(s.count(), 0);
    }

    #[test]
    fn window_status_format() {
        assert_eq!(window_status(1, "foo.rs", true), "[1] foo.rs[+]");
        assert_eq!(window_status(2, "bar.rs", false), "[2] bar.rs");
    }

    #[test]
    fn snapshot_fields() {
        let snap = WindowSnapshot { win_id: 1, buf_id: 1, options: WindowOptions::default(),
            top_line: 0, cursor_line: 5, cursor_col: 10, width: 80, height: 24 };
        assert_eq!(snap.width, 80);
    }
}
