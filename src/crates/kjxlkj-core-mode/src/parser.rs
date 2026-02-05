//! Key sequence parser for multi-key commands.

use kjxlkj_core_types::KeyEvent;

/// A sequence of key events.
#[derive(Debug, Clone, Default)]
pub struct KeySequence {
    keys: Vec<KeyEvent>,
}

impl KeySequence {
    /// Create a new empty sequence.
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Push a key event.
    pub fn push(&mut self, key: KeyEvent) {
        self.keys.push(key);
    }

    /// Clear the sequence.
    pub fn clear(&mut self) {
        self.keys.clear();
    }

    /// Check if the sequence is empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Get the keys.
    pub fn keys(&self) -> &[KeyEvent] {
        &self.keys
    }
}

/// Result of parsing a key sequence.
#[derive(Debug, Clone, PartialEq)]
pub enum ParseResult {
    /// The sequence is incomplete, need more keys.
    Incomplete,
    /// The sequence matched a command.
    Complete(String),
    /// The sequence does not match any command.
    NoMatch,
}

/// Key sequence parser.
#[derive(Debug, Default)]
pub struct Parser {
    sequence: KeySequence,
}

impl Parser {
    /// Create a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Feed a key event and get the parse result.
    pub fn feed(&mut self, key: KeyEvent) -> ParseResult {
        self.sequence.push(key.clone());

        // Check for known multi-key sequences
        match self.sequence.keys() {
            [KeyEvent::Char('g', _), KeyEvent::Char('g', _)] => {
                self.sequence.clear();
                ParseResult::Complete("gg".to_string())
            }
            [KeyEvent::Char('g', _)] => ParseResult::Incomplete,
            [KeyEvent::Char('z', _), KeyEvent::Char('z', _)] => {
                self.sequence.clear();
                ParseResult::Complete("zz".to_string())
            }
            [KeyEvent::Char('z', _), KeyEvent::Char('t', _)] => {
                self.sequence.clear();
                ParseResult::Complete("zt".to_string())
            }
            [KeyEvent::Char('z', _), KeyEvent::Char('b', _)] => {
                self.sequence.clear();
                ParseResult::Complete("zb".to_string())
            }
            [KeyEvent::Char('z', _)] => ParseResult::Incomplete,
            _ => {
                self.sequence.clear();
                ParseResult::NoMatch
            }
        }
    }

    /// Reset the parser state.
    pub fn reset(&mut self) {
        self.sequence.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gg_sequence() {
        let mut parser = Parser::new();
        let r1 = parser.feed(KeyEvent::char('g'));
        assert_eq!(r1, ParseResult::Incomplete);
        let r2 = parser.feed(KeyEvent::char('g'));
        assert_eq!(r2, ParseResult::Complete("gg".to_string()));
    }

    #[test]
    fn test_no_match() {
        let mut parser = Parser::new();
        let r = parser.feed(KeyEvent::char('x'));
        assert_eq!(r, ParseResult::NoMatch);
    }

    #[test]
    fn test_zz_sequence() {
        let mut parser = Parser::new();
        parser.feed(KeyEvent::char('z'));
        let r = parser.feed(KeyEvent::char('z'));
        assert_eq!(r, ParseResult::Complete("zz".to_string()));
    }
}
