//! Buffer-related types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::BufferId;

/// Buffer metadata (excludes content for efficiency).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferMeta {
    /// Stable buffer identifier.
    pub id: BufferId,
    /// Display name for the buffer.
    pub name: BufferName,
    /// Optional filesystem path (None for scratch buffers).
    pub path: Option<PathBuf>,
    /// Whether the buffer has unsaved modifications.
    pub modified: bool,
    /// Monotonic version counter.
    pub version: BufferVersion,
    /// Line ending style.
    pub line_ending: LineEnding,
}

/// Buffer display name.
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

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Monotonic buffer version for snapshot tagging.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Create a new version.
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    /// Increment the version.
    pub fn increment(&mut self) {
        self.0 = self.0.saturating_add(1);
    }

    /// Get the next version.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum LineEnding {
    /// Unix-style (LF).
    #[default]
    Lf,
    /// Windows-style (CRLF).
    CrLf,
}

impl LineEnding {
    /// Get the line ending as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::CrLf => "\r\n",
        }
    }
}
