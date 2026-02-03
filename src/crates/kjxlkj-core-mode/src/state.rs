//! Mode state tracking.

use kjxlkj_core_types::Mode;

/// Represents a pending operator awaiting a motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PendingOperator {
    Delete,
    Yank,
    Change,
    Indent,
    Outdent,
    Format,
    Lowercase,
    Uppercase,
    ToggleCase,
}

impl PendingOperator {
    /// Returns the operator character.
    pub fn char(&self) -> char {
        match self {
            PendingOperator::Delete => 'd',
            PendingOperator::Yank => 'y',
            PendingOperator::Change => 'c',
            PendingOperator::Indent => '>',
            PendingOperator::Outdent => '<',
            PendingOperator::Format => '=',
            PendingOperator::Lowercase => 'u',
            PendingOperator::Uppercase => 'U',
            PendingOperator::ToggleCase => '~',
        }
    }
}

/// Tracks the current mode and pending input.
#[derive(Debug, Default)]
pub struct ModeState {
    mode: Mode,
    pending_keys: Vec<char>,
    count: Option<u32>,
    pending_operator: Option<PendingOperator>,
}

impl ModeState {
    /// Creates a new mode state in Normal mode.
    pub fn new() -> Self {
        Self {
            mode: Mode::Normal,
            pending_keys: Vec::new(),
            count: None,
            pending_operator: None,
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
        self.pending_operator = None;
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

    /// Returns the count without clearing it.
    pub fn count(&self) -> Option<u32> {
        self.count
    }

    /// Sets a pending operator.
    pub fn set_pending_operator(&mut self, op: PendingOperator) {
        self.pending_operator = Some(op);
    }

    /// Returns the pending operator.
    pub fn pending_operator(&self) -> Option<PendingOperator> {
        self.pending_operator
    }

    /// Takes and clears the pending operator.
    pub fn take_pending_operator(&mut self) -> Option<PendingOperator> {
        self.pending_operator.take()
    }

    /// Returns true if in operator-pending state.
    pub fn is_operator_pending(&self) -> bool {
        self.pending_operator.is_some()
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

    #[test]
    fn operator_pending_state() {
        let mut state = ModeState::new();
        state.set_pending_operator(PendingOperator::Delete);
        assert!(state.is_operator_pending());
        assert_eq!(state.pending_operator(), Some(PendingOperator::Delete));
        
        let op = state.take_pending_operator();
        assert_eq!(op, Some(PendingOperator::Delete));
        assert!(!state.is_operator_pending());
    }
}
