//! Command-line state for rendering.

use serde::{Deserialize, Serialize};

/// Current command-line state for rendering.
///
/// When in Command mode, the render task uses this to display
/// the command line at the bottom of the terminal.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CmdlineState {
    /// Whether command line is active.
    pub active: bool,
    /// Prompt character (':', '/', '?').
    pub prompt: char,
    /// Current input text.
    pub content: String,
    /// Cursor position (byte offset within content).
    pub cursor: usize,
    /// Completion candidates (if any).
    pub completions: Vec<String>,
    /// Selected completion index.
    pub completion_index: Option<usize>,
}

impl CmdlineState {
    /// Create an inactive cmdline state.
    pub fn inactive() -> Self {
        Self::default()
    }

    /// Create an active cmdline state.
    pub fn active(prompt: char) -> Self {
        Self {
            active: true,
            prompt,
            content: String::new(),
            cursor: 0,
            completions: Vec::new(),
            completion_index: None,
        }
    }

    /// Full display string including prompt.
    pub fn display_string(&self) -> String {
        format!("{}{}", self.prompt, self.content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inactive_default() {
        let state = CmdlineState::inactive();
        assert!(!state.active);
    }

    #[test]
    fn active_state() {
        let state = CmdlineState::active(':');
        assert!(state.active);
        assert_eq!(state.prompt, ':');
        assert_eq!(state.display_string(), ":");
    }
}
