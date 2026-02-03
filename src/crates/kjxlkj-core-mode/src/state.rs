//! Mode state tracking.

use kjxlkj_core_types::Mode;

/// Tracks the current mode and pending input.
#[derive(Debug, Default)]
pub struct ModeState {
    mode: Mode,
    pending_keys: Vec<char>,
    count: Option<u32>,
}

impl ModeState {
    /// Creates a new mode state in Normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            pending_keys: Vec::new(),
            count: None,
        }
    }

    /// Returns the current mode.
    pub fn mode(&self) -> Mode {
        self.mode
    }

    /// Sets the current mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.pending_keys.clear();
        self.count = None;
    }

    /// Returns the pending keys.
    pub fn pending_keys(&self) -> &[char] {
        &self.pending_keys
    }

    /// Adds a pending key.
    pub fn push_key(&mut self, key: char) {
        self.pending_keys.push(key);
    }

    /// Clears pending keys.
    pub fn clear_pending(&mut self) {
        self.pending_keys.clear();
    }

    /// Sets the count prefix.
    pub fn set_count(&mut self, count: u32) {
        self.count = Some(count);
    }

    /// Takes and clears the count.
    pub fn take_count(&mut self) -> Option<u32> {
        self.count.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_mode_is_normal() {
        let state = ModeState::new();
        assert_eq!(state.mode(), Mode::Normal);
    }

    #[test]
    fn mode_transition() {
        let mut state = ModeState::new();
        state.set_mode(Mode::Insert);
        assert_eq!(state.mode(), Mode::Insert);
    }
}
