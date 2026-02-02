//! Command mode state.

use serde::{Deserialize, Serialize};

/// Command mode state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandState {
    /// Command line content.
    pub line: String,
    /// Cursor position in the command line.
    pub cursor: usize,
    /// History index.
    pub history_index: Option<usize>,
    /// Command type prefix.
    pub prefix: CommandPrefix,
}

/// Command prefix type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CommandPrefix {
    /// Ex command (:).
    #[default]
    Ex,
    /// Forward search (/).
    SearchForward,
    /// Backward search (?).
    SearchBackward,
}

impl CommandState {
    /// Creates a new command mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the state.
    pub fn reset(&mut self) {
        self.line.clear();
        self.cursor = 0;
        self.history_index = None;
    }

    /// Sets the prefix.
    pub fn with_prefix(mut self, prefix: CommandPrefix) -> Self {
        self.prefix = prefix;
        self
    }

    /// Inserts a character.
    pub fn insert(&mut self, c: char) {
        self.line.insert(self.cursor, c);
        self.cursor += 1;
    }

    /// Deletes the character before cursor.
    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.line.remove(self.cursor);
        }
    }

    /// Moves cursor left.
    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Moves cursor right.
    pub fn move_right(&mut self) {
        if self.cursor < self.line.len() {
            self.cursor += 1;
        }
    }

    /// Returns the command line content.
    pub fn content(&self) -> &str {
        &self.line
    }
}
