//! Explorer tree state: entry types, navigation, expand/collapse.

use std::path::PathBuf;

/// An entry in the explorer tree.
#[derive(Debug, Clone)]
pub struct ExplorerEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub depth: usize,
    pub expanded: bool,
}

/// Explorer state kept in EditorState.
#[derive(Debug, Default)]
pub struct ExplorerState {
    pub entries: Vec<ExplorerEntry>,
    pub selected: usize,
    pub root: PathBuf,
}

impl ExplorerState {
    pub fn new(root: PathBuf) -> Self {
        Self {
            entries: Vec::new(),
            selected: 0,
            root,
        }
    }
    pub fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }
    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    pub fn selected_entry(&self) -> Option<&ExplorerEntry> {
        self.entries.get(self.selected)
    }
    /// Expand directory at selection (l key). Returns file path if file selected.
    pub fn expand_or_open(&mut self) -> Option<PathBuf> {
        let sel = self.selected;
        if let Some(e) = self.entries.get_mut(sel) {
            if e.is_dir {
                if !e.expanded {
                    e.expanded = true;
                    let path = e.path.clone();
                    let depth = e.depth + 1;
                    if let Ok(rd) = std::fs::read_dir(&path) {
                        let mut children = Self::read_sorted(rd, depth);
                        let insert_at = sel + 1;
                        for (i, c) in children.drain(..).enumerate() {
                            self.entries.insert(insert_at + i, c);
                        }
                    }
                }
                None
            } else {
                Some(e.path.clone())
            }
        } else {
            None
        }
    }
    /// Collapse directory at selection or go to parent (h key).
    pub fn collapse_or_parent(&mut self) {
        let sel = self.selected;
        if let Some(e) = self.entries.get(sel) {
            if e.is_dir && e.expanded {
                let depth = e.depth;
                self.entries[sel].expanded = false;
                while sel + 1 < self.entries.len() && self.entries[sel + 1].depth > depth {
                    self.entries.remove(sel + 1);
                }
            } else if e.depth > 0 {
                for i in (0..sel).rev() {
                    if self.entries[i].is_dir && self.entries[i].depth < e.depth {
                        self.selected = i;
                        break;
                    }
                }
            }
        }
    }
    /// Refresh root directory listing.
    pub fn refresh(&mut self) {
        self.entries.clear();
        self.selected = 0;
        if let Ok(rd) = std::fs::read_dir(&self.root) {
            self.entries = Self::read_sorted(rd, 0);
        }
    }
    fn read_sorted(rd: std::fs::ReadDir, depth: usize) -> Vec<ExplorerEntry> {
        let mut v: Vec<ExplorerEntry> = rd
            .filter_map(|e| {
                let e = e.ok()?;
                let is_dir = e.file_type().ok()?.is_dir();
                let name = e.file_name().to_string_lossy().to_string();
                Some(ExplorerEntry {
                    path: e.path(),
                    name,
                    is_dir,
                    depth,
                    expanded: false,
                })
            })
            .collect();
        v.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));
        v
    }
    /// Add a file entry for testing.
    pub fn add_entry(&mut self, path: PathBuf, is_dir: bool) {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        self.entries.push(ExplorerEntry {
            path,
            name,
            is_dir,
            depth: 0,
            expanded: false,
        });
    }
}
