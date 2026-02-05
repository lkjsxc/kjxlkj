//! Key mapping system for kjxlkj editor.
//!
//! Implements key mappings as specified in `/docs/spec/scripting/mappings/`.
//!
//! This module provides:
//! - Mode-specific key mappings
//! - Recursive and non-recursive mappings
//! - Special key handling
//! - Mapping expansion and timeout handling

use kjxlkj_core_types::{KeyEvent, Mode};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// The mode context for a mapping.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MapMode {
    /// Normal mode mappings.
    Normal,
    /// Insert mode mappings.
    Insert,
    /// Visual mode mappings (all visual variants).
    Visual,
    /// Select mode mappings.
    Select,
    /// Command-line mode mappings.
    Command,
    /// Operator-pending mode mappings.
    OperatorPending,
    /// Terminal mode mappings.
    Terminal,
}

impl From<Mode> for MapMode {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::Normal => MapMode::Normal,
            Mode::Insert => MapMode::Insert,
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => MapMode::Visual,
            Mode::Command => MapMode::Command,
            Mode::Replace => MapMode::Insert, // Replace uses insert mappings
        }
    }
}

/// Mapping flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MapFlags {
    /// Whether the mapping is recursive.
    pub recursive: bool,
    /// Whether the mapping is silent (no command-line echo).
    pub silent: bool,
    /// Whether to wait for additional keys.
    pub nowait: bool,
    /// Whether to use expression for RHS.
    pub expr: bool,
    /// Whether the mapping applies to all modes.
    pub all_modes: bool,
}

/// A single key mapping.
#[derive(Debug, Clone)]
pub struct Mapping {
    /// The left-hand side (trigger keys).
    pub lhs: Vec<KeyEvent>,
    /// The right-hand side (replacement keys).
    pub rhs: Vec<KeyEvent>,
    /// The mode this mapping applies to.
    pub mode: MapMode,
    /// Mapping flags.
    pub flags: MapFlags,
    /// Description of the mapping.
    pub description: Option<String>,
}

impl Mapping {
    /// Create a new mapping.
    pub fn new(lhs: Vec<KeyEvent>, rhs: Vec<KeyEvent>, mode: MapMode) -> Self {
        Self {
            lhs,
            rhs,
            mode,
            flags: MapFlags::default(),
            description: None,
        }
    }

    /// Set the mapping as recursive.
    pub fn recursive(mut self) -> Self {
        self.flags.recursive = true;
        self
    }

    /// Set the mapping as silent.
    pub fn silent(mut self) -> Self {
        self.flags.silent = true;
        self
    }

    /// Add a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// The mapping registry storing all defined mappings.
#[derive(Debug, Default)]
pub struct MappingRegistry {
    /// Mappings indexed by mode and LHS.
    mappings: HashMap<MapMode, HashMap<Vec<KeyEvent>, Mapping>>,
}

impl MappingRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mapping.
    pub fn add(&mut self, mapping: Mapping) {
        let mode_mappings = self.mappings.entry(mapping.mode).or_default();
        mode_mappings.insert(mapping.lhs.clone(), mapping);
    }

    /// Remove a mapping.
    pub fn remove(&mut self, mode: MapMode, lhs: &[KeyEvent]) -> Option<Mapping> {
        self.mappings.get_mut(&mode)?.remove(lhs)
    }

    /// Clear all mappings for a mode.
    pub fn clear_mode(&mut self, mode: MapMode) {
        self.mappings.remove(&mode);
    }

    /// Get a mapping by LHS.
    pub fn get(&self, mode: MapMode, lhs: &[KeyEvent]) -> Option<&Mapping> {
        self.mappings.get(&mode)?.get(lhs)
    }

    /// Check if there's a potential prefix match (for timeout handling).
    pub fn has_prefix(&self, mode: MapMode, prefix: &[KeyEvent]) -> bool {
        if let Some(mode_mappings) = self.mappings.get(&mode) {
            for lhs in mode_mappings.keys() {
                if lhs.len() > prefix.len() && lhs.starts_with(prefix) {
                    return true;
                }
            }
        }
        false
    }

    /// List all mappings for a mode.
    pub fn list(&self, mode: MapMode) -> Vec<&Mapping> {
        self.mappings
            .get(&mode)
            .map(|m| m.values().collect())
            .unwrap_or_default()
    }

    /// Get the total number of mappings.
    pub fn len(&self) -> usize {
        self.mappings.values().map(|m| m.len()).sum()
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.mappings.values().all(|m| m.is_empty())
    }
}

/// Configuration for mapping expansion.
#[derive(Debug, Clone)]
pub struct MappingConfig {
    /// Timeout for waiting for additional keys (milliseconds).
    pub timeout: Duration,
    /// Maximum recursion depth for recursive mappings.
    pub max_recursion: usize,
}

impl Default for MappingConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_millis(1000),
            max_recursion: 100,
        }
    }
}

/// State for tracking mapping expansion.
#[derive(Debug)]
pub struct MappingState {
    /// Pending keys waiting for match.
    pending: Vec<KeyEvent>,
    /// When the first pending key was received.
    pending_since: Option<Instant>,
    /// Current recursion depth.
    recursion_depth: usize,
    /// Configuration.
    config: MappingConfig,
}

impl MappingState {
    /// Create a new mapping state.
    pub fn new(config: MappingConfig) -> Self {
        Self {
            pending: Vec::new(),
            pending_since: None,
            recursion_depth: 0,
            config,
        }
    }

    /// Push a key to the pending buffer.
    pub fn push_key(&mut self, key: KeyEvent) {
        if self.pending.is_empty() {
            self.pending_since = Some(Instant::now());
        }
        self.pending.push(key);
    }

    /// Get the pending keys.
    pub fn pending_keys(&self) -> &[KeyEvent] {
        &self.pending
    }

    /// Clear pending keys.
    pub fn clear_pending(&mut self) {
        self.pending.clear();
        self.pending_since = None;
    }

    /// Check if the pending keys have timed out.
    pub fn is_timed_out(&self) -> bool {
        self.pending_since
            .map(|t| t.elapsed() >= self.config.timeout)
            .unwrap_or(false)
    }

    /// Take the pending keys and clear the buffer.
    pub fn take_pending(&mut self) -> Vec<KeyEvent> {
        self.pending_since = None;
        std::mem::take(&mut self.pending)
    }

    /// Check if recursion limit is reached.
    pub fn can_recurse(&self) -> bool {
        self.recursion_depth < self.config.max_recursion
    }

    /// Enter a recursion level.
    pub fn enter_recursion(&mut self) {
        self.recursion_depth += 1;
    }

    /// Exit a recursion level.
    pub fn exit_recursion(&mut self) {
        self.recursion_depth = self.recursion_depth.saturating_sub(1);
    }

    /// Reset recursion depth.
    pub fn reset_recursion(&mut self) {
        self.recursion_depth = 0;
    }
}

/// Result of trying to expand a mapping.
#[derive(Debug, Clone, PartialEq)]
pub enum MapResult {
    /// No mapping found, pass through the keys.
    NoMatch(Vec<KeyEvent>),
    /// Found a complete mapping, return expanded keys.
    Expanded(Vec<KeyEvent>),
    /// Waiting for more keys (prefix match).
    Pending,
    /// Recursion limit exceeded.
    RecursionLimit,
}

/// Expand mappings for incoming keys.
pub fn expand_mapping(
    registry: &MappingRegistry,
    state: &mut MappingState,
    mode: MapMode,
    key: KeyEvent,
) -> MapResult {
    state.push_key(key);
    let pending = state.pending_keys();

    // Check for exact match
    if let Some(mapping) = registry.get(mode, pending) {
        let result = mapping.rhs.clone();
        state.clear_pending();

        // Handle recursion for recursive mappings
        if mapping.flags.recursive {
            if !state.can_recurse() {
                return MapResult::RecursionLimit;
            }
            // Note: actual recursive expansion would be done by the caller
        }

        return MapResult::Expanded(result);
    }

    // Check for prefix match
    if registry.has_prefix(mode, pending) && !state.is_timed_out() {
        return MapResult::Pending;
    }

    // No match and no prefix, return pending keys
    MapResult::NoMatch(state.take_pending())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(c: char) -> KeyEvent {
        KeyEvent::char(c)
    }

    #[test]
    fn test_map_mode_from_mode() {
        assert_eq!(MapMode::from(Mode::Normal), MapMode::Normal);
        assert_eq!(MapMode::from(Mode::Insert), MapMode::Insert);
        assert_eq!(MapMode::from(Mode::Visual), MapMode::Visual);
        assert_eq!(MapMode::from(Mode::VisualLine), MapMode::Visual);
        assert_eq!(MapMode::from(Mode::VisualBlock), MapMode::Visual);
        assert_eq!(MapMode::from(Mode::Command), MapMode::Command);
    }

    #[test]
    fn test_mapping_new() {
        let mapping = Mapping::new(vec![key('j'), key('k')], vec![key('j')], MapMode::Insert);
        assert_eq!(mapping.lhs, vec![key('j'), key('k')]);
        assert_eq!(mapping.rhs, vec![key('j')]);
        assert_eq!(mapping.mode, MapMode::Insert);
        assert!(!mapping.flags.recursive);
    }

    #[test]
    fn test_mapping_flags() {
        let mapping = Mapping::new(vec![key('j')], vec![key('k')], MapMode::Normal)
            .recursive()
            .silent()
            .with_description("test mapping");

        assert!(mapping.flags.recursive);
        assert!(mapping.flags.silent);
        assert_eq!(mapping.description, Some("test mapping".to_string()));
    }

    #[test]
    fn test_registry_add_get() {
        let mut registry = MappingRegistry::new();
        let mapping = Mapping::new(vec![key('j'), key('j')], vec![key('j')], MapMode::Normal);

        registry.add(mapping);

        let retrieved = registry.get(MapMode::Normal, &[key('j'), key('j')]);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().rhs, vec![key('j')]);
    }

    #[test]
    fn test_registry_remove() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(vec![key('j')], vec![key('k')], MapMode::Normal));

        let removed = registry.remove(MapMode::Normal, &[key('j')]);
        assert!(removed.is_some());
        assert!(registry.get(MapMode::Normal, &[key('j')]).is_none());
    }

    #[test]
    fn test_registry_has_prefix() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(
            vec![key('j'), key('k')],
            vec![key('x')],
            MapMode::Normal,
        ));

        assert!(registry.has_prefix(MapMode::Normal, &[key('j')]));
        assert!(!registry.has_prefix(MapMode::Normal, &[key('x')]));
    }

    #[test]
    fn test_registry_list() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(vec![key('a')], vec![key('b')], MapMode::Normal));
        registry.add(Mapping::new(vec![key('c')], vec![key('d')], MapMode::Normal));

        let list = registry.list(MapMode::Normal);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_registry_len() {
        let mut registry = MappingRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        registry.add(Mapping::new(vec![key('a')], vec![key('b')], MapMode::Normal));
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_mapping_config_default() {
        let config = MappingConfig::default();
        assert_eq!(config.timeout, Duration::from_millis(1000));
        assert_eq!(config.max_recursion, 100);
    }

    #[test]
    fn test_mapping_state_pending() {
        let mut state = MappingState::new(MappingConfig::default());
        assert!(state.pending_keys().is_empty());

        state.push_key(key('j'));
        assert_eq!(state.pending_keys().len(), 1);

        state.clear_pending();
        assert!(state.pending_keys().is_empty());
    }

    #[test]
    fn test_mapping_state_take() {
        let mut state = MappingState::new(MappingConfig::default());
        state.push_key(key('a'));
        state.push_key(key('b'));

        let taken = state.take_pending();
        assert_eq!(taken, vec![key('a'), key('b')]);
        assert!(state.pending_keys().is_empty());
    }

    #[test]
    fn test_mapping_state_recursion() {
        let config = MappingConfig {
            max_recursion: 2,
            ..Default::default()
        };
        let mut state = MappingState::new(config);

        assert!(state.can_recurse());
        state.enter_recursion();
        assert!(state.can_recurse());
        state.enter_recursion();
        assert!(!state.can_recurse());

        state.exit_recursion();
        assert!(state.can_recurse());

        state.reset_recursion();
        assert!(state.can_recurse());
    }

    #[test]
    fn test_expand_mapping_no_match() {
        let registry = MappingRegistry::new();
        let mut state = MappingState::new(MappingConfig::default());

        let result = expand_mapping(&registry, &mut state, MapMode::Normal, key('x'));
        assert_eq!(result, MapResult::NoMatch(vec![key('x')]));
    }

    #[test]
    fn test_expand_mapping_exact_match() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(vec![key('j')], vec![key('k')], MapMode::Normal));

        let mut state = MappingState::new(MappingConfig::default());

        let result = expand_mapping(&registry, &mut state, MapMode::Normal, key('j'));
        assert_eq!(result, MapResult::Expanded(vec![key('k')]));
    }

    #[test]
    fn test_expand_mapping_multi_key() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(
            vec![key('j'), key('k')],
            vec![key('x')],
            MapMode::Normal,
        ));

        let mut state = MappingState::new(MappingConfig::default());

        // First key should be pending
        let result = expand_mapping(&registry, &mut state, MapMode::Normal, key('j'));
        assert_eq!(result, MapResult::Pending);

        // Second key should complete the mapping
        let result = expand_mapping(&registry, &mut state, MapMode::Normal, key('k'));
        assert_eq!(result, MapResult::Expanded(vec![key('x')]));
    }

    #[test]
    fn test_registry_clear_mode() {
        let mut registry = MappingRegistry::new();
        registry.add(Mapping::new(vec![key('a')], vec![key('b')], MapMode::Normal));
        registry.add(Mapping::new(vec![key('c')], vec![key('d')], MapMode::Insert));

        registry.clear_mode(MapMode::Normal);

        assert!(registry.list(MapMode::Normal).is_empty());
        assert_eq!(registry.list(MapMode::Insert).len(), 1);
    }

    #[test]
    fn test_map_flags_default() {
        let flags = MapFlags::default();
        assert!(!flags.recursive);
        assert!(!flags.silent);
        assert!(!flags.nowait);
        assert!(!flags.expr);
        assert!(!flags.all_modes);
    }
}
