//! Spell checking interface.
//!
//! Provides spell checking integration for the editor.

pub use crate::spell_checker::{
    levenshtein, SimpleSpellChecker, SpellChecker, SpellingError,
};

/// Spell check state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct SpellState {
    /// Errors found in the buffer.
    errors: Vec<SpellingError>,
    /// Whether spell checking is enabled.
    enabled: bool,
}

impl SpellState {
    /// Creates a new spell state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the errors.
    pub fn set_errors(&mut self, errors: Vec<SpellingError>) {
        self.errors = errors;
    }

    /// Returns the errors.
    pub fn errors(&self) -> &[SpellingError] {
        &self.errors
    }

    /// Enables spell checking.
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disables spell checking.
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Returns whether spell checking is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Clears all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_state() {
        let mut state = SpellState::new();
        assert!(!state.is_enabled());

        state.enable();
        assert!(state.is_enabled());

        state.disable();
        assert!(!state.is_enabled());
    }

    #[test]
    fn test_spell_state_errors() {
        let mut state = SpellState::new();
        state.set_errors(vec![SpellingError::new(0..3, "teh")]);

        assert_eq!(state.errors().len(), 1);
        state.clear();
        assert!(state.errors().is_empty());
    }
}
