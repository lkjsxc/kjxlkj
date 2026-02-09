//! Popup API per /docs/spec/features/ui/popup-api.md.
//!
//! Generic popup/menu system for various editor features.

/// Popup position anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupAnchor {
    /// Relative to cursor position.
    Cursor,
    /// Centered in editor.
    Center,
    /// Absolute position.
    Position(u16, u16),
}

/// A menu item in a popup.
#[derive(Debug, Clone)]
pub struct PopupItem {
    /// Display label.
    pub label: String,
    /// Optional detail/description.
    pub detail: Option<String>,
    /// Optional icon/prefix.
    pub icon: Option<String>,
    /// Whether this item is selectable.
    pub selectable: bool,
}

/// Popup/menu state.
#[derive(Debug, Clone)]
pub struct PopupMenu {
    /// Whether the popup is visible.
    pub visible: bool,
    /// Items in the menu.
    pub items: Vec<PopupItem>,
    /// Currently selected index.
    pub selected: usize,
    /// Anchor position.
    pub anchor: PopupAnchor,
    /// Maximum visible items.
    pub max_height: usize,
    /// Scroll offset.
    pub scroll_offset: usize,
    /// Title text.
    pub title: Option<String>,
}

impl Default for PopupMenu {
    fn default() -> Self {
        Self {
            visible: false,
            items: Vec::new(),
            selected: 0,
            anchor: PopupAnchor::Cursor,
            max_height: 10,
            scroll_offset: 0,
            title: None,
        }
    }
}

impl PopupMenu {
    /// Create new popup.
    pub fn new() -> Self {
        Self::default()
    }

    /// Show popup with items.
    pub fn show(
        &mut self,
        items: Vec<PopupItem>,
        anchor: PopupAnchor,
    ) {
        self.items = items;
        self.anchor = anchor;
        self.selected = 0;
        self.scroll_offset = 0;
        self.visible = true;
    }

    /// Hide popup.
    pub fn hide(&mut self) {
        self.visible = false;
        self.items.clear();
    }

    /// Select next item.
    pub fn next(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected =
            (self.selected + 1) % self.items.len();
        self.ensure_visible();
    }

    /// Select previous item.
    pub fn prev(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.items.len() - 1;
        } else {
            self.selected -= 1;
        }
        self.ensure_visible();
    }

    /// Get selected item.
    pub fn selected_item(
        &self,
    ) -> Option<&PopupItem> {
        self.items.get(self.selected)
    }

    /// Ensure selected is visible.
    fn ensure_visible(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected
            >= self.scroll_offset + self.max_height
        {
            self.scroll_offset =
                self.selected - self.max_height + 1;
        }
    }

    /// Get visible items slice.
    pub fn visible_items(&self) -> &[PopupItem] {
        let end = (self.scroll_offset + self.max_height)
            .min(self.items.len());
        &self.items[self.scroll_offset..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn popup_show_hide() {
        let mut popup = PopupMenu::new();
        popup.show(
            vec![PopupItem {
                label: "item".into(),
                detail: None,
                icon: None,
                selectable: true,
            }],
            PopupAnchor::Center,
        );
        assert!(popup.visible);
        popup.hide();
        assert!(!popup.visible);
    }

    #[test]
    fn popup_navigation() {
        let mut popup = PopupMenu::new();
        let items = (0..3)
            .map(|i| PopupItem {
                label: format!("item{}", i),
                detail: None,
                icon: None,
                selectable: true,
            })
            .collect();
        popup.show(items, PopupAnchor::Cursor);
        assert_eq!(popup.selected, 0);
        popup.next();
        assert_eq!(popup.selected, 1);
        popup.prev();
        assert_eq!(popup.selected, 0);
        popup.prev(); // Wraps.
        assert_eq!(popup.selected, 2);
    }
}
