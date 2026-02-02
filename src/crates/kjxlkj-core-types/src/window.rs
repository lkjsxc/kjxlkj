//! Window identity types.

use serde::{Deserialize, Serialize};

/// Stable identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(u64);

impl WindowId {
    /// Creates a new window ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self(0)
    }
}
