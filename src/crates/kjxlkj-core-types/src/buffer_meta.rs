//! Buffer metadata.
//!
//! Extended metadata for buffers beyond basic content.

use std::path::PathBuf;
use std::time::SystemTime;

/// Buffer type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    /// Normal file buffer.
    Normal,
    /// Help buffer.
    Help,
    /// Quickfix buffer.
    Quickfix,
    /// Terminal buffer.
    Terminal,
    /// Prompt buffer.
    Prompt,
    /// Popup buffer.
    Popup,
    /// Scratch buffer.
    Scratch,
}

impl Default for BufferType {
    fn default() -> Self {
        Self::Normal
    }
}

/// Buffer listing flags.
#[derive(Debug, Clone, Copy, Default)]
pub struct BufferFlags {
    /// Listed in :ls.
    pub listed: bool,
    /// Scratch buffer.
    pub scratch: bool,
    /// Modified.
    pub modified: bool,
    /// Read-only.
    pub readonly: bool,
    /// New buffer (not written).
    pub new_file: bool,
}

impl BufferFlags {
    /// Creates default flags for a normal buffer.
    pub fn normal() -> Self {
        Self {
            listed: true,
            ..Default::default()
        }
    }

    /// Creates flags for a scratch buffer.
    pub fn scratch() -> Self {
        Self {
            listed: false,
            scratch: true,
            ..Default::default()
        }
    }
}

/// Buffer metadata.
#[derive(Debug, Clone)]
pub struct BufferMetadata {
    /// Buffer number.
    pub number: usize,
    /// File path (if any).
    pub path: Option<PathBuf>,
    /// Buffer name (displayed).
    pub name: String,
    /// Buffer type.
    pub buffer_type: BufferType,
    /// Flags.
    pub flags: BufferFlags,
    /// Last modified time.
    pub mtime: Option<SystemTime>,
    /// File size in bytes.
    pub size: Option<u64>,
    /// Change tick counter.
    pub changedtick: u64,
}

impl BufferMetadata {
    /// Creates new buffer metadata.
    pub fn new(number: usize) -> Self {
        Self {
            number,
            path: None,
            name: format!("[No Name] #{}", number),
            buffer_type: BufferType::Normal,
            flags: BufferFlags::normal(),
            mtime: None,
            size: None,
            changedtick: 0,
        }
    }

    /// Creates metadata for a file.
    pub fn for_file(number: usize, path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        Self {
            number,
            path: Some(path),
            name,
            buffer_type: BufferType::Normal,
            flags: BufferFlags::normal(),
            mtime: None,
            size: None,
            changedtick: 0,
        }
    }

    /// Creates a scratch buffer.
    pub fn scratch(number: usize, name: &str) -> Self {
        Self {
            number,
            path: None,
            name: name.to_string(),
            buffer_type: BufferType::Scratch,
            flags: BufferFlags::scratch(),
            mtime: None,
            size: None,
            changedtick: 0,
        }
    }

    /// Increments the changedtick.
    pub fn tick(&mut self) {
        self.changedtick += 1;
    }

    /// Returns whether this buffer has unsaved changes.
    pub fn is_modified(&self) -> bool {
        self.flags.modified
    }

    /// Returns display name for status line.
    pub fn display_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_type_default() {
        assert_eq!(BufferType::default(), BufferType::Normal);
    }

    #[test]
    fn test_buffer_flags_normal() {
        let flags = BufferFlags::normal();
        assert!(flags.listed);
        assert!(!flags.scratch);
    }

    #[test]
    fn test_buffer_flags_scratch() {
        let flags = BufferFlags::scratch();
        assert!(!flags.listed);
        assert!(flags.scratch);
    }

    #[test]
    fn test_buffer_metadata_new() {
        let meta = BufferMetadata::new(1);
        assert_eq!(meta.number, 1);
        assert!(meta.path.is_none());
    }

    #[test]
    fn test_buffer_metadata_for_file() {
        let meta = BufferMetadata::for_file(1, PathBuf::from("/tmp/test.rs"));
        assert_eq!(meta.name, "test.rs");
        assert!(meta.path.is_some());
    }

    #[test]
    fn test_buffer_metadata_tick() {
        let mut meta = BufferMetadata::new(1);
        assert_eq!(meta.changedtick, 0);
        meta.tick();
        assert_eq!(meta.changedtick, 1);
    }
}
