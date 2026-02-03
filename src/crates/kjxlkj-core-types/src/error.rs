//! Error types for kjxlkj.

use thiserror::Error;

use crate::{BufferId, WindowId};

/// Core editor errors.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum EditorError {
    /// Buffer not found.
    #[error("buffer not found: {0:?}")]
    BufferNotFound(BufferId),

    /// Window not found.
    #[error("window not found: {0:?}")]
    WindowNotFound(WindowId),

    /// Invalid position (out of bounds).
    #[error("invalid position: line {line}, col {col}")]
    InvalidPosition { line: u32, col: u32 },

    /// Invalid range.
    #[error("invalid range")]
    InvalidRange,

    /// Buffer is read-only.
    #[error("buffer is read-only")]
    ReadOnly,

    /// No file path set for buffer.
    #[error("no file path set for buffer")]
    NoFilePath,

    /// File I/O error.
    #[error("file error: {0}")]
    FileError(String),

    /// Command parse error.
    #[error("command error: {0}")]
    CommandError(String),

    /// Unsupported operation.
    #[error("unsupported operation: {0}")]
    Unsupported(String),
}

/// Result type for editor operations.
pub type EditorResult<T> = Result<T, EditorError>;
