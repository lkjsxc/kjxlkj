//! Spell checking.
//!
//! Provides spell check highlighting and suggestions.

use std::collections::{HashMap, HashSet};

/// Spell check result type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellResult {
    /// Word is correctly spelled.
    Correct,
    /// Word is unknown/misspelled.
    Misspelled,
    /// Word is rare but valid.
    Rare,
    /// Word has wrong capitalization.
    WrongCase,
}

/// A misspelled word with location.
#[derive(Debug, Clone)]
pub struct SpellError {
    /// Byte offset in line.
    pub start: usize,
    /// Byte length.
    pub len: usize,
    /// The word.
    pub word: String,
    /// Error type.
    pub result: SpellResult,
    /// Suggestions.
    pub suggestions: Vec<String>,
}

impl SpellError {
    /// Create a new spell error.
    pub fn new(start: usize, word: impl Into<String>, result: SpellResult) -> Self {
        let word = word.into();
        let len = word.len();
        Self {
            start,
            len,
            word,
            result,
            suggestions: Vec::new(),
        }
    }

    /// Add suggestions.
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

/// Spell check configuration.
#[derive(Debug, Clone)]
pub struct SpellConfig {
    /// Enable spell checking.
    pub enabled: bool,
    /// Languages (e.g., "en_US").
    pub languages: Vec<String>,
    /// Check code comments only.
    pub code_comments_only: bool,
    /// Minimum word length to check.
    pub min_word_length: usize,
    /// Patterns to ignore (regexes).
    pub ignore_patterns: Vec<String>,
    /// Filetypes to enable for.
    pub enabled_filetypes: Vec<String>,
    /// Filetypes to disable for.
    pub disabled_filetypes: Vec<String>,
}

impl Default for SpellConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            languages: vec!["en_US".to_string()],
            code_comments_only: true,
            min_word_length: 2,
            ignore_patterns: vec![
                // Hex codes
                r"#[0-9a-fA-F]{6}".to_string(),
                // URLs
                r"https?://\S+".to_string(),
                // Emails
                r"\S+@\S+".to_string(),
                // File paths
                r"/[\w/.-]+".to_string(),
            ],
            enabled_filetypes: vec![
                "markdown".to_string(),
                "text".to_string(),
                "gitcommit".to_string(),
            ],
            disabled_filetypes: Vec::new(),
        }
    }
}

impl SpellConfig {
    /// Create config with spell check enabled.
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Check if spell should run for filetype.
    pub fn should_check(&self, filetype: &str) -> bool {
        if !self.enabled {
            return false;
        }

        if self.disabled_filetypes.iter().any(|ft| ft == filetype) {
            return false;
        }

        if self.enabled_filetypes.is_empty() {
            return true;
        }

        self.enabled_filetypes.iter().any(|ft| ft == filetype)
    }
}

/// Word dictionary.
#[derive(Debug, Default)]
pub struct Dictionary {
    /// Known correct words.
    words: HashSet<String>,
    /// Words marked as wrong.
    wrong_words: HashSet<String>,
    /// Custom user additions.
    user_words: HashSet<String>,
    /// Language code.
    language: String,
}

impl Dictionary {
    /// Create a new dictionary.
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            words: HashSet::new(),
            wrong_words: HashSet::new(),
            user_words: HashSet::new(),
            language: language.into(),
        }
    }

    /// Add a word to the dictionary.
    pub fn add_word(&mut self, word: &str) {
        self.words.insert(word.to_lowercase());
    }

    /// Add a user word.
    pub fn add_user_word(&mut self, word: &str) {
        self.user_words.insert(word.to_lowercase());
    }

    /// Mark a word as wrong.
    pub fn mark_wrong(&mut self, word: &str) {
        self.wrong_words.insert(word.to_lowercase());
    }

    /// Remove user word.
    pub fn remove_user_word(&mut self, word: &str) {
        self.user_words.remove(&word.to_lowercase());
    }

    /// Remove wrong mark.
    pub fn unmark_wrong(&mut self, word: &str) {
        self.wrong_words.remove(&word.to_lowercase());
    }

    /// Check if word is known.
    pub fn contains(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.words.contains(&lower) || self.user_words.contains(&lower)
    }

    /// Check if word is marked wrong.
    pub fn is_wrong(&self, word: &str) -> bool {
        self.wrong_words.contains(&word.to_lowercase())
    }

    /// Get language.
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Load basic English words.
    pub fn load_basic_english(&mut self) {
        // Common English words for basic spell checking
        let common_words = [
            "the", "be", "to", "of", "and", "a", "in", "that", "have", "i",
            "it", "for", "not", "on", "with", "he", "as", "you", "do", "at",
            "this", "but", "his", "by", "from", "they", "we", "say", "her", "she",
            "or", "an", "will", "my", "one", "all", "would", "there", "their", "what",
            "so", "up", "out", "if", "about", "who", "get", "which", "go", "me",
            "when", "make", "can", "like", "time", "no", "just", "him", "know", "take",
            "people", "into", "year", "your", "good", "some", "could", "them", "see", "other",
            "than", "then", "now", "look", "only", "come", "its", "over", "think", "also",
            "back", "after", "use", "two", "how", "our", "work", "first", "well", "way",
            "even", "new", "want", "because", "any", "these", "give", "day", "most", "us",
        ];

        for word in common_words {
            self.words.insert(word.to_string());
        }
    }
}

/// Spell checker.
#[derive(Debug)]
pub struct SpellChecker {
    /// Configuration.
    config: SpellConfig,
    /// Dictionaries by language.
    dictionaries: HashMap<String, Dictionary>,
    /// Cache of checked words.
    cache: HashMap<String, SpellResult>,
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SpellChecker {
    /// Create new spell checker.
    pub fn new() -> Self {
        Self::with_config(SpellConfig::default())
    }

    /// Create with config.
    pub fn with_config(config: SpellConfig) -> Self {
        Self {
            config,
            dictionaries: HashMap::new(),
            cache: HashMap::new(),
        }
    }

    /// Get config.
    pub fn config(&self) -> &SpellConfig {
        &self.config
    }

    /// Get mutable config.
    pub fn config_mut(&mut self) -> &mut SpellConfig {
        &mut self.config
    }

    /// Enable spell checking.
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }

    /// Disable spell checking.
    pub fn disable(&mut self) {
        self.config.enabled = false;
    }

    /// Add a dictionary.
    pub fn add_dictionary(&mut self, dict: Dictionary) {
        self.dictionaries.insert(dict.language.clone(), dict);
    }

    /// Get dictionary for language.
    pub fn dictionary(&self, language: &str) -> Option<&Dictionary> {
        self.dictionaries.get(language)
    }

    /// Get mutable dictionary.
    pub fn dictionary_mut(&mut self, language: &str) -> Option<&mut Dictionary> {
        self.dictionaries.get_mut(language)
    }

    /// Check a single word.
    pub fn check_word(&mut self, word: &str) -> SpellResult {
        // Check cache
        if let Some(&result) = self.cache.get(word) {
            return result;
        }

        // Skip short words
        if word.len() < self.config.min_word_length {
            return SpellResult::Correct;
        }

        // Skip non-alphabetic
        if !word.chars().all(|c| c.is_alphabetic()) {
            return SpellResult::Correct;
        }

        // Check dictionaries
        let mut found = false;
        for dict in self.dictionaries.values() {
            if dict.is_wrong(word) {
                let result = SpellResult::Misspelled;
                self.cache.insert(word.to_string(), result);
                return result;
            }
            if dict.contains(word) {
                found = true;
            }
        }

        let result = if found {
            SpellResult::Correct
        } else if !self.dictionaries.is_empty() {
            SpellResult::Misspelled
        } else {
            // No dictionaries loaded, assume correct
            SpellResult::Correct
        };

        self.cache.insert(word.to_string(), result);
        result
    }

    /// Check a line for spelling errors.
    pub fn check_line(&mut self, line: &str) -> Vec<SpellError> {
        if !self.config.enabled {
            return Vec::new();
        }

        let mut errors = Vec::new();
        let mut word_start = None;

        for (i, c) in line.char_indices() {
            if c.is_alphabetic() {
                if word_start.is_none() {
                    word_start = Some(i);
                }
            } else if let Some(start) = word_start {
                let word = &line[start..i];
                if let SpellResult::Misspelled | SpellResult::Rare | SpellResult::WrongCase = self.check_word(word) {
                    let result = self.check_word(word);
                    let suggestions = self.suggest(word, 5);
                    errors.push(SpellError::new(start, word, result).with_suggestions(suggestions));
                }
                word_start = None;
            }
        }

        // Check last word
        if let Some(start) = word_start {
            let word = &line[start..];
            if let SpellResult::Misspelled | SpellResult::Rare | SpellResult::WrongCase = self.check_word(word) {
                let result = self.check_word(word);
                let suggestions = self.suggest(word, 5);
                errors.push(SpellError::new(start, word, result).with_suggestions(suggestions));
            }
        }

        errors
    }

    /// Get spelling suggestions.
    pub fn suggest(&self, word: &str, max: usize) -> Vec<String> {
        let mut suggestions = Vec::new();
        let lower = word.to_lowercase();

        for dict in self.dictionaries.values() {
            for known in &dict.words {
                if edit_distance(&lower, known) <= 2 {
                    suggestions.push(known.clone());
                    if suggestions.len() >= max {
                        break;
                    }
                }
            }
            if suggestions.len() >= max {
                break;
            }
        }

        // Sort by edit distance
        suggestions.sort_by_key(|s| edit_distance(&lower, s));
        suggestions.truncate(max);
        suggestions
    }

    /// Add word to user dictionary.
    pub fn add_word(&mut self, word: &str, language: &str) {
        if let Some(dict) = self.dictionaries.get_mut(language) {
            dict.add_user_word(word);
            self.cache.remove(word);
        }
    }

    /// Mark word as wrong.
    pub fn mark_wrong(&mut self, word: &str, language: &str) {
        if let Some(dict) = self.dictionaries.get_mut(language) {
            dict.mark_wrong(word);
            self.cache.remove(word);
        }
    }

    /// Clear cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

/// Calculate edit distance (Levenshtein).
fn edit_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for (i, item) in dp.iter_mut().enumerate().take(m + 1) {
        item[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(n + 1) {
        *cell = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

/// Spell navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpellNav {
    /// Next misspelled word.
    Next,
    /// Previous misspelled word.
    Prev,
    /// Next bad word (skip rare).
    NextBad,
    /// Previous bad word.
    PrevBad,
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
        let err = SpellError::new(5, "tset", SpellResult::Misspelled);
        assert_eq!(err.start, 5);
        assert_eq!(err.word, "tset");
        assert_eq!(err.len, 4);
    }

    #[test]
    fn test_spell_config_default() {
        let config = SpellConfig::default();
        assert!(!config.enabled);
        assert!(!config.languages.is_empty());
    }

    #[test]
    fn test_spell_config_should_check() {
        let config = SpellConfig::enabled();
        assert!(config.should_check("markdown"));
        assert!(!config.should_check("rust"));
    }

    #[test]
    fn test_dictionary_new() {
        let dict = Dictionary::new("en_US");
        assert_eq!(dict.language(), "en_US");
    }

    #[test]
    fn test_dictionary_add_word() {
        let mut dict = Dictionary::new("en_US");
        dict.add_word("hello");
        assert!(dict.contains("hello"));
        assert!(dict.contains("HELLO")); // Case insensitive
    }

    #[test]
    fn test_dictionary_user_word() {
        let mut dict = Dictionary::new("en_US");
        dict.add_user_word("kjxlkj");
        assert!(dict.contains("kjxlkj"));

        dict.remove_user_word("kjxlkj");
        assert!(!dict.contains("kjxlkj"));
    }

    #[test]
    fn test_dictionary_wrong_word() {
        let mut dict = Dictionary::new("en_US");
        dict.mark_wrong("definately");
        assert!(dict.is_wrong("definately"));

        dict.unmark_wrong("definately");
        assert!(!dict.is_wrong("definately"));
    }

    #[test]
    fn test_spell_checker_new() {
        let checker = SpellChecker::new();
        assert!(!checker.config().enabled);
    }

    #[test]
    fn test_spell_checker_check_word() {
        let mut checker = SpellChecker::new();
        let mut dict = Dictionary::new("en_US");
        dict.load_basic_english();
        checker.add_dictionary(dict);

        assert_eq!(checker.check_word("the"), SpellResult::Correct);
        assert_eq!(checker.check_word("xyzzy"), SpellResult::Misspelled);
    }

    #[test]
    fn test_spell_checker_check_line() {
        let mut checker = SpellChecker::with_config(SpellConfig::enabled());
        let mut dict = Dictionary::new("en_US");
        dict.load_basic_english();
        checker.add_dictionary(dict);

        let errors = checker.check_line("the quikc brown fox");
        assert!(!errors.is_empty());
        assert!(errors.iter().any(|e| e.word == "quikc"));
    }

    #[test]
    fn test_spell_suggest() {
        let mut checker = SpellChecker::new();
        let mut dict = Dictionary::new("en_US");
        dict.add_word("hello");
        dict.add_word("help");
        dict.add_word("helm");
        checker.add_dictionary(dict);

        let suggestions = checker.suggest("helo", 3);
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_edit_distance() {
        assert_eq!(edit_distance("hello", "hello"), 0);
        assert_eq!(edit_distance("hello", "helo"), 1);
        assert_eq!(edit_distance("hello", "world"), 4);
        assert_eq!(edit_distance("", "abc"), 3);
    }

    #[test]
    fn test_spell_add_word() {
        let mut checker = SpellChecker::new();
        checker.add_dictionary(Dictionary::new("en_US"));
        checker.add_word("kjxlkj", "en_US");

        assert_eq!(checker.check_word("kjxlkj"), SpellResult::Correct);
    }
}
