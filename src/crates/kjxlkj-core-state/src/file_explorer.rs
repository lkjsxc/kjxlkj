//! File explorer state.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// File system entry type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    /// Directory.
    Directory,
    /// Regular file.
    File,
}

/// A file system entry in the explorer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    /// Entry name.
    pub name: String,
    /// Full path.
    pub path: PathBuf,
    /// Entry type.
    pub kind: EntryKind,
    /// Nesting depth.
    pub depth: usize,
    /// Is expanded (for directories).
    pub expanded: bool,
}

impl FileEntry {
    /// Creates a new directory entry.
    pub fn directory(name: impl Into<String>, path: PathBuf, depth: usize) -> Self {
        Self {
            name: name.into(),
            path,
            kind: EntryKind::Directory,
            depth,
            expanded: false,
        }
    }

    /// Creates a new file entry.
    pub fn file(name: impl Into<String>, path: PathBuf, depth: usize) -> Self {
        Self {
            name: name.into(),
            path,
            kind: EntryKind::File,
            depth,
            expanded: false,
        }
    }

    /// Returns true if this is a directory.
    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }
}

/// File explorer state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileExplorerState {
    /// Root directory.
    pub root: Option<PathBuf>,
    /// Visible entries.
    pub entries: Vec<FileEntry>,
    /// Selected index.
    pub selected: usize,
    /// Is explorer open.
    pub open: bool,
    /// Explorer width.
    pub width: u16,
    /// Show hidden files.
    pub show_hidden: bool,
}

impl FileExplorerState {
    /// Creates a new file explorer.
    pub fn new() -> Self {
        Self {
            root: None,
            entries: Vec::new(),
            selected: 0,
            open: false,
            width: 30,
            show_hidden: false,
        }
    }

    /// Opens the explorer at a path.
    pub fn open(&mut self, path: PathBuf) {
        self.root = Some(path.clone());
        self.open = true;
        self.refresh();
    }

    /// Closes the explorer.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Toggles the explorer.
    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else if let Some(root) = self.root.clone() {
            self.open(root);
        } else {
            self.open(PathBuf::from("."));
        }
    }

    /// Refreshes the file list.
    pub fn refresh(&mut self) {
        self.entries.clear();
        if let Some(ref root) = self.root {
            self.load_entries(root.clone(), 0);
        }
    }

    /// Loads entries from a directory.
    fn load_entries(&mut self, path: PathBuf, depth: usize) {
        let Ok(entries) = std::fs::read_dir(&path) else {
            return;
        };

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !self.show_hidden && name.starts_with('.') {
                continue;
            }
            let path = entry.path();
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);

            if is_dir {
                dirs.push(FileEntry::directory(name, path, depth));
            } else {
                files.push(FileEntry::file(name, path, depth));
            }
        }

        // Sort alphabetically, dirs first
        dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for dir in dirs {
            let expanded = dir.expanded;
            let dir_path = dir.path.clone();
            self.entries.push(dir);
            if expanded {
                self.load_entries(dir_path, depth + 1);
            }
        }
        for file in files {
            self.entries.push(file);
        }
    }

    /// Moves selection down.
    pub fn move_down(&mut self) {
        if !self.entries.is_empty() {
            self.selected = (self.selected + 1).min(self.entries.len() - 1);
        }
    }

    /// Moves selection up.
    pub fn move_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// Expands or collapses a directory, or returns file path to open.
    pub fn activate(&mut self) -> Option<PathBuf> {
        if self.selected >= self.entries.len() {
            return None;
        }

        let entry = &self.entries[self.selected];
        if entry.is_dir() {
            let expanded = entry.expanded;
            self.entries[self.selected].expanded = !expanded;
            self.refresh();
            None
        } else {
            Some(entry.path.clone())
        }
    }

    /// Collapses directory or moves to parent.
    pub fn collapse(&mut self) {
        if self.selected >= self.entries.len() {
            return;
        }

        let entry = &self.entries[self.selected];
        if entry.is_dir() && entry.expanded {
            self.entries[self.selected].expanded = false;
            self.refresh();
        } else if entry.depth > 0 {
            // Find parent directory
            for i in (0..self.selected).rev() {
                if self.entries[i].is_dir() && self.entries[i].depth < entry.depth {
                    self.selected = i;
                    break;
                }
            }
        }
    }

    /// Returns the selected entry.
    pub fn selected_entry(&self) -> Option<&FileEntry> {
        self.entries.get(self.selected)
    }

    /// Toggles hidden files.
    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.refresh();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_explorer_new() {
        let explorer = FileExplorerState::new();
        assert!(!explorer.open);
        assert!(explorer.root.is_none());
        assert!(explorer.entries.is_empty());
        assert_eq!(explorer.selected, 0);
        assert_eq!(explorer.width, 30);
        assert!(!explorer.show_hidden);
    }

    #[test]
    fn test_file_explorer_toggle() {
        let mut explorer = FileExplorerState::new();
        explorer.root = Some(PathBuf::from("."));
        
        assert!(!explorer.open);
        explorer.toggle();
        assert!(explorer.open);
        explorer.toggle();
        assert!(!explorer.open);
    }

    #[test]
    fn test_file_explorer_open_close() {
        let mut explorer = FileExplorerState::new();
        explorer.open(PathBuf::from("."));
        
        assert!(explorer.open);
        assert_eq!(explorer.root, Some(PathBuf::from(".")));
        
        explorer.close();
        assert!(!explorer.open);
    }

    #[test]
    fn test_file_explorer_navigation() {
        let mut explorer = FileExplorerState::new();
        explorer.entries.push(FileEntry::directory("dir1", PathBuf::from("dir1"), 0));
        explorer.entries.push(FileEntry::file("file1.rs", PathBuf::from("file1.rs"), 0));
        explorer.entries.push(FileEntry::file("file2.rs", PathBuf::from("file2.rs"), 0));
        
        assert_eq!(explorer.selected, 0);
        
        explorer.move_down();
        assert_eq!(explorer.selected, 1);
        
        explorer.move_down();
        assert_eq!(explorer.selected, 2);
        
        explorer.move_down();
        assert_eq!(explorer.selected, 2);
        
        explorer.move_up();
        assert_eq!(explorer.selected, 1);
        
        explorer.move_up();
        assert_eq!(explorer.selected, 0);
        
        explorer.move_up();
        assert_eq!(explorer.selected, 0);
    }

    #[test]
    fn test_file_entry_is_dir() {
        let dir = FileEntry::directory("test", PathBuf::from("test"), 0);
        let file = FileEntry::file("test.rs", PathBuf::from("test.rs"), 0);
        
        assert!(dir.is_dir());
        assert!(!file.is_dir());
    }

    #[test]
    fn test_file_explorer_toggle_hidden() {
        let mut explorer = FileExplorerState::new();
        assert!(!explorer.show_hidden);
        
        explorer.toggle_hidden();
        assert!(explorer.show_hidden);
        
        explorer.toggle_hidden();
        assert!(!explorer.show_hidden);
    }

    #[test]
    fn test_file_explorer_selected_entry() {
        let mut explorer = FileExplorerState::new();
        assert!(explorer.selected_entry().is_none());
        
        explorer.entries.push(FileEntry::file("test.rs", PathBuf::from("test.rs"), 0));
        let entry = explorer.selected_entry().unwrap();
        assert_eq!(entry.name, "test.rs");
    }
}
