//! Popup menu overlay — completion menus, hover boxes, context menus.

use crate::component::Rect;

/// Popup anchor point relative to the cursor/trigger.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupAnchor { AboveCursor, BelowCursor, ScreenCenter, AtPosition(u16, u16) }

/// A popup menu item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PopupItem {
    pub label: String,
    pub detail: Option<String>,
    pub icon: Option<char>,
}

impl PopupItem {
    pub fn new(label: &str) -> Self { Self { label: label.into(), detail: None, icon: None } }
    pub fn with_detail(mut self, d: &str) -> Self { self.detail = Some(d.into()); self }
    pub fn with_icon(mut self, i: char) -> Self { self.icon = Some(i); self }
}

/// A popup menu with items, selection, and scroll state.
#[derive(Debug, Clone)]
pub struct PopupMenu {
    pub items: Vec<PopupItem>,
    pub selected: usize,
    pub scroll_offset: usize,
    pub max_visible: usize,
    pub anchor: PopupAnchor,
    pub visible: bool,
}

impl PopupMenu {
    pub fn new(items: Vec<PopupItem>, anchor: PopupAnchor) -> Self {
        Self { items, selected: 0, scroll_offset: 0, max_visible: 10, anchor, visible: true }
    }

    pub fn select_next(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = (self.selected + 1) % self.items.len();
        self.ensure_visible();
    }

    pub fn select_prev(&mut self) {
        if self.items.is_empty() { return; }
        self.selected = if self.selected == 0 { self.items.len() - 1 } else { self.selected - 1 };
        self.ensure_visible();
    }

    pub fn selected_item(&self) -> Option<&PopupItem> { self.items.get(self.selected) }

    fn ensure_visible(&mut self) {
        if self.selected < self.scroll_offset { self.scroll_offset = self.selected; }
        if self.selected >= self.scroll_offset + self.max_visible {
            self.scroll_offset = self.selected + 1 - self.max_visible;
        }
    }

    pub fn visible_items(&self) -> &[PopupItem] {
        let end = (self.scroll_offset + self.max_visible).min(self.items.len());
        &self.items[self.scroll_offset..end]
    }

    pub fn dismiss(&mut self) { self.visible = false; }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    /// Compute the popup rect given anchor, item count, and screen bounds.
    pub fn compute_rect(&self, screen_w: u16, screen_h: u16, cursor_x: u16, cursor_y: u16) -> Rect {
        let w = self.items.iter().map(|i| i.label.len()).max().unwrap_or(10).min(40) as u16 + 4;
        let h = self.visible_items().len().min(self.max_visible) as u16;
        match self.anchor {
            PopupAnchor::BelowCursor => {
                let y = (cursor_y + 1).min(screen_h.saturating_sub(h));
                let x = cursor_x.min(screen_w.saturating_sub(w));
                Rect::new(x, y, w, h)
            }
            PopupAnchor::AboveCursor => {
                let y = cursor_y.saturating_sub(h);
                let x = cursor_x.min(screen_w.saturating_sub(w));
                Rect::new(x, y, w, h)
            }
            PopupAnchor::ScreenCenter => {
                let x = screen_w.saturating_sub(w) / 2;
                let y = screen_h.saturating_sub(h) / 2;
                Rect::new(x, y, w, h)
            }
            PopupAnchor::AtPosition(x, y) => Rect::new(x, y, w, h),
        }
    }
}

/// A hover tooltip with positioned content.
#[derive(Debug, Clone)]
pub struct HoverTooltip {
    pub content: String,
    pub rect: Option<Rect>,
    pub visible: bool,
}

impl HoverTooltip {
    pub fn new(content: &str) -> Self { Self { content: content.into(), rect: None, visible: true } }
    pub fn dismiss(&mut self) { self.visible = false; }
    pub fn position(&mut self, x: u16, y: u16, w: u16) {
        let lines = self.content.lines().count().max(1) as u16;
        self.rect = Some(Rect::new(x, y.saturating_sub(lines), w, lines));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items(n: usize) -> Vec<PopupItem> {
        (0..n).map(|i| PopupItem::new(&format!("item_{}", i))).collect()
    }

    #[test]
    fn select_cycles() {
        let mut menu = PopupMenu::new(items(3), PopupAnchor::BelowCursor);
        assert_eq!(menu.selected, 0);
        menu.select_next(); assert_eq!(menu.selected, 1);
        menu.select_next(); assert_eq!(menu.selected, 2);
        menu.select_next(); assert_eq!(menu.selected, 0); // wrap
    }

    #[test]
    fn select_prev_cycles() {
        let mut menu = PopupMenu::new(items(3), PopupAnchor::BelowCursor);
        menu.select_prev(); assert_eq!(menu.selected, 2); // wrap backwards
    }

    #[test]
    fn visible_items_window() {
        let mut menu = PopupMenu::new(items(20), PopupAnchor::BelowCursor);
        menu.max_visible = 5;
        assert_eq!(menu.visible_items().len(), 5);
    }

    #[test]
    fn scroll_follows_selection() {
        let mut menu = PopupMenu::new(items(20), PopupAnchor::BelowCursor);
        menu.max_visible = 5;
        for _ in 0..7 { menu.select_next(); }
        assert!(menu.scroll_offset > 0);
        assert!(menu.visible_items().iter().any(|i| i.label == "item_7"));
    }

    #[test]
    fn compute_rect_below() {
        let menu = PopupMenu::new(items(3), PopupAnchor::BelowCursor);
        let r = menu.compute_rect(80, 24, 10, 5);
        assert_eq!(r.y, 6); // below cursor
    }

    #[test]
    fn compute_rect_center() {
        let menu = PopupMenu::new(items(3), PopupAnchor::ScreenCenter);
        let r = menu.compute_rect(80, 24, 0, 0);
        assert!(r.x > 0 && r.y > 0);
    }

    #[test]
    fn hover_tooltip() {
        let mut tt = HoverTooltip::new("fn main()");
        tt.position(10, 5, 20);
        assert!(tt.rect.is_some());
        tt.dismiss();
        assert!(!tt.visible);
    }

    #[test]
    fn popup_item_builder() {
        let item = PopupItem::new("test").with_detail("detail").with_icon('f');
        assert_eq!(item.icon, Some('f'));
        assert_eq!(item.detail.as_deref(), Some("detail"));
    }
}
