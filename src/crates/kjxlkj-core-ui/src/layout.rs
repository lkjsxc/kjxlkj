//! Layout model: rectangles, layout nodes, and standard layout computation.

use serde::{Deserialize, Serialize};

/// Axis-aligned rectangle for layout.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub const fn new(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self { x, y, w, h }
    }

    /// Check whether point (px, py) lies inside this rectangle.
    pub fn contains(&self, px: u16, py: u16) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    /// Split horizontally at row `at` (relative to origin). Returns (top, bottom).
    pub fn split_horizontal(&self, at: u16) -> (Rect, Rect) {
        let at = at.min(self.h);
        (
            Rect::new(self.x, self.y, self.w, at),
            Rect::new(self.x, self.y + at, self.w, self.h.saturating_sub(at)),
        )
    }

    /// Split vertically at column `at` (relative to origin). Returns (left, right).
    pub fn split_vertical(&self, at: u16) -> (Rect, Rect) {
        let at = at.min(self.w);
        (
            Rect::new(self.x, self.y, at, self.h),
            Rect::new(self.x + at, self.y, self.w.saturating_sub(at), self.h),
        )
    }

    /// Area in cells.
    pub fn area(&self) -> u32 {
        self.w as u32 * self.h as u32
    }

    /// Check whether this rectangle overlaps with another.
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}

/// Kind of UI component a layout node represents.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentKind {
    BufferView,
    StatusLine,
    TabLine,
    CommandLine,
    LineNumbers,
    SignColumn,
}

/// A positioned UI component in the layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutNode {
    pub id: u64,
    pub kind: ComponentKind,
    pub rect: Rect,
    pub visible: bool,
}

// Re-export standard layout from dedicated module.
pub use crate::layout_standard::standard_layout;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_contains() {
        let r = Rect::new(5, 5, 10, 10);
        assert!(r.contains(5, 5));
        assert!(r.contains(14, 14));
        assert!(!r.contains(15, 5));
    }

    #[test]
    fn rect_split_horiz() {
        let r = Rect::new(0, 0, 80, 24);
        let (top, bot) = r.split_horizontal(10);
        assert_eq!(top.h, 10);
        assert_eq!(bot.h, 14);
        assert_eq!(bot.y, 10);
    }

    #[test]
    fn rect_split_vert() {
        let r = Rect::new(0, 0, 80, 24);
        let (left, right) = r.split_vertical(40);
        assert_eq!(left.w, 40);
        assert_eq!(right.w, 40);
    }

    #[test]
    fn rect_area() {
        assert_eq!(Rect::new(0, 0, 10, 20).area(), 200);
    }

    #[test]
    fn rect_overlaps() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        let c = Rect::new(20, 20, 5, 5);
        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }
}
