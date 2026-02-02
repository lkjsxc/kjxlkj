//! Sign column support.
//!
//! Provides sign column markers for breakpoints, errors, etc.

use std::collections::HashMap;

pub use crate::sign_types::{Sign, SignDefinition, SignPriority};

/// Sign column state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct SignColumn {
    /// Sign definitions.
    definitions: HashMap<String, SignDefinition>,
    /// Placed signs by ID.
    signs: HashMap<usize, Sign>,
    /// Signs by line.
    by_line: HashMap<usize, Vec<usize>>,
    /// Next sign ID.
    next_id: usize,
}

impl SignColumn {
    /// Creates a new sign column.
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ..Default::default()
        }
    }

    /// Defines a sign.
    pub fn define(&mut self, def: SignDefinition) {
        self.definitions.insert(def.name.clone(), def);
    }

    /// Gets a sign definition.
    pub fn definition(&self, name: &str) -> Option<&SignDefinition> {
        self.definitions.get(name)
    }

    /// Places a sign.
    pub fn place(&mut self, name: &str, line: usize) -> Option<usize> {
        if !self.definitions.contains_key(name) {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let sign = Sign::new(id, name, line);
        self.signs.insert(id, sign);
        self.by_line.entry(line).or_default().push(id);

        Some(id)
    }

    /// Removes a sign by ID.
    pub fn unplace(&mut self, id: usize) -> bool {
        if let Some(sign) = self.signs.remove(&id) {
            if let Some(ids) = self.by_line.get_mut(&sign.line) {
                ids.retain(|&i| i != id);
                if ids.is_empty() {
                    self.by_line.remove(&sign.line);
                }
            }
            true
        } else {
            false
        }
    }

    /// Sets the priority for a placed sign.
    pub fn set_priority(&mut self, id: usize, priority: SignPriority) -> bool {
        if let Some(sign) = self.signs.get_mut(&id) {
            sign.priority = priority;
            true
        } else {
            false
        }
    }

    /// Gets signs at a line.
    pub fn signs_at(&self, line: usize) -> Vec<&Sign> {
        self.by_line
            .get(&line)
            .map(|ids| ids.iter().filter_map(|id| self.signs.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets the highest priority sign at a line.
    pub fn top_sign_at(&self, line: usize) -> Option<&Sign> {
        self.signs_at(line)
            .into_iter()
            .max_by_key(|s| s.priority)
    }

    /// Clears all signs.
    pub fn clear(&mut self) {
        self.signs.clear();
        self.by_line.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_definition() {
        let def = SignDefinition::new("Error", ">>")
            .with_text_highlight("ErrorSign")
            .with_line_highlight("ErrorLine");

        assert_eq!(def.name, "Error");
        assert_eq!(def.text, ">>");
        assert_eq!(def.line_highlight.as_deref(), Some("ErrorLine"));
    }

    #[test]
    fn test_sign_column_define() {
        let mut col = SignColumn::new();
        col.define(SignDefinition::new("Error", "E"));

        assert!(col.definition("Error").is_some());
    }

    #[test]
    fn test_sign_column_place() {
        let mut col = SignColumn::new();
        col.define(SignDefinition::new("Error", "E"));

        let id = col.place("Error", 10).unwrap();
        assert!(id > 0);
        assert_eq!(col.signs_at(10).len(), 1);
    }

    #[test]
    fn test_sign_column_unplace() {
        let mut col = SignColumn::new();
        col.define(SignDefinition::new("Error", "E"));
        let id = col.place("Error", 10).unwrap();

        assert!(col.unplace(id));
        assert!(col.signs_at(10).is_empty());
    }

    #[test]
    fn test_sign_column_top_sign() {
        let mut col = SignColumn::new();
        col.define(SignDefinition::new("Error", "E"));
        col.define(SignDefinition::new("Warning", "W"));

        col.place("Warning", 10);
        let id2 = col.place("Error", 10).unwrap();
        col.signs.get_mut(&id2).unwrap().priority = 20;

        let top = col.top_sign_at(10).unwrap();
        assert_eq!(top.name, "Error");
    }

    #[test]
    fn test_sign_column_clear() {
        let mut col = SignColumn::new();
        col.define(SignDefinition::new("Error", "E"));
        col.place("Error", 10);

        col.clear();
        assert!(col.signs_at(10).is_empty());
    }
}
