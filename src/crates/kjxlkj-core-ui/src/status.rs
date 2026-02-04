//! Status line model.

use kjxlkj_core_types::Mode;

/// Status line content.
#[derive(Debug, Clone, Default)]
pub struct StatusLine {
    /// Current mode display.
    pub mode: Mode,
    /// File path or buffer name.
    pub file_name: Option<String>,
    /// Whether buffer is modified.
    pub modified: bool,
    /// Cursor line (1-based for display).
    pub line: usize,
    /// Cursor column (1-based for display).
    pub col: usize,
    /// Total lines in buffer.
    pub total_lines: usize,
    /// Status message (temporary).
    pub message: Option<String>,
    /// Whether message is an error.
    pub is_error: bool,
}

impl StatusLine {
    /// Create a new status line.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a status message.
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
        self.is_error = false;
    }

    /// Set an error message.
    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
        self.is_error = true;
    }

    /// Clear the message.
    pub fn clear_message(&mut self) {
        self.message = None;
        self.is_error = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_message() {
        let mut status = StatusLine::new();
        status.set_message("Hello");
        assert_eq!(status.message, Some("Hello".to_string()));
        assert!(!status.is_error);
    }

    #[test]
    fn test_set_error() {
        let mut status = StatusLine::new();
        status.set_error("Error!");
        assert_eq!(status.message, Some("Error!".to_string()));
        assert!(status.is_error);
    }
}
