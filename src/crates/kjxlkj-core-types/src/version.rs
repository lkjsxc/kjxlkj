//! Buffer version tracking.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Monotonic version number for buffer snapshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BufferVersion(u64);

impl BufferVersion {
    /// Create initial version.
    pub fn initial() -> Self {
        Self(0)
    }

    /// Create a version from a raw value.
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    /// Increment and return the next version.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }

    /// Get the raw version number.
    pub fn as_u64(self) -> u64 {
        self.0
    }
}

impl Default for BufferVersion {
    fn default() -> Self {
        Self::initial()
    }
}

/// Atomic version counter for thread-safe version generation.
pub struct VersionCounter {
    counter: AtomicU64,
}

impl VersionCounter {
    /// Create a new counter starting at 0.
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(0),
        }
    }

    /// Get the next version, incrementing the counter.
    pub fn next(&self) -> BufferVersion {
        let v = self.counter.fetch_add(1, Ordering::SeqCst);
        BufferVersion(v + 1)
    }

    /// Get current version without incrementing.
    pub fn current(&self) -> BufferVersion {
        BufferVersion(self.counter.load(Ordering::SeqCst))
    }
}

impl Default for VersionCounter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_ordering() {
        let v1 = BufferVersion::new(1);
        let v2 = BufferVersion::new(2);
        assert!(v1 < v2);
    }

    #[test]
    fn version_next() {
        let v = BufferVersion::initial();
        assert_eq!(v.as_u64(), 0);
        let v2 = v.next();
        assert_eq!(v2.as_u64(), 1);
    }

    #[test]
    fn version_counter() {
        let counter = VersionCounter::new();
        assert_eq!(counter.current().as_u64(), 0);
        assert_eq!(counter.next().as_u64(), 1);
        assert_eq!(counter.next().as_u64(), 2);
        assert_eq!(counter.current().as_u64(), 2);
    }
}
