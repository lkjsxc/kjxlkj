//! Search state and execution.
//! See /docs/spec/editing/search/README.md.

use kjxlkj_core_edit::vim_to_rust_regex;
use kjxlkj_core_text::Buffer;
use regex::Regex;

/// Search direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection {
    Forward,
    Backward,
}

/// Persistent search state for the editor.
#[derive(Debug)]
pub struct SearchState {
    pub pattern: Option<String>,
    pub compiled: Option<Regex>,
    pub direction: SearchDirection,
}

impl Default for SearchState {
    fn default() -> Self {
        Self { pattern: None, compiled: None, direction: SearchDirection::Forward }
    }
}

impl SearchState {
    pub fn new() -> Self { Self::default() }

    /// Set a new search pattern, compiling it.
    pub fn set_pattern(
        &mut self, vim_pat: &str, dir: SearchDirection,
    ) -> Result<(), String> {
        let rust_pat = vim_to_rust_regex(vim_pat)?;
        let re = Regex::new(&rust_pat).map_err(|e| format!("{e}"))?;
        self.pattern = Some(vim_pat.to_string());
        self.compiled = Some(re);
        self.direction = dir;
        Ok(())
    }

    /// Find next match in buffer starting after (row, col).
    /// Returns (row, col) of the match start, wrapping.
    pub fn find_next(
        &self,
        buf: &Buffer,
        row: usize,
        col: usize,
    ) -> Option<(usize, usize)> {
        let re = self.compiled.as_ref()?;
        let line_count = buf.line_count();
        if line_count == 0 {
            return None;
        }
        // Search from current line, offset past col.
        if let Some(cur_line) = buf.line(row) {
            let start_byte = byte_offset(&cur_line, col + 1);
            if let Some(m) = re.find(&cur_line[start_byte..]) {
                let mc = char_offset(&cur_line[start_byte..], m.start());
                return Some((row, col + 1 + mc));
            }
        }
        // Wrap through subsequent lines.
        for offset in 1..=line_count {
            let r = (row + offset) % line_count;
            if let Some(line) = buf.line(r) {
                if let Some(m) = re.find(&line) {
                    return Some((r, char_offset(&line, m.start())));
                }
            }
        }
        None
    }

    /// Find previous match in buffer before (row, col).
    pub fn find_prev(
        &self,
        buf: &Buffer,
        row: usize,
        col: usize,
    ) -> Option<(usize, usize)> {
        let re = self.compiled.as_ref()?;
        let line_count = buf.line_count();
        if line_count == 0 {
            return None;
        }
        // Search current line: last match starting < col.
        if let Some(cur_line) = buf.line(row) {
            if let Some(pos) =
                last_match_before(re, &cur_line, col)
            {
                return Some((
                    row,
                    char_offset(&cur_line, pos),
                ));
            }
        }
        // Wrap through preceding lines.
        for offset in 1..=line_count {
            let r =
                (row + line_count - offset) % line_count;
            if let Some(line) = buf.line(r) {
                if let Some(pos) = last_match(re, &line) {
                    return Some((r, char_offset(&line, pos)));
                }
            }
        }
        None
    }
}

/// Byte offset for the nth character in a string.
fn byte_offset(s: &str, n: usize) -> usize {
    s.char_indices()
        .nth(n)
        .map(|(i, _)| i)
        .unwrap_or(s.len())
}

/// Character offset for a byte position.
fn char_offset(s: &str, byte_pos: usize) -> usize {
    s[..byte_pos].chars().count()
}

/// Find the start byte of the last match in a string.
fn last_match(re: &Regex, s: &str) -> Option<usize> {
    let mut last = None;
    for m in re.find_iter(s) {
        last = Some(m.start());
    }
    last
}

/// Find the last match whose char-start is strictly < col.
fn last_match_before(
    re: &Regex,
    s: &str,
    col: usize,
) -> Option<usize> {
    let col_byte = byte_offset(s, col);
    let mut last = None;
    for m in re.find_iter(s) {
        if m.start() < col_byte {
            last = Some(m.start());
        }
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    fn buf_with(text: &str) -> Buffer {
        let mut b = Buffer::new_scratch(BufferId(0));
        if !text.is_empty() { b.insert(0, 0, text).unwrap(); }
        b
    }

    #[test]
    fn find_forward_simple() {
        let buf = buf_with("hello world");
        let mut ss = SearchState::new();
        ss.set_pattern("world", SearchDirection::Forward).unwrap();
        assert_eq!(ss.find_next(&buf, 0, 0), Some((0, 6)));
    }

    #[test]
    fn find_forward_wraps() {
        let buf = buf_with("aaa\nbbb\nccc");
        let mut ss = SearchState::new();
        ss.set_pattern("aaa", SearchDirection::Forward).unwrap();
        assert_eq!(ss.find_next(&buf, 1, 0), Some((0, 0)));
    }

    #[test]
    fn find_backward_simple() {
        let buf = buf_with("foo bar foo");
        let mut ss = SearchState::new();
        ss.set_pattern("foo", SearchDirection::Backward).unwrap();
        assert_eq!(ss.find_prev(&buf, 0, 10), Some((0, 8)));
    }

    #[test]
    fn vim_regex_search() {
        let buf = buf_with("count 42 items");
        let mut ss = SearchState::new();
        ss.set_pattern(r"\d\+", SearchDirection::Forward).unwrap();
        assert_eq!(ss.find_next(&buf, 0, 0), Some((0, 6)));
    }

    #[test]
    fn invalid_pattern() {
        let mut ss = SearchState::new();
        assert!(ss.set_pattern("[invalid", SearchDirection::Forward).is_err());
    }
}
