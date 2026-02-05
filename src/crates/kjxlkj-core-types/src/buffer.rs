//! Buffer identification and metadata types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Stable buffer identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Buffer name for display purposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

/// Monotonic buffer version for snapshot tagging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Create a new version.
    pub fn new(version: u64) -> Self {
        Self(version)
    }

    /// Increment the version.
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    /// Get the next version.
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

/// Buffer metadata for display and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferMeta {
    /// Unique buffer identifier.
    pub id: BufferId,
    /// Display name.
    pub name: BufferName,
    /// Optional file path.
    pub path: Option<PathBuf>,
    /// Whether the buffer has unsaved changes.
    pub modified: bool,
    /// Line count.
    pub line_count: usize,
    /// Current version.
    pub version: BufferVersion,
}

impl BufferMeta {
    /// Create new buffer metadata.
    pub fn new(id: BufferId, name: BufferName) -> Self {
        Self {
            id,
            name,
            path: None,
            modified: false,
            line_count: 1,
            version: BufferVersion::new(0),
        }
    }
}
