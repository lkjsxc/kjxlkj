//! Key mapping.

use std::collections::HashMap;
use kjxlkj_core_types::mode::Mode;
use crate::key::KeyEvent;

/// A key mapping entry.
#[derive(Debug, Clone)]
pub struct KeyMapping {
    /// The key sequence to match.
    pub from: Vec<KeyEvent>,
    /// The action or key sequence to execute.
    pub to: MappingTarget,
    /// Whether this is a recursive mapping.
    pub recursive: bool,
}

/// Target of a key mapping.
#[derive(Debug, Clone)]
pub enum MappingTarget {
    /// Map to another key sequence.
    Keys(Vec<KeyEvent>),
    /// Map to a command.
    Command(String),
    /// Map to a built-in action name.
    Action(String),
}

/// Manages key mappings.
#[derive(Debug, Default)]
pub struct KeyMapper {
    /// Mappings by mode.
    mappings: HashMap<Mode, Vec<KeyMapping>>,
}

impl KeyMapper {
    /// Creates a new key mapper.
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Adds a mapping.
    pub fn add_mapping(&mut self, mode: Mode, mapping: KeyMapping) {
        self.mappings.entry(mode).or_default().push(mapping);
    }

    /// Finds a mapping for a key sequence.
    pub fn find(&self, mode: Mode, keys: &[KeyEvent]) -> Option<&KeyMapping> {
        self.mappings.get(&mode).and_then(|mappings| {
            mappings.iter().find(|m| m.from == keys)
        })
    }

    /// Finds all mappings that could match a prefix.
    pub fn find_prefix(&self, mode: Mode, prefix: &[KeyEvent]) -> Vec<&KeyMapping> {
        self.mappings
            .get(&mode)
            .map(|mappings| {
                mappings
                    .iter()
                    .filter(|m| m.from.starts_with(prefix))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns all mappings for a mode.
    pub fn mappings_for_mode(&self, mode: Mode) -> Vec<&KeyMapping> {
        self.mappings
            .get(&mode)
            .map(|m| m.iter().collect())
            .unwrap_or_default()
    }

    /// Clears all mappings for a mode.
    pub fn clear_mode(&mut self, mode: Mode) {
        self.mappings.remove(&mode);
    }
}
