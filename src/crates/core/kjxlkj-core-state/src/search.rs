//! Search state and execution.
//! See /docs/spec/editing/search/README.md.

use kjxlkj_core_edit::vim_to_rust_regex;
use kjxlkj_core_text::Buffer;
use regex::Regex;

use crate::search_util::{byte_offset, char_offset, last_match, last_match_before};

/// Search direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchDirection { Forward, Backward }

/// Persistent search state for the editor.
#[derive(Debug)]
pub struct SearchState {
    pub pattern: Option<String>,
    pub compiled: Option<Regex>,
    pub direction: SearchDirection,
    pub hlsearch: bool,
    pub history: Vec<String>,
    pub ignorecase: bool,
    pub smartcase: bool,
}

impl Default for SearchState {
    fn default() -> Self {
        Self { pattern: None, compiled: None, direction: SearchDirection::Forward, hlsearch: true, history: Vec::new(), ignorecase: false, smartcase: true }
    }
}

impl SearchState {
    pub fn new() -> Self { Self::default() }

    /// Set a new search pattern, compiling it. Re-enables hlsearch.
    pub fn set_pattern(&mut self, vim_pat: &str, dir: SearchDirection) -> Result<(), String> {
        let rust_pat = vim_to_rust_regex(vim_pat)?;
        let final_pat = self.apply_case_flags(&rust_pat, vim_pat);
        let re = Regex::new(&final_pat).map_err(|e| format!("{e}"))?;
        self.pattern = Some(vim_pat.to_string());
        self.compiled = Some(re);
        self.direction = dir;
        self.hlsearch = true;
        self.push_history(vim_pat);
        Ok(())
    }

    /// Set a raw Rust regex pattern directly (for * and # word search).
    pub fn set_raw_pattern(&mut self, display: &str, rust_pat: &str, dir: SearchDirection) -> Result<(), String> {
        let re = Regex::new(rust_pat).map_err(|e| format!("{e}"))?;
        self.pattern = Some(display.to_string());
        self.compiled = Some(re);
        self.direction = dir;
        self.hlsearch = true;
        Ok(())
    }

    pub fn clear_highlight(&mut self) { self.hlsearch = false; }
    pub fn history(&self) -> &[String] { &self.history }

    fn push_history(&mut self, pat: &str) {
        if self.history.last().map_or(true, |l| l != pat) {
            self.history.push(pat.to_string());
            if self.history.len() > 100 { self.history.remove(0); }
        }
    }

    fn apply_case_flags(&self, rust_pat: &str, vim_pat: &str) -> String {
        if vim_pat.contains("\\C") { return rust_pat.to_string(); }
        if vim_pat.contains("\\c") || (self.ignorecase && !(self.smartcase && vim_pat.chars().any(|c| c.is_uppercase()))) {
            format!("(?i){}", rust_pat)
        } else { rust_pat.to_string() }
    }

    pub fn match_count(&self, buf: &Buffer) -> usize {
        let re = match self.compiled.as_ref() { Some(r) => r, None => return 0 };
        (0..buf.line_count()).map(|i| buf.line(i).map_or(0, |l| re.find_iter(&l).count())).sum()
    }

    /// Find next match in buffer starting after (row, col), wrapping.
    pub fn find_next(&self, buf: &Buffer, row: usize, col: usize) -> Option<(usize, usize)> {
        let re = self.compiled.as_ref()?;
        let lc = buf.line_count();
        if lc == 0 { return None; }
        if let Some(cur) = buf.line(row) {
            let sb = byte_offset(&cur, col + 1);
            if let Some(m) = re.find(&cur[sb..]) { return Some((row, col + 1 + char_offset(&cur[sb..], m.start()))); }
        }
        for off in 1..=lc {
            let r = (row + off) % lc;
            if let Some(line) = buf.line(r) {
                if let Some(m) = re.find(&line) { return Some((r, char_offset(&line, m.start()))); }
            }
        }
        None
    }

    /// Find previous match in buffer before (row, col), wrapping.
    pub fn find_prev(&self, buf: &Buffer, row: usize, col: usize) -> Option<(usize, usize)> {
        let re = self.compiled.as_ref()?;
        let lc = buf.line_count();
        if lc == 0 { return None; }
        if let Some(cur) = buf.line(row) {
            if let Some(pos) = last_match_before(re, &cur, col) { return Some((row, char_offset(&cur, pos))); }
        }
        for off in 1..=lc {
            let r = (row + lc - off) % lc;
            if let Some(line) = buf.line(r) {
                if let Some(pos) = last_match(re, &line) { return Some((r, char_offset(&line, pos))); }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;
    fn buf(text: &str) -> Buffer {
        let mut b = Buffer::new_scratch(BufferId(0));
        if !text.is_empty() { b.insert(0, 0, text).unwrap(); }
        b
    }
    fn ss(pat: &str, dir: SearchDirection) -> SearchState {
        let mut s = SearchState::new(); s.set_pattern(pat, dir).unwrap(); s
    }
    #[test]
    fn find_directions() {
        assert_eq!(ss("world", SearchDirection::Forward).find_next(&buf("hello world"), 0, 0), Some((0, 6)));
        assert_eq!(ss("aaa", SearchDirection::Forward).find_next(&buf("aaa\nbbb\nccc"), 1, 0), Some((0, 0)));
        assert_eq!(ss("foo", SearchDirection::Backward).find_prev(&buf("foo bar foo"), 0, 10), Some((0, 8)));
    }
    #[test]
    fn vim_regex_search() {
        assert_eq!(ss(r"\d\+", SearchDirection::Forward).find_next(&buf("count 42 items"), 0, 0), Some((0, 6)));
    }
    #[test]
    fn edge_cases() {
        assert!(SearchState::new().set_pattern("[invalid", SearchDirection::Forward).is_err());
        let empty = Buffer::new_scratch(BufferId(0));
        let s = ss("x", SearchDirection::Forward);
        assert_eq!(s.find_next(&empty, 0, 0), None);
        assert_eq!(s.find_prev(&empty, 0, 0), None);
        let s2 = ss("xyz", SearchDirection::Forward);
        assert_eq!(s2.find_next(&buf("hello world"), 0, 0), None);
        assert_eq!(s2.find_prev(&buf("hello world"), 0, 10), None);
    }
    #[test]
    fn hlsearch_and_match_count() {
        let mut s = SearchState::new();
        assert!(s.hlsearch);
        s.clear_highlight();
        assert!(!s.hlsearch);
        s.set_pattern("foo", SearchDirection::Forward).unwrap();
        assert!(s.hlsearch);
        assert_eq!(s.match_count(&buf("foo bar foo baz foo")), 3);
        assert_eq!(SearchState::new().match_count(&buf("hello")), 0);
    }
    #[test]
    fn set_raw_pattern_works() {
        let mut s = SearchState::new();
        s.set_raw_pattern(r"\bhello\b", r"\bhello\b", SearchDirection::Forward).unwrap();
        assert_eq!(s.find_next(&buf("hello world hello"), 0, 0), Some((0, 12)));
    }
    #[test]
    fn history_tracks_patterns() {
        let mut s = SearchState::new();
        s.set_pattern("foo", SearchDirection::Forward).unwrap();
        s.set_pattern("bar", SearchDirection::Forward).unwrap();
        s.set_pattern("bar", SearchDirection::Forward).unwrap(); // dedup
        assert_eq!(s.history(), &["foo", "bar"]);
    }
    #[test]
    fn ignorecase_makes_case_insensitive() {
        let mut s = SearchState::new();
        s.ignorecase = true;
        s.set_pattern("hello", SearchDirection::Forward).unwrap();
        assert_eq!(s.find_next(&buf("HELLO world"), 0, 0), Some((0, 0)));
    }
    #[test]
    fn smartcase_uppercase_is_sensitive() {
        let mut s = SearchState::new();
        s.ignorecase = true;
        s.smartcase = true;
        s.set_pattern("Hello", SearchDirection::Forward).unwrap();
        // "Hello" has uppercase → smartcase makes it case-sensitive.
        assert_eq!(s.find_next(&buf("hello world"), 0, 0), None);
        assert_eq!(s.find_next(&buf("Hello world"), 0, 0), Some((0, 0)));
    }
}
