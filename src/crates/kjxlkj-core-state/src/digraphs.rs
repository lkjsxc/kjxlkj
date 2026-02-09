//! Digraph input per /docs/spec/editing/text-manipulation/digraphs.md.
//!
//! Two-character sequences that produce special characters.

use std::collections::HashMap;

/// Digraph table mapping two-char pairs to output chars.
#[derive(Debug, Clone)]
pub struct DigraphTable {
    /// Digraph entries: (c1, c2) → output.
    entries: HashMap<(char, char), char>,
}

impl Default for DigraphTable {
    fn default() -> Self {
        let mut entries = HashMap::new();
        // Standard RFC 1345 digraphs subset.
        entries.insert(('C', 'o'), '©');
        entries.insert(('R', 'g'), '®');
        entries.insert(('T', 'M'), '™');
        entries.insert(('1', '2'), '½');
        entries.insert(('1', '4'), '¼');
        entries.insert(('3', '4'), '¾');
        entries.insert(('D', 'G'), '°');
        entries.insert(('P', 'M'), '±');
        entries.insert(('M', 'y'), 'µ');
        entries.insert(('P', 'I'), 'π');
        entries.insert(('O', 'K'), '✓');
        entries.insert(('X', 'X'), '✗');
        entries.insert(('.', '.'), '…');
        entries.insert(('-', '-'), '—');
        entries.insert(('<', '<'), '«');
        entries.insert(('>', '>'), '»');
        entries.insert(('!', '!'), '¡');
        entries.insert(('?', '?'), '¿');
        entries.insert(('E', 'u'), '€');
        entries.insert(('L', 'b'), '£');
        entries.insert(('Y', 'e'), '¥');
        entries.insert(('a', ':'), 'ä');
        entries.insert(('o', ':'), 'ö');
        entries.insert(('u', ':'), 'ü');
        entries.insert(('s', 's'), 'ß');
        entries.insert(('n', '~'), 'ñ');
        entries.insert(('e', '\''), 'é');
        entries.insert(('a', '`'), 'à');
        entries.insert(('c', ','), 'ç');
        Self { entries }
    }
}

impl DigraphTable {
    /// Create new table with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Look up a digraph.
    pub fn lookup(&self, c1: char, c2: char) -> Option<char> {
        self.entries
            .get(&(c1, c2))
            .or_else(|| self.entries.get(&(c2, c1)))
            .copied()
    }

    /// Add a custom digraph.
    pub fn define(&mut self, c1: char, c2: char, out: char) {
        self.entries.insert((c1, c2), out);
    }

    /// Number of defined digraphs.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no digraphs defined.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Digraph input state.
#[derive(Debug, Clone, Default)]
pub struct DigraphInputState {
    /// Whether digraph input is active.
    pub active: bool,
    /// First character entered.
    pub first_char: Option<char>,
}

impl DigraphInputState {
    /// Start digraph input.
    pub fn start(&mut self) {
        self.active = true;
        self.first_char = None;
    }

    /// Feed a character. Returns result if complete.
    pub fn feed(&mut self, c: char, table: &DigraphTable) -> Option<char> {
        if let Some(first) = self.first_char {
            self.active = false;
            self.first_char = None;
            table.lookup(first, c)
        } else {
            self.first_char = Some(c);
            None
        }
    }

    /// Cancel digraph input.
    pub fn cancel(&mut self) {
        self.active = false;
        self.first_char = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_copyright() {
        let table = DigraphTable::new();
        assert_eq!(table.lookup('C', 'o'), Some('©'),);
    }

    #[test]
    fn lookup_reversed() {
        let table = DigraphTable::new();
        // Reversed lookup works.
        assert_eq!(table.lookup('o', 'C'), Some('©'),);
    }

    #[test]
    fn digraph_input_flow() {
        let table = DigraphTable::new();
        let mut state = DigraphInputState::default();
        state.start();
        assert!(state.active);
        assert!(state.feed('C', &table).is_none());
        let result = state.feed('o', &table);
        assert_eq!(result, Some('©'));
        assert!(!state.active);
    }
}
