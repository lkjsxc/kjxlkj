//! Window layout tree for rendering.

use serde::{Deserialize, Serialize};

use kjxlkj_core_types::WindowId;

/// A rectangle in terminal cell coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    /// Column of the top-left corner.
    pub x: u16,
    /// Row of the top-left corner.
    pub y: u16,
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Area in cells.
    pub fn area(&self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Right edge (exclusive).
    pub fn right(&self) -> u16 {
        self.x + self.width
    }

    /// Bottom edge (exclusive).
    pub fn bottom(&self) -> u16 {
        self.y + self.height
    }

    /// Whether a point is inside this rect.
    pub fn contains(&self, col: u16, row: u16) -> bool {
        col >= self.x
            && col < self.right()
            && row >= self.y
            && row < self.bottom()
    }
}

/// Layout tree node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayoutNode {
    /// A leaf window.
    Leaf {
        window_id: WindowId,
        rect: Rect,
    },
    /// Horizontal split (children arranged top-to-bottom).
    HorizontalSplit {
        children: Vec<LayoutNode>,
        rect: Rect,
    },
    /// Vertical split (children arranged left-to-right).
    VerticalSplit {
        children: Vec<LayoutNode>,
        rect: Rect,
    },
}

impl LayoutNode {
    /// Get the bounding rect of this node.
    pub fn rect(&self) -> Rect {
        match self {
            LayoutNode::Leaf { rect, .. } => *rect,
            LayoutNode::HorizontalSplit { rect, .. } => *rect,
            LayoutNode::VerticalSplit { rect, .. } => *rect,
        }
    }

    /// Collect all leaf window IDs.
    pub fn window_ids(&self) -> Vec<WindowId> {
        match self {
            LayoutNode::Leaf { window_id, .. } => vec![*window_id],
            LayoutNode::HorizontalSplit { children, .. }
            | LayoutNode::VerticalSplit { children, .. } => {
                children.iter().flat_map(|c| c.window_ids()).collect()
            }
        }
    }

    /// Find the rect for a specific window ID.
    pub fn find_window(&self, id: WindowId) -> Option<Rect> {
        match self {
            LayoutNode::Leaf { window_id, rect } => {
                if *window_id == id {
                    Some(*rect)
                } else {
                    None
                }
            }
            LayoutNode::HorizontalSplit { children, .. }
            | LayoutNode::VerticalSplit { children, .. } => {
                children.iter().find_map(|c| c.find_window(id))
            }
        }
    }
}

/// Resolved window layout with all rectangles computed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowLayout {
    /// The root layout node.
    pub root: LayoutNode,
    /// The focused window ID.
    pub focused: WindowId,
}

impl WindowLayout {
    /// Create a single-window layout filling the given rectangle.
    pub fn single(
        window_id: WindowId,
        rect: Rect,
    ) -> Self {
        Self {
            root: LayoutNode::Leaf { window_id, rect },
            focused: window_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_contains() {
        let r = Rect::new(5, 5, 10, 10);
        assert!(r.contains(5, 5));
        assert!(r.contains(14, 14));
        assert!(!r.contains(15, 15));
    }

    #[test]
    fn single_layout() {
        let wid = WindowId(1);
        let layout =
            WindowLayout::single(wid, Rect::new(0, 0, 80, 24));
        assert_eq!(layout.root.window_ids(), vec![wid]);
    }

    #[test]
    fn find_window() {
        let wid = WindowId(1);
        let layout =
            WindowLayout::single(wid, Rect::new(0, 0, 80, 24));
        assert!(layout.root.find_window(wid).is_some());
        assert!(layout.root.find_window(WindowId(2)).is_none());
    }
}
