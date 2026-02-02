//! Normal mode state.

use kjxlkj_core_edit::{Operator, OperatorKind};
use serde::{Deserialize, Serialize};

/// Normal mode state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NormalState {
    /// Pending count.
    pub count: Option<usize>,
    /// Pending operator.
    pub pending_operator: Option<Operator>,
    /// Pending register.
    pub pending_register: Option<char>,
    /// Last find character.
    pub last_find_char: Option<(char, bool, bool)>,
}

impl NormalState {
    /// Creates a new normal mode state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets the state.
    pub fn reset(&mut self) {
        self.count = None;
        self.pending_operator = None;
        self.pending_register = None;
    }

    /// Accumulates a digit.
    pub fn accumulate_count(&mut self, digit: u8) {
        let current = self.count.unwrap_or(0);
        self.count = Some(current * 10 + digit as usize);
    }

    /// Returns and clears the count.
    pub fn take_count(&mut self) -> usize {
        self.count.take().unwrap_or(1)
    }

    /// Sets a pending operator.
    pub fn set_operator(&mut self, kind: OperatorKind) {
        self.pending_operator = Some(Operator::new(kind));
    }

    /// Takes the pending operator.
    pub fn take_operator(&mut self) -> Option<Operator> {
        self.pending_operator.take()
    }

    /// Returns true if an operator is pending.
    pub fn has_operator(&self) -> bool {
        self.pending_operator.is_some()
    }

    /// Sets the last find character.
    pub fn set_find(&mut self, c: char, forward: bool, till: bool) {
        self.last_find_char = Some((c, forward, till));
    }
}
