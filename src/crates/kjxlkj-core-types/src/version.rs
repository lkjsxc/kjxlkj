//! Buffer version types.

use serde::{Deserialize, Serialize};

/// Monotonically increasing buffer version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BufferVersion(u64);

impl BufferVersion {
    /// Creates a new buffer version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Returns the initial version.
    pub fn initial() -> Self {
        Self(0)
    }

    /// Returns the next version.
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    /// Returns the raw version number.
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Default for BufferVersion {
    fn default() -> Self {
        Self::initial()
    }
}
