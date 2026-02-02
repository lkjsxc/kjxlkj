//! Buffer metadata types for kjxlkj editor.

use crate::ids::BufferId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Buffer name or path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BufferName {
    /// A file path.
    File(PathBuf),
    /// A scratch buffer with a display name.
    Scratch(String),
    /// An unnamed buffer.
    #[default]
    Unnamed,
}

impl BufferName {
    /// Returns the display name for this buffer.
    pub fn display_name(&self) -> String {
        match self {
            Self::File(path) => path
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or_else(|| "[no name]".to_string()),
            Self::Scratch(name) => name.clone(),
            Self::Unnamed => "[No Name]".to_string(),
        }
    }

    /// Returns the full path if this is a file buffer.
    pub fn path(&self) -> Option<&PathBuf> {
        match self {
            Self::File(path) => Some(path),
            _ => None,
        }
    }

    /// Returns true if this buffer is file-backed.
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }
}

impl std::fmt::Display for BufferName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

bitflags::bitflags! {
    /// Buffer state flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
    pub struct BufferFlags: u32 {
        /// The buffer has been modified since last save.
        const MODIFIED = 1 << 0;
        /// The buffer is read-only.
        const READONLY = 1 << 1;
        /// The buffer is a scratch buffer (not file-backed).
        const SCRATCH = 1 << 2;
        /// The buffer is hidden from buffer lists.
        const HIDDEN = 1 << 3;
        /// The buffer is listed (shown in :buffers).
        const LISTED = 1 << 4;
        /// The buffer has unsaved changes.
        const UNSAVED = 1 << 5;
    }
}

/// Light metadata about a buffer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferInfo {
    /// Unique buffer identifier.
    pub id: BufferId,
    /// Buffer name or path.
    pub name: BufferName,
    /// Buffer flags.
    pub flags: BufferFlags,
    /// Line count (cached for performance).
    pub line_count: usize,
}

impl BufferInfo {
    /// Creates a new buffer info.
    pub fn new(id: BufferId, name: BufferName) -> Self {
        Self {
            id,
            name,
            flags: BufferFlags::LISTED,
            line_count: 1,
        }
    }

    /// Returns true if the buffer is modified.
    pub fn is_modified(&self) -> bool {
        self.flags.contains(BufferFlags::MODIFIED)
    }

    /// Returns true if the buffer is read-only.
    pub fn is_readonly(&self) -> bool {
        self.flags.contains(BufferFlags::READONLY)
    }
}
