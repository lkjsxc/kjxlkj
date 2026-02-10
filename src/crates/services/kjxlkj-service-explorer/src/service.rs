//! Explorer service.

use crate::node::NodeId;
use crate::state::ExplorerState;
use kjxlkj_service_fs::FsService;
use std::path::PathBuf;
use thiserror::Error;
use tracing::info;

/// Explorer service error.
#[derive(Debug, Error)]
pub enum ExplorerError {
    #[error("FS error: {0}")]
    Fs(#[from] kjxlkj_service_fs::FsError),
    #[error("Path not found: {0}")]
    NotFound(PathBuf),
    #[error("Not a directory: {0}")]
    NotDirectory(PathBuf),
}

/// Explorer service.
pub struct ExplorerService {
    fs: FsService,
}

impl Default for ExplorerService {
    fn default() -> Self {
        Self::new()
    }
}

impl ExplorerService {
    /// Create a new explorer service.
    pub fn new() -> Self {
        Self {
            fs: FsService::new(),
        }
    }

    /// Create an explorer state for a root path.
    pub fn create_state(&self, root: PathBuf) -> ExplorerState {
        ExplorerState::new(root)
    }

    /// Refresh the directory children for a node.
    pub async fn refresh_directory(
        &self,
        state: &mut ExplorerState,
        node_id: NodeId,
    ) -> Result<(), ExplorerError> {
        let path = state
            .get_node(node_id)
            .ok_or_else(|| ExplorerError::NotFound(PathBuf::new()))?
            .path
            .clone();

        if !path.is_dir() {
            return Err(ExplorerError::NotDirectory(path));
        }

        info!(?path, "Refreshing directory");

        let entries = self.fs.list_dir(&path).await?;

        // Sort entries: directories first, then files, both alphabetically.
        let mut dirs: Vec<PathBuf> = entries.iter().filter(|p| p.is_dir()).cloned().collect();
        let mut files: Vec<PathBuf> = entries.iter().filter(|p| !p.is_dir()).cloned().collect();
        dirs.sort();
        files.sort();

        // Add children to state.
        for dir in dirs {
            state.add_child(node_id, dir, true);
        }
        for file in files {
            state.add_child(node_id, file, false);
        }

        state.rebuild_visible();
        Ok(())
    }

    /// Refresh the entire tree from root.
    pub async fn refresh_all(&self, state: &mut ExplorerState) -> Result<(), ExplorerError> {
        state.clear_children();

        // Get root node ID.
        let root_id = state.selected_id().or_else(|| {
            // Fall back to first visible node.
            state.visible_nodes().next().map(|n| n.id)
        });

        if let Some(root_id) = root_id {
            self.refresh_directory(state, root_id).await?;
        }

        state.rebuild_visible();
        Ok(())
    }

    /// Toggle expansion of selected directory.
    pub async fn toggle_selected(&self, state: &mut ExplorerState) -> Result<(), ExplorerError> {
        if let Some(node) = state.selected_node() {
            if node.is_dir {
                let node_id = node.id;
                if state.is_expanded(node_id) {
                    state.collapse(node_id);
                } else {
                    state.expand(node_id);
                    // Load children if not already loaded.
                    if let Some(n) = state.get_node(node_id) {
                        if n.children.is_empty() {
                            self.refresh_directory(state, node_id).await?;
                        }
                    }
                }
                state.rebuild_visible();
            }
        }
        Ok(())
    }

    /// Get the path of the selected node.
    pub fn selected_path(&self, state: &ExplorerState) -> Option<PathBuf> {
        state.selected_node().map(|n| n.path.clone())
    }
}
