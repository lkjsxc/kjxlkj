//! Buffer identity types.

use serde::{Deserialize, Serialize};

/// Unique, stable identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(u64);

impl BufferId {
    /// Create a new buffer ID from a raw value.
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

impl Default for BufferName {
    fn default() -> Self {
        Self::unnamed()
    }
}

/// Monotonic version number for buffer content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize, Default)]
pub struct BufferVersion(u64);

impl BufferVersion {
    /// Create a new version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Get the next version.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
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
        let v1 = BufferVersion::new(1);
        let v2 = v1.next();
        assert!(v2 > v1);
    }

    #[test]
    fn buffer_name_default_is_unnamed() {
        let name = BufferName::default();
        assert_eq!(name.as_str(), "[No Name]");
    }
}
