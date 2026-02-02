//! Workspace support.
//!
//! Manages project directories and workspace configuration.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Workspace root detection result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootMarker {
    /// Git repository.
    Git,
    /// Cargo project.
    Cargo,
    /// Node.js project.
    Node,
    /// Custom marker file.
    Custom(String),
}

/// A workspace definition.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Root directory.
    pub root: PathBuf,
    /// How the root was detected.
    pub marker: RootMarker,
    /// Workspace-local settings.
    pub settings: HashMap<String, String>,
}

impl Workspace {
    /// Creates a new workspace.
    pub fn new(root: PathBuf, marker: RootMarker) -> Self {
        Self {
            root,
            marker,
            settings: HashMap::new(),
        }
    }

    /// Returns the workspace name.
    pub fn name(&self) -> &str {
        self.root
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("workspace")
    }

    /// Returns whether a path is inside this workspace.
    pub fn contains(&self, path: &Path) -> bool {
        path.starts_with(&self.root)
    }

    /// Returns a relative path from workspace root.
    pub fn relative_path(&self, path: &Path) -> Option<PathBuf> {
        path.strip_prefix(&self.root).ok().map(PathBuf::from)
    }

    /// Sets a workspace setting.
    pub fn set_setting(&mut self, key: &str, value: &str) {
        self.settings.insert(key.to_string(), value.to_string());
    }

    /// Gets a workspace setting.
    pub fn get_setting(&self, key: &str) -> Option<&str> {
        self.settings.get(key).map(|s| s.as_str())
    }
}

/// Workspace manager.
#[derive(Debug, Default)]
pub struct WorkspaceManager {
    /// Active workspaces.
    workspaces: Vec<Workspace>,
    /// Current workspace index.
    current: Option<usize>,
}

impl WorkspaceManager {
    /// Creates a new workspace manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a workspace.
    pub fn add(&mut self, workspace: Workspace) -> usize {
        let idx = self.workspaces.len();
        self.workspaces.push(workspace);
        if self.current.is_none() {
            self.current = Some(idx);
        }
        idx
    }

    /// Returns the current workspace.
    pub fn current(&self) -> Option<&Workspace> {
        self.current.and_then(|i| self.workspaces.get(i))
    }

    /// Sets the current workspace.
    pub fn set_current(&mut self, index: usize) -> bool {
        if index < self.workspaces.len() {
            self.current = Some(index);
            true
        } else {
            false
        }
    }

    /// Finds workspace containing a path.
    pub fn find_for_path(&self, path: &Path) -> Option<&Workspace> {
        self.workspaces.iter().find(|w| w.contains(path))
    }

    /// Returns all workspaces.
    pub fn all(&self) -> &[Workspace] {
        &self.workspaces
    }

    /// Returns workspace count.
    pub fn len(&self) -> usize {
        self.workspaces.len()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.workspaces.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_new() {
        let ws = Workspace::new(PathBuf::from("/project"), RootMarker::Git);
        assert_eq!(ws.name(), "project");
    }

    #[test]
    fn test_workspace_contains() {
        let ws = Workspace::new(PathBuf::from("/project"), RootMarker::Git);
        assert!(ws.contains(Path::new("/project/src/main.rs")));
        assert!(!ws.contains(Path::new("/other/file.rs")));
    }

    #[test]
    fn test_workspace_relative_path() {
        let ws = Workspace::new(PathBuf::from("/project"), RootMarker::Git);
        let rel = ws.relative_path(Path::new("/project/src/main.rs"));
        assert_eq!(rel, Some(PathBuf::from("src/main.rs")));
    }

    #[test]
    fn test_workspace_settings() {
        let mut ws = Workspace::new(PathBuf::from("/project"), RootMarker::Cargo);
        ws.set_setting("tabstop", "4");
        assert_eq!(ws.get_setting("tabstop"), Some("4"));
    }

    #[test]
    fn test_workspace_manager_add() {
        let mut mgr = WorkspaceManager::new();
        let ws = Workspace::new(PathBuf::from("/project"), RootMarker::Git);
        mgr.add(ws);
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_workspace_manager_find() {
        let mut mgr = WorkspaceManager::new();
        let ws = Workspace::new(PathBuf::from("/project"), RootMarker::Git);
        mgr.add(ws);
        let found = mgr.find_for_path(Path::new("/project/src/main.rs"));
        assert!(found.is_some());
    }
}
