//! Insert mode state.

use serde::{Deserialize, Serialize};

/// Insert mode state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InsertState {
    /// Text inserted in this session.
    pub inserted_text: String,
    /// Whether completion is active.
    pub completion_active: bool,
}

impl InsertState {
    /// Creates a new insert mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the state.
    pub fn reset(&mut self) {
        self.inserted_text.clear();
        self.completion_active = false;
    }

    /// Appends text.
    pub fn append(&mut self, text: &str) {
        self.inserted_text.push_str(text);
    }

    /// Handles backspace.
    pub fn backspace(&mut self) {
        self.inserted_text.pop();
    }

    /// Returns the inserted text.
    pub fn text(&self) -> &str {
        &self.inserted_text
    }
}
