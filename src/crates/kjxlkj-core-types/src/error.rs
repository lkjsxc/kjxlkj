//! Error types for kjxlkj editor.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Editor error.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// Buffer not found.
    BufferNotFound(u64),
    /// Window not found.
    WindowNotFound(u64),
    /// File not found.
    FileNotFound(PathBuf),
    /// Permission denied.
    PermissionDenied(PathBuf),
    /// Invalid range.
    InvalidRange { start: usize, end: usize },
    /// Invalid position.
    InvalidPosition { line: usize, col: usize },
    /// Invalid command.
    InvalidCommand(String),
    /// Unknown command.
    UnknownCommand(String),
    /// Invalid argument.
    InvalidArgument(String),
    /// Readonly buffer.
    ReadonlyBuffer,
    /// Modified buffer.
    ModifiedBuffer,
    /// Operation cancelled.
    Cancelled,
    /// Regex error.
    Regex(String),
    /// Internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::BufferNotFound(id) => write!(f, "Buffer {} not found", id),
            Self::WindowNotFound(id) => write!(f, "Window {} not found", id),
            Self::FileNotFound(path) => write!(f, "File not found: {}", path.display()),
            Self::PermissionDenied(path) => write!(f, "Permission denied: {}", path.display()),
            Self::InvalidRange { start, end } => {
                write!(f, "Invalid range: {}..{}", start, end)
            }
            Self::InvalidPosition { line, col } => {
                write!(f, "Invalid position: {}:{}", line, col)
            }
            Self::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            Self::UnknownCommand(cmd) => write!(f, "Unknown command: {}", cmd),
            Self::InvalidArgument(arg) => write!(f, "Invalid argument: {}", arg),
            Self::ReadonlyBuffer => write!(f, "Buffer is readonly"),
            Self::ModifiedBuffer => write!(f, "Buffer has unsaved changes"),
            Self::Cancelled => write!(f, "Operation cancelled"),
            Self::Regex(e) => write!(f, "Regex error: {}", e),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// Result type for editor operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Error severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// Error (operation failed).
    Error,
    /// Warning (operation succeeded with issues).
    Warning,
    /// Info message.
    Info,
}

/// A message to display to the user.
#[derive(Debug, Clone)]
pub struct Message {
    /// Message text.
    pub text: String,
    /// Severity level.
    pub severity: Severity,
}

impl Message {
    /// Creates an error message.
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Error,
        }
    }

    /// Creates a warning message.
    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Warning,
        }
    }

    /// Creates an info message.
    pub fn info(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            severity: Severity::Info,
        }
    }
}

impl From<&Error> for Message {
    fn from(e: &Error) -> Self {
        Self::error(e.to_string())
    }
}

impl From<Error> for Message {
    fn from(e: Error) -> Self {
        Self::error(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let e = Error::BufferNotFound(42);
        assert!(e.to_string().contains("42"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "test");
        let e: Error = io_err.into();
        assert!(matches!(e, Error::Io(_)));
    }

    #[test]
    fn test_message_error() {
        let msg = Message::error("Test error");
        assert_eq!(msg.severity, Severity::Error);
    }

    #[test]
    fn test_message_from_error() {
        let e = Error::ReadonlyBuffer;
        let msg: Message = e.into();
        assert!(msg.text.contains("readonly"));
    }

    #[test]
    fn test_message_warning() {
        let msg = Message::warning("Test warning");
        assert_eq!(msg.severity, Severity::Warning);
    }

    #[test]
    fn test_message_info() {
        let msg = Message::info("Test info");
        assert_eq!(msg.severity, Severity::Info);
    }
}
