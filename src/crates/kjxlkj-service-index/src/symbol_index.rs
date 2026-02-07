//! Symbol indexing for workspace-wide symbol search.

use crate::fuzzy;
use serde::{Deserialize, Serialize};

/// Kind of symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Class,
    Method,
    Variable,
    Constant,
    Module,
    Struct,
    Enum,
    Interface,
    Type,
}

/// A single symbol entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolEntry {
    pub name: String,
    pub kind: SymbolKind,
    pub file: String,
    pub line: usize,
}

/// Index of workspace symbols.
#[derive(Debug, Default)]
pub struct SymbolIndex {
    pub entries: Vec<SymbolEntry>,
}

impl SymbolIndex {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    /// Add a symbol to the index.
    pub fn add(&mut self, entry: SymbolEntry) {
        self.entries.push(entry);
    }

    /// Exact prefix search (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<&SymbolEntry> {
        let lower = query.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.name.to_lowercase().starts_with(&lower))
            .collect()
    }

    /// Fuzzy search, returning entries sorted by score.
    pub fn search_fuzzy(&self, pattern: &str) -> Vec<&SymbolEntry> {
        let names: Vec<&str> = self.entries.iter().map(|e| e.name.as_str()).collect();
        let ranked = fuzzy::rank_candidates(pattern, &names);
        ranked.iter().map(|(i, _)| &self.entries[*i]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_index() -> SymbolIndex {
        let mut idx = SymbolIndex::new();
        idx.add(SymbolEntry {
            name: "foo_bar".into(),
            kind: SymbolKind::Function,
            file: "a.rs".into(),
            line: 10,
        });
        idx.add(SymbolEntry {
            name: "FooService".into(),
            kind: SymbolKind::Struct,
            file: "b.rs".into(),
            line: 1,
        });
        idx.add(SymbolEntry {
            name: "baz_qux".into(),
            kind: SymbolKind::Function,
            file: "c.rs".into(),
            line: 20,
        });
        idx
    }

    #[test]
    fn search_prefix() {
        let idx = sample_index();
        let results = idx.search("foo");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn search_fuzzy_finds() {
        let idx = sample_index();
        let results = idx.search_fuzzy("fb");
        assert!(!results.is_empty());
        assert_eq!(results[0].name, "foo_bar");
    }
}
