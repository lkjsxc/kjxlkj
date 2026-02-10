//! File explorer service.
//!
//! Provides file tree navigation and state management.

mod node;
mod service;
mod state;

pub use node::{ClipboardState, NodeId, TreeNode};
pub use service::{ExplorerError, ExplorerService};
pub use state::ExplorerState;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_explorer_state_creation() {
        let root = PathBuf::from("/tmp/test");
        let state = ExplorerState::new(root.clone());
        assert_eq!(state.root, root);
        assert_eq!(state.visible_count(), 1); // Root node.
    }

    #[test]
    fn test_explorer_state_navigation() {
        let root = PathBuf::from("/tmp");
        let mut state = ExplorerState::new(root);

        // Initially at index 0.
        assert_eq!(state.selected_index(), 0);

        // Add some children to root.
        let root_id = state.selected_id().unwrap();
        state.add_child(root_id, PathBuf::from("/tmp/a"), false);
        state.add_child(root_id, PathBuf::from("/tmp/b"), false);
        state.rebuild_visible();

        // Now we have 3 visible nodes.
        assert_eq!(state.visible_count(), 3);

        // Move down.
        state.move_down();
        assert_eq!(state.selected_index(), 1);

        state.move_down();
        assert_eq!(state.selected_index(), 2);

        // Can't go past end.
        state.move_down();
        assert_eq!(state.selected_index(), 2);

        // Move up.
        state.move_up();
        assert_eq!(state.selected_index(), 1);
    }

    #[test]
    fn test_explorer_state_expand_collapse() {
        let root = PathBuf::from("/tmp");
        let mut state = ExplorerState::new(root);

        let root_id = state.selected_id().unwrap();

        // Root is initially expanded.
        assert!(state.is_expanded(root_id));

        // Add a subdirectory.
        let sub_id = state.add_child(root_id, PathBuf::from("/tmp/sub"), true);
        state.rebuild_visible();

        // Subdirectory is not expanded.
        assert!(!state.is_expanded(sub_id));

        // Expand it.
        state.expand(sub_id);
        assert!(state.is_expanded(sub_id));

        // Toggle to collapse.
        state.toggle_expand(sub_id);
        assert!(!state.is_expanded(sub_id));
    }

    #[test]
    fn test_explorer_service_creation() {
        let service = ExplorerService::new();
        let state = service.create_state(PathBuf::from("/tmp"));
        assert_eq!(state.visible_count(), 1);
    }
}
