//! Spell checking per /docs/spec/features/editing/spell.md.
//!
//! Simple built-in spell checker using word lists.

use std::collections::HashSet;

/// A spelling error.
#[derive(Debug, Clone)]
pub struct SpellError {
    /// Line number (0-indexed).
    pub line: usize,
    /// Column/grapheme offset.
    pub col: usize,
    /// The misspelled word.
    pub word: String,
    /// Suggested corrections.
    pub suggestions: Vec<String>,
}

/// Spell checker state.
#[derive(Debug, Clone)]
pub struct SpellChecker {
    /// Whether spell checking is enabled.
    pub enabled: bool,
    /// Known good words.
    pub dictionary: HashSet<String>,
    /// User-added words.
    pub user_words: HashSet<String>,
    /// Current errors in buffer.
    pub errors: Vec<SpellError>,
    /// Current error index for navigation.
    pub current_error: usize,
    /// Language code.
    pub language: String,
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self {
            enabled: false,
            dictionary: HashSet::new(),
            user_words: HashSet::new(),
            errors: Vec::new(),
            current_error: 0,
            language: "en".to_string(),
        }
    }
}

impl SpellChecker {
    /// Create new spell checker.
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle spell checking.
    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if !self.enabled {
            self.errors.clear();
        }
    }

    /// Add word to user dictionary.
    pub fn add_word(&mut self, word: &str) {
        self.user_words.insert(word.to_lowercase());
        // Remove any errors for this word.
        let lower = word.to_lowercase();
        self.errors.retain(|e| e.word.to_lowercase() != lower);
    }

    /// Check if a word is correctly spelled.
    pub fn is_correct(&self, word: &str) -> bool {
        let lower = word.to_lowercase();
        self.dictionary.contains(&lower)
            || self.user_words.contains(&lower)
            || word.len() <= 1
            || word.chars().all(|c| c.is_ascii_digit())
    }

    /// Check a line of text for errors.
    pub fn check_line(&self, line_idx: usize, text: &str) -> Vec<SpellError> {
        if !self.enabled {
            return Vec::new();
        }
        let mut errors = Vec::new();
        let mut word_start = None;
        for (i, c) in text.char_indices() {
            if c.is_alphanumeric() || c == '\'' {
                if word_start.is_none() {
                    word_start = Some(i);
                }
            } else if let Some(start) = word_start {
                let word = &text[start..i];
                if !self.is_correct(word) {
                    errors.push(SpellError {
                        line: line_idx,
                        col: start,
                        word: word.to_string(),
                        suggestions: Vec::new(),
                    });
                }
                word_start = None;
            }
        }
        // Check last word.
        if let Some(start) = word_start {
            let word = &text[start..];
            if !self.is_correct(word) {
                errors.push(SpellError {
                    line: line_idx,
                    col: start,
                    word: word.to_string(),
                    suggestions: Vec::new(),
                });
            }
        }
        errors
    }

    /// Navigate to next spelling error.
    pub fn next_error(&mut self) -> Option<&SpellError> {
        if self.errors.is_empty() {
            return None;
        }
        self.current_error = (self.current_error + 1) % self.errors.len();
        Some(&self.errors[self.current_error])
    }

    /// Navigate to previous spelling error.
    pub fn prev_error(&mut self) -> Option<&SpellError> {
        if self.errors.is_empty() {
            return None;
        }
        if self.current_error == 0 {
            self.current_error = self.errors.len() - 1;
        } else {
            self.current_error -= 1;
        }
        Some(&self.errors[self.current_error])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spell_toggle() {
        let mut sc = SpellChecker::new();
        assert!(!sc.enabled);
        sc.toggle();
        assert!(sc.enabled);
    }

    #[test]
    fn spell_check_line() {
        let mut sc = SpellChecker::new();
        sc.enabled = true;
        sc.dictionary.insert("hello".into());
        sc.dictionary.insert("world".into());
        let errs = sc.check_line(0, "hello wrold world");
        assert_eq!(errs.len(), 1);
        assert_eq!(errs[0].word, "wrold");
    }

    #[test]
    fn spell_user_word() {
        let mut sc = SpellChecker::new();
        sc.enabled = true;
        sc.add_word("kjxlkj");
        assert!(sc.is_correct("kjxlkj"));
    }
}
