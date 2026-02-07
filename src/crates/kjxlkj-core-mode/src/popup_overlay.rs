//! Popup and overlay management for completion menus, hover info, etc.

use serde::{Deserialize, Serialize};

/// The kind of popup being displayed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PopupKind {
    Completion,
    Hover,
    SignatureHelp,
    ContextMenu,
    Wildmenu,
    CommandPalette,
}

/// Anchor point for popup positioning.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PopupAnchor {
    Cursor,
    TopLeft,
    Center,
    CmdLine,
}

/// State for a single popup overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopupState {
    pub kind: PopupKind,
    pub items: Vec<String>,
    pub selected: usize,
    pub visible: bool,
    pub max_visible: usize,
    pub scroll_offset: usize,
}

impl PopupState {
    /// Show the popup with the given items.
    pub fn show(&mut self, items: Vec<String>) {
        self.items = items;
        self.selected = 0;
        self.scroll_offset = 0;
        self.visible = true;
    }

    /// Hide the popup.
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Move selection to next item.
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1).min(self.items.len() - 1);
            // Adjust scroll offset to keep selection visible.
            if self.selected >= self.scroll_offset + self.max_visible {
                self.scroll_offset = self.selected + 1 - self.max_visible;
            }
        }
    }

    /// Move selection to previous item.
    pub fn select_prev(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        }
    }

    /// Return the currently visible slice of items.
    pub fn visible_items(&self) -> &[String] {
        let end = (self.scroll_offset + self.max_visible).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    /// Return the currently selected item, if any.
    pub fn current(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }
}

// Re-export from dedicated module.
pub use crate::popup_overlay_mgr::{compute_popup_rect, OverlayManager};

#[cfg(test)]
mod tests {
    use super::*;

    fn popup() -> PopupState {
        PopupState {
            kind: PopupKind::Completion,
            items: vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()],
            selected: 0,
            visible: true,
            max_visible: 3,
            scroll_offset: 0,
        }
    }

    #[test]
    fn navigation() {
        let mut p = popup();
        p.select_next();
        assert_eq!(p.selected, 1);
        p.select_next();
        p.select_next(); // triggers scroll
        assert_eq!(p.selected, 3);
        assert_eq!(p.scroll_offset, 1);
        let mut p2 = popup();
        p2.select_prev();
        assert_eq!(p2.selected, 0);
    }

    #[test]
    fn visible_and_current() {
        let p = popup();
        assert_eq!(p.visible_items(), &["a", "b", "c"]);
        assert_eq!(p.current(), Some("a"));
    }

    #[test]
    fn show_hide() {
        let mut p = popup();
        p.hide();
        assert!(!p.visible);
        p.show(vec!["x".into(), "y".into()]);
        assert!(p.visible);
        assert_eq!(p.items.len(), 2);
    }
}
