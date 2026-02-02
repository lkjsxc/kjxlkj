//! Parser helper functions and utilities.

use crate::Range;

/// Helper struct for parser utilities.
#[derive(Debug, Default)]
pub struct ParserState {
    pub input: String,
    pub pos: usize,
}

impl ParserState {
    /// Creates a new parser state.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
            pos: 0,
        }
    }

    /// Skips whitespace.
    pub fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    /// Returns the next character without advancing.
    pub fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Advances past the current character.
    pub fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    /// Parses a number.
    pub fn parse_number(&mut self) -> Option<usize> {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.pos > start {
            self.input[start..self.pos].parse().ok()
        } else {
            None
        }
    }

    /// Returns remaining input as trimmed string.
    pub fn remaining(&self) -> String {
        self.input[self.pos..].trim().to_string()
    }

    /// Parses a command name with force flag.
    pub fn parse_command_name(&mut self) -> (String, bool) {
        let mut name = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '!' {
                name.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let force = name.ends_with('!');
        if force {
            name.pop();
        }
        (name, force)
    }

    /// Parses a range specification.
    pub fn parse_range(&mut self) -> Option<Range> {
        let start = self.pos;

        if self.peek() == Some('%') {
            self.advance();
            return Some(Range::All);
        }

        if self.peek() == Some('.') {
            self.advance();
            if self.peek() == Some(',') {
                self.advance();
                if self.peek() == Some('$') {
                    self.advance();
                    return Some(Range::All);
                }
            }
            return Some(Range::Current);
        }

        if self.peek() == Some('$') {
            self.advance();
            return Some(Range::Last);
        }

        // Try to parse line number
        if let Some(n) = self.parse_number() {
            if self.peek() == Some(',') {
                self.advance();
                if let Some(m) = self.parse_number() {
                    return Some(Range::FromTo(n.saturating_sub(1), m.saturating_sub(1)));
                }
            }
            return Some(Range::Line(n.saturating_sub(1)));
        }

        self.pos = start;
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_state_new() {
        let state = ParserState::new("hello");
        assert_eq!(state.input, "hello");
        assert_eq!(state.pos, 0);
    }

    #[test]
    fn test_parser_state_peek() {
        let state = ParserState::new("abc");
        assert_eq!(state.peek(), Some('a'));
    }

    #[test]
    fn test_parser_state_advance() {
        let mut state = ParserState::new("abc");
        state.advance();
        assert_eq!(state.peek(), Some('b'));
    }

    #[test]
    fn test_parser_state_skip_whitespace() {
        let mut state = ParserState::new("  abc");
        state.skip_whitespace();
        assert_eq!(state.peek(), Some('a'));
    }

    #[test]
    fn test_parser_state_parse_number() {
        let mut state = ParserState::new("123abc");
        assert_eq!(state.parse_number(), Some(123));
        assert_eq!(state.peek(), Some('a'));
    }

    #[test]
    fn test_parser_state_remaining() {
        let mut state = ParserState::new("hello world");
        state.pos = 6;
        assert_eq!(state.remaining(), "world");
    }

    #[test]
    fn test_parse_command_name() {
        let mut state = ParserState::new("write! file");
        let (name, force) = state.parse_command_name();
        assert_eq!(name, "write");
        assert!(force);
    }

    #[test]
    fn test_parse_range_all() {
        let mut state = ParserState::new("%");
        assert_eq!(state.parse_range(), Some(Range::All));
    }

    #[test]
    fn test_parse_range_current() {
        let mut state = ParserState::new(".");
        assert_eq!(state.parse_range(), Some(Range::Current));
    }

    #[test]
    fn test_parse_range_last() {
        let mut state = ParserState::new("$");
        assert_eq!(state.parse_range(), Some(Range::Last));
    }

    #[test]
    fn test_parse_range_line() {
        let mut state = ParserState::new("5");
        assert_eq!(state.parse_range(), Some(Range::Line(4)));
    }

    #[test]
    fn test_parse_range_from_to() {
        let mut state = ParserState::new("1,10");
        assert_eq!(state.parse_range(), Some(Range::FromTo(0, 9)));
    }
}
