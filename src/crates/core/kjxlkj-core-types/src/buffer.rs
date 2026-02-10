//! Buffer identity and metadata types.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Unique buffer identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

impl BufferId {
    /// Create a new buffer ID.
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Buffer version for change tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    /// Create a new version.
    pub fn new(v: u64) -> Self {
        Self(v)
    }

    /// Increment the version.
    pub fn increment(&mut self) {
        self.0 += 1;
    }
}

impl Default for BufferVersion {
    fn default() -> Self {
        Self(0)
    }
}

/// Buffer display name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferName {
    /// Named buffer with a display name.
    Named(String),
    /// Unnamed scratch buffer.
    Scratch,
}

impl Default for BufferName {
    fn default() -> Self {
        Self::Scratch
    }
}

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferName::Named(name) => write!(f, "{}", name),
            BufferName::Scratch => write!(f, "[No Name]"),
        }
    }
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEnding {
    /// Unix line ending (LF).
    #[default]
    Lf,
    /// Windows line ending (CRLF).
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

/// Character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Encoding {
    /// UTF-8 encoding.
    #[default]
    Utf8,
}

/// Buffer metadata.
#[derive(Debug, Clone)]
pub struct BufferMeta {
    /// Buffer ID.
    pub id: BufferId,
    /// Display name.
    pub name: BufferName,
    /// Filesystem path if file-backed.
    pub path: Option<PathBuf>,
    /// Whether buffer has unsaved changes.
    pub modified: bool,
    /// Edit version counter.
    pub version: BufferVersion,
    /// Character encoding.
    pub encoding: Encoding,
    /// Line ending style.
    pub line_ending: LineEnding,
    /// Read-only flag.
    pub readonly: bool,
}

impl BufferMeta {
    /// Create metadata for a new scratch buffer.
    pub fn scratch(id: BufferId) -> Self {
        Self {
            id,
            name: BufferName::Scratch,
            path: None,
            modified: false,
            version: BufferVersion::default(),
            encoding: Encoding::Utf8,
            line_ending: LineEnding::Lf,
            readonly: false,
        }
    }

    /// Create metadata for a file-backed buffer.
    pub fn from_path(id: BufferId, path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| BufferName::Named(n.to_string_lossy().into_owned()))
            .unwrap_or(BufferName::Scratch);

        Self {
            id,
            name,
            path: Some(path),
            modified: false,
            version: BufferVersion::default(),
            encoding: Encoding::Utf8,
            line_ending: LineEnding::Lf,
            readonly: false,
        }
    }
}
