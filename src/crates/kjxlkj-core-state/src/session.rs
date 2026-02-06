//! Session persistence: types and stubs for save/restore, swap files, undo files.

use std::path::PathBuf;

/// Describes a saved editor session.
#[derive(Debug, Clone)]
pub struct Session {
    pub name: String, pub working_dir: PathBuf,
    pub buffers: Vec<SessionBuffer>, pub layout: SessionLayout,
}

/// A buffer entry in a saved session.
#[derive(Debug, Clone)]
pub struct SessionBuffer {
    pub file_path: Option<String>, pub cursor_line: usize, pub cursor_col: usize,
}

/// Window layout in a saved session.
#[derive(Debug, Clone)]
pub enum SessionLayout {
    Single,
    Split { direction: SplitDirection, children: Vec<SessionLayout> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection { Horizontal, Vertical }

/// Auto-save configuration.
#[derive(Debug, Clone)]
pub struct AutoSaveConfig {
    pub enabled: bool, pub interval_ms: u64, pub on_focus_lost: bool,
}

impl Default for AutoSaveConfig {
    fn default() -> Self { Self { enabled: false, interval_ms: 30_000, on_focus_lost: true } }
}

/// Swap file metadata for crash recovery.
#[derive(Debug, Clone)]
pub struct SwapFile {
    pub buffer_path: String, pub swap_path: PathBuf,
    pub pid: u32, pub hostname: String, pub modified: bool,
}

impl SwapFile {
    /// Compute swap file path from a buffer file path.
    pub fn path_for(buffer_path: &str, swap_dir: &std::path::Path) -> PathBuf {
        let encoded = buffer_path.replace('/', "%");
        swap_dir.join(format!(".{encoded}.swp"))
    }
}

/// Persistent undo file metadata.
#[derive(Debug, Clone)]
pub struct UndoFile {
    pub buffer_path: String, pub undo_path: PathBuf, pub levels: usize,
}

impl UndoFile {
    /// Compute undo file path from a buffer file path.
    pub fn path_for(buffer_path: &str, undo_dir: &std::path::Path) -> PathBuf {
        let encoded = buffer_path.replace('/', "%");
        undo_dir.join(format!("{encoded}.un~"))
    }
}

/// Recent files tracker.
#[derive(Debug, Clone, Default)]
pub struct RecentFiles { pub entries: Vec<RecentFile>, pub max_entries: usize }

/// A single recent file entry.
#[derive(Debug, Clone)]
pub struct RecentFile { pub path: String, pub line: usize, pub col: usize }

impl RecentFiles {
    pub fn new(max: usize) -> Self { Self { entries: Vec::new(), max_entries: max } }

    pub fn push(&mut self, path: &str, line: usize, col: usize) {
        self.entries.retain(|e| e.path != path);
        self.entries.insert(0, RecentFile { path: path.to_string(), line, col });
        if self.entries.len() > self.max_entries { self.entries.truncate(self.max_entries); }
    }

    pub fn find(&self, path: &str) -> Option<&RecentFile> {
        self.entries.iter().find(|e| e.path == path)
    }
}

/// Workspace definition for multi-root editing.
#[derive(Debug, Clone)]
pub struct Workspace {
    pub name: String, pub folders: Vec<WorkspaceFolder>,
}

/// A folder within a workspace.
#[derive(Debug, Clone)]
pub struct WorkspaceFolder { pub path: PathBuf, pub name: String }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_files_push_and_dedup() {
        let mut rf = RecentFiles::new(3);
        rf.push("/a.txt", 1, 0);
        rf.push("/b.txt", 5, 3);
        rf.push("/a.txt", 10, 2);
        assert_eq!(rf.entries.len(), 2);
        assert_eq!(rf.entries[0].path, "/a.txt");
        assert_eq!(rf.entries[0].line, 10);
    }

    #[test]
    fn recent_files_max_entries() {
        let mut rf = RecentFiles::new(2);
        rf.push("/a.txt", 0, 0); rf.push("/b.txt", 0, 0); rf.push("/c.txt", 0, 0);
        assert_eq!(rf.entries.len(), 2);
        assert_eq!(rf.entries[0].path, "/c.txt");
    }

    #[test]
    fn autosave_default() {
        let cfg = AutoSaveConfig::default();
        assert!(!cfg.enabled);
        assert!(cfg.on_focus_lost);
    }

    #[test]
    fn session_layout_variants() {
        let single = SessionLayout::Single;
        let split = SessionLayout::Split {
            direction: SplitDirection::Horizontal,
            children: vec![SessionLayout::Single, SessionLayout::Single],
        };
        assert!(matches!(single, SessionLayout::Single));
        assert!(matches!(split, SessionLayout::Split { .. }));
    }

    #[test]
    fn swap_file_path() {
        let p = SwapFile::path_for("/home/user/file.rs", std::path::Path::new("/tmp"));
        assert!(p.to_string_lossy().contains(".swp"));
        assert!(p.to_string_lossy().contains("%home%user%file.rs"));
    }

    #[test]
    fn undo_file_path() {
        let p = UndoFile::path_for("/home/user/file.rs", std::path::Path::new("/tmp"));
        assert!(p.to_string_lossy().contains(".un~"));
    }

    #[test]
    fn workspace_creation() {
        let ws = Workspace {
            name: "my-proj".into(),
            folders: vec![WorkspaceFolder { path: "/src".into(), name: "src".into() }],
        };
        assert_eq!(ws.folders.len(), 1);
    }
}
