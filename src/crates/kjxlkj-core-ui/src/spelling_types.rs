//! Spelling types for spell checking.
//!
//! Types for representing spelling errors and configuration.

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
