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

/// Manages a stack of popup overlays.
#[derive(Debug, Clone, Default)]
pub struct OverlayManager {
    stack: Vec<PopupState>,
}

impl OverlayManager {
    pub fn open(&mut self, popup: PopupState) {
        self.stack.push(popup);
    }

    pub fn close_kind(&mut self, kind: PopupKind) {
        self.stack.retain(|p| p.kind != kind);
    }

    pub fn close_all(&mut self) {
        self.stack.clear();
    }

    pub fn top(&self) -> Option<&PopupState> {
        self.stack.last()
    }
}

/// Compute the rectangle `(x, y, w, h)` for a popup given an anchor and
/// screen/popup dimensions.
pub fn compute_popup_rect(
    anchor: PopupAnchor,
    screen_w: u16,
    screen_h: u16,
    popup_w: u16,
    popup_h: u16,
) -> (u16, u16, u16, u16) {
    let w = popup_w.min(screen_w);
    let h = popup_h.min(screen_h);
    match anchor {
        PopupAnchor::TopLeft => (0, 0, w, h),
        PopupAnchor::Center => {
            let x = screen_w.saturating_sub(w) / 2;
            let y = screen_h.saturating_sub(h) / 2;
            (x, y, w, h)
        }
        PopupAnchor::CmdLine => {
            let y = screen_h.saturating_sub(h).saturating_sub(1);
            (0, y, w, h)
        }
        PopupAnchor::Cursor => {
            // Default: near top-left with small offset for cursor popup.
            let x = 1_u16.min(screen_w.saturating_sub(w));
            let y = 1_u16.min(screen_h.saturating_sub(h));
            (x, y, w, h)
        }
    }
}

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
    fn overlay_manager() {
        let mut mgr = OverlayManager::default();
        mgr.open(popup());
        let mut h = popup();
        h.kind = PopupKind::Hover;
        mgr.open(h);
        assert_eq!(mgr.top().unwrap().kind, PopupKind::Hover);
        mgr.close_kind(PopupKind::Hover);
        assert_eq!(mgr.top().unwrap().kind, PopupKind::Completion);
        mgr.close_all();
        assert!(mgr.top().is_none());
    }

    #[test]
    fn popup_rects() {
        assert_eq!(
            compute_popup_rect(PopupAnchor::Center, 80, 24, 20, 10),
            (30, 7, 20, 10)
        );
        let (_, _, w, h) = compute_popup_rect(PopupAnchor::TopLeft, 10, 5, 20, 10);
        assert_eq!((w, h), (10, 5));
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
