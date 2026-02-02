//! Abbreviation support.
//!
//! Handles :ab and :iab style abbreviations for text expansion.

use std::collections::HashMap;

/// Abbreviation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbbrMode {
    /// Insert mode abbreviations.
    Insert,
    /// Command-line mode abbreviations.
    Command,
    /// Both modes.
    All,
}

/// A single abbreviation.
#[derive(Debug, Clone)]
pub struct Abbreviation {
    /// The trigger text.
    pub trigger: String,
    /// The expansion text.
    pub expansion: String,
    /// The mode(s) this abbreviation applies to.
    pub mode: AbbrMode,
}

impl Abbreviation {
    /// Creates a new abbreviation.
    pub fn new(trigger: &str, expansion: &str, mode: AbbrMode) -> Self {
        Self {
            trigger: trigger.to_string(),
            expansion: expansion.to_string(),
            mode,
        }
    }
}

/// Abbreviation store.
#[derive(Debug, Clone, Default)]
pub struct AbbrStore {
    /// Abbreviations by trigger.
    abbreviations: HashMap<String, Abbreviation>,
}

impl AbbrStore {
    /// Creates a new empty abbreviation store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an abbreviation.
    pub fn add(&mut self, abbr: Abbreviation) {
        self.abbreviations.insert(abbr.trigger.clone(), abbr);
    }

    /// Removes an abbreviation.
    pub fn remove(&mut self, trigger: &str) -> bool {
        self.abbreviations.remove(trigger).is_some()
    }

    /// Gets an abbreviation by trigger.
    pub fn get(&self, trigger: &str) -> Option<&Abbreviation> {
        self.abbreviations.get(trigger)
    }

    /// Checks if an abbreviation exists.
    pub fn contains(&self, trigger: &str) -> bool {
        self.abbreviations.contains_key(trigger)
    }

    /// Returns all abbreviations.
    pub fn all(&self) -> Vec<&Abbreviation> {
        self.abbreviations.values().collect()
    }

    /// Returns abbreviations for a specific mode.
    pub fn for_mode(&self, mode: AbbrMode) -> Vec<&Abbreviation> {
        self.abbreviations
            .values()
            .filter(|a| a.mode == mode || a.mode == AbbrMode::All)
            .collect()
    }

    /// Expands text using abbreviations for the given mode.
    pub fn expand(&self, text: &str, mode: AbbrMode) -> Option<&str> {
        self.abbreviations
            .get(text)
            .filter(|a| a.mode == mode || a.mode == AbbrMode::All)
            .map(|a| a.expansion.as_str())
    }

    /// Clears all abbreviations.
    pub fn clear(&mut self) {
        self.abbreviations.clear();
    }

    /// Returns the number of abbreviations.
    pub fn len(&self) -> usize {
        self.abbreviations.len()
    }

    /// Returns whether the store is empty.
    pub fn is_empty(&self) -> bool {
        self.abbreviations.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviation_new() {
        let abbr = Abbreviation::new("btw", "by the way", AbbrMode::Insert);
        assert_eq!(abbr.trigger, "btw");
        assert_eq!(abbr.expansion, "by the way");
        assert_eq!(abbr.mode, AbbrMode::Insert);
    }

    #[test]
    fn test_abbr_store_add_get() {
        let mut store = AbbrStore::new();
        store.add(Abbreviation::new("btw", "by the way", AbbrMode::Insert));

        let abbr = store.get("btw").unwrap();
        assert_eq!(abbr.expansion, "by the way");
    }

    #[test]
    fn test_abbr_store_remove() {
        let mut store = AbbrStore::new();
        store.add(Abbreviation::new("btw", "by the way", AbbrMode::Insert));

        assert!(store.remove("btw"));
        assert!(!store.contains("btw"));
    }

    #[test]
    fn test_abbr_store_expand() {
        let mut store = AbbrStore::new();
        store.add(Abbreviation::new("btw", "by the way", AbbrMode::Insert));

        assert_eq!(store.expand("btw", AbbrMode::Insert), Some("by the way"));
        assert_eq!(store.expand("btw", AbbrMode::Command), None);
    }

    #[test]
    fn test_abbr_store_for_mode() {
        let mut store = AbbrStore::new();
        store.add(Abbreviation::new("btw", "by the way", AbbrMode::Insert));
        store.add(Abbreviation::new("teh", "the", AbbrMode::All));

        let insert_abbrs = store.for_mode(AbbrMode::Insert);
        assert_eq!(insert_abbrs.len(), 2); // btw + teh (All)
    }

    #[test]
    fn test_abbr_store_all_mode() {
        let mut store = AbbrStore::new();
        store.add(Abbreviation::new("teh", "the", AbbrMode::All));

        assert_eq!(store.expand("teh", AbbrMode::Insert), Some("the"));
        assert_eq!(store.expand("teh", AbbrMode::Command), Some("the"));
    }
}
