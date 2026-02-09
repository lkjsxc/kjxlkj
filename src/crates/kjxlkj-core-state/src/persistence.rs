//! Auto-save per /docs/spec/features/session/auto_save.md.
//!
//! Periodic auto-save of modified buffers.

/// Auto-save configuration.
#[derive(Debug, Clone)]
pub struct AutoSaveConfig {
    /// Whether auto-save is enabled.
    pub enabled: bool,
    /// Interval in milliseconds.
    pub interval_ms: u64,
    /// Save on focus lost.
    pub on_focus_lost: bool,
    /// Save on buffer change.
    pub on_buffer_change: bool,
    /// Excluded filetypes.
    pub exclude_filetypes: Vec<String>,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            interval_ms: 5000,
            on_focus_lost: true,
            on_buffer_change: false,
            exclude_filetypes: Vec::new(),
        }
    }
}

/// Swap file management.
#[derive(Debug, Clone)]
pub struct SwapFileConfig {
    /// Whether swap files are enabled.
    pub enabled: bool,
    /// Directory for swap files.
    pub directory: String,
    /// Update interval in ms.
    pub update_interval: u64,
}

impl Default for SwapFileConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            directory: String::from("~/.local/state/kjxlkj/swap"),
            update_interval: 4000,
        }
    }
}

/// Undo persistence configuration.
#[derive(Debug, Clone)]
pub struct UndoPersistConfig {
    /// Whether undo persistence is enabled.
    pub enabled: bool,
    /// Directory for undo files.
    pub directory: String,
    /// Maximum undo file size in bytes.
    pub max_size: usize,
}

impl Default for UndoPersistConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            directory: String::from(
                "~/.local/state/kjxlkj/undo",
            ),
            max_size: 10 * 1024 * 1024,
        }
    }
}

/// Combined session persistence config.
#[derive(Debug, Clone, Default)]
pub struct PersistenceConfig {
    /// Auto-save settings.
    pub auto_save: AutoSaveConfig,
    /// Swap file settings.
    pub swap_file: SwapFileConfig,
    /// Undo persistence settings.
    pub undo_persist: UndoPersistConfig,
}

impl PersistenceConfig {
    /// Create with defaults.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_save_defaults() {
        let cfg = AutoSaveConfig::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.interval_ms, 5000);
        assert!(cfg.on_focus_lost);
    }

    #[test]
    fn swap_file_defaults() {
        let cfg = SwapFileConfig::default();
        assert!(cfg.enabled);
        assert!(cfg.directory.contains("swap"));
    }

    #[test]
    fn undo_persist_defaults() {
        let cfg = UndoPersistConfig::default();
        assert!(!cfg.enabled);
        assert_eq!(cfg.max_size, 10 * 1024 * 1024);
    }
}
