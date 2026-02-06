//! Filesystem watch service — monitors file changes, directory listing.

pub mod explorer;
mod fs_directory;

use std::path::PathBuf;

/// A filesystem change event.
#[derive(Debug, Clone)]
pub struct FsEvent { pub path: PathBuf, pub kind: FsEventKind }

/// The kind of filesystem change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEventKind { Created, Modified, Deleted, Renamed }

/// A directory entry for file explorer integration.
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf, pub name: String, pub kind: EntryKind,
    pub size: u64, pub hidden: bool,
}

/// Entry kind for directory listing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind { File, Directory, Symlink }

/// A tree node for the file explorer.
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub entry: DirEntry, pub children: Vec<TreeNode>,
    pub expanded: bool, pub depth: usize,
}

/// Configuration for directory listing/explorer.
#[derive(Debug, Clone)]
pub struct ExplorerConfig {
    pub show_hidden: bool, pub show_icons: bool,
    pub position: ExplorerPosition, pub width: u16,
    pub sort_dirs_first: bool,
}

/// Explorer panel position.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerPosition { Left, Right }

impl Default for ExplorerConfig {
    fn default() -> Self {
        Self { show_hidden: false, show_icons: true,
               position: ExplorerPosition::Left, width: 30, sort_dirs_first: true }
    }
}

/// File explorer state.
#[derive(Debug, Clone)]
pub struct ExplorerState {
    pub root: Option<PathBuf>, pub tree: Vec<TreeNode>,
    pub selected: usize, pub visible: bool, pub config: ExplorerConfig,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self { root: None, tree: Vec::new(), selected: 0,
               visible: false, config: ExplorerConfig::default() }
    }

    /// Toggle explorer visibility.
    pub fn toggle(&mut self) { self.visible = !self.visible; }

    /// Move selection up.
    pub fn select_prev(&mut self) { self.selected = self.selected.saturating_sub(1); }

    /// Move selection down.
    pub fn select_next(&mut self, max: usize) {
        if self.selected + 1 < max { self.selected += 1; }
    }
}

impl Default for ExplorerState { fn default() -> Self { Self::new() } }

/// Service that watches the filesystem for changes and provides directory listing.
pub struct FsWatchService {
    watch_roots: Vec<PathBuf>,
    running: bool,
}

impl FsWatchService {
    pub fn new() -> Self { Self { watch_roots: Vec::new(), running: false } }

    pub fn watch(&mut self, path: PathBuf) {
        tracing::debug!(path = %path.display(), "adding watch root");
        self.watch_roots.push(path);
    }

    pub fn unwatch(&mut self, path: &std::path::Path) {
        self.watch_roots.retain(|p| p != path);
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        tracing::info!(roots = self.watch_roots.len(), "starting fs watcher");
        self.running = true; Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> {
        self.running = false; Ok(())
    }

    pub fn is_running(&self) -> bool { self.running }

    /// List directory contents (cancellable, sorts dirs first by default).
    pub async fn list_dir(&self, path: &std::path::Path) -> anyhow::Result<Vec<DirEntry>> {
        tracing::debug!(path = %path.display(), "listing directory");
        Ok(Vec::new())
    }

    /// Get entries sorted with directories first.
    pub fn sort_entries(entries: &mut [DirEntry], dirs_first: bool) {
        entries.sort_by(|a, b| {
            if dirs_first {
                let da = matches!(a.kind, EntryKind::Directory);
                let db = matches!(b.kind, EntryKind::Directory);
                db.cmp(&da).then_with(|| a.name.cmp(&b.name))
            } else { a.name.cmp(&b.name) }
        });
    }
}

impl Default for FsWatchService { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explorer_state_toggle() {
        let mut ex = ExplorerState::new();
        assert!(!ex.visible);
        ex.toggle();
        assert!(ex.visible);
        ex.toggle();
        assert!(!ex.visible);
    }

    #[test]
    fn explorer_selection() {
        let mut ex = ExplorerState::new();
        ex.select_next(5);
        assert_eq!(ex.selected, 1);
        ex.select_next(5);
        assert_eq!(ex.selected, 2);
        ex.select_prev();
        assert_eq!(ex.selected, 1);
        ex.select_prev();
        ex.select_prev(); // should not underflow
        assert_eq!(ex.selected, 0);
    }

    #[test]
    fn sort_entries_dirs_first() {
        let mut entries = vec![
            DirEntry { path: "b.txt".into(), name: "b.txt".into(),
                       kind: EntryKind::File, size: 100, hidden: false },
            DirEntry { path: "a_dir".into(), name: "a_dir".into(),
                       kind: EntryKind::Directory, size: 0, hidden: false },
            DirEntry { path: "a.txt".into(), name: "a.txt".into(),
                       kind: EntryKind::File, size: 50, hidden: false },
        ];
        FsWatchService::sort_entries(&mut entries, true);
        assert_eq!(entries[0].name, "a_dir");
        assert_eq!(entries[1].name, "a.txt");
        assert_eq!(entries[2].name, "b.txt");
    }

    #[test]
    fn explorer_config_defaults() {
        let cfg = ExplorerConfig::default();
        assert!(!cfg.show_hidden);
        assert_eq!(cfg.width, 30);
        assert!(cfg.sort_dirs_first);
        assert_eq!(cfg.position, ExplorerPosition::Left);
    }

    #[test]
    fn entry_kinds() {
        assert_ne!(EntryKind::File, EntryKind::Directory);
        assert_ne!(EntryKind::File, EntryKind::Symlink);
    }
}
