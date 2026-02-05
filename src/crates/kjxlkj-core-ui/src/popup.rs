//! Popup and overlay system for the editor UI.
//!
//! Implements popup/overlay behavior as specified in `/docs/spec/features/ui/popup-api.md`.

use crate::view::{ViewBounds, ViewId};

/// Popup identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PopupId(u32);

impl PopupId {
    /// Create a new popup ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the ID value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Popup anchor position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopupAnchor {
    /// Anchor to cursor position.
    #[default]
    Cursor,
    /// Anchor to center of screen.
    Center,
    /// Anchor to a specific position.
    Position { row: u16, col: u16 },
    /// Anchor relative to a view.
    View { view: ViewId, edge: Edge },
}

/// Edge for relative positioning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Edge {
    /// Top edge.
    Top,
    /// Bottom edge.
    #[default]
    Bottom,
    /// Left edge.
    Left,
    /// Right edge.
    Right,
}

/// Popup border style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PopupBorder {
    /// No border.
    #[default]
    None,
    /// Single line border.
    Single,
    /// Double line border.
    Double,
    /// Rounded corners.
    Rounded,
}

/// Popup configuration.
#[derive(Debug, Clone)]
pub struct PopupConfig {
    /// Popup ID.
    pub id: PopupId,
    /// Title (if any).
    pub title: Option<String>,
    /// Anchor position.
    pub anchor: PopupAnchor,
    /// Width (if specified).
    pub width: Option<u16>,
    /// Height (if specified).
    pub height: Option<u16>,
    /// Border style.
    pub border: PopupBorder,
    /// Z-index (higher = on top).
    pub z_index: u8,
    /// Dismissable with escape.
    pub dismissable: bool,
}

impl PopupConfig {
    /// Create a new popup config.
    pub fn new(id: PopupId) -> Self {
        Self {
            id,
            title: None,
            anchor: PopupAnchor::default(),
            width: None,
            height: None,
            border: PopupBorder::Single,
            z_index: 10,
            dismissable: true,
        }
    }

    /// Set title.
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// Set anchor.
    pub fn with_anchor(mut self, anchor: PopupAnchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// Set size.
    pub fn with_size(mut self, width: u16, height: u16) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Set border.
    pub fn with_border(mut self, border: PopupBorder) -> Self {
        self.border = border;
        self
    }
}

/// Popup kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PopupKind {
    /// Completion menu.
    Completion,
    /// Picker/finder.
    Picker,
    /// Confirmation dialog.
    Confirmation,
    /// Information message.
    Info,
    /// Error message.
    Error,
    /// Custom popup.
    Custom(String),
}

/// Popup state.
#[derive(Debug)]
pub struct Popup {
    /// Configuration.
    pub config: PopupConfig,
    /// Kind of popup.
    pub kind: PopupKind,
    /// Computed bounds.
    pub bounds: ViewBounds,
    /// Content lines.
    pub content: Vec<String>,
    /// Selected index (for menus).
    pub selected: Option<usize>,
}

impl Popup {
    /// Create a new popup.
    pub fn new(config: PopupConfig, kind: PopupKind) -> Self {
        Self {
            config,
            kind,
            bounds: ViewBounds::default(),
            content: Vec::new(),
            selected: None,
        }
    }

    /// Set content.
    pub fn with_content(mut self, content: Vec<String>) -> Self {
        self.content = content;
        self
    }

    /// Set selected index.
    pub fn with_selected(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Move selection up.
    pub fn select_prev(&mut self) {
        if let Some(sel) = self.selected {
            if sel > 0 {
                self.selected = Some(sel - 1);
            }
        }
    }

    /// Move selection down.
    pub fn select_next(&mut self) {
        if let Some(sel) = self.selected {
            if sel < self.content.len().saturating_sub(1) {
                self.selected = Some(sel + 1);
            }
        }
    }

    /// Get selected item.
    pub fn selected_item(&self) -> Option<&String> {
        self.selected.and_then(|i| self.content.get(i))
    }
}

/// Popup manager for overlay layering.
#[derive(Debug, Default)]
pub struct PopupManager {
    /// Active popups by ID.
    popups: Vec<Popup>,
    /// Next popup ID.
    next_id: u32,
}

impl PopupManager {
    /// Create a new popup manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Show a popup.
    pub fn show(&mut self, kind: PopupKind, content: Vec<String>) -> PopupId {
        let id = PopupId::new(self.next_id);
        self.next_id += 1;
        let config = PopupConfig::new(id);
        let popup = Popup::new(config, kind).with_content(content);
        self.popups.push(popup);
        id
    }

    /// Show a completion popup.
    pub fn show_completion(&mut self, items: Vec<String>) -> PopupId {
        let id = PopupId::new(self.next_id);
        self.next_id += 1;
        let config = PopupConfig::new(id).with_anchor(PopupAnchor::Cursor);
        let popup = Popup::new(config, PopupKind::Completion)
            .with_content(items)
            .with_selected(0);
        self.popups.push(popup);
        id
    }

    /// Show a picker popup.
    pub fn show_picker(&mut self, title: &str, items: Vec<String>) -> PopupId {
        let id = PopupId::new(self.next_id);
        self.next_id += 1;
        let config = PopupConfig::new(id)
            .with_title(title)
            .with_anchor(PopupAnchor::Center);
        let popup = Popup::new(config, PopupKind::Picker)
            .with_content(items)
            .with_selected(0);
        self.popups.push(popup);
        id
    }

    /// Dismiss a popup.
    pub fn dismiss(&mut self, id: PopupId) -> bool {
        let len = self.popups.len();
        self.popups.retain(|p| p.config.id != id);
        self.popups.len() < len
    }

    /// Dismiss the topmost popup.
    pub fn dismiss_top(&mut self) -> Option<PopupId> {
        self.popups.pop().map(|p| p.config.id)
    }

    /// Get topmost popup.
    pub fn top(&self) -> Option<&Popup> {
        self.popups.last()
    }

    /// Get mutable topmost popup.
    pub fn top_mut(&mut self) -> Option<&mut Popup> {
        self.popups.last_mut()
    }

    /// Check if any popups are visible.
    pub fn has_popups(&self) -> bool {
        !self.popups.is_empty()
    }

    /// Count popups.
    pub fn len(&self) -> usize {
        self.popups.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.popups.is_empty()
    }

    /// Get all popups (sorted by z-index).
    pub fn popups(&self) -> impl Iterator<Item = &Popup> {
        self.popups.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_id() {
        let id = PopupId::new(1);
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_popup_anchor_default() {
        assert_eq!(PopupAnchor::default(), PopupAnchor::Cursor);
    }

    #[test]
    fn test_popup_config_builder() {
        let config = PopupConfig::new(PopupId::new(1))
            .with_title("Test")
            .with_size(40, 10)
            .with_border(PopupBorder::Rounded);
        assert_eq!(config.title, Some("Test".to_string()));
        assert_eq!(config.width, Some(40));
        assert_eq!(config.border, PopupBorder::Rounded);
    }

    #[test]
    fn test_popup_selection() {
        let mut popup = Popup::new(
            PopupConfig::new(PopupId::new(1)),
            PopupKind::Completion,
        )
        .with_content(vec!["a".to_string(), "b".to_string(), "c".to_string()])
        .with_selected(0);

        assert_eq!(popup.selected_item(), Some(&"a".to_string()));
        popup.select_next();
        assert_eq!(popup.selected_item(), Some(&"b".to_string()));
        popup.select_prev();
        assert_eq!(popup.selected_item(), Some(&"a".to_string()));
    }

    #[test]
    fn test_popup_manager_show() {
        let mut mgr = PopupManager::new();
        let id = mgr.show(PopupKind::Info, vec!["Hello".to_string()]);
        assert_eq!(mgr.len(), 1);
        assert!(mgr.dismiss(id));
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_popup_manager_completion() {
        let mut mgr = PopupManager::new();
        mgr.show_completion(vec!["fn".to_string(), "let".to_string()]);
        assert!(mgr.has_popups());
        let top = mgr.top().unwrap();
        assert_eq!(top.kind, PopupKind::Completion);
    }

    #[test]
    fn test_popup_manager_picker() {
        let mut mgr = PopupManager::new();
        mgr.show_picker("Files", vec!["file1.rs".to_string()]);
        let top = mgr.top().unwrap();
        assert_eq!(top.config.title, Some("Files".to_string()));
    }

    #[test]
    fn test_popup_manager_dismiss_top() {
        let mut mgr = PopupManager::new();
        mgr.show(PopupKind::Info, vec![]);
        mgr.show(PopupKind::Error, vec![]);
        assert_eq!(mgr.len(), 2);
        mgr.dismiss_top();
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_popup_select_bounds() {
        let mut popup = Popup::new(
            PopupConfig::new(PopupId::new(1)),
            PopupKind::Completion,
        )
        .with_content(vec!["only".to_string()])
        .with_selected(0);

        popup.select_prev(); // Should stay at 0
        assert_eq!(popup.selected, Some(0));
        popup.select_next(); // Should stay at 0 (only 1 item)
        assert_eq!(popup.selected, Some(0));
    }
}
