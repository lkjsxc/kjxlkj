//! Popup menu item type.

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_item_new() {
        let item = PopupItem::new("cut", "Cut");
        assert_eq!(item.id, "cut");
        assert_eq!(item.text, "Cut");
        assert!(item.enabled);
        assert!(!item.separator);
    }

    #[test]
    fn test_popup_item_with_shortcut() {
        let item = PopupItem::new("cut", "Cut").with_shortcut("Ctrl+X");
        assert_eq!(item.shortcut, Some("Ctrl+X".to_string()));
    }

    #[test]
    fn test_popup_item_disabled() {
        let item = PopupItem::new("cut", "Cut").disabled();
        assert!(!item.enabled);
    }

    #[test]
    fn test_popup_item_separator() {
        let item = PopupItem::separator();
        assert!(item.separator);
        assert!(!item.enabled);
    }

    #[test]
    fn test_popup_item_submenu() {
        let item = PopupItem::new("edit", "Edit").with_submenu(vec![
            PopupItem::new("undo", "Undo"),
            PopupItem::new("redo", "Redo"),
        ]);

        assert!(item.submenu.is_some());
        assert_eq!(item.submenu.unwrap().len(), 2);
    }
}
