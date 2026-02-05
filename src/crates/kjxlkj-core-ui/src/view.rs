//! View composition model for the editor UI.
//!
//! Implements view layout and focus as specified in `/docs/spec/ui/views.md`.

use std::collections::HashMap;

/// View identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ViewId(u32);

impl ViewId {
    /// Create a new view ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the ID value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// View type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewType {
    /// Main editor view.
    #[default]
    Editor,
    /// Command line view.
    CommandLine,
    /// Status line view.
    StatusLine,
    /// File explorer view.
    Explorer,
    /// Terminal pane view.
    Terminal,
    /// Popup/floating view.
    Popup,
    /// Message/notification view.
    Message,
}

/// View visibility state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewVisibility {
    /// View is visible and can receive focus.
    #[default]
    Visible,
    /// View is hidden.
    Hidden,
    /// View is collapsed (takes no space).
    Collapsed,
}

/// View bounds (position and size).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ViewBounds {
    /// Top-left row.
    pub row: u16,
    /// Top-left column.
    pub col: u16,
    /// Height in rows.
    pub height: u16,
    /// Width in columns.
    pub width: u16,
}

impl ViewBounds {
    /// Create new view bounds.
    pub fn new(row: u16, col: u16, height: u16, width: u16) -> Self {
        Self { row, col, height, width }
    }

    /// Check if bounds contain a position.
    pub fn contains(&self, row: u16, col: u16) -> bool {
        row >= self.row
            && row < self.row + self.height
            && col >= self.col
            && col < self.col + self.width
    }

    /// Get the area (rows * cols).
    pub fn area(&self) -> u32 {
        self.height as u32 * self.width as u32
    }
}

/// View configuration.
#[derive(Debug, Clone)]
pub struct ViewConfig {
    /// View ID.
    pub id: ViewId,
    /// View type.
    pub view_type: ViewType,
    /// Visibility state.
    pub visibility: ViewVisibility,
    /// Current bounds.
    pub bounds: ViewBounds,
    /// Can receive focus.
    pub focusable: bool,
}

impl ViewConfig {
    /// Create a new view config.
    pub fn new(id: ViewId, view_type: ViewType) -> Self {
        Self {
            id,
            view_type,
            visibility: ViewVisibility::default(),
            bounds: ViewBounds::default(),
            focusable: matches!(
                view_type,
                ViewType::Editor | ViewType::CommandLine | ViewType::Explorer | ViewType::Terminal
            ),
        }
    }

    /// Set bounds.
    pub fn with_bounds(mut self, bounds: ViewBounds) -> Self {
        self.bounds = bounds;
        self
    }

    /// Set visibility.
    pub fn with_visibility(mut self, visibility: ViewVisibility) -> Self {
        self.visibility = visibility;
        self
    }
}

/// Focus manager for views.
#[derive(Debug, Default)]
pub struct FocusManager {
    /// Currently focused view.
    focused: Option<ViewId>,
    /// Focus history (for returning focus).
    history: Vec<ViewId>,
    /// Maximum history size.
    max_history: usize,
}

impl FocusManager {
    /// Create a new focus manager.
    pub fn new() -> Self {
        Self {
            focused: None,
            history: Vec::new(),
            max_history: 10,
        }
    }

    /// Get the currently focused view.
    pub fn focused(&self) -> Option<ViewId> {
        self.focused
    }

    /// Set focus to a view.
    pub fn focus(&mut self, id: ViewId) {
        if let Some(prev) = self.focused {
            if prev != id {
                self.history.push(prev);
                if self.history.len() > self.max_history {
                    self.history.remove(0);
                }
            }
        }
        self.focused = Some(id);
    }

    /// Return focus to previous view.
    pub fn focus_previous(&mut self) -> Option<ViewId> {
        if let Some(prev) = self.history.pop() {
            self.focused = Some(prev);
            Some(prev)
        } else {
            None
        }
    }

    /// Clear focus.
    pub fn clear(&mut self) {
        self.focused = None;
    }
}

/// View layout manager.
#[derive(Debug, Default)]
pub struct ViewLayout {
    /// Registered views.
    views: HashMap<ViewId, ViewConfig>,
    /// Focus manager.
    focus: FocusManager,
    /// Next view ID.
    next_id: u32,
    /// Terminal size.
    size: (u16, u16),
}

impl ViewLayout {
    /// Create a new view layout.
    pub fn new(rows: u16, cols: u16) -> Self {
        Self {
            views: HashMap::new(),
            focus: FocusManager::new(),
            next_id: 1,
            size: (rows, cols),
        }
    }

    /// Create a view.
    pub fn create_view(&mut self, view_type: ViewType) -> ViewId {
        let id = ViewId::new(self.next_id);
        self.next_id += 1;
        let config = ViewConfig::new(id, view_type);
        self.views.insert(id, config);
        id
    }

    /// Get view config.
    pub fn get(&self, id: ViewId) -> Option<&ViewConfig> {
        self.views.get(&id)
    }

    /// Get mutable view config.
    pub fn get_mut(&mut self, id: ViewId) -> Option<&mut ViewConfig> {
        self.views.get_mut(&id)
    }

    /// Remove a view.
    pub fn remove(&mut self, id: ViewId) -> Option<ViewConfig> {
        self.views.remove(&id)
    }

    /// Get focused view.
    pub fn focused(&self) -> Option<ViewId> {
        self.focus.focused()
    }

    /// Set focus.
    pub fn focus(&mut self, id: ViewId) {
        if self.views.get(&id).is_some_and(|v| v.focusable) {
            self.focus.focus(id);
        }
    }

    /// Return to previous focus.
    pub fn focus_previous(&mut self) -> Option<ViewId> {
        self.focus.focus_previous()
    }

    /// Update terminal size.
    pub fn resize(&mut self, rows: u16, cols: u16) {
        self.size = (rows, cols);
    }

    /// Get terminal size.
    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    /// Get visible views sorted by z-order (back to front).
    pub fn visible_views(&self) -> Vec<&ViewConfig> {
        let mut views: Vec<_> = self
            .views
            .values()
            .filter(|v| v.visibility == ViewVisibility::Visible)
            .collect();
        // Sort: Editor first, then others, popups last
        views.sort_by_key(|v| match v.view_type {
            ViewType::Editor => 0,
            ViewType::StatusLine => 1,
            ViewType::CommandLine => 2,
            ViewType::Explorer => 3,
            ViewType::Terminal => 4,
            ViewType::Message => 5,
            ViewType::Popup => 6,
        });
        views
    }

    /// Count views.
    pub fn len(&self) -> usize {
        self.views.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.views.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_id() {
        let id = ViewId::new(1);
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_view_type_default() {
        assert_eq!(ViewType::default(), ViewType::Editor);
    }

    #[test]
    fn test_view_visibility_default() {
        assert_eq!(ViewVisibility::default(), ViewVisibility::Visible);
    }

    #[test]
    fn test_view_bounds_contains() {
        let bounds = ViewBounds::new(10, 20, 5, 10);
        assert!(bounds.contains(10, 20));
        assert!(bounds.contains(14, 29));
        assert!(!bounds.contains(15, 20));
        assert!(!bounds.contains(10, 30));
    }

    #[test]
    fn test_view_bounds_area() {
        let bounds = ViewBounds::new(0, 0, 10, 20);
        assert_eq!(bounds.area(), 200);
    }

    #[test]
    fn test_view_config_focusable() {
        let config = ViewConfig::new(ViewId::new(1), ViewType::Editor);
        assert!(config.focusable);
        let config2 = ViewConfig::new(ViewId::new(2), ViewType::StatusLine);
        assert!(!config2.focusable);
    }

    #[test]
    fn test_focus_manager() {
        let mut fm = FocusManager::new();
        assert!(fm.focused().is_none());
        fm.focus(ViewId::new(1));
        assert_eq!(fm.focused(), Some(ViewId::new(1)));
    }

    #[test]
    fn test_focus_manager_history() {
        let mut fm = FocusManager::new();
        fm.focus(ViewId::new(1));
        fm.focus(ViewId::new(2));
        assert_eq!(fm.focused(), Some(ViewId::new(2)));
        assert_eq!(fm.focus_previous(), Some(ViewId::new(1)));
    }

    #[test]
    fn test_view_layout_create() {
        let mut layout = ViewLayout::new(24, 80);
        let id = layout.create_view(ViewType::Editor);
        assert!(layout.get(id).is_some());
    }

    #[test]
    fn test_view_layout_focus() {
        let mut layout = ViewLayout::new(24, 80);
        let id = layout.create_view(ViewType::Editor);
        layout.focus(id);
        assert_eq!(layout.focused(), Some(id));
    }
}
