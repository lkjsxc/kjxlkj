//! Buffer identity types.

use serde::{Deserialize, Serialize};

/// Unique, stable identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Creates a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Monotonically increasing buffer version for snapshot tagging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Creates a new buffer version.
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    /// Returns the next version.
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for BufferVersion {
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

    #[test]
    fn buffer_version_ordering() {
        let v1 = BufferVersion::new(1);
        let v2 = v1.next();
        assert!(v2 > v1);
    }
}
