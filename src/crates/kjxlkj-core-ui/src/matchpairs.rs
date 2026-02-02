//! Match pairs.
//!
//! Matching bracket/pair highlighting.

use std::collections::HashMap;

/// A matching pair definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchPair {
    /// Opening character.
    pub open: char,
    /// Closing character.
    pub close: char,
}

impl MatchPair {
    /// Creates a new pair.
    pub const fn new(open: char, close: char) -> Self {
        Self { open, close }
    }

    /// Returns whether a character is the opener.
    pub fn is_open(&self, ch: char) -> bool {
        ch == self.open
    }

    /// Returns whether a character is the closer.
    pub fn is_close(&self, ch: char) -> bool {
        ch == self.close
    }
}

/// Default pairs.
pub const DEFAULT_PAIRS: &[MatchPair] = &[
    MatchPair::new('(', ')'),
    MatchPair::new('[', ']'),
    MatchPair::new('{', '}'),
    MatchPair::new('<', '>'),
];

/// Match result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MatchResult {
    /// Position of the match (line, column).
    pub position: (usize, usize),
    /// Whether this is an opening bracket.
    pub is_open: bool,
}

/// Pair matcher.
#[derive(Debug, Clone)]
pub struct PairMatcher {
    /// Open to close map.
    open_to_close: HashMap<char, char>,
    /// Close to open map.
    close_to_open: HashMap<char, char>,
}

impl Default for PairMatcher {
    fn default() -> Self {
        let mut matcher = Self {
            open_to_close: HashMap::new(),
            close_to_open: HashMap::new(),
        };
        for pair in DEFAULT_PAIRS {
            matcher.add_pair(pair.open, pair.close);
        }
        matcher
    }
}

impl PairMatcher {
    /// Creates a new pair matcher.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a pair.
    pub fn add_pair(&mut self, open: char, close: char) {
        self.open_to_close.insert(open, close);
        self.close_to_open.insert(close, open);
    }

    /// Returns whether a character is an opener.
    pub fn is_open(&self, ch: char) -> bool {
        self.open_to_close.contains_key(&ch)
    }

    /// Returns whether a character is a closer.
    pub fn is_close(&self, ch: char) -> bool {
        self.close_to_open.contains_key(&ch)
    }

    /// Gets the matching character.
    pub fn matching(&self, ch: char) -> Option<char> {
        self.open_to_close
            .get(&ch)
            .or_else(|| self.close_to_open.get(&ch))
            .copied()
    }

    /// Finds matching bracket in text.
    pub fn find_match(&self, text: &str, pos: usize) -> Option<usize> {
        let chars: Vec<char> = text.chars().collect();
        if pos >= chars.len() {
            return None;
        }

        let ch = chars[pos];
        let (target, direction) = if let Some(&close) = self.open_to_close.get(&ch) {
            (close, 1i32)
        } else if let Some(&open) = self.close_to_open.get(&ch) {
            (open, -1i32)
        } else {
            return None;
        };

        let mut depth = 1;
        let mut i = pos as i32 + direction;

        while i >= 0 && (i as usize) < chars.len() {
            let c = chars[i as usize];
            if c == ch {
                depth += 1;
            } else if c == target {
                depth -= 1;
                if depth == 0 {
                    return Some(i as usize);
                }
            }
            i += direction;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_pair() {
        let pair = MatchPair::new('(', ')');
        assert!(pair.is_open('('));
        assert!(pair.is_close(')'));
    }

    #[test]
    fn test_pair_matcher_is_open() {
        let matcher = PairMatcher::new();
        assert!(matcher.is_open('('));
        assert!(matcher.is_open('{'));
        assert!(!matcher.is_open('x'));
    }

    #[test]
    fn test_pair_matcher_matching() {
        let matcher = PairMatcher::new();
        assert_eq!(matcher.matching('('), Some(')'));
        assert_eq!(matcher.matching(')'), Some('('));
    }

    #[test]
    fn test_pair_matcher_find_simple() {
        let matcher = PairMatcher::new();
        let text = "(hello)";
        assert_eq!(matcher.find_match(text, 0), Some(6));
        assert_eq!(matcher.find_match(text, 6), Some(0));
    }

    #[test]
    fn test_pair_matcher_find_nested() {
        let matcher = PairMatcher::new();
        let text = "((inner))";
        assert_eq!(matcher.find_match(text, 0), Some(8));
        assert_eq!(matcher.find_match(text, 1), Some(7));
    }

    #[test]
    fn test_pair_matcher_no_match() {
        let matcher = PairMatcher::new();
        let text = "(unclosed";
        assert_eq!(matcher.find_match(text, 0), None);
    }
}
