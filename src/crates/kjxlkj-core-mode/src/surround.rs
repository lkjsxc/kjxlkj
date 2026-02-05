//! Surround operations (vim-surround style).
//!
//! Provides add, delete, and change operations for surrounding pairs.

use std::collections::HashMap;

/// Surround pair definition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurroundPair {
    /// Opening delimiter.
    pub open: String,
    /// Closing delimiter.
    pub close: String,
    /// Add space inside when using open char.
    pub space: bool,
}

impl SurroundPair {
    /// Create a new surround pair.
    pub fn new(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            close: close.into(),
            space: false,
        }
    }

    /// Create with space option.
    pub fn with_space(open: impl Into<String>, close: impl Into<String>) -> Self {
        Self {
            open: open.into(),
            close: close.into(),
            space: true,
        }
    }
}

/// Surround registry.
#[derive(Debug, Default)]
pub struct SurroundRegistry {
    /// Registered pairs by trigger character.
    pairs: HashMap<char, SurroundPair>,
}

impl SurroundRegistry {
    /// Create a new registry with defaults.
    pub fn new() -> Self {
        let mut registry = Self::default();
        registry.register_defaults();
        registry
    }

    fn register_defaults(&mut self) {
        // Brackets with spacing for open char.
        self.pairs.insert('(', SurroundPair::with_space("( ", " )"));
        self.pairs.insert(')', SurroundPair::new("(", ")"));
        self.pairs.insert('b', SurroundPair::new("(", ")"));

        self.pairs.insert('{', SurroundPair::with_space("{ ", " }"));
        self.pairs.insert('}', SurroundPair::new("{", "}"));
        self.pairs.insert('B', SurroundPair::new("{", "}"));

        self.pairs.insert('[', SurroundPair::with_space("[ ", " ]"));
        self.pairs.insert(']', SurroundPair::new("[", "]"));
        self.pairs.insert('r', SurroundPair::new("[", "]"));

        self.pairs.insert('<', SurroundPair::with_space("< ", " >"));
        self.pairs.insert('>', SurroundPair::new("<", ">"));
        self.pairs.insert('a', SurroundPair::new("<", ">"));

        // Quotes.
        self.pairs.insert('"', SurroundPair::new("\"", "\""));
        self.pairs.insert('\'', SurroundPair::new("'", "'"));
        self.pairs.insert('`', SurroundPair::new("`", "`"));
    }

    /// Get a pair by trigger character.
    pub fn get(&self, c: char) -> Option<&SurroundPair> {
        self.pairs.get(&c)
    }

    /// Register a custom pair.
    pub fn register(&mut self, c: char, pair: SurroundPair) {
        self.pairs.insert(c, pair);
    }
}

/// Surround action type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurroundAction {
    /// Add surrounding (ys).
    Add,
    /// Delete surrounding (ds).
    Delete,
    /// Change surrounding (cs).
    Change,
}

/// Result of a surround operation.
#[derive(Debug, Clone)]
pub struct SurroundResult {
    /// New text after operation.
    pub text: String,
    /// Cursor offset adjustment.
    pub cursor_offset: isize,
}

/// Surround operations.
#[derive(Debug)]
pub struct Surround {
    /// Registry of surround pairs.
    registry: SurroundRegistry,
}

impl Default for Surround {
    fn default() -> Self {
        Self::new()
    }
}

impl Surround {
    /// Create new surround with default registry.
    pub fn new() -> Self {
        Self {
            registry: SurroundRegistry::new(),
        }
    }

    /// Create with custom registry.
    pub fn with_registry(registry: SurroundRegistry) -> Self {
        Self { registry }
    }

    /// Add surrounding to text.
    pub fn add(&self, text: &str, c: char) -> Option<SurroundResult> {
        let pair = self.registry.get(c)?;
        let result = format!("{}{}{}", pair.open, text, pair.close);
        Some(SurroundResult {
            cursor_offset: pair.open.len() as isize,
            text: result,
        })
    }

    /// Delete surrounding from text.
    pub fn delete(&self, text: &str, c: char) -> Option<SurroundResult> {
        let pair = self.registry.get(c)?;

        // Check if text starts with open and ends with close.
        if text.starts_with(&pair.open) && text.ends_with(&pair.close) {
            let inner = &text[pair.open.len()..text.len() - pair.close.len()];
            return Some(SurroundResult {
                text: inner.to_string(),
                cursor_offset: -(pair.open.len() as isize),
            });
        }

        // Handle spaced version.
        let open_spaced = format!("{} ", pair.open.trim());
        let close_spaced = format!(" {}", pair.close.trim());
        if text.starts_with(&open_spaced) && text.ends_with(&close_spaced) {
            let inner = &text[open_spaced.len()..text.len() - close_spaced.len()];
            return Some(SurroundResult {
                text: inner.to_string(),
                cursor_offset: -(open_spaced.len() as isize),
            });
        }

        None
    }

    /// Change surrounding.
    pub fn change(&self, text: &str, from: char, to: char) -> Option<SurroundResult> {
        let deleted = self.delete(text, from)?;
        self.add(&deleted.text, to)
    }

    /// Find surrounding in text around cursor.
    pub fn find_surrounding(&self, text: &str, cursor: usize, c: char) -> Option<(usize, usize)> {
        let pair = self.registry.get(c)?;
        let open = pair.open.trim();
        let close = pair.close.trim();

        // Search backwards for open.
        let before = &text[..cursor];
        let open_pos = before.rfind(open)?;

        // Search forwards for close.
        let after = &text[cursor..];
        let close_pos = after.find(close)?;

        Some((open_pos, cursor + close_pos + close.len()))
    }

    /// Get the registry.
    pub fn registry(&self) -> &SurroundRegistry {
        &self.registry
    }

    /// Get mutable registry.
    pub fn registry_mut(&mut self) -> &mut SurroundRegistry {
        &mut self.registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surround_pair_new() {
        let pair = SurroundPair::new("(", ")");
        assert_eq!(pair.open, "(");
        assert_eq!(pair.close, ")");
        assert!(!pair.space);
    }

    #[test]
    fn test_surround_registry_defaults() {
        let reg = SurroundRegistry::new();
        assert!(reg.get('"').is_some());
        assert!(reg.get('(').is_some());
        assert!(reg.get(')').is_some());
    }

    #[test]
    fn test_surround_add_quotes() {
        let surround = Surround::new();
        let result = surround.add("word", '"').unwrap();
        assert_eq!(result.text, "\"word\"");
    }

    #[test]
    fn test_surround_add_parens() {
        let surround = Surround::new();
        let result = surround.add("word", ')').unwrap();
        assert_eq!(result.text, "(word)");
    }

    #[test]
    fn test_surround_add_parens_spaced() {
        let surround = Surround::new();
        let result = surround.add("word", '(').unwrap();
        assert_eq!(result.text, "( word )");
    }

    #[test]
    fn test_surround_delete_quotes() {
        let surround = Surround::new();
        let result = surround.delete("\"word\"", '"').unwrap();
        assert_eq!(result.text, "word");
    }

    #[test]
    fn test_surround_delete_parens() {
        let surround = Surround::new();
        let result = surround.delete("(word)", ')').unwrap();
        assert_eq!(result.text, "word");
    }

    #[test]
    fn test_surround_change() {
        let surround = Surround::new();
        let result = surround.change("\"word\"", '"', '\'').unwrap();
        assert_eq!(result.text, "'word'");
    }

    #[test]
    fn test_surround_find() {
        let surround = Surround::new();
        let text = "hello (world) there";
        let result = surround.find_surrounding(text, 8, ')');
        assert_eq!(result, Some((6, 13)));
    }

    #[test]
    fn test_surround_registry_custom() {
        let mut reg = SurroundRegistry::new();
        reg.register('!', SurroundPair::new("/*", "*/"));
        assert!(reg.get('!').is_some());
        assert_eq!(reg.get('!').unwrap().open, "/*");
    }
}
