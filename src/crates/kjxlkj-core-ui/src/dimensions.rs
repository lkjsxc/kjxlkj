//! Dimension types.

use serde::{Deserialize, Serialize};

/// Terminal dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Dimensions {
    /// Width in columns.
    pub width: u16,
    /// Height in rows.
    pub height: u16,
}

impl Dimensions {
    /// Creates new dimensions.
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Returns the area (width * height).
    pub fn area(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Returns true if dimensions are zero.
    pub fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}
