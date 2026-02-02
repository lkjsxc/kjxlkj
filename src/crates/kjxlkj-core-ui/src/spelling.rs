//! Spelling support for kjxlkj editor.
//!
//! Provides spell checking and correction suggestions.

pub use crate::spelling_types::{SpellConfig, SpellError, SpellResult, UserWordList};

/// Manages spell checking state.
#[derive(Debug)]
pub struct SpellState {
    /// Configuration.
    config: SpellConfig,
    /// User word list.
    user_words: UserWordList,
    /// Cached errors per buffer.
    errors: Vec<SpellError>,
}

impl SpellState {
    /// Creates a new spell state.
    pub fn new() -> Self {
        Self {
            config: SpellConfig::default(),
            user_words: UserWordList::new(),
            errors: Vec::new(),
        }
    }

    /// Creates with config.
    pub fn with_config(config: SpellConfig) -> Self {
        Self {
            config,
            user_words: UserWordList::new(),
            errors: Vec::new(),
        }
    }

    /// Gets configuration.
    pub fn config(&self) -> &SpellConfig {
        &self.config
    }

    /// Gets user word list.
    pub fn user_words(&self) -> &UserWordList {
        &self.user_words
    }

    /// Gets mutable user word list.
    pub fn user_words_mut(&mut self) -> &mut UserWordList {
        &mut self.user_words
    }

    /// Records spell errors.
    pub fn set_errors(&mut self, errors: Vec<SpellError>) {
        self.errors = errors;
    }

    /// Gets current errors.
    pub fn errors(&self) -> &[SpellError] {
        &self.errors
    }

    /// Clears errors.
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
}

impl Default for SpellState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spell_result() {
        assert_eq!(SpellResult::Correct, SpellResult::Correct);
        assert_ne!(SpellResult::Correct, SpellResult::Misspelled);
    }

    #[test]
    fn test_spell_error() {
        let error = SpellError::new(5, 10, 15, "teh".to_string())
            .with_suggestions(vec!["the".to_string()]);
        assert_eq!(error.line, 5);
        assert_eq!(error.word, "teh");
        assert_eq!(error.suggestions.len(), 1);
    }

    #[test]
    fn test_spell_config_default() {
        let config = SpellConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.language, "en_US");
    }

    #[test]
    fn test_user_word_list() {
        let mut list = UserWordList::new();
        list.add_good("customword".to_string());
        assert!(list.is_good("customword"));
        assert!(!list.is_bad("customword"));
    }

    #[test]
    fn test_user_word_list_toggle() {
        let mut list = UserWordList::new();
        list.add_good("word".to_string());
        assert!(list.is_good("word"));
        list.add_bad("word".to_string());
        assert!(!list.is_good("word"));
        assert!(list.is_bad("word"));
    }

    #[test]
    fn test_spell_state() {
        let mut state = SpellState::new();
        state.user_words_mut().add_good("kjxlkj".to_string());
        assert!(state.user_words().is_good("kjxlkj"));
        assert_eq!(state.errors().len(), 0);
    }
}
