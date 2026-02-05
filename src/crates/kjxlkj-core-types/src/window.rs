//! Window identification.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(u64);

impl WindowId {
    /// Create a new window ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw ID value.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl fmt::Display for WindowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Window({})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_id_equality() {
        let id1 = WindowId::new(1);
        let id2 = WindowId::new(1);
        let id3 = WindowId::new(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }
}
