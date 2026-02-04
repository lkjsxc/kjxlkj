//! Buffer identifier.

use serde::{Deserialize, Serialize};

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl Default for BufferId {
    fn default() -> Self {
        Self(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_id_equality() {
        let a = BufferId::new(1);
        let b = BufferId::new(1);
        let c = BufferId::new(2);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
