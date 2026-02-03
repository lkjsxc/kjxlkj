//! Window identity types.

use serde::{Deserialize, Serialize};

/// Unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

impl WindowId {
    /// Creates a new window ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_id_equality() {
        let a = WindowId::new(1);
        let b = WindowId::new(1);
        assert_eq!(a, b);
    }
}
