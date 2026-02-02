//! File system events.

use std::path::PathBuf;

/// Kind of filesystem event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEventKind {
    /// File created.
    Create,
    /// File modified.
    Modify,
    /// File deleted.
    Delete,
    /// File renamed.
    Rename,
    /// Access changed.
    Access,
    /// Other event.
    Other,
}

/// A filesystem event.
#[derive(Debug, Clone)]
pub struct FsEvent {
    /// Event kind.
    pub kind: FsEventKind,
    /// Affected path.
    pub path: PathBuf,
    /// New path (for rename events).
    pub new_path: Option<PathBuf>,
}

impl FsEvent {
    /// Creates a new filesystem event.
    pub fn new(kind: FsEventKind, path: PathBuf) -> Self {
        Self {
            kind,
            path,
            new_path: None,
        }
    }

    /// Creates a rename event.
    pub fn rename(from: PathBuf, to: PathBuf) -> Self {
        Self {
            kind: FsEventKind::Rename,
            path: from,
            new_path: Some(to),
        }
    }
}
