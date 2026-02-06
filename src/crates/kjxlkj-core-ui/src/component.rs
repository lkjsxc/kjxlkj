//! UI component model — composable, deterministic view components.
//!
//! Components are layout-agnostic building blocks that the renderer
//! assembles into the final frame. Each produces a snapshot that
//! the renderer draws, enabling deterministic snapshot testing.

use kjxlkj_core_types::Size;

/// A unique component identifier within a layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(pub u32);

/// Component kind — which UI element this represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentKind {
    BufferView,
    StatusLine,
    CommandLine,
    TabLine,
    LineNumbers,
    SignColumn,
    Explorer,
    Popup,
    Notification,
    FloatingWindow,
}

/// Component visibility state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility { Visible, Hidden, Collapsed }

/// Layout direction for component composition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutDirection { Horizontal, Vertical }

/// A constraint on component sizing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeConstraint {
    /// Fixed pixel/cell size.
    Fixed(u16),
    /// Percentage of parent (0–100).
    Percent(u8),
    /// Fill remaining space.
    Fill,
    /// Minimum size.
    Min(u16),
}

/// A rectangular region on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect { pub x: u16, pub y: u16, pub width: u16, pub height: u16 }

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }
    pub fn from_size(size: Size) -> Self {
        Self { x: 0, y: 0, width: size.width, height: size.height }
    }
    pub fn area(&self) -> u32 { self.width as u32 * self.height as u32 }
    pub fn contains(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
    /// Split horizontally at offset, returning (top, bottom).
    pub fn split_horizontal(&self, at: u16) -> (Rect, Rect) {
        let at = at.min(self.height);
        (Rect::new(self.x, self.y, self.width, at),
         Rect::new(self.x, self.y + at, self.width, self.height.saturating_sub(at)))
    }
    /// Split vertically at offset, returning (left, right).
    pub fn split_vertical(&self, at: u16) -> (Rect, Rect) {
        let at = at.min(self.width);
        (Rect::new(self.x, self.y, at, self.height),
         Rect::new(self.x + at, self.y, self.width.saturating_sub(at), self.height))
    }
}

/// A component layout node for the UI tree.
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: ComponentId,
    pub kind: ComponentKind,
    pub rect: Rect,
    pub visibility: Visibility,
}

impl LayoutNode {
    pub fn new(id: ComponentId, kind: ComponentKind, rect: Rect) -> Self {
        Self { id, kind, rect, visibility: Visibility::Visible }
    }
    pub fn is_visible(&self) -> bool { self.visibility == Visibility::Visible }
}

/// Compute a standard editor layout: tab line, buffer, status line, command line.
pub fn standard_layout(size: Size) -> Vec<LayoutNode> {
    let w = size.width;
    let h = size.height;
    if h < 3 { return Vec::new(); }
    let tab_h = 1u16;
    let cmd_h = 1u16;
    let status_h = 1u16;
    let buf_h = h.saturating_sub(tab_h + status_h + cmd_h);
    vec![
        LayoutNode::new(ComponentId(0), ComponentKind::TabLine, Rect::new(0, 0, w, tab_h)),
        LayoutNode::new(ComponentId(1), ComponentKind::BufferView, Rect::new(0, tab_h, w, buf_h)),
        LayoutNode::new(ComponentId(2), ComponentKind::StatusLine, Rect::new(0, tab_h + buf_h, w, status_h)),
        LayoutNode::new(ComponentId(3), ComponentKind::CommandLine, Rect::new(0, h - cmd_h, w, cmd_h)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_area_and_contains() {
        let r = Rect::new(5, 10, 20, 10);
        assert_eq!(r.area(), 200);
        assert!(r.contains(5, 10));
        assert!(r.contains(24, 19));
        assert!(!r.contains(25, 10));
        assert!(!r.contains(5, 20));
    }

    #[test]
    fn rect_split_horizontal() {
        let r = Rect::new(0, 0, 80, 24);
        let (top, bot) = r.split_horizontal(10);
        assert_eq!(top.height, 10);
        assert_eq!(bot.y, 10);
        assert_eq!(bot.height, 14);
    }

    #[test]
    fn rect_split_vertical() {
        let r = Rect::new(0, 0, 80, 24);
        let (left, right) = r.split_vertical(30);
        assert_eq!(left.width, 30);
        assert_eq!(right.x, 30);
        assert_eq!(right.width, 50);
    }

    #[test]
    fn standard_layout_components() {
        let layout = standard_layout(Size::new(80, 24));
        assert_eq!(layout.len(), 4);
        assert_eq!(layout[0].kind, ComponentKind::TabLine);
        assert_eq!(layout[1].kind, ComponentKind::BufferView);
        assert_eq!(layout[2].kind, ComponentKind::StatusLine);
        assert_eq!(layout[3].kind, ComponentKind::CommandLine);
        // Buffer view should take most space
        assert!(layout[1].rect.height > 10);
    }

    #[test]
    fn standard_layout_too_small() {
        let layout = standard_layout(Size::new(80, 2));
        assert!(layout.is_empty(), "too small for standard layout");
    }

    #[test]
    fn component_kinds_distinct() {
        assert_ne!(ComponentKind::BufferView, ComponentKind::StatusLine);
        assert_ne!(ComponentKind::Explorer, ComponentKind::Popup);
    }

    #[test]
    fn layout_node_visibility() {
        let mut node = LayoutNode::new(ComponentId(0), ComponentKind::StatusLine, Rect::new(0, 0, 80, 1));
        assert!(node.is_visible());
        node.visibility = Visibility::Hidden;
        assert!(!node.is_visible());
    }

    #[test]
    fn rect_from_size() {
        let r = Rect::from_size(Size::new(120, 40));
        assert_eq!(r.x, 0);
        assert_eq!(r.y, 0);
        assert_eq!(r.width, 120);
        assert_eq!(r.height, 40);
    }
}
