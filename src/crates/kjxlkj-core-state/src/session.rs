//! Session persistence for kjxlkj editor.
//!
//! Implements session-related persistence behaviors as specified in
//! `/docs/spec/commands/session/`.
//!
//! This module provides:
//! - Swap file management for crash recovery
//! - Undo persistence across sessions
//! - Auto-save functionality
//! - Recent files tracking

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Configuration for session persistence.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Directory for swap files.
    pub swap_dir: Option<PathBuf>,
    /// Whether swap files are enabled.
    pub swap_enabled: bool,
    /// Directory for undo files.
    pub undo_dir: Option<PathBuf>,
    /// Whether undo persistence is enabled.
    pub undo_enabled: bool,
    /// Maximum undo levels to persist.
    pub undo_levels: usize,
    /// Auto-save configuration.
    pub autosave: AutoSaveConfig,
    /// Recent files tracking.
    pub recent_files: RecentFilesConfig,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            swap_dir: dirs_next::data_local_dir().map(|d| d.join("kjxlkj").join("swap")),
            swap_enabled: true,
            undo_dir: dirs_next::data_local_dir().map(|d| d.join("kjxlkj").join("undo")),
            undo_enabled: true,
            undo_levels: 1000,
            autosave: AutoSaveConfig::default(),
            recent_files: RecentFilesConfig::default(),
        }
    }
}

/// Auto-save configuration.
#[derive(Debug, Clone)]
pub struct AutoSaveConfig {
    /// Whether auto-save is enabled.
    pub enabled: bool,
    /// Delay before auto-save (debounce).
    pub delay: Duration,
    /// Save on focus lost.
    pub on_focus_lost: bool,
    /// Save on window change.
    pub on_window_change: bool,
}

impl Default for AutoSaveConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            delay: Duration::from_secs(5),
            on_focus_lost: true,
            on_window_change: false,
        }
    }
}

/// Recent files configuration.
#[derive(Debug, Clone)]
pub struct RecentFilesConfig {
    /// Maximum number of recent files to track.
    pub max_files: usize,
    /// Whether to track recent files.
    pub enabled: bool,
}

impl Default for RecentFilesConfig {
    fn default() -> Self {
        Self {
            max_files: 100,
            enabled: true,
        }
    }
}

/// Swap file entry.
#[derive(Debug, Clone)]
pub struct SwapFile {
    /// Path to the original file.
    pub file_path: PathBuf,
    /// Path to the swap file.
    pub swap_path: PathBuf,
    /// Process ID that created the swap file.
    pub pid: u32,
    /// Hostname where the swap file was created.
    pub hostname: String,
    /// Last modification time.
    pub modified: Option<std::time::SystemTime>,
}

impl SwapFile {
    /// Generate swap file name from original file path.
    pub fn swap_name_for(path: &Path) -> String {
        // Format: .{filename}.swp
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        format!(".{}.swp", filename)
    }

    /// Get the swap file path for a given file.
    pub fn swap_path_for(file_path: &Path, swap_dir: Option<&Path>) -> PathBuf {
        let swap_name = Self::swap_name_for(file_path);
        match swap_dir {
            Some(dir) => {
                // Use encoded path in swap directory
                let encoded = encode_path(file_path);
                dir.join(format!("{}.swp", encoded))
            }
            None => {
                // Same directory as original file
                file_path.with_file_name(swap_name)
            }
        }
    }
}

/// Undo file entry.
#[derive(Debug, Clone)]
pub struct UndoFile {
    /// Path to the original file.
    pub file_path: PathBuf,
    /// Path to the undo file.
    pub undo_path: PathBuf,
}

impl UndoFile {
    /// Get the undo file path for a given file.
    pub fn undo_path_for(file_path: &Path, undo_dir: &Path) -> PathBuf {
        let encoded = encode_path(file_path);
        undo_dir.join(format!("{}.un~", encoded))
    }
}

/// Recent file entry.
#[derive(Debug, Clone)]
pub struct RecentFile {
    /// Path to the file.
    pub path: PathBuf,
    /// Last access time.
    pub accessed: Instant,
}

/// Recent files tracker.
#[derive(Debug, Default)]
pub struct RecentFiles {
    /// List of recent files (most recent first).
    files: VecDeque<RecentFile>,
    /// Maximum number of files to track.
    max_files: usize,
}

impl RecentFiles {
    /// Create a new recent files tracker.
    pub fn new(max_files: usize) -> Self {
        Self {
            files: VecDeque::new(),
            max_files,
        }
    }

    /// Add a file to the recent list.
    pub fn add(&mut self, path: PathBuf) {
        // Remove if already present
        self.files.retain(|f| f.path != path);

        // Add to front
        self.files.push_front(RecentFile {
            path,
            accessed: Instant::now(),
        });

        // Trim to max size
        while self.files.len() > self.max_files {
            self.files.pop_back();
        }
    }

    /// Get the list of recent files.
    pub fn list(&self) -> impl Iterator<Item = &PathBuf> {
        self.files.iter().map(|f| &f.path)
    }

    /// Get the number of recent files.
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Clear all recent files.
    pub fn clear(&mut self) {
        self.files.clear();
    }
}

/// Auto-save state.
#[derive(Debug)]
pub struct AutoSaveState {
    /// Configuration.
    config: AutoSaveConfig,
    /// Last change time.
    last_change: Option<Instant>,
    /// Last save time.
    last_save: Option<Instant>,
    /// Whether there are pending changes.
    has_changes: bool,
}

impl AutoSaveState {
    /// Create a new auto-save state.
    pub fn new(config: AutoSaveConfig) -> Self {
        Self {
            config,
            last_change: None,
            last_save: None,
            has_changes: false,
        }
    }

    /// Mark that a change occurred.
    pub fn mark_changed(&mut self) {
        self.last_change = Some(Instant::now());
        self.has_changes = true;
    }

    /// Mark that the file was saved.
    pub fn mark_saved(&mut self) {
        self.last_save = Some(Instant::now());
        self.has_changes = false;
    }

    /// Check if auto-save should trigger.
    pub fn should_save(&self) -> bool {
        if !self.config.enabled || !self.has_changes {
            return false;
        }

        match self.last_change {
            Some(last) => last.elapsed() >= self.config.delay,
            None => false,
        }
    }

    /// Check if save should trigger on focus lost.
    pub fn should_save_on_focus_lost(&self) -> bool {
        self.config.enabled && self.config.on_focus_lost && self.has_changes
    }

    /// Get whether there are pending changes.
    pub fn has_pending_changes(&self) -> bool {
        self.has_changes
    }
}

/// Encode a file path for use in swap/undo file names.
fn encode_path(path: &Path) -> String {
    // Replace path separators with %
    let s = path.to_string_lossy();
    s.replace(['/', '\\'], "%")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_config_default() {
        let config = SessionConfig::default();
        assert!(config.swap_enabled);
        assert!(config.undo_enabled);
        assert_eq!(config.undo_levels, 1000);
    }

    #[test]
    fn test_autosave_config_default() {
        let config = AutoSaveConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.delay, Duration::from_secs(5));
        assert!(config.on_focus_lost);
    }

    #[test]
    fn test_recent_files_config_default() {
        let config = RecentFilesConfig::default();
        assert_eq!(config.max_files, 100);
        assert!(config.enabled);
    }

    #[test]
    fn test_swap_file_name() {
        let path = PathBuf::from("/home/user/test.txt");
        let name = SwapFile::swap_name_for(&path);
        assert_eq!(name, ".test.txt.swp");
    }

    #[test]
    fn test_swap_file_path_same_dir() {
        let path = PathBuf::from("/home/user/test.txt");
        let swap = SwapFile::swap_path_for(&path, None);
        assert_eq!(swap, PathBuf::from("/home/user/.test.txt.swp"));
    }

    #[test]
    fn test_swap_file_path_swap_dir() {
        let path = PathBuf::from("/home/user/test.txt");
        let swap_dir = PathBuf::from("/tmp/swap");
        let swap = SwapFile::swap_path_for(&path, Some(&swap_dir));
        assert!(swap.starts_with(&swap_dir));
        assert!(swap.to_string_lossy().ends_with(".swp"));
    }

    #[test]
    fn test_undo_file_path() {
        let path = PathBuf::from("/home/user/test.txt");
        let undo_dir = PathBuf::from("/tmp/undo");
        let undo = UndoFile::undo_path_for(&path, &undo_dir);
        assert!(undo.starts_with(&undo_dir));
        assert!(undo.to_string_lossy().ends_with(".un~"));
    }

    #[test]
    fn test_recent_files_add() {
        let mut recent = RecentFiles::new(3);
        recent.add(PathBuf::from("/a"));
        recent.add(PathBuf::from("/b"));
        recent.add(PathBuf::from("/c"));
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_recent_files_max() {
        let mut recent = RecentFiles::new(2);
        recent.add(PathBuf::from("/a"));
        recent.add(PathBuf::from("/b"));
        recent.add(PathBuf::from("/c"));
        assert_eq!(recent.len(), 2);
        // /a should be removed, /c and /b remain
        let files: Vec<_> = recent.list().collect();
        assert_eq!(files[0], &PathBuf::from("/c"));
        assert_eq!(files[1], &PathBuf::from("/b"));
    }

    #[test]
    fn test_recent_files_dedup() {
        let mut recent = RecentFiles::new(5);
        recent.add(PathBuf::from("/a"));
        recent.add(PathBuf::from("/b"));
        recent.add(PathBuf::from("/a")); // Re-add /a
        assert_eq!(recent.len(), 2);
        // /a should now be first
        let files: Vec<_> = recent.list().collect();
        assert_eq!(files[0], &PathBuf::from("/a"));
    }

    #[test]
    fn test_recent_files_clear() {
        let mut recent = RecentFiles::new(5);
        recent.add(PathBuf::from("/a"));
        recent.clear();
        assert!(recent.is_empty());
    }

    #[test]
    fn test_autosave_state_new() {
        let config = AutoSaveConfig::default();
        let state = AutoSaveState::new(config);
        assert!(!state.has_pending_changes());
        assert!(!state.should_save());
    }

    #[test]
    fn test_autosave_state_mark_changed() {
        let config = AutoSaveConfig {
            enabled: true,
            delay: Duration::from_millis(0),
            ..Default::default()
        };
        let mut state = AutoSaveState::new(config);
        state.mark_changed();
        assert!(state.has_pending_changes());
        // With 0 delay, should save immediately
        assert!(state.should_save());
    }

    #[test]
    fn test_autosave_state_mark_saved() {
        let config = AutoSaveConfig {
            enabled: true,
            delay: Duration::from_millis(0),
            ..Default::default()
        };
        let mut state = AutoSaveState::new(config);
        state.mark_changed();
        state.mark_saved();
        assert!(!state.has_pending_changes());
        assert!(!state.should_save());
    }

    #[test]
    fn test_autosave_disabled() {
        let config = AutoSaveConfig {
            enabled: false,
            ..Default::default()
        };
        let mut state = AutoSaveState::new(config);
        state.mark_changed();
        assert!(!state.should_save());
    }

    #[test]
    fn test_encode_path_unix() {
        let path = PathBuf::from("/home/user/file.txt");
        let encoded = encode_path(&path);
        assert!(!encoded.contains('/'));
        assert!(encoded.contains('%'));
    }

    #[test]
    fn test_autosave_focus_lost() {
        let config = AutoSaveConfig {
            enabled: true,
            on_focus_lost: true,
            ..Default::default()
        };
        let mut state = AutoSaveState::new(config);
        state.mark_changed();
        assert!(state.should_save_on_focus_lost());
    }

    #[test]
    fn test_autosave_focus_lost_disabled() {
        let config = AutoSaveConfig {
            enabled: true,
            on_focus_lost: false,
            ..Default::default()
        };
        let mut state = AutoSaveState::new(config);
        state.mark_changed();
        assert!(!state.should_save_on_focus_lost());
    }
}
