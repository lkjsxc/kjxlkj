//! Repeat command (.) support.
//!
//! Tracks the last repeatable command for the . command.

use serde::{Deserialize, Serialize};

/// Kind of repeatable action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepeatKind {
    /// Insert action (i, a, o, etc.).
    Insert {
        /// Text inserted.
        text: String,
    },
    /// Change action (c + motion).
    Change {
        /// Motion command.
        motion: String,
        /// New text.
        text: String,
    },
    /// Delete action (d + motion).
    Delete {
        /// Motion command.
        motion: String,
    },
    /// Replace action (r).
    Replace {
        /// Replacement character.
        char: char,
    },
    /// Substitute action (s).
    Substitute {
        /// Replacement text.
        text: String,
    },
    /// Put action (p, P).
    Put {
        /// After cursor.
        after: bool,
    },
    /// Indent action (>, <).
    Indent {
        /// Motion command.
        motion: String,
        /// Direction (true = right).
        right: bool,
    },
    /// Join lines (J).
    JoinLines,
    /// Toggle case (~).
    ToggleCase,
}

/// Last repeatable action.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepeatState {
    /// Last action.
    action: Option<RepeatKind>,
    /// Count used.
    count: usize,
    /// Register used.
    register: Option<char>,
}

impl RepeatState {
    /// Creates a new repeat state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an action.
    pub fn record(&mut self, action: RepeatKind, count: usize, register: Option<char>) {
        self.action = Some(action);
        self.count = count;
        self.register = register;
    }

    /// Returns the last action.
    pub fn last_action(&self) -> Option<&RepeatKind> {
        self.action.as_ref()
    }

    /// Returns the count.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the register.
    pub fn register(&self) -> Option<char> {
        self.register
    }

    /// Returns whether there's a repeatable action.
    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    /// Clears the repeat state.
    pub fn clear(&mut self) {
        self.action = None;
        self.count = 0;
        self.register = None;
    }

    /// Creates a repeat of the last action with optional new count.
    pub fn repeat(&self, new_count: Option<usize>) -> Option<(RepeatKind, usize, Option<char>)> {
        self.action.as_ref().map(|action| {
            (
                action.clone(),
                new_count.unwrap_or(self.count),
                self.register,
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_state_new() {
        let state = RepeatState::new();
        assert!(!state.has_action());
    }

    #[test]
    fn test_record_insert() {
        let mut state = RepeatState::new();
        state.record(RepeatKind::Insert { text: "hello".into() }, 1, None);
        assert!(state.has_action());
    }

    #[test]
    fn test_record_delete() {
        let mut state = RepeatState::new();
        state.record(RepeatKind::Delete { motion: "w".into() }, 2, Some('a'));
        assert_eq!(state.count(), 2);
        assert_eq!(state.register(), Some('a'));
    }

    #[test]
    fn test_repeat() {
        let mut state = RepeatState::new();
        state.record(RepeatKind::Delete { motion: "w".into() }, 2, None);

        let (action, count, _) = state.repeat(Some(3)).unwrap();
        assert_eq!(count, 3);
        assert!(matches!(action, RepeatKind::Delete { .. }));
    }

    #[test]
    fn test_repeat_without_count() {
        let mut state = RepeatState::new();
        state.record(RepeatKind::JoinLines, 5, None);

        let (_, count, _) = state.repeat(None).unwrap();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_clear() {
        let mut state = RepeatState::new();
        state.record(RepeatKind::ToggleCase, 1, None);
        state.clear();
        assert!(!state.has_action());
    }
}
