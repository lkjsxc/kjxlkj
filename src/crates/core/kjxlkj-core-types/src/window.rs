//! Window types and layout structures.

use crate::BufferId;
use serde::{Deserialize, Serialize};

/// Unique window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct WindowId(pub u64);

impl WindowId {
    /// Create a new window ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Terminal instance identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct TerminalId(pub u32);

impl TerminalId {
    /// Create a new terminal ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

/// Explorer state identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExplorerStateId(pub u64);

impl ExplorerStateId {
    /// Create a new explorer state ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Window content type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowContent {
    /// Buffer editing window.
    Buffer(BufferId),
    /// File explorer window.
    Explorer(ExplorerStateId),
    /// Terminal window.
    Terminal(TerminalId),
}

/// Viewport state for a window.
#[derive(Debug, Clone, Default)]
pub struct Viewport {
    /// First visible line.
    pub top_line: usize,
    /// First visible column (no-wrap mode).
    pub left_col: usize,
    /// Soft-wrap enabled.
    pub wrap: bool,
    /// Visible text rows.
    pub text_rows: u16,
    /// Visible text columns.
    pub text_cols: u16,
    /// Vertical scroll margin.
    pub scrolloff: usize,
    /// Horizontal scroll margin.
    pub sidescrolloff: usize,
}

/// Window options.
#[derive(Debug, Clone)]
pub struct WindowOptions {
    /// Show line numbers.
    pub number: bool,
    /// Show relative line numbers.
    pub relativenumber: bool,
    /// Wrap lines.
    pub wrap: bool,
    /// Sign column width.
    pub signcolumn: u8,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            number: true,
            relativenumber: false,
            wrap: true,
            signcolumn: 2,
        }
    }
}

/// Rectangle for window geometry.
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    /// X position (column).
    pub x: u16,
    /// Y position (row).
    pub y: u16,
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
}

impl Rect {
    /// Create a new rectangle.
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }

    /// Check if the rectangle has any area.
    pub fn has_area(&self) -> bool {
        self.width > 0 && self.height > 0
    }
}

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
    /// Horizontal split (children arranged top-to-bottom).
    Horizontal,
    /// Vertical split (children arranged left-to-right).
    Vertical,
}
