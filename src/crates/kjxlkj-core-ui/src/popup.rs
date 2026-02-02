//! Popup menu support.
//!
//! Provides popup menus for context menus, right-click, etc.

/// Popup menu item.
#[derive(Debug, Clone)]
pub struct PopupItem {
    /// Item ID.
    pub id: String,
    /// Display text.
    pub text: String,
    /// Keyboard shortcut hint.
    pub shortcut: Option<String>,
    /// Whether item is enabled.
    pub enabled: bool,
    /// Whether item is a separator.
    pub separator: bool,
    /// Submenu items.
    pub submenu: Option<Vec<PopupItem>>,
}

impl PopupItem {
    /// Creates a new popup item.
    pub fn new(id: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            text: text.to_string(),
            shortcut: None,
            enabled: true,
            separator: false,
            submenu: None,
        }
    }

    /// Creates a separator.
    pub fn separator() -> Self {
        Self {
            id: String::new(),
            text: String::new(),
            shortcut: None,
            enabled: false,
            separator: true,
            submenu: None,
        }
    }

    /// Sets the shortcut hint.
    pub fn with_shortcut(mut self, shortcut: &str) -> Self {
        self.shortcut = Some(shortcut.to_string());
        self
    }

    /// Disables the item.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Adds a submenu.
    pub fn with_submenu(mut self, items: Vec<PopupItem>) -> Self {
        self.submenu = Some(items);
        self
    }
}

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
    pub fn next(&mut self) {
        let selectable: Vec<usize> = self.items
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
        let selectable: Vec<usize> = self.items
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
    fn test_popup_item() {
        let item = PopupItem::new("cut", "Cut")
            .with_shortcut("Ctrl+X")
            .disabled();

        assert_eq!(item.id, "cut");
        assert!(!item.enabled);
    }

    #[test]
    fn test_popup_menu_navigation() {
        let mut menu = PopupMenu::new(sample_items(), 10, 20);

        menu.next();
        assert_eq!(menu.selected, Some(0));

        menu.next();
        assert_eq!(menu.selected, Some(1));

        // Skip separator
        menu.next();
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
        menu.next();

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

    #[test]
    fn test_popup_item_submenu() {
        let item = PopupItem::new("edit", "Edit")
            .with_submenu(vec![
                PopupItem::new("undo", "Undo"),
                PopupItem::new("redo", "Redo"),
            ]);

        assert!(item.submenu.is_some());
        assert_eq!(item.submenu.unwrap().len(), 2);
    }
}
