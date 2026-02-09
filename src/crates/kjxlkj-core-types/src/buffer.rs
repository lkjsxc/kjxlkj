use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Stable unique buffer identifier (monotonic counter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Monotonically increasing edit counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferVersion(pub u64);

impl BufferVersion {
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

/// Display name for a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferName {
    /// File-backed buffer with filename.
    File(String),
    /// Scratch buffer with no file.
    Scratch,
}

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(name) => write!(f, "{name}"),
            Self::Scratch => write!(f, "[No Name]"),
        }
    }
}

impl BufferName {
    pub fn from_path(path: &std::path::Path) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.to_string_lossy().into_owned());
        Self::File(name)
    }
}

/// Character encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Encoding {
    #[default]
    Utf8,
}

/// Line ending style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineEnding {
    #[default]
    Lf,
    CrLf,
}

/// File info for path-backed buffers.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileInfo {
    pub path: PathBuf,
    pub encoding: Encoding,
    pub line_ending: LineEnding,
}
