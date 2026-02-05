//! Unique identifiers.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a new unique ID.
pub fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

/// Window identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WindowId(pub u64);

impl WindowId {
    /// Create a new window ID.
    pub fn new() -> Self {
        Self(next_id())
    }

    /// Create from raw value.
    pub fn from_raw(id: u64) -> Self {
        Self(id)
    }
}

impl Default for WindowId {
    fn default() -> Self {
        Self::new()
    }
}

/// Tab identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TabId(pub u64);

impl TabId {
    /// Create a new tab ID.
    pub fn new() -> Self {
        Self(next_id())
    }
}

impl Default for TabId {
    fn default() -> Self {
        Self::new()
    }
}

/// Service request identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestId(pub u64);

impl RequestId {
    /// Create a new request ID.
    pub fn new() -> Self {
        Self(next_id())
    }
}

impl Default for RequestId {
    fn default() -> Self {
        Self::new()
    }
}
