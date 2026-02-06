//! User-defined key mappings: :map, :nmap, :imap, :vmap.

use std::collections::HashMap;
use kjxlkj_core_types::Mode;

/// A user-defined key mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mapping {
    pub lhs: String,
    pub rhs: String,
    pub recursive: bool,
}

/// Storage for all user-defined mappings, keyed by mode.
#[derive(Debug, Clone, Default)]
pub struct MappingTable {
    maps: HashMap<MappingMode, Vec<Mapping>>,
}

/// Modes that support mappings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MappingMode {
    Normal, Insert, Visual, Command, OperatorPending, All,
}

impl MappingTable {
    pub fn new() -> Self { Self { maps: HashMap::new() } }

    /// Add a mapping for a mode.
    pub fn add(&mut self, mode: MappingMode, lhs: &str, rhs: &str, recursive: bool) {
        let entry = self.maps.entry(mode).or_insert_with(Vec::new);
        entry.retain(|m| m.lhs != lhs);
        entry.push(Mapping { lhs: lhs.to_string(), rhs: rhs.to_string(), recursive });
    }

    /// Remove a mapping for a mode.
    pub fn remove(&mut self, mode: MappingMode, lhs: &str) -> bool {
        if let Some(entry) = self.maps.get_mut(&mode) {
            let len = entry.len();
            entry.retain(|m| m.lhs != lhs);
            return entry.len() < len;
        }
        false
    }

    /// Look up a mapping for a mode.
    pub fn get(&self, mode: MappingMode, lhs: &str) -> Option<&Mapping> {
        if let Some(entry) = self.maps.get(&mode) {
            if let Some(m) = entry.iter().find(|m| m.lhs == lhs) { return Some(m); }
        }
        if mode != MappingMode::All {
            if let Some(entry) = self.maps.get(&MappingMode::All) {
                return entry.iter().find(|m| m.lhs == lhs);
            }
        }
        None
    }

    /// List all mappings for a mode.
    pub fn list(&self, mode: MappingMode) -> Vec<&Mapping> {
        let mut result = Vec::new();
        if let Some(entry) = self.maps.get(&mode) { result.extend(entry.iter()); }
        if mode != MappingMode::All {
            if let Some(entry) = self.maps.get(&MappingMode::All) { result.extend(entry.iter()); }
        }
        result
    }

    /// Clear all mappings for a mode.
    pub fn clear(&mut self, mode: MappingMode) { self.maps.remove(&mode); }

    /// Display all mappings as a string.
    pub fn display_all(&self) -> String {
        let mut lines = Vec::new();
        for (mode, mappings) in &self.maps {
            let prefix = match mode {
                MappingMode::Normal => "n", MappingMode::Insert => "i",
                MappingMode::Visual => "v", MappingMode::Command => "c",
                MappingMode::OperatorPending => "o", MappingMode::All => "",
            };
            for m in mappings {
                let kind = if m.recursive { "map" } else { "noremap" };
                lines.push(format!("{}{}\t{}\t{}", prefix, kind, m.lhs, m.rhs));
            }
        }
        if lines.is_empty() { "No mappings".into() } else { lines.join("\n") }
    }
}

/// Convert editor mode to mapping mode.
pub fn mode_to_mapping_mode(mode: Mode) -> MappingMode {
    match mode {
        Mode::Normal | Mode::InsertNormal => MappingMode::Normal,
        Mode::Insert => MappingMode::Insert,
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => MappingMode::Visual,
        Mode::Command => MappingMode::Command,
        Mode::OperatorPending => MappingMode::OperatorPending,
        Mode::Replace | Mode::Terminal => MappingMode::Normal,
    }
}
