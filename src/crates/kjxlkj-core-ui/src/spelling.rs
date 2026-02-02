//! Spelling support for kjxlkj editor.
//!
//! Provides spell checking and correction suggestions.

use std::collections::HashSet;
use std::path::PathBuf;

/// Spell check result for a word.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpellResult {
    /// Word is correct.
    Correct,
    /// Word is misspelled.
    Misspelled,
    /// Word is rare but correct.
    Rare,
    /// Word is a region-specific spelling.
    Regional,
    /// Word matches a compound pattern.
    Compound,
}

/// A misspelled word in a buffer.
#[derive(Debug, Clone)]
pub struct SpellError {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column start (0-indexed, byte).
    pub col_start: usize,
    /// Column end (exclusive).
    pub col_end: usize,
    /// The misspelled word.
    pub word: String,
    /// Suggestions for correction.
    pub suggestions: Vec<String>,
}

impl SpellError {
    /// Creates a new spell error.
    pub fn new(line: usize, col_start: usize, col_end: usize, word: String) -> Self {
        Self {
            line,
            col_start,
            col_end,
            word,
            suggestions: Vec::new(),
        }
    }

    /// Adds suggestions.
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

/// Spell checking configuration.
#[derive(Debug, Clone)]
pub struct SpellConfig {
    /// Whether spell checking is enabled.
    pub enabled: bool,
    /// Language code (e.g., "en_US").
    pub language: String,
    /// Additional dictionary files.
    pub dict_files: Vec<PathBuf>,
    /// Word patterns to ignore.
    pub ignore_patterns: Vec<String>,
    /// Maximum suggestions per word.
    pub max_suggestions: usize,
}

impl Default for SpellConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            language: "en_US".to_string(),
            dict_files: Vec::new(),
            ignore_patterns: Vec::new(),
            max_suggestions: 10,
        }
    }
}

/// User word list for custom words.
#[derive(Debug, Default)]
pub struct UserWordList {
    /// Words marked as correct.
    good: HashSet<String>,
    /// Words marked as wrong.
    bad: HashSet<String>,
}

impl UserWordList {
    /// Creates a new user word list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a word as correct.
    pub fn add_good(&mut self, word: String) {
        self.bad.remove(&word);
        self.good.insert(word);
    }

    /// Adds a word as wrong.
    pub fn add_bad(&mut self, word: String) {
        self.good.remove(&word);
        self.bad.insert(word);
    }

    /// Checks if word is marked good.
    pub fn is_good(&self, word: &str) -> bool {
        self.good.contains(word)
    }

    /// Checks if word is marked bad.
    pub fn is_bad(&self, word: &str) -> bool {
        self.bad.contains(word)
    }

    /// Gets good word count.
    pub fn good_count(&self) -> usize {
        self.good.len()
    }

    /// Gets bad word count.
    pub fn bad_count(&self) -> usize {
        self.bad.len()
    }
}

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
