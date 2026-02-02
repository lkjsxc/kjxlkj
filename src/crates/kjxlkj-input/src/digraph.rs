//! Digraph input support.
//!
//! Handles Ctrl-K style digraph input for special characters.

use std::collections::HashMap;

/// A digraph definition.
#[derive(Debug, Clone)]
pub struct Digraph {
    /// First character.
    pub char1: char,
    /// Second character.
    pub char2: char,
    /// Result character.
    pub result: char,
}

impl Digraph {
    /// Creates a new digraph.
    pub fn new(char1: char, char2: char, result: char) -> Self {
        Self { char1, char2, result }
    }

    /// Returns the key for this digraph.
    pub fn key(&self) -> (char, char) {
        (self.char1, self.char2)
    }
}

/// Digraph table for special character input.
#[derive(Debug, Clone)]
pub struct DigraphTable {
    /// Digraphs by (char1, char2).
    digraphs: HashMap<(char, char), char>,
}

impl Default for DigraphTable {
    fn default() -> Self {
        Self::new()
    }
}

impl DigraphTable {
    /// Creates a new digraph table with default digraphs.
    pub fn new() -> Self {
        let mut table = Self {
            digraphs: HashMap::new(),
        };
        table.add_defaults();
        table
    }

    /// Creates an empty digraph table.
    pub fn empty() -> Self {
        Self {
            digraphs: HashMap::new(),
        }
    }

    /// Adds default RFC 1345 digraphs.
    fn add_defaults(&mut self) {
        // Currency
        self.add('C', 't', '¢');
        self.add('P', 'd', '£');
        self.add('C', 'u', '¤');
        self.add('Y', 'e', '¥');
        self.add('E', 'u', '€');

        // Latin letters with accents
        self.add('A', '\'', 'á');
        self.add('E', '\'', 'é');
        self.add('I', '\'', 'í');
        self.add('O', '\'', 'ó');
        self.add('U', '\'', 'ú');
        self.add('A', '`', 'à');
        self.add('E', '`', 'è');
        self.add('A', '^', 'â');
        self.add('E', '^', 'ê');
        self.add('A', ':', 'ä');
        self.add('O', ':', 'ö');
        self.add('U', ':', 'ü');
        self.add('N', '~', 'ñ');

        // Greek letters
        self.add('a', '*', 'α');
        self.add('b', '*', 'β');
        self.add('g', '*', 'γ');
        self.add('d', '*', 'δ');
        self.add('p', '*', 'π');

        // Math symbols
        self.add('+', '-', '±');
        self.add('D', 'G', '°');
        self.add('M', 'y', 'µ');
        self.add('*', 'X', '×');
        self.add('-', ':', '÷');
        self.add('!', '=', '≠');
        self.add('<', '=', '≤');
        self.add('>', '=', '≥');

        // Arrows
        self.add('<', '-', '←');
        self.add('-', '>', '→');
        self.add('-', '!', '↑');
        self.add('-', 'v', '↓');

        // Box drawing
        self.add('h', 'h', '─');
        self.add('v', 'v', '│');
    }

    /// Adds a digraph.
    pub fn add(&mut self, char1: char, char2: char, result: char) {
        self.digraphs.insert((char1, char2), result);
    }

    /// Looks up a digraph.
    pub fn lookup(&self, char1: char, char2: char) -> Option<char> {
        self.digraphs
            .get(&(char1, char2))
            .or_else(|| self.digraphs.get(&(char2, char1)))
            .copied()
    }

    /// Removes a digraph.
    pub fn remove(&mut self, char1: char, char2: char) -> bool {
        self.digraphs.remove(&(char1, char2)).is_some()
    }

    /// Returns all digraphs.
    pub fn all(&self) -> Vec<Digraph> {
        self.digraphs
            .iter()
            .map(|(&(c1, c2), &r)| Digraph::new(c1, c2, r))
            .collect()
    }

    /// Returns the number of digraphs.
    pub fn len(&self) -> usize {
        self.digraphs.len()
    }

    /// Returns whether the table is empty.
    pub fn is_empty(&self) -> bool {
        self.digraphs.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digraph_new() {
        let d = Digraph::new('a', '*', 'α');
        assert_eq!(d.char1, 'a');
        assert_eq!(d.char2, '*');
        assert_eq!(d.result, 'α');
    }

    #[test]
    fn test_digraph_table_lookup() {
        let table = DigraphTable::new();
        assert_eq!(table.lookup('a', '*'), Some('α'));
        assert_eq!(table.lookup('E', 'u'), Some('€'));
    }

    #[test]
    fn test_digraph_table_lookup_reverse() {
        let table = DigraphTable::new();
        // Should work in reverse order too
        assert_eq!(table.lookup('*', 'a'), Some('α'));
    }

    #[test]
    fn test_digraph_table_custom() {
        let mut table = DigraphTable::empty();
        table.add('x', 'y', '♠');
        assert_eq!(table.lookup('x', 'y'), Some('♠'));
    }

    #[test]
    fn test_digraph_table_remove() {
        let mut table = DigraphTable::new();
        assert!(table.remove('a', '*'));
        assert_eq!(table.lookup('a', '*'), None);
    }

    #[test]
    fn test_digraph_table_defaults() {
        let table = DigraphTable::new();
        assert!(!table.is_empty());
        assert!(table.len() > 20);
    }
}
