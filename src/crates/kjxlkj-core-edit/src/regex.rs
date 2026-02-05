//! Regex engine for search and substitution.
//!
//! Implements regex behavior as specified in `/docs/spec/editing/regex/`.
//!
//! This module provides a Vim-compatible regex interface that translates
//! Vim magic patterns to the Rust regex crate's syntax.

use regex::Regex as RustRegex;
use std::collections::HashMap;

/// The magic mode for regex interpretation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MagicMode {
    /// No magic - all special characters need escaping.
    NoMagic,
    /// Magic mode (Vim default) - some chars are special.
    #[default]
    Magic,
    /// Very magic - most chars are special (like Perl/PCRE).
    VeryMagic,
    /// Very nomagic - almost nothing is special.
    VeryNoMagic,
}

/// Options for regex compilation.
#[derive(Debug, Clone, Default)]
pub struct RegexOptions {
    /// Magic mode for pattern interpretation.
    pub magic: MagicMode,
    /// Case sensitivity.
    pub case_insensitive: bool,
    /// Multiline mode (^ and $ match line boundaries).
    pub multiline: bool,
}

/// A compiled regex pattern.
#[derive(Debug, Clone)]
pub struct Pattern {
    /// The original pattern string.
    source: String,
    /// The compiled regex.
    regex: RustRegex,
    /// Compilation options used.
    #[allow(dead_code)]
    options: RegexOptions,
}

impl Pattern {
    /// Compile a pattern with default options.
    pub fn new(pattern: &str) -> Result<Self, RegexError> {
        Self::with_options(pattern, RegexOptions::default())
    }

    /// Compile a pattern with specific options.
    pub fn with_options(pattern: &str, options: RegexOptions) -> Result<Self, RegexError> {
        let translated = translate_pattern(pattern, &options);
        let regex = RustRegex::new(&translated).map_err(|e| RegexError::Compile(e.to_string()))?;

        Ok(Self {
            source: pattern.to_string(),
            regex,
            options,
        })
    }

    /// Get the original source pattern.
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Check if the pattern matches anywhere in the text.
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Find the first match in the text.
    pub fn find(&self, text: &str) -> Option<Match> {
        self.regex.find(text).map(|m| Match {
            start: m.start(),
            end: m.end(),
            text: m.as_str().to_string(),
        })
    }

    /// Find all matches in the text.
    pub fn find_all(&self, text: &str) -> Vec<Match> {
        self.regex
            .find_iter(text)
            .map(|m| Match {
                start: m.start(),
                end: m.end(),
                text: m.as_str().to_string(),
            })
            .collect()
    }

    /// Find the first match starting at or after the given position.
    pub fn find_from(&self, text: &str, start: usize) -> Option<Match> {
        if start >= text.len() {
            return None;
        }
        self.regex.find_at(text, start).map(|m| Match {
            start: m.start(),
            end: m.end(),
            text: m.as_str().to_string(),
        })
    }

    /// Replace the first match with the replacement string.
    pub fn replace(&self, text: &str, replacement: &str) -> String {
        self.regex.replace(text, replacement).to_string()
    }

    /// Replace all matches with the replacement string.
    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        self.regex.replace_all(text, replacement).to_string()
    }

    /// Get captures from the first match.
    pub fn captures(&self, text: &str) -> Option<Captures> {
        self.regex.captures(text).map(|caps| {
            let groups: Vec<Option<Match>> = caps
                .iter()
                .map(|m| {
                    m.map(|m| Match {
                        start: m.start(),
                        end: m.end(),
                        text: m.as_str().to_string(),
                    })
                })
                .collect();
            Captures { groups }
        })
    }
}

/// A regex match result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
    /// Start byte offset.
    pub start: usize,
    /// End byte offset (exclusive).
    pub end: usize,
    /// The matched text.
    pub text: String,
}

impl Match {
    /// Get the length of the match.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the match is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// Captured groups from a match.
#[derive(Debug, Clone)]
pub struct Captures {
    /// All groups (index 0 is full match).
    groups: Vec<Option<Match>>,
}

impl Captures {
    /// Get the full match (group 0).
    pub fn full_match(&self) -> Option<&Match> {
        self.groups.first().and_then(|m| m.as_ref())
    }

    /// Get a specific group by index.
    pub fn get(&self, index: usize) -> Option<&Match> {
        self.groups.get(index).and_then(|m| m.as_ref())
    }

    /// Get the number of groups (including full match).
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Check if there are no groups.
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }
}

/// Regex compilation or execution error.
#[derive(Debug, Clone, PartialEq)]
pub enum RegexError {
    /// Failed to compile the pattern.
    Compile(String),
    /// Invalid pattern syntax.
    Syntax(String),
}

impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compile(msg) => write!(f, "Regex compile error: {}", msg),
            Self::Syntax(msg) => write!(f, "Regex syntax error: {}", msg),
        }
    }
}

impl std::error::Error for RegexError {}

/// A cache for compiled patterns.
#[derive(Debug, Default)]
pub struct PatternCache {
    cache: HashMap<String, Pattern>,
    max_size: usize,
}

impl PatternCache {
    /// Create a new cache with default max size.
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_size: 100,
        }
    }

    /// Create a cache with specific max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get a pattern from cache, or compile and cache it.
    pub fn get(&mut self, pattern: &str) -> Result<&Pattern, RegexError> {
        if !self.cache.contains_key(pattern) {
            // Evict oldest if at capacity
            if self.cache.len() >= self.max_size {
                if let Some(key) = self.cache.keys().next().cloned() {
                    self.cache.remove(&key);
                }
            }
            let compiled = Pattern::new(pattern)?;
            self.cache.insert(pattern.to_string(), compiled);
        }
        Ok(self.cache.get(pattern).unwrap())
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get the number of cached patterns.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

/// Translate a Vim-style pattern to Rust regex syntax.
fn translate_pattern(pattern: &str, options: &RegexOptions) -> String {
    // Build prefix for options
    let mut result = String::new();

    // Add case insensitivity flag
    if options.case_insensitive {
        result.push_str("(?i)");
    }

    // Add multiline flag
    if options.multiline {
        result.push_str("(?m)");
    }

    // Translate based on magic mode
    match options.magic {
        MagicMode::VeryMagic => {
            // Very magic: pass through mostly as-is (like Perl/PCRE)
            result.push_str(pattern);
        }
        MagicMode::VeryNoMagic => {
            // Very nomagic: escape everything except \
            for c in pattern.chars() {
                if c == '\\' {
                    // Handle escape sequences
                    result.push(c);
                } else if is_regex_metachar(c) {
                    result.push('\\');
                    result.push(c);
                } else {
                    result.push(c);
                }
            }
        }
        MagicMode::NoMagic => {
            // Nomagic: only ^ at start and $ at end are special
            translate_nomagic(pattern, &mut result);
        }
        MagicMode::Magic => {
            // Magic (default): translate Vim metacharacters
            translate_magic(pattern, &mut result);
        }
    }

    result
}

/// Check if a character is a regex metacharacter.
fn is_regex_metachar(c: char) -> bool {
    matches!(
        c,
        '.' | '*' | '+' | '?' | '(' | ')' | '[' | ']' | '{' | '}' | '|' | '^' | '$' | '\\'
    )
}

/// Translate a magic-mode pattern.
fn translate_magic(pattern: &str, result: &mut String) {
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '\\' && i + 1 < chars.len() {
            let next = chars[i + 1];
            match next {
                // Vim uses \( \) for groups, translate to ( )
                '(' => {
                    result.push('(');
                    i += 2;
                }
                ')' => {
                    result.push(')');
                    i += 2;
                }
                // Vim uses \| for alternation
                '|' => {
                    result.push('|');
                    i += 2;
                }
                // Vim uses \+ for one or more
                '+' => {
                    result.push('+');
                    i += 2;
                }
                // Vim uses \? for optional
                '?' => {
                    result.push('?');
                    i += 2;
                }
                // Vim uses \= for optional (same as \?)
                '=' => {
                    result.push('?');
                    i += 2;
                }
                // Vim uses \< and \> for word boundaries
                '<' => {
                    result.push_str("\\b");
                    i += 2;
                }
                '>' => {
                    result.push_str("\\b");
                    i += 2;
                }
                // Character classes
                'd' => {
                    result.push_str("\\d");
                    i += 2;
                }
                'D' => {
                    result.push_str("\\D");
                    i += 2;
                }
                'w' => {
                    result.push_str("\\w");
                    i += 2;
                }
                'W' => {
                    result.push_str("\\W");
                    i += 2;
                }
                's' => {
                    result.push_str("\\s");
                    i += 2;
                }
                'S' => {
                    result.push_str("\\S");
                    i += 2;
                }
                // Backreferences
                '1'..='9' => {
                    result.push('\\');
                    result.push(next);
                    i += 2;
                }
                // Escape the backslash itself
                '\\' => {
                    result.push_str("\\\\");
                    i += 2;
                }
                // Literal metachar
                _ => {
                    result.push('\\');
                    result.push(next);
                    i += 2;
                }
            }
        } else if c == '*' {
            // In magic mode, * is special
            result.push_str(".*");
            i += 1;
        } else if c == '.' {
            // In magic mode, . matches any char
            result.push('.');
            i += 1;
        } else if c == '^' && i == 0 {
            // ^ at start is anchor
            result.push('^');
            i += 1;
        } else if c == '$' && i == chars.len() - 1 {
            // $ at end is anchor
            result.push('$');
            i += 1;
        } else if c == '[' {
            // Character class - pass through to closing ]
            result.push('[');
            i += 1;
            while i < chars.len() && chars[i] != ']' {
                result.push(chars[i]);
                i += 1;
            }
            if i < chars.len() {
                result.push(']');
                i += 1;
            }
        } else if is_regex_metachar(c) && c != '^' && c != '$' {
            // Escape other metacharacters
            result.push('\\');
            result.push(c);
            i += 1;
        } else {
            result.push(c);
            i += 1;
        }
    }
}

/// Translate a nomagic-mode pattern.
fn translate_nomagic(pattern: &str, result: &mut String) {
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c == '\\' && i + 1 < chars.len() {
            let next = chars[i + 1];
            match next {
                '.' => {
                    result.push('.');
                    i += 2;
                }
                '*' => {
                    result.push_str(".*");
                    i += 2;
                }
                _ => {
                    result.push('\\');
                    result.push(next);
                    i += 2;
                }
            }
        } else if c == '^' && i == 0 {
            result.push('^');
            i += 1;
        } else if c == '$' && i == chars.len() - 1 {
            result.push('$');
            i += 1;
        } else if is_regex_metachar(c) {
            result.push('\\');
            result.push(c);
            i += 1;
        } else {
            result.push(c);
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_simple_match() {
        let pattern = Pattern::new("hello").unwrap();
        assert!(pattern.is_match("hello world"));
        assert!(!pattern.is_match("goodbye world"));
    }

    #[test]
    fn test_pattern_find() {
        let pattern = Pattern::new("world").unwrap();
        let m = pattern.find("hello world").unwrap();
        assert_eq!(m.start, 6);
        assert_eq!(m.end, 11);
        assert_eq!(m.text, "world");
    }

    #[test]
    fn test_pattern_find_all() {
        let pattern = Pattern::new("o").unwrap();
        let matches = pattern.find_all("hello world");
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_pattern_replace() {
        let pattern = Pattern::new("world").unwrap();
        let result = pattern.replace("hello world", "rust");
        assert_eq!(result, "hello rust");
    }

    #[test]
    fn test_pattern_replace_all() {
        let pattern = Pattern::new("o").unwrap();
        let result = pattern.replace_all("hello world", "0");
        assert_eq!(result, "hell0 w0rld");
    }

    #[test]
    fn test_pattern_dot() {
        let pattern = Pattern::new("h.llo").unwrap();
        assert!(pattern.is_match("hello"));
        assert!(pattern.is_match("hallo"));
    }

    #[test]
    fn test_pattern_star() {
        let pattern = Pattern::new("hel*o").unwrap();
        assert!(pattern.is_match("helo"));
        assert!(pattern.is_match("hello"));
        assert!(pattern.is_match("helllo"));
    }

    #[test]
    fn test_pattern_anchors() {
        let pattern = Pattern::new("^hello").unwrap();
        assert!(pattern.is_match("hello world"));
        assert!(!pattern.is_match("say hello"));

        let pattern = Pattern::new("world$").unwrap();
        assert!(pattern.is_match("hello world"));
        assert!(!pattern.is_match("world hello"));
    }

    #[test]
    fn test_pattern_char_class() {
        let pattern = Pattern::new("[aeiou]").unwrap();
        assert!(pattern.is_match("hello"));
        assert!(!pattern.is_match("rhythms"));
    }

    #[test]
    fn test_pattern_case_insensitive() {
        let options = RegexOptions {
            case_insensitive: true,
            ..Default::default()
        };
        let pattern = Pattern::with_options("hello", options).unwrap();
        assert!(pattern.is_match("HELLO"));
        assert!(pattern.is_match("Hello"));
    }

    #[test]
    fn test_pattern_multiline() {
        let options = RegexOptions {
            multiline: true,
            ..Default::default()
        };
        let pattern = Pattern::with_options("^world", options).unwrap();
        assert!(pattern.is_match("hello\nworld"));
    }

    #[test]
    fn test_pattern_captures() {
        let pattern = Pattern::new(r"\(hello\) \(world\)").unwrap();
        let caps = pattern.captures("hello world").unwrap();
        assert_eq!(caps.len(), 3);
        assert_eq!(caps.get(1).unwrap().text, "hello");
        assert_eq!(caps.get(2).unwrap().text, "world");
    }

    #[test]
    fn test_magic_mode_groups() {
        let pattern = Pattern::new(r"\(foo\)").unwrap();
        assert!(pattern.is_match("foo"));
    }

    #[test]
    fn test_magic_mode_alternation() {
        let pattern = Pattern::new(r"foo\|bar").unwrap();
        assert!(pattern.is_match("foo"));
        assert!(pattern.is_match("bar"));
    }

    #[test]
    fn test_magic_mode_word_boundary() {
        let pattern = Pattern::new(r"\<word\>").unwrap();
        assert!(pattern.is_match("a word here"));
        assert!(!pattern.is_match("awordhere"));
    }

    #[test]
    fn test_match_len() {
        let m = Match {
            start: 0,
            end: 5,
            text: "hello".to_string(),
        };
        assert_eq!(m.len(), 5);
        assert!(!m.is_empty());
    }

    #[test]
    fn test_pattern_cache() {
        let mut cache = PatternCache::new();
        assert!(cache.is_empty());

        cache.get("hello").unwrap();
        assert_eq!(cache.len(), 1);

        cache.get("hello").unwrap();
        assert_eq!(cache.len(), 1); // Still 1, reused

        cache.get("world").unwrap();
        assert_eq!(cache.len(), 2);

        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_pattern_cache_max_size() {
        let mut cache = PatternCache::with_max_size(2);
        cache.get("a").unwrap();
        cache.get("b").unwrap();
        assert_eq!(cache.len(), 2);

        cache.get("c").unwrap();
        assert_eq!(cache.len(), 2); // Evicted one
    }

    #[test]
    fn test_regex_error_display() {
        let err = RegexError::Compile("test error".to_string());
        assert!(err.to_string().contains("test error"));

        let err = RegexError::Syntax("bad syntax".to_string());
        assert!(err.to_string().contains("bad syntax"));
    }

    #[test]
    fn test_magic_mode_enum() {
        assert_eq!(MagicMode::default(), MagicMode::Magic);
    }

    #[test]
    fn test_very_magic_mode() {
        let options = RegexOptions {
            magic: MagicMode::VeryMagic,
            ..Default::default()
        };
        // In very magic, parentheses are special without escaping
        let pattern = Pattern::with_options("(foo)", options).unwrap();
        let caps = pattern.captures("foo").unwrap();
        assert_eq!(caps.get(1).unwrap().text, "foo");
    }

    #[test]
    fn test_find_from() {
        let pattern = Pattern::new("o").unwrap();
        let m = pattern.find_from("hello world", 5).unwrap();
        assert_eq!(m.start, 7); // Second 'o' in 'world'
    }

    #[test]
    fn test_captures_len() {
        let caps = Captures { groups: vec![] };
        assert!(caps.is_empty());
        assert_eq!(caps.len(), 0);
    }
}
