//! Popup menu support.
//!
//! Provides popup menus for context menus, right-click, etc.

use crate::popup_item::PopupItem;

/// Popup menu.
#[derive(Debug, Clone)]
pub struct PopupMenu {
    /// Menu items.
    pub items: Vec<PopupItem>,
    /// Screen row.
    pub row: usize,
    /// Screen column.
    pub col: usize,
    /// Selected item index.
    pub selected: Option<usize>,
}

impl PopupMenu {
    /// Creates a new popup menu.
    pub fn new(items: Vec<PopupItem>, row: usize, col: usize) -> Self {
        Self {
            items,
            row,
            col,
            selected: None,
        }
    }

    /// Selects next item.
    pub fn select_next(&mut self) {
        let selectable: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.enabled && !item.separator)
            .map(|(i, _)| i)
            .collect();

        if selectable.is_empty() {
            return;
        }

        self.selected = match self.selected {
            None => Some(selectable[0]),
            Some(idx) => {
                let pos = selectable.iter().position(|&i| i == idx);
                match pos {
                    Some(p) if p + 1 < selectable.len() => Some(selectable[p + 1]),
                    _ => Some(selectable[0]),
                }
            }
        };
    }

    /// Selects previous item.
    pub fn prev(&mut self) {
        let selectable: Vec<usize> = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| item.enabled && !item.separator)
            .map(|(i, _)| i)
            .collect();

        if selectable.is_empty() {
            return;
        }

        self.selected = match self.selected {
            None => Some(*selectable.last().unwrap()),
            Some(idx) => {
                let pos = selectable.iter().position(|&i| i == idx);
                match pos {
                    Some(0) => Some(*selectable.last().unwrap()),
                    Some(p) => Some(selectable[p - 1]),
                    None => Some(*selectable.last().unwrap()),
                }
            }
        };
    }

    /// Returns the selected item.
    pub fn selected_item(&self) -> Option<&PopupItem> {
        self.selected.and_then(|idx| self.items.get(idx))
    }

    /// Returns visible item count (excluding hidden).
    pub fn visible_count(&self) -> usize {
        self.items.len()
    }
}

/// Popup menu state.
#[derive(Debug, Clone, Default)]
pub struct PopupState {
    /// Active popup menu.
    menu: Option<PopupMenu>,
}

impl PopupState {
    /// Creates new popup state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Opens a popup menu.
    pub fn open(&mut self, menu: PopupMenu) {
        self.menu = Some(menu);
    }

    /// Closes the popup menu.
    pub fn close(&mut self) {
        self.menu = None;
    }

    /// Returns whether a popup is open.
    pub fn is_open(&self) -> bool {
        self.menu.is_some()
    }

    /// Returns the active menu.
    pub fn menu(&self) -> Option<&PopupMenu> {
        self.menu.as_ref()
    }

    /// Returns mutable menu.
    pub fn menu_mut(&mut self) -> Option<&mut PopupMenu> {
        self.menu.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_items() -> Vec<PopupItem> {
        vec![
            PopupItem::new("cut", "Cut").with_shortcut("Ctrl+X"),
            PopupItem::new("copy", "Copy").with_shortcut("Ctrl+C"),
            PopupItem::separator(),
            PopupItem::new("paste", "Paste").with_shortcut("Ctrl+V"),
        ]
    }

    #[test]
    fn test_popup_menu_navigation() {
        let mut menu = PopupMenu::new(sample_items(), 10, 20);

        menu.select_next();
        assert_eq!(menu.selected, Some(0));

        menu.select_next();
        assert_eq!(menu.selected, Some(1));

        // Skip separator
        menu.select_next();
        assert_eq!(menu.selected, Some(3));
    }

    #[test]
    fn test_popup_menu_prev() {
        let mut menu = PopupMenu::new(sample_items(), 10, 20);
        menu.selected = Some(3);

        menu.prev();
        assert_eq!(menu.selected, Some(1)); // Skip separator
    }

    #[test]
    fn test_popup_menu_selected_item() {
        let mut menu = PopupMenu::new(sample_items(), 10, 20);
        menu.select_next();

        let item = menu.selected_item().unwrap();
        assert_eq!(item.id, "cut");
    }

    #[test]
    fn test_popup_state() {
        let mut state = PopupState::new();
        assert!(!state.is_open());

        state.open(PopupMenu::new(sample_items(), 10, 20));
        assert!(state.is_open());

        state.close();
        assert!(!state.is_open());
    }
}
