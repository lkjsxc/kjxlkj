//! Completion source management.
//!
//! Manages multiple completion sources.

use std::collections::HashMap;

/// Completion source type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceKind {
    /// Buffer words.
    Buffer,
    /// LSP completions.
    Lsp,
    /// Snippets.
    Snippet,
    /// File paths.
    Path,
    /// Command line.
    CmdLine,
    /// Dictionary words.
    Dictionary,
    /// Tags.
    Tags,
    /// Custom source.
    Custom,
}

/// Completion source priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePriority(pub u8);

impl Default for SourcePriority {
    fn default() -> Self {
        Self(50)
    }
}

/// Completion source configuration.
#[derive(Debug, Clone)]
pub struct SourceConfig {
    /// Source kind.
    pub kind: SourceKind,
    /// Priority (higher = shown first).
    pub priority: SourcePriority,
    /// Whether enabled.
    pub enabled: bool,
    /// Trigger characters.
    pub triggers: Vec<char>,
}

impl SourceConfig {
    /// Creates a new source config.
    pub fn new(kind: SourceKind) -> Self {
        Self {
            kind,
            priority: SourcePriority::default(),
            enabled: true,
            triggers: Vec::new(),
        }
    }

    /// Sets priority.
    pub fn with_priority(mut self, priority: u8) -> Self {
        self.priority = SourcePriority(priority);
        self
    }

    /// Sets trigger characters.
    pub fn with_triggers(mut self, triggers: &[char]) -> Self {
        self.triggers = triggers.to_vec();
        self
    }
}

/// Source manager.
#[derive(Debug, Default)]
pub struct SourceManager {
    /// Configured sources.
    sources: HashMap<SourceKind, SourceConfig>,
}

impl SourceManager {
    /// Creates a new source manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a source.
    pub fn register(&mut self, config: SourceConfig) {
        self.sources.insert(config.kind, config);
    }

    /// Gets a source config.
    pub fn get(&self, kind: SourceKind) -> Option<&SourceConfig> {
        self.sources.get(&kind)
    }

    /// Enables a source.
    pub fn enable(&mut self, kind: SourceKind) {
        if let Some(src) = self.sources.get_mut(&kind) {
            src.enabled = true;
        }
    }

    /// Disables a source.
    pub fn disable(&mut self, kind: SourceKind) {
        if let Some(src) = self.sources.get_mut(&kind) {
            src.enabled = false;
        }
    }

    /// Returns enabled sources sorted by priority.
    pub fn enabled_sorted(&self) -> Vec<&SourceConfig> {
        let mut sources: Vec<_> = self.sources.values().filter(|s| s.enabled).collect();
        sources.sort_by(|a, b| b.priority.cmp(&a.priority));
        sources
    }

    /// Gets sources triggered by a character.
    pub fn triggered_by(&self, ch: char) -> Vec<&SourceConfig> {
        self.sources
            .values()
            .filter(|s| s.enabled && s.triggers.contains(&ch))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_kind() {
        assert_ne!(SourceKind::Buffer, SourceKind::Lsp);
    }

    #[test]
    fn test_source_config_new() {
        let cfg = SourceConfig::new(SourceKind::Lsp);
        assert!(cfg.enabled);
    }

    #[test]
    fn test_source_config_priority() {
        let cfg = SourceConfig::new(SourceKind::Lsp).with_priority(100);
        assert_eq!(cfg.priority.0, 100);
    }

    #[test]
    fn test_source_manager_register() {
        let mut mgr = SourceManager::new();
        mgr.register(SourceConfig::new(SourceKind::Buffer));
        assert!(mgr.get(SourceKind::Buffer).is_some());
    }

    #[test]
    fn test_source_manager_enable_disable() {
        let mut mgr = SourceManager::new();
        mgr.register(SourceConfig::new(SourceKind::Lsp));
        mgr.disable(SourceKind::Lsp);
        assert!(!mgr.get(SourceKind::Lsp).unwrap().enabled);
    }

    #[test]
    fn test_source_manager_sorted() {
        let mut mgr = SourceManager::new();
        mgr.register(SourceConfig::new(SourceKind::Buffer).with_priority(10));
        mgr.register(SourceConfig::new(SourceKind::Lsp).with_priority(100));
        let sorted = mgr.enabled_sorted();
        assert_eq!(sorted[0].kind, SourceKind::Lsp);
    }
}
