//! Popup menu overlay for auto-complete and tooltips.

use serde::{Deserialize, Serialize};

/// Anchor position for a popup menu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PopupAnchor {
    AboveCursor,
    BelowCursor,
    ScreenCenter,
    AtPosition(u16, u16),
}

/// A scrollable popup menu.
#[derive(Debug, Clone)]
pub struct PopupMenu {
    pub items: Vec<String>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub max_visible: usize,
    pub anchor: PopupAnchor,
}

impl PopupMenu {
    /// Create a new popup menu.
    pub fn new(items: Vec<String>, max_visible: usize, anchor: PopupAnchor) -> Self {
        Self { items, selected: 0, scroll_offset: 0, max_visible, anchor }
    }

    /// Select the next item, scrolling if needed.
    pub fn select_next(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = (self.selected + 1).min(self.items.len() - 1);
        if self.selected >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.selected + 1 - self.max_visible;
        }
    }

    /// Select the previous item, scrolling if needed.
    pub fn select_prev(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = self.selected.saturating_sub(1);
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        }
    }

    /// Return the slice of items currently visible.
    pub fn visible_items(&self) -> &[String] {
        let end = (self.scroll_offset + self.max_visible).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    /// Return the currently selected item, if any.
    pub fn current(&self) -> Option<&str> {
        self.items.get(self.selected).map(|s| s.as_str())
    }
}

/// Compute the rectangle `(x, y, w, h)` for a popup given anchoring.
pub fn compute_rect(
    anchor: &PopupAnchor, screen_w: u16, screen_h: u16, popup_w: u16, popup_h: u16,
) -> (u16, u16, u16, u16) {
    match anchor {
        PopupAnchor::AboveCursor => (0, 0, popup_w.min(screen_w), popup_h.min(screen_h)),
        PopupAnchor::BelowCursor => (0, 1, popup_w.min(screen_w), popup_h.min(screen_h)),
        PopupAnchor::ScreenCenter => {
            let x = screen_w.saturating_sub(popup_w) / 2;
            let y = screen_h.saturating_sub(popup_h) / 2;
            (x, y, popup_w.min(screen_w), popup_h.min(screen_h))
        }
        PopupAnchor::AtPosition(px, py) => {
            let w = popup_w.min(screen_w.saturating_sub(*px));
            let h = popup_h.min(screen_h.saturating_sub(*py));
            (*px, *py, w, h)
        }
    }
}

/// A hover tooltip.
#[derive(Debug, Clone)]
pub struct HoverTooltip {
    pub text: String,
    pub position: (u16, u16),
    pub visible: bool,
}

impl HoverTooltip {
    /// Show a tooltip with the given text at position.
    pub fn show_tooltip(text: impl Into<String>, row: u16, col: u16) -> Self {
        Self { text: text.into(), position: (row, col), visible: true }
    }

    /// Dismiss the tooltip.
    pub fn dismiss_tooltip(&mut self) {
        self.visible = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popup_navigation() {
        let mut menu = PopupMenu::new(vec!["a".into(), "b".into(), "c".into()], 2, PopupAnchor::BelowCursor);
        assert_eq!(menu.current(), Some("a"));
        menu.select_next();
        assert_eq!(menu.current(), Some("b"));
        menu.select_next();
        assert_eq!(menu.current(), Some("c"));
        menu.select_prev();
        assert_eq!(menu.current(), Some("b"));
    }

    #[test]
    fn popup_visible_items() {
        let menu = PopupMenu::new(vec!["a".into(), "b".into(), "c".into()], 2, PopupAnchor::BelowCursor);
        assert_eq!(menu.visible_items().len(), 2);
    }

    #[test]
    fn popup_scroll() {
        let mut menu = PopupMenu::new(vec!["a".into(), "b".into(), "c".into(), "d".into()], 2, PopupAnchor::BelowCursor);
        menu.select_next(); // b
        menu.select_next(); // c -> scroll
        assert_eq!(menu.scroll_offset, 1);
    }

    #[test]
    fn compute_rect_center() {
        let (x, y, w, h) = compute_rect(&PopupAnchor::ScreenCenter, 80, 24, 20, 10);
        assert_eq!(x, 30);
        assert_eq!(y, 7);
        assert_eq!(w, 20);
        assert_eq!(h, 10);
    }

    #[test]
    fn compute_rect_at_position() {
        let (x, y, _, _) = compute_rect(&PopupAnchor::AtPosition(5, 10), 80, 24, 20, 10);
        assert_eq!(x, 5);
        assert_eq!(y, 10);
    }

    #[test]
    fn hover_tooltip() {
        let mut tt = HoverTooltip::show_tooltip("info", 5, 10);
        assert!(tt.visible);
        assert_eq!(tt.text, "info");
        tt.dismiss_tooltip();
        assert!(!tt.visible);
    }

    #[test]
    fn empty_popup() {
        let mut menu = PopupMenu::new(vec![], 5, PopupAnchor::BelowCursor);
        assert_eq!(menu.current(), None);
        menu.select_next(); // should not panic
        menu.select_prev();
    }
}
