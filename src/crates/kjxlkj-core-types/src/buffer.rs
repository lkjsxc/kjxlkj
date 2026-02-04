//! Buffer identification types.

use serde::{Deserialize, Serialize};

/// Unique, stable identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the raw ID value.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl Default for BufferId {
    fn default() -> Self {
        Self(0)
    }
}

/// Monotonically increasing version for buffer content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct BufferVersion(u64);

impl BufferVersion {
    /// Create a new buffer version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Increment the version.
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Get the raw version value.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_id_equality() {
        let id1 = BufferId::new(1);
        let id2 = BufferId::new(1);
        let id3 = BufferId::new(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn buffer_version_ordering() {
        let mut v = BufferVersion::default();
        assert_eq!(v.as_u64(), 0);
        v.increment();
        assert_eq!(v.as_u64(), 1);
    }
}
