//! Buffer identity and metadata types.

use serde::{Deserialize, Serialize};

/// Stable unique buffer identifier (monotonic counter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Display name for a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BufferName {
    /// File-backed buffer.
    File(String),
    /// Unnamed scratch buffer.
    Scratch,
}

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferName::File(name) => write!(f, "{name}"),
            BufferName::Scratch => write!(f, "[No Name]"),
        }
    }
}

/// Monotonically increasing edit counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Create a new version starting at 0.
    pub fn new() -> Self {
        Self(0)
    }

    /// Increment and return new version.
    pub fn increment(&mut self) -> Self {
        self.0 += 1;
        *self
    }
}

impl Default for BufferVersion {
    fn default() -> Self {
        Self::new()
    }
}

/// Character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Encoding {
    #[default]
    Utf8,
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LineEnding {
    #[default]
    Lf,
    CrLf,
}
