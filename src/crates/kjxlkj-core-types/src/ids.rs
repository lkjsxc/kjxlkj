//! Stable identity types for buffers, windows, and tabs.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Unique identifier for a window/viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

/// Unique identifier for a tab page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

/// Monotonic buffer version for snapshot tagging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

static NEXT_BUFFER_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_WINDOW_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_TAB_ID: AtomicU64 = AtomicU64::new(1);

impl BufferId {
    pub fn next() -> Self {
        Self(NEXT_BUFFER_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl WindowId {
    pub fn next() -> Self {
        Self(NEXT_WINDOW_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl TabId {
    pub fn next() -> Self {
        Self(NEXT_TAB_ID.fetch_add(1, Ordering::Relaxed))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_id_uniqueness() {
        let a = BufferId::next();
        let b = BufferId::next();
        assert_ne!(a, b);
    }

    #[test]
    fn buffer_version_monotonic() {
        let v = BufferVersion(1);
        assert_eq!(v.next(), BufferVersion(2));
        assert!(v < v.next());
    }

    #[test]
    fn window_id_uniqueness() {
        let a = WindowId::next();
        let b = WindowId::next();
        assert_ne!(a, b);
    }
}
