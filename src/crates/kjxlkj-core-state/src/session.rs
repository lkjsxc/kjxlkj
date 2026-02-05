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

// ============================================================================
// Macros
// ============================================================================

/// A recorded macro.
#[derive(Debug, Clone)]
pub struct Macro {
    /// Register name (a-z).
    pub register: char,
    /// Recorded keystrokes.
    pub keys: Vec<KeyStroke>,
}

impl Macro {
    /// Create a new macro.
    pub fn new(register: char) -> Self {
        Self {
            register,
            keys: Vec::new(),
        }
    }

    /// Add a keystroke to the macro.
    pub fn push(&mut self, key: KeyStroke) {
        self.keys.push(key);
    }

    /// Get the number of keystrokes.
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

/// A keystroke for macro recording.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyStroke {
    /// Key code or character.
    pub key: String,
    /// Modifier keys.
    pub modifiers: KeyModifiers,
}

impl KeyStroke {
    /// Create a simple character keystroke.
    pub fn char(c: char) -> Self {
        Self {
            key: c.to_string(),
            modifiers: KeyModifiers::empty(),
        }
    }

    /// Create a keystroke with key name.
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            key: name.into(),
            modifiers: KeyModifiers::empty(),
        }
    }

    /// Add control modifier.
    pub fn ctrl(mut self) -> Self {
        self.modifiers.ctrl = true;
        self
    }

    /// Add alt modifier.
    pub fn alt(mut self) -> Self {
        self.modifiers.alt = true;
        self
    }

    /// Add shift modifier.
    pub fn shift(mut self) -> Self {
        self.modifiers.shift = true;
        self
    }
}

/// Key modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers {
    /// Control key pressed.
    pub ctrl: bool,
    /// Alt key pressed.
    pub alt: bool,
    /// Shift key pressed.
    pub shift: bool,
}

impl KeyModifiers {
    /// No modifiers.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Check if any modifier is set.
    pub fn any(&self) -> bool {
        self.ctrl || self.alt || self.shift
    }
}

/// Macro recorder state.
#[derive(Debug, Default)]
pub struct MacroRecorder {
    /// Currently recording macro.
    recording: Option<Macro>,
    /// Stored macros.
    macros: std::collections::HashMap<char, Macro>,
    /// Last played macro register.
    last_played: Option<char>,
}

impl MacroRecorder {
    /// Create a new macro recorder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Start recording to a register.
    pub fn start(&mut self, register: char) {
        self.recording = Some(Macro::new(register));
    }

    /// Stop recording and store the macro.
    pub fn stop(&mut self) {
        if let Some(macro_) = self.recording.take() {
            if !macro_.is_empty() {
                self.macros.insert(macro_.register, macro_);
            }
        }
    }

    /// Check if currently recording.
    pub fn is_recording(&self) -> bool {
        self.recording.is_some()
    }

    /// Get the register being recorded to.
    pub fn recording_register(&self) -> Option<char> {
        self.recording.as_ref().map(|m| m.register)
    }

    /// Record a keystroke.
    pub fn record(&mut self, key: KeyStroke) {
        if let Some(ref mut macro_) = self.recording {
            macro_.push(key);
        }
    }

    /// Get a stored macro.
    pub fn get(&self, register: char) -> Option<&Macro> {
        self.macros.get(&register)
    }

    /// Play a macro, returning its keystrokes.
    pub fn play(&mut self, register: char) -> Option<Vec<KeyStroke>> {
        if let Some(macro_) = self.macros.get(&register) {
            self.last_played = Some(register);
            Some(macro_.keys.clone())
        } else {
            None
        }
    }

    /// Replay the last played macro.
    pub fn replay(&mut self) -> Option<Vec<KeyStroke>> {
        self.last_played.and_then(|r| self.play(r))
    }
}

// ============================================================================
// Session Persistence
// ============================================================================

/// A saved session.
#[derive(Debug, Clone)]
pub struct Session {
    /// Session name.
    pub name: String,
    /// Working directory.
    pub cwd: PathBuf,
    /// Open buffer paths.
    pub buffers: Vec<PathBuf>,
    /// Window layout.
    pub layout: SessionLayout,
    /// Active buffer index.
    pub active_buffer: usize,
}

impl Session {
    /// Create a new session.
    pub fn new(name: impl Into<String>, cwd: PathBuf) -> Self {
        Self {
            name: name.into(),
            cwd,
            buffers: Vec::new(),
            layout: SessionLayout::default(),
            active_buffer: 0,
        }
    }

    /// Add a buffer to the session.
    pub fn add_buffer(&mut self, path: PathBuf) {
        if !self.buffers.contains(&path) {
            self.buffers.push(path);
        }
    }
}

/// Session layout info.
#[derive(Debug, Clone, Default)]
pub struct SessionLayout {
    /// Window splits.
    pub splits: Vec<SessionSplit>,
}

/// A window split.
#[derive(Debug, Clone)]
pub struct SessionSplit {
    /// Split direction.
    pub direction: SplitDirection,
    /// Buffer index or nested layout.
    pub content: SplitContent,
    /// Size ratio (0.0-1.0).
    pub ratio: f32,
}

/// Split direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitDirection {
    /// Horizontal split (side by side).
    #[default]
    Horizontal,
    /// Vertical split (stacked).
    Vertical,
}

/// Split content.
#[derive(Debug, Clone)]
pub enum SplitContent {
    /// A buffer by index.
    Buffer(usize),
    /// Nested splits.
    Nested(Vec<SessionSplit>),
}

// ============================================================================
// Workspaces
// ============================================================================

/// A workspace with multiple folders.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Workspace name.
    pub name: Option<String>,
    /// Workspace file path.
    pub file_path: Option<PathBuf>,
    /// Root folders.
    pub folders: Vec<WorkspaceFolder>,
    /// Workspace-level settings.
    pub settings: WorkspaceSettings,
}

impl Workspace {
    /// Create a new empty workspace.
    pub fn new() -> Self {
        Self {
            name: None,
            file_path: None,
            folders: Vec::new(),
            settings: WorkspaceSettings::default(),
        }
    }

    /// Create a single-folder workspace.
    pub fn single(folder: PathBuf) -> Self {
        Self {
            name: folder.file_name().map(|n| n.to_string_lossy().into_owned()),
            file_path: None,
            folders: vec![WorkspaceFolder::new(folder)],
            settings: WorkspaceSettings::default(),
        }
    }

    /// Add a folder to the workspace.
    pub fn add_folder(&mut self, folder: PathBuf) {
        if !self.folders.iter().any(|f| f.path == folder) {
            self.folders.push(WorkspaceFolder::new(folder));
        }
    }

    /// Remove a folder from the workspace.
    pub fn remove_folder(&mut self, folder: &Path) -> bool {
        let len = self.folders.len();
        self.folders.retain(|f| f.path != folder);
        self.folders.len() < len
    }

    /// Get number of folders.
    pub fn folder_count(&self) -> usize {
        self.folders.len()
    }

    /// Check if workspace is multi-root.
    pub fn is_multi_root(&self) -> bool {
        self.folders.len() > 1
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

/// A folder in a workspace.
#[derive(Debug, Clone)]
pub struct WorkspaceFolder {
    /// Folder path.
    pub path: PathBuf,
    /// Display name (optional override).
    pub name: Option<String>,
}

impl WorkspaceFolder {
    /// Create a new workspace folder.
    pub fn new(path: PathBuf) -> Self {
        Self { path, name: None }
    }

    /// Set a custom display name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Get the display name.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or_else(|| {
            self.path
                .file_name()
                .map(|n| n.to_str().unwrap_or("unknown"))
                .unwrap_or("unknown")
        })
    }
}

/// Workspace settings.
#[derive(Debug, Clone, Default)]
pub struct WorkspaceSettings {
    /// Per-folder settings overrides.
    pub folder_settings: std::collections::HashMap<PathBuf, FolderSettings>,
}

/// Per-folder settings.
#[derive(Debug, Clone, Default)]
pub struct FolderSettings {
    /// Excluded patterns.
    pub exclude: Vec<String>,
    /// Include patterns.
    pub include: Vec<String>,
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

    // ═══════════════════════════════════════════════════════════════════════════════
    // Macro Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_keystroke_char() {
        let k = KeyStroke::char('a');
        assert_eq!(k.key, "a");
        assert!(!k.modifiers.any());
    }

    #[test]
    fn test_keystroke_named() {
        let k = KeyStroke::named("Enter");
        assert_eq!(k.key, "Enter");
    }

    #[test]
    fn test_keystroke_modifiers() {
        let k = KeyStroke::char('c').ctrl();
        assert!(k.modifiers.ctrl);
        assert!(!k.modifiers.alt);

        let k2 = KeyStroke::char('a').alt().shift();
        assert!(k2.modifiers.alt);
        assert!(k2.modifiers.shift);
    }

    #[test]
    fn test_key_modifiers_empty() {
        let m = KeyModifiers::empty();
        assert!(!m.any());
    }

    #[test]
    fn test_macro_new() {
        let m = Macro::new('a');
        assert_eq!(m.register, 'a');
        assert!(m.is_empty());
    }

    #[test]
    fn test_macro_push() {
        let mut m = Macro::new('b');
        m.push(KeyStroke::char('x'));
        m.push(KeyStroke::char('y'));
        assert_eq!(m.len(), 2);
    }

    #[test]
    fn test_macro_recorder_start_stop() {
        let mut rec = MacroRecorder::new();
        assert!(!rec.is_recording());

        rec.start('a');
        assert!(rec.is_recording());
        assert_eq!(rec.recording_register(), Some('a'));

        rec.record(KeyStroke::char('x'));
        rec.stop();

        assert!(!rec.is_recording());
        assert!(rec.get('a').is_some());
    }

    #[test]
    fn test_macro_recorder_play() {
        let mut rec = MacroRecorder::new();
        rec.start('c');
        rec.record(KeyStroke::char('j'));
        rec.record(KeyStroke::char('j'));
        rec.stop();

        let keys = rec.play('c').unwrap();
        assert_eq!(keys.len(), 2);
    }

    #[test]
    fn test_macro_recorder_replay() {
        let mut rec = MacroRecorder::new();
        rec.start('d');
        rec.record(KeyStroke::char('w'));
        rec.stop();

        rec.play('d');
        let keys = rec.replay().unwrap();
        assert_eq!(keys.len(), 1);
    }

    #[test]
    fn test_macro_recorder_empty_not_stored() {
        let mut rec = MacroRecorder::new();
        rec.start('e');
        rec.stop();
        assert!(rec.get('e').is_none());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Session Persistence Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_session_new() {
        let session = Session::new("test", PathBuf::from("/project"));
        assert_eq!(session.name, "test");
        assert!(session.buffers.is_empty());
    }

    #[test]
    fn test_session_add_buffer() {
        let mut session = Session::new("test", PathBuf::from("/project"));
        session.add_buffer(PathBuf::from("/a.txt"));
        session.add_buffer(PathBuf::from("/b.txt"));
        session.add_buffer(PathBuf::from("/a.txt")); // Duplicate
        assert_eq!(session.buffers.len(), 2);
    }

    #[test]
    fn test_split_direction_default() {
        assert_eq!(SplitDirection::default(), SplitDirection::Horizontal);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Workspace Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_workspace_new() {
        let ws = Workspace::new();
        assert_eq!(ws.folder_count(), 0);
        assert!(!ws.is_multi_root());
    }

    #[test]
    fn test_workspace_single() {
        let ws = Workspace::single(PathBuf::from("/project"));
        assert_eq!(ws.folder_count(), 1);
        assert!(!ws.is_multi_root());
    }

    #[test]
    fn test_workspace_add_folder() {
        let mut ws = Workspace::new();
        ws.add_folder(PathBuf::from("/a"));
        ws.add_folder(PathBuf::from("/b"));
        ws.add_folder(PathBuf::from("/a")); // Duplicate
        assert_eq!(ws.folder_count(), 2);
        assert!(ws.is_multi_root());
    }

    #[test]
    fn test_workspace_remove_folder() {
        let mut ws = Workspace::new();
        ws.add_folder(PathBuf::from("/a"));
        ws.add_folder(PathBuf::from("/b"));

        assert!(ws.remove_folder(Path::new("/a")));
        assert_eq!(ws.folder_count(), 1);
        assert!(!ws.remove_folder(Path::new("/c"))); // Not found
    }

    #[test]
    fn test_workspace_folder_display_name() {
        let folder = WorkspaceFolder::new(PathBuf::from("/home/user/project"));
        assert_eq!(folder.display_name(), "project");

        let named = WorkspaceFolder::new(PathBuf::from("/x")).with_name("Custom");
        assert_eq!(named.display_name(), "Custom");
    }

    #[test]
    fn test_workspace_default() {
        let ws = Workspace::default();
        assert!(ws.folders.is_empty());
    }
}
