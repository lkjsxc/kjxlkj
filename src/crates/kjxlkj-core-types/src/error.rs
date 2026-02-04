//! Error types for the editor.

use thiserror::Error;

/// Core editor error type.
#[derive(Debug, Error)]
pub enum EditorError {
    /// IO error during file operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Buffer not found.
    #[error("Buffer not found: {0}")]
    BufferNotFound(u64),

    /// Invalid position in buffer.
    #[error("Invalid position: line {line}, col {col}")]
    InvalidPosition { line: usize, col: usize },

    /// Invalid range in buffer.
    #[error("Invalid range")]
    InvalidRange,

    /// Command parse error.
    #[error("Command error: {0}")]
    Command(String),

    /// Generic internal error.
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Result type for editor operations.
pub type EditorResult<T> = Result<T, EditorError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let err = EditorError::BufferNotFound(42);
        assert!(err.to_string().contains("42"));
    }
}
