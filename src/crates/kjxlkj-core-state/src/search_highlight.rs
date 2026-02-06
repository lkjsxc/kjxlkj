//! Search highlight model — match positions, hlsearch, incremental search state.

use kjxlkj_core_types::Position;

/// A single search match in a buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchMatch {
    pub start: Position,
    pub end: Position,
    pub is_current: bool,
}

/// Collection of search matches with current-match cursor.
#[derive(Debug, Clone)]
pub struct SearchHighlights {
    matches: Vec<SearchMatch>,
    current: Option<usize>,
    pub pattern: String,
    pub hlsearch: bool,
    pub incsearch: bool,
}

impl SearchHighlights {
    pub fn new() -> Self {
        Self { matches: Vec::new(), current: None, pattern: String::new(),
               hlsearch: true, incsearch: true }
    }

    /// Set matches from a new search, resetting current to first.
    pub fn set_matches(&mut self, pattern: &str, matches: Vec<(Position, Position)>) {
        self.pattern = pattern.to_string();
        self.matches = matches.into_iter()
            .enumerate()
            .map(|(i, (start, end))| SearchMatch { start, end, is_current: i == 0 })
            .collect();
        self.current = if self.matches.is_empty() { None } else { Some(0) };
    }

    pub fn clear(&mut self) { self.matches.clear(); self.current = None; self.pattern.clear(); }

    pub fn match_count(&self) -> usize { self.matches.len() }

    pub fn current_index(&self) -> Option<usize> { self.current }

    pub fn current_match(&self) -> Option<&SearchMatch> {
        self.current.and_then(|i| self.matches.get(i))
    }

    /// Advance to next match, wrapping around. Returns (index, wrapped).
    pub fn next(&mut self) -> Option<(usize, bool)> {
        if self.matches.is_empty() { return None; }
        let prev = self.current.unwrap_or(0);
        if let Some(m) = self.matches.get_mut(prev) { m.is_current = false; }
        let next = (prev + 1) % self.matches.len();
        let wrapped = next == 0 && prev != 0;
        self.current = Some(next);
        if let Some(m) = self.matches.get_mut(next) { m.is_current = true; }
        Some((next, wrapped))
    }

    /// Move to previous match, wrapping around. Returns (index, wrapped).
    pub fn prev(&mut self) -> Option<(usize, bool)> {
        if self.matches.is_empty() { return None; }
        let cur = self.current.unwrap_or(0);
        if let Some(m) = self.matches.get_mut(cur) { m.is_current = false; }
        let prev = if cur == 0 { self.matches.len() - 1 } else { cur - 1 };
        let wrapped = prev == self.matches.len() - 1 && cur == 0;
        self.current = Some(prev);
        if let Some(m) = self.matches.get_mut(prev) { m.is_current = true; }
        Some((prev, wrapped))
    }

    /// Return matches visible in [top_line..top_line+height].
    pub fn visible_matches(&self, top_line: usize, height: usize) -> Vec<&SearchMatch> {
        if !self.hlsearch { return Vec::new(); }
        let bot = top_line + height;
        self.matches.iter()
            .filter(|m| m.start.line < bot && m.end.line >= top_line)
            .collect()
    }

    /// Toggle hlsearch on/off (`:set hlsearch!`).
    pub fn toggle_hlsearch(&mut self) { self.hlsearch = !self.hlsearch; }

    /// Status text like "[3/15]" or "[0/0]".
    pub fn status_text(&self) -> String {
        match self.current {
            Some(i) => format!("[{}/{}]", i + 1, self.matches.len()),
            None => format!("[0/{}]", self.matches.len()),
        }
    }

    /// Find nearest match at or after a cursor position.
    pub fn nearest_at_or_after(&self, pos: Position) -> Option<usize> {
        self.matches.iter().position(|m| m.start >= pos)
            .or_else(|| if self.matches.is_empty() { None } else { Some(0) })
    }
}

impl Default for SearchHighlights {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(l: usize, c: usize) -> Position { Position::new(l, c) }

    #[test]
    fn set_matches_and_count() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("foo", vec![(pos(0, 0), pos(0, 3)), (pos(2, 5), pos(2, 8))]);
        assert_eq!(sh.match_count(), 2);
        assert_eq!(sh.current_index(), Some(0));
        assert!(sh.current_match().unwrap().is_current);
    }

    #[test]
    fn next_wraps() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("x", vec![(pos(0, 0), pos(0, 1)), (pos(1, 0), pos(1, 1))]);
        assert_eq!(sh.next(), Some((1, false)));
        assert_eq!(sh.next(), Some((0, true))); // wrapped
    }

    #[test]
    fn prev_wraps() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("x", vec![(pos(0, 0), pos(0, 1)), (pos(1, 0), pos(1, 1))]);
        assert_eq!(sh.prev(), Some((1, true))); // wrap
    }

    #[test]
    fn visible_matches_filtered() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("y", vec![
            (pos(0, 0), pos(0, 1)), (pos(5, 0), pos(5, 1)),
            (pos(10, 0), pos(10, 1)), (pos(25, 0), pos(25, 1)),
        ]);
        let vis = sh.visible_matches(4, 10); // lines 4..14
        assert_eq!(vis.len(), 2);
    }

    #[test]
    fn hlsearch_toggle_hides() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("z", vec![(pos(0, 0), pos(0, 1))]);
        assert_eq!(sh.visible_matches(0, 10).len(), 1);
        sh.toggle_hlsearch();
        assert!(sh.visible_matches(0, 10).is_empty());
    }

    #[test]
    fn status_text_format() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("q", vec![(pos(0, 0), pos(0, 1)), (pos(1, 0), pos(1, 1))]);
        assert_eq!(sh.status_text(), "[1/2]");
        sh.next();
        assert_eq!(sh.status_text(), "[2/2]");
    }

    #[test]
    fn clear_resets() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("a", vec![(pos(0, 0), pos(0, 1))]);
        sh.clear();
        assert_eq!(sh.match_count(), 0);
        assert!(sh.current_match().is_none());
    }

    #[test]
    fn nearest_at_or_after() {
        let mut sh = SearchHighlights::new();
        sh.set_matches("b", vec![(pos(2, 0), pos(2, 1)), (pos(5, 0), pos(5, 1))]);
        assert_eq!(sh.nearest_at_or_after(pos(3, 0)), Some(1));
        assert_eq!(sh.nearest_at_or_after(pos(0, 0)), Some(0));
    }

    #[test]
    fn empty_next_returns_none() {
        let mut sh = SearchHighlights::new();
        assert_eq!(sh.next(), None);
    }
}
