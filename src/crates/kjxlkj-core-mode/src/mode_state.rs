//! Modal state machine tracking current mode, pending operator, counts, etc.

use kjxlkj_core_types::{Mode, Operator};
use serde::{Deserialize, Serialize};

use crate::transitions::validate_transition;

/// Record of a completed change for repeat (`.`) support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeRecord {
    pub action: String,
    pub count: Option<usize>,
    pub register: Option<char>,
    pub text: String,
}

/// The modal state machine.
#[derive(Debug, Clone)]
pub struct ModeState {
    current: Mode,
    previous: Mode,
    pending_operator: Option<Operator>,
    count: Option<usize>,
    register: Option<char>,
    last_insert_text: String,
    last_change: Option<ChangeRecord>,
}

impl ModeState {
    /// Create a new `ModeState` starting in Normal mode.
    pub fn new() -> Self {
        Self {
            current: Mode::Normal,
            previous: Mode::Normal,
            pending_operator: None,
            count: None,
            register: None,
            last_insert_text: String::new(),
            last_change: None,
        }
    }

    /// Current mode.
    pub fn current(&self) -> Mode {
        self.current
    }

    /// Previous mode (before last transition).
    pub fn previous(&self) -> Mode {
        self.previous
    }

    /// Transition to `mode`, validating per spec. Clears pending operator.
    pub fn transition(&mut self, mode: Mode) {
        if let Err(e) = validate_transition(self.current, mode) {
            tracing_or_ignore(e);
            return;
        }
        self.previous = self.current;
        self.current = mode;
        // Invariant: pending operator state clears on mode change.
        if self.previous != mode {
            self.pending_operator = None;
        }
    }

    /// Set the pending operator (e.g. `d` waiting for a motion).
    pub fn set_pending_operator(&mut self, op: Operator) {
        self.pending_operator = Some(op);
    }

    /// Clear and return the pending operator.
    pub fn clear_pending_operator(&mut self) -> Option<Operator> {
        self.pending_operator.take()
    }

    /// Peek at the pending operator.
    pub fn pending_operator(&self) -> Option<&Operator> {
        self.pending_operator.as_ref()
    }

    /// Set the numeric count prefix.
    pub fn set_count(&mut self, n: usize) {
        self.count = Some(n);
    }

    /// Take the count, resetting it to `None`.
    pub fn take_count(&mut self) -> Option<usize> {
        self.count.take()
    }

    /// Set the register for the next operation.
    pub fn set_register(&mut self, r: char) {
        self.register = Some(r);
    }

    /// Take the register, resetting it to `None`.
    pub fn take_register(&mut self) -> Option<char> {
        self.register.take()
    }

    /// Record text entered during insert mode.
    pub fn record_insert_text(&mut self, text: &str) {
        self.last_insert_text = text.to_string();
    }

    /// Get the last text entered during insert mode.
    pub fn last_insert_text(&self) -> &str {
        &self.last_insert_text
    }

    /// Record a completed change for repeat.
    pub fn record_change(&mut self, record: ChangeRecord) {
        self.last_change = Some(record);
    }

    /// Get the last recorded change.
    pub fn last_change(&self) -> Option<&ChangeRecord> {
        self.last_change.as_ref()
    }
}

impl Default for ModeState {
    fn default() -> Self {
        Self::new()
    }
}

/// Silently ignore transition errors (no tracing crate dependency).
fn tracing_or_ignore(_e: crate::transitions::TransitionError) {
    // In production this would log via tracing; here we just ignore.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_in_normal() {
        let ms = ModeState::new();
        assert_eq!(ms.current(), Mode::Normal);
        assert_eq!(ms.previous(), Mode::Normal);
    }

    #[test]
    fn valid_transition() {
        let mut ms = ModeState::new();
        ms.transition(Mode::Insert);
        assert_eq!(ms.current(), Mode::Insert);
        assert_eq!(ms.previous(), Mode::Normal);
    }

    #[test]
    fn invalid_transition_stays() {
        let mut ms = ModeState::new();
        ms.transition(Mode::Insert);
        ms.transition(Mode::Replace); // invalid
        assert_eq!(ms.current(), Mode::Insert);
    }

    #[test]
    fn pending_operator_clears_on_transition() {
        let mut ms = ModeState::new();
        ms.set_pending_operator(Operator::Delete);
        ms.transition(Mode::Insert);
        assert!(ms.pending_operator().is_none());
    }

    #[test]
    fn count_and_register() {
        let mut ms = ModeState::new();
        ms.set_count(5);
        ms.set_register('a');
        assert_eq!(ms.take_count(), Some(5));
        assert_eq!(ms.take_register(), Some('a'));
        assert_eq!(ms.take_count(), None);
        assert_eq!(ms.take_register(), None);
    }

    #[test]
    fn change_record() {
        let mut ms = ModeState::new();
        let rec = ChangeRecord {
            action: "delete".into(),
            count: Some(2),
            register: None,
            text: String::new(),
        };
        ms.record_change(rec.clone());
        assert_eq!(ms.last_change(), Some(&rec));
    }

    #[test]
    fn insert_text_record() {
        let mut ms = ModeState::new();
        ms.record_insert_text("hello");
        assert_eq!(ms.last_insert_text(), "hello");
    }
}
