//! Buffer identity and versioning types.

use serde::{Deserialize, Serialize};

/// Unique identifier for a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Get the ID value.
    pub fn value(self) -> u64 {
        self.0
    }
}

/// Display name for a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferName(pub String);

impl BufferName {
    /// Create a new buffer name.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Get the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for BufferName {
    fn default() -> Self {
        Self("[No Name]".to_string())
    }
}

/// Monotonically increasing buffer version for change tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Initial version.
    pub const INITIAL: Self = Self(0);

    /// Create a new version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Get the version value.
    pub fn value(self) -> u64 {
        self.0
    }

    /// Increment the version.
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

impl Default for BufferVersion {
    fn default() -> Self {
        Self::INITIAL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_id() {
        let id = BufferId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn test_buffer_name() {
        let name = BufferName::new("test.rs");
        assert_eq!(name.as_str(), "test.rs");
    }

    #[test]
    fn test_buffer_version_next() {
        let v0 = BufferVersion::INITIAL;
        let v1 = v0.next();
        assert!(v1 > v0);
        assert_eq!(v1.0, 1);
    }

    #[test]
    fn test_buffer_id_value() {
        let id = BufferId::new(123);
        assert_eq!(id.value(), 123);
    }

    #[test]
    fn test_buffer_id_equality() {
        let id1 = BufferId::new(1);
        let id2 = BufferId::new(1);
        let id3 = BufferId::new(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_buffer_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(BufferId::new(1));
        assert!(set.contains(&BufferId::new(1)));
    }

    #[test]
    fn test_buffer_name_default() {
        let name = BufferName::default();
        assert_eq!(name.as_str(), "[No Name]");
    }

    #[test]
    fn test_buffer_name_from_string() {
        let name = BufferName::new(String::from("file.txt"));
        assert_eq!(name.as_str(), "file.txt");
    }

    #[test]
    fn test_buffer_version_value() {
        let v = BufferVersion::new(42);
        assert_eq!(v.value(), 42);
    }

    #[test]
    fn test_buffer_version_default() {
        let v = BufferVersion::default();
        assert_eq!(v, BufferVersion::INITIAL);
    }

    #[test]
    fn test_buffer_version_ordering() {
        let v1 = BufferVersion::new(1);
        let v2 = BufferVersion::new(2);
        assert!(v1 < v2);
    }

    #[test]
    fn test_buffer_version_chain() {
        let v0 = BufferVersion::INITIAL;
        let v1 = v0.next();
        let v2 = v1.next();
        let v3 = v2.next();
        assert_eq!(v3.value(), 3);
    }
}

