//! Search state and search operations.

use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{Direction, Position};

use crate::search_regex;

/// Search state tracking.
#[derive(Debug, Clone)]
pub struct SearchState {
    pub pattern: Option<String>,
    pub direction: Direction,
    pub last_match: Option<Position>,
    pub wrap_scan: bool,
    pub ignore_case: bool,
    pub smart_case: bool,
    pub hl_search: bool,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            pattern: None,
            direction: Direction::Forward,
            last_match: None,
            wrap_scan: true,
            ignore_case: false,
            smart_case: false,
            hl_search: true,
        }
    }

    /// Determine effective case sensitivity considering smart_case.
    pub fn effective_case_sensitive(&self, pattern: &str) -> bool {
        if self.smart_case && pattern.chars().any(|c| c.is_uppercase()) {
            true
        } else {
            !self.ignore_case
        }
    }
}

impl Default for SearchState {
    fn default() -> Self {
        Self::new()
    }
}

/// Search forward from position, returning next match position.
pub fn search_forward(buffer: &TextBuffer, pattern: &str, from: Position) -> Option<Position> {
    let re = search_regex::compile_pattern(pattern, true).ok()?;
    let total = buffer.line_count();
    if total == 0 {
        return None;
    }
    for offset in 0..total {
        let line_idx = (from.line + offset) % total;
        let line = buffer.line(line_idx)?;
        let start_byte = if offset == 0 {
            char_to_byte(&line, from.col + 1)
        } else {
            0
        };
        if let Some(m) = re.find_at(&line, start_byte) {
            let col = byte_to_char(&line, m.start());
            return Some(Position::new(line_idx, col));
        }
    }
    None
}

/// Search backward from position, returning previous match position.
pub fn search_backward(buffer: &TextBuffer, pattern: &str, from: Position) -> Option<Position> {
    let re = search_regex::compile_pattern(pattern, true).ok()?;
    let total = buffer.line_count();
    if total == 0 {
        return None;
    }
    // First check current line before cursor
    let line = buffer.line(from.line)?;
    let end_byte = char_to_byte(&line, from.col);
    if let Some((start, _)) = search_regex::find_prev(&line, &re, end_byte) {
        let col = byte_to_char(&line, start);
        return Some(Position::new(from.line, col));
    }
    // Then search previous lines, wrapping around
    for offset in 1..total {
        let line_idx = (from.line + total - offset) % total;
        let line = buffer.line(line_idx)?;
        if let Some((start, _)) = search_regex::find_prev(&line, &re, line.len()) {
            let col = byte_to_char(&line, start);
            return Some(Position::new(line_idx, col));
        }
    }
    None
}

/// Convert char offset to byte offset in a string.
fn char_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Convert byte offset to char offset in a string.
fn byte_to_char(s: &str, byte_idx: usize) -> usize {
    s[..byte_idx.min(s.len())].chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn search_fwd_finds_match() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "hello world\nfoo bar\n");
        let pos = search_forward(&buf, "foo", Position::ZERO);
        assert_eq!(pos, Some(Position::new(1, 0)));
    }

    #[test]
    fn search_fwd_wraps() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "abc\ndef\n");
        let pos = search_forward(&buf, "abc", Position::new(1, 0));
        assert_eq!(pos, Some(Position::new(0, 0)));
    }

    #[test]
    fn search_bwd_finds_match() {
        let buf = TextBuffer::from_text(BufferId(1), "t".into(), "abc\ndef\nabc\n");
        let pos = search_backward(&buf, "abc", Position::new(2, 2));
        assert_eq!(pos, Some(Position::new(2, 0)));
    }

    #[test]
    fn smart_case() {
        let ss = SearchState::new();
        // default ignore_case=false so always case sensitive
        assert!(ss.effective_case_sensitive("hello"));
        let ss2 = SearchState {
            smart_case: true,
            ignore_case: true,
            ..SearchState::new()
        };
        assert!(ss2.effective_case_sensitive("Hello"));
        assert!(!ss2.effective_case_sensitive("hello"));
    }
}
