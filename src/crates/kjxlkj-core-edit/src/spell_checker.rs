//! Spell checker types and implementation.

use std::collections::HashSet;
use std::ops::Range;

/// A spelling error.
#[derive(Debug, Clone)]
pub struct SpellingError {
    /// Byte range in the text.
    pub range: Range<usize>,
    /// The misspelled word.
    pub word: String,
    /// Suggested corrections.
    pub suggestions: Vec<String>,
}

impl SpellingError {
    /// Creates a new spelling error.
    pub fn new(range: Range<usize>, word: &str) -> Self {
        Self {
            range,
            word: word.to_string(),
            suggestions: Vec::new(),
        }
    }

    /// Adds suggestions.
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
}

/// Spell checker trait.
pub trait SpellChecker: Send + Sync {
    /// Checks a word for spelling errors.
    fn check_word(&self, word: &str) -> bool;

    /// Gets suggestions for a misspelled word.
    fn suggest(&self, word: &str) -> Vec<String>;

    /// Adds a word to the dictionary.
    fn add_word(&mut self, word: &str);

    /// Checks if a word is in the dictionary.
    fn is_known(&self, word: &str) -> bool;
}

/// A simple in-memory spell checker for testing.
#[derive(Debug, Clone, Default)]
pub struct SimpleSpellChecker {
    dictionary: HashSet<String>,
}

impl SimpleSpellChecker {
    /// Creates a new simple spell checker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a spell checker with initial words.
    pub fn with_words(words: &[&str]) -> Self {
        let mut checker = Self::new();
        for word in words {
            checker.add_word(word);
        }
        checker
    }
}

impl SpellChecker for SimpleSpellChecker {
    fn check_word(&self, word: &str) -> bool {
        self.dictionary.contains(&word.to_lowercase())
    }

    fn suggest(&self, word: &str) -> Vec<String> {
        let word_lower = word.to_lowercase();
        self.dictionary
            .iter()
            .filter(|w| levenshtein(&word_lower, &w.to_lowercase()) <= 2)
            .take(5)
            .cloned()
            .collect()
    }

    fn add_word(&mut self, word: &str) {
        self.dictionary.insert(word.to_lowercase());
    }

    fn is_known(&self, word: &str) -> bool {
        self.dictionary.contains(&word.to_lowercase())
    }
}

/// Simple Levenshtein distance.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let m = a_chars.len();
    let n = b_chars.len();

    if m == 0 { return n; }
    if n == 0 { return m; }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }

    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spelling_error_new() {
        let err = SpellingError::new(0..5, "teh");
        assert_eq!(err.word, "teh");
        assert_eq!(err.range, 0..5);
    }

    #[test]
    fn test_simple_spell_checker() {
        let mut checker = SimpleSpellChecker::new();
        checker.add_word("hello");
        checker.add_word("world");

        assert!(checker.check_word("hello"));
        assert!(!checker.check_word("helo"));
    }

    #[test]
    fn test_spell_checker_suggestions() {
        let checker = SimpleSpellChecker::with_words(&["hello", "help", "held"]);
        let suggestions = checker.suggest("helo");
        assert!(!suggestions.is_empty());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein("hello", "hello"), 0);
        assert_eq!(levenshtein("hello", "helo"), 1);
        assert_eq!(levenshtein("hello", "help"), 2);
    }
}
