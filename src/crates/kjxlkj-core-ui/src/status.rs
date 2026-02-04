//! Status line model.

use kjxlkj_core_types::Mode;
use serde::{Deserialize, Serialize};

/// A section of the status line.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusSection {
    /// Section content.
    pub content: String,
    /// Whether this section is highlighted.
    pub highlight: bool,
}

impl StatusSection {
    /// Create a new section.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            highlight: false,
        }
    }

    /// Create a highlighted section.
    pub fn highlighted(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            highlight: true,
        }
    }
}

/// Status line content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusLine {
    /// Left-aligned sections.
    pub left: Vec<StatusSection>,
    /// Right-aligned sections.
    pub right: Vec<StatusSection>,
    /// Command line input (when in command mode).
    pub command_line: Option<String>,
    /// Error/info message.
    pub message: Option<String>,
}

impl StatusLine {
    /// Create a new empty status line.
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
            command_line: None,
            message: None,
        }
    }

    /// Build a status line from editor state.
    pub fn from_state(
        mode: Mode,
        filename: &str,
        modified: bool,
        line: usize,
        col: usize,
        total_lines: usize,
    ) -> Self {
        let mut status = Self::new();

        // Mode indicator
        status.left.push(StatusSection::highlighted(format!(" {} ", mode.name())));

        // Filename
        let file_display = if modified {
            format!(" {} [+]", filename)
        } else {
            format!(" {}", filename)
        };
        status.left.push(StatusSection::new(file_display));

        // Position
        status.right.push(StatusSection::new(format!(
            " {}:{} ",
            line + 1,
            col + 1
        )));

        // Percentage
        let pct = if total_lines == 0 {
            "Top".to_string()
        } else if line == 0 {
            "Top".to_string()
        } else if line >= total_lines.saturating_sub(1) {
            "Bot".to_string()
        } else {
            format!("{}%", (line * 100) / total_lines)
        };
        status.right.push(StatusSection::new(format!(" {} ", pct)));

        status
    }

    /// Set the command line content.
    pub fn set_command(&mut self, cmd: &str) {
        self.command_line = Some(format!(":{}", cmd));
    }

    /// Set a message.
    pub fn set_message(&mut self, msg: impl Into<String>) {
        self.message = Some(msg.into());
    }

    /// Clear the message.
    pub fn clear_message(&mut self) {
        self.message = None;
    }
}

impl Default for StatusLine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_state() {
        let status = StatusLine::from_state(Mode::Normal, "test.txt", false, 0, 0, 100);
        assert!(!status.left.is_empty());
        assert!(!status.right.is_empty());
    }

    #[test]
    fn modified_indicator() {
        let status = StatusLine::from_state(Mode::Normal, "test.txt", true, 0, 0, 100);
        let left_text: String = status.left.iter().map(|s| s.content.clone()).collect();
        assert!(left_text.contains("[+]"));
    }
}
