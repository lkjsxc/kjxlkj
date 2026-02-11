//! File explorer service.
//!
//! Manages tree state, filesystem ops, reveal, and refresh.
//! See /docs/spec/features/navigation/file_explorer.md.

/// Explorer tree node.
#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub name: String,
    pub is_dir: bool,
    pub expanded: bool,
    pub children: Vec<ExplorerNode>,
    pub depth: usize,
}

/// Explorer state.
#[derive(Debug, Default)]
pub struct ExplorerState {
    pub root: Option<ExplorerNode>,
    pub selected_index: usize,
}
