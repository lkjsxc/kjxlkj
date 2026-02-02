//! User-defined key mappings.
//!
//! Custom key bindings defined by the user via :map commands.

use std::collections::HashMap;

/// User mapping mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserMapMode {
    /// Normal mode.
    Normal,
    /// Insert mode.
    Insert,
    /// Visual mode.
    Visual,
    /// Select mode.
    Select,
    /// Command-line mode.
    Command,
    /// Operator-pending mode.
    OperatorPending,
    /// Terminal mode.
    Terminal,
}

impl UserMapMode {
    /// Returns the mode character for display.
    pub fn char(&self) -> char {
        match self {
            Self::Normal => 'n',
            Self::Insert => 'i',
            Self::Visual => 'v',
            Self::Select => 's',
            Self::Command => 'c',
            Self::OperatorPending => 'o',
            Self::Terminal => 't',
        }
    }

    /// Parses from mode string.
    pub fn from_str(s: &str) -> Vec<Self> {
        let mut modes = Vec::new();
        for c in s.chars() {
            match c {
                'n' => modes.push(Self::Normal),
                'i' => modes.push(Self::Insert),
                'v' => modes.push(Self::Visual),
                's' => modes.push(Self::Select),
                'c' => modes.push(Self::Command),
                'o' => modes.push(Self::OperatorPending),
                't' => modes.push(Self::Terminal),
                _ => {}
            }
        }
        modes
    }
}

/// User mapping flags.
#[derive(Debug, Clone, Copy, Default)]
pub struct UserMapFlags {
    /// No remapping.
    pub noremap: bool,
    /// Silent (no echo).
    pub silent: bool,
    /// Expression mapping.
    pub expr: bool,
    /// Buffer-local.
    pub buffer: bool,
}

/// A user key mapping.
#[derive(Debug, Clone)]
pub struct UserMapping {
    /// Left-hand side (keys).
    pub lhs: String,
    /// Right-hand side (expansion).
    pub rhs: String,
    /// Mode.
    pub mode: UserMapMode,
    /// Flags.
    pub flags: UserMapFlags,
}

impl UserMapping {
    /// Creates a new mapping.
    pub fn new(mode: UserMapMode, lhs: &str, rhs: &str) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            mode,
            flags: UserMapFlags::default(),
        }
    }

    /// Creates a noremap mapping.
    pub fn noremap(mode: UserMapMode, lhs: &str, rhs: &str) -> Self {
        Self {
            lhs: lhs.to_string(),
            rhs: rhs.to_string(),
            mode,
            flags: UserMapFlags {
                noremap: true,
                ..Default::default()
            },
        }
    }
}

/// User mapping store.
#[derive(Debug, Default)]
pub struct UserMapStore {
    /// Mappings by mode.
    mappings: HashMap<UserMapMode, HashMap<String, UserMapping>>,
}

impl UserMapStore {
    /// Creates a new store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a mapping.
    pub fn add(&mut self, mapping: UserMapping) {
        self.mappings
            .entry(mapping.mode)
            .or_default()
            .insert(mapping.lhs.clone(), mapping);
    }

    /// Gets a mapping.
    pub fn get(&self, mode: UserMapMode, lhs: &str) -> Option<&UserMapping> {
        self.mappings.get(&mode).and_then(|m| m.get(lhs))
    }

    /// Removes a mapping.
    pub fn remove(&mut self, mode: UserMapMode, lhs: &str) -> bool {
        self.mappings
            .get_mut(&mode)
            .map(|m| m.remove(lhs).is_some())
            .unwrap_or(false)
    }

    /// Lists mappings for a mode.
    pub fn list(&self, mode: UserMapMode) -> Vec<&UserMapping> {
        self.mappings
            .get(&mode)
            .map(|m| m.values().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_map_mode_char() {
        assert_eq!(UserMapMode::Normal.char(), 'n');
        assert_eq!(UserMapMode::Insert.char(), 'i');
    }

    #[test]
    fn test_user_map_mode_from_str() {
        let modes = UserMapMode::from_str("nvo");
        assert_eq!(modes.len(), 3);
    }

    #[test]
    fn test_user_mapping_new() {
        let m = UserMapping::new(UserMapMode::Normal, "jk", "<Esc>");
        assert_eq!(m.lhs, "jk");
        assert!(!m.flags.noremap);
    }

    #[test]
    fn test_user_mapping_noremap() {
        let m = UserMapping::noremap(UserMapMode::Insert, "jk", "<Esc>");
        assert!(m.flags.noremap);
    }

    #[test]
    fn test_user_map_store_add_get() {
        let mut store = UserMapStore::new();
        store.add(UserMapping::new(UserMapMode::Normal, "jj", ":w<CR>"));
        assert!(store.get(UserMapMode::Normal, "jj").is_some());
    }

    #[test]
    fn test_user_map_store_remove() {
        let mut store = UserMapStore::new();
        store.add(UserMapping::new(UserMapMode::Normal, "jj", ":w<CR>"));
        assert!(store.remove(UserMapMode::Normal, "jj"));
        assert!(store.get(UserMapMode::Normal, "jj").is_none());
    }
}
