//! Buffer identification types.

use serde::{Deserialize, Serialize};

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Buffer name for display.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferName(String);

impl BufferName {
    /// Create a new buffer name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Create an unnamed buffer name.
    pub fn unnamed() -> Self {
        Self("[No Name]".to_string())
    }
}

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
