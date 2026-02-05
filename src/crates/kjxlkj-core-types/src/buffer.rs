//! Buffer identification types.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Unique identifier for a buffer.
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

impl fmt::Display for BufferId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer({})", self.0)
    }
}

/// Display name for a buffer.
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

impl fmt::Display for BufferName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
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
    fn buffer_name_display() {
        let name = BufferName::new("test.rs");
        assert_eq!(name.as_str(), "test.rs");
        assert_eq!(format!("{}", name), "test.rs");
    }

    #[test]
    fn buffer_name_unnamed() {
        let name = BufferName::unnamed();
        assert_eq!(name.as_str(), "[No Name]");
    }
}
