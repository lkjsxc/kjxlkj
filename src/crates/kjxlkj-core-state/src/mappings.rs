//! Key mapping engine for user-defined key bindings.
//!
//! Supports per-mode mappings with recursive and non-recursive variants.

use std::collections::HashMap;

use kjxlkj_core_types::{Key, Mode};

/// A single key mapping definition.
#[derive(Debug, Clone)]
pub struct KeyMapping {
    /// The source key sequence (LHS).
    pub from: Vec<Key>,
    /// The target key sequence (RHS).
    pub to: Vec<Key>,
    /// Whether this mapping is recursive.
    pub recursive: bool,
    /// Description or source of this mapping.
    pub description: Option<String>,
}

/// Which modes a mapping applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapMode {
    Normal,
    Insert,
    Visual,
    Select,
    CmdLine,
    OperatorPending,
    Terminal,
}

impl MapMode {
    /// Convert from editor Mode to MapMode.
    pub fn from_mode(mode: &Mode) -> Self {
        match mode {
            Mode::Normal => MapMode::Normal,
            Mode::Insert => MapMode::Insert,
            Mode::Visual(_) => MapMode::Visual,
            Mode::Command(_) => MapMode::CmdLine,
            Mode::OperatorPending(_) => MapMode::OperatorPending,
            Mode::Replace => MapMode::Normal, // fallback
            Mode::TerminalInsert => MapMode::Terminal,
            Mode::InsertNormal => MapMode::Normal,
        }
    }
}

/// The mapping table: stores mappings per mode.
#[derive(Debug, Clone)]
pub struct MappingTable {
    /// Mappings indexed by mode.
    maps: HashMap<MapMode, Vec<KeyMapping>>,
}

impl MappingTable {
    pub fn new() -> Self {
        Self {
            maps: HashMap::new(),
        }
    }

    /// Add a mapping for a mode.
    pub fn add(&mut self, mode: MapMode, mapping: KeyMapping) {
        self.maps.entry(mode).or_default().push(mapping);
    }

    /// Remove a mapping for a mode (by LHS match).
    pub fn remove(&mut self, mode: MapMode, from: &[Key]) {
        if let Some(mappings) = self.maps.get_mut(&mode) {
            mappings.retain(|m| m.from != from);
        }
    }

    /// Clear all mappings for a mode.
    pub fn clear(&mut self, mode: MapMode) {
        self.maps.remove(&mode);
    }

    /// Look up a mapping for the given mode and key sequence.
    /// Returns the first matching mapping (longest prefix match).
    pub fn lookup(&self, mode: MapMode, keys: &[Key]) -> MappingLookup {
        let mappings = match self.maps.get(&mode) {
            Some(m) => m,
            None => return MappingLookup::NoMatch,
        };

        let mut exact = None;
        let mut has_prefix = false;

        for mapping in mappings {
            if mapping.from == keys {
                exact = Some(mapping.clone());
            } else if mapping.from.len() > keys.len() && mapping.from.starts_with(keys) {
                has_prefix = true;
            }
        }

        if let Some(m) = exact {
            if has_prefix {
                // Could match more keys — ambiguous
                MappingLookup::Ambiguous(m)
            } else {
                MappingLookup::Exact(m)
            }
        } else if has_prefix {
            MappingLookup::Prefix
        } else {
            MappingLookup::NoMatch
        }
    }

    /// List all mappings for a mode.
    pub fn list(&self, mode: MapMode) -> &[KeyMapping] {
        self.maps.get(&mode).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

impl Default for MappingTable {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of a mapping lookup.
#[derive(Debug, Clone)]
pub enum MappingLookup {
    /// Exact match found.
    Exact(KeyMapping),
    /// There is a longer mapping that starts with this prefix. Wait for more keys.
    Prefix,
    /// An exact match exists, but longer matches are also possible.
    Ambiguous(KeyMapping),
    /// No mapping matches.
    NoMatch,
}
