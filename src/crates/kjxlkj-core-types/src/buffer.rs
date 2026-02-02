//! Buffer identity types.

use serde::{Deserialize, Serialize};

/// Stable identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(u64);

impl BufferId {
    /// Creates a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw ID value.
    pub fn raw(&self) -> u64 {
        self.0
    }
}

impl Default for BufferId {
    fn default() -> Self {
        Self(0)
    }
}

/// Display name for a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferName(String);

impl BufferName {
    /// Creates a new buffer name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for BufferName {
    fn default() -> Self {
        Self(String::from("[No Name]"))
    }
}

impl From<String> for BufferName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for BufferName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
