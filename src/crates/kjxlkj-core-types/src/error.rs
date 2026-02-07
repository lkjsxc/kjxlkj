//! Error types for the editor.

use crate::types::{BufferId, Position, WindowId};

/// All editor error variants.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum EditorError {
    #[error("buffer not found: {0}")]
    BufferNotFound(BufferId),

    #[error("window not found: {0}")]
    WindowNotFound(WindowId),

    #[error("invalid position: {0}")]
    InvalidPosition(Position),

    #[error("invalid command: {0}")]
    InvalidCommand(String),

    #[error("I/O error: {0}")]
    IoError(String),

    #[error("out of range: {0}")]
    OutOfRange(String),

    #[error("buffer is read-only")]
    ReadOnly,

    #[error("invalid regex: {0}")]
    InvalidRegex(String),
}

impl EditorError {
    /// Returns a short label for the error kind.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::BufferNotFound(_) => "buffer-not-found",
            Self::WindowNotFound(_) => "window-not-found",
            Self::InvalidPosition(_) => "invalid-position",
            Self::InvalidCommand(_) => "invalid-command",
            Self::IoError(_) => "io-error",
            Self::OutOfRange(_) => "out-of-range",
            Self::ReadOnly => "read-only",
            Self::InvalidRegex(_) => "invalid-regex",
        }
    }
}

impl From<std::io::Error> for EditorError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e.to_string())
    }
}

/// A convenience `Result` alias.
pub type EditorResult<T> = Result<T, EditorError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        let e = EditorError::BufferNotFound(BufferId(42));
        assert_eq!(e.to_string(), "buffer not found: Buffer(42)");
    }

    #[test]
    fn error_kind_label() {
        assert_eq!(EditorError::ReadOnly.kind(), "read-only");
    }

    #[test]
    fn io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "gone");
        let e: EditorError = io_err.into();
        assert!(matches!(e, EditorError::IoError(_)));
    }
}
