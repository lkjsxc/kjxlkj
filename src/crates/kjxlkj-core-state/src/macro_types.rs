//! Macro types.
//!
//! Types for macro recording and playback.

use kjxlkj_input::Key;
use serde::{Deserialize, Serialize};

/// A recorded macro.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Macro {
    /// Keys in the macro.
    keys: Vec<Key>,
}

impl Macro {
    /// Creates a new empty macro.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a macro from keys.
    pub fn from_keys(keys: Vec<Key>) -> Self {
        Self { keys }
    }

    /// Returns the keys.
    pub fn keys(&self) -> &[Key] {
        &self.keys
    }

    /// Returns if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns the length.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Adds a key.
    pub fn push(&mut self, key: Key) {
        self.keys.push(key);
    }

    /// Appends another macro.
    pub fn append(&mut self, other: &Macro) {
        self.keys.extend(other.keys.iter().cloned());
    }

    /// Clears the macro.
    pub fn clear(&mut self) {
        self.keys.clear();
    }
}

/// Recording state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordingState {
    /// Not recording.
    Idle,
    /// Recording to a register.
    Recording(char),
}

impl Default for RecordingState {
    fn default() -> Self {
        Self::Idle
    }
}
