//! Mode state machine.

use kjxlkj_core_types::Mode;

/// State for modal input handling.
#[derive(Debug, Default)]
pub struct ModeState {
    /// Current mode.
    pub mode: Mode,
    /// Pending operator (for operator-pending state).
    pub pending_operator: Option<char>,
    /// Count prefix.
    pub count: Option<usize>,
    /// Register for next operation.
    pub register: Option<char>,
    /// Last find character and direction.
    pub last_find: Option<(char, bool, bool)>,
    /// Whether we're recording a macro.
    pub recording_macro: Option<char>,
    /// Pending key sequence (for multi-key commands).
    pub pending_keys: Vec<char>,
}

impl ModeState {
    /// Create a new mode state starting in Normal mode.
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset to clean Normal mode state.
    pub fn reset_to_normal(&mut self) {
        self.mode = Mode::Normal;
        self.pending_operator = None;
        self.count = None;
        self.pending_keys.clear();
    }

    /// Set the mode.
    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.pending_operator = None;
        self.pending_keys.clear();
    }

    /// Get effective count (default to 1).
    pub fn effective_count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Accumulate a digit into the count.
    pub fn accumulate_count(&mut self, digit: char) {
        let d = digit.to_digit(10).unwrap_or(0) as usize;
        self.count = Some(self.count.unwrap_or(0) * 10 + d);
    }

    /// Clear count and operator state.
    pub fn clear_pending(&mut self) {
        self.count = None;
        self.pending_operator = None;
        self.pending_keys.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_count() {
        let state = ModeState::new();
        assert_eq!(state.effective_count(), 1);
    }

    #[test]
    fn test_accumulate_count() {
        let mut state = ModeState::new();
        state.accumulate_count('2');
        state.accumulate_count('5');
        assert_eq!(state.count, Some(25));
    }
}
