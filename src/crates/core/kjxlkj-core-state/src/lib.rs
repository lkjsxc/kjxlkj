//! Core editor state.
//!
//! This crate provides the main EditorState type.

mod buffer;
mod buffer_list;
mod editor;
mod layout;
mod split;
mod tree;
mod window;
mod word_nav;

pub use buffer::*;
pub use buffer_list::*;
pub use editor::*;
pub use layout::*;
pub use tree::*;
pub use window::*;

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::BufferId;

    #[test]
    fn test_window_tree_add_buffer_window() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        let window_id = tree.add_buffer_window(buffer_id);

        assert_eq!(tree.count(), 1);
        assert!(tree.get(window_id).is_some());
        assert_eq!(tree.focused_id(), Some(window_id));
    }

    #[test]
    fn test_window_tree_split_horizontal() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        let first_win = tree.add_buffer_window(buffer_id);
        let second_win = tree.split_horizontal(buffer_id);

        assert_eq!(tree.count(), 2);
        assert!(tree.get(first_win).is_some());
        assert!(tree.get(second_win).is_some());
        // New window is focused after split.
        assert_eq!(tree.focused_id(), Some(second_win));
    }

    #[test]
    fn test_window_tree_split_vertical() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        let _first = tree.add_buffer_window(buffer_id);
        let second = tree.split_vertical(buffer_id);

        assert_eq!(tree.count(), 2);
        assert_eq!(tree.focused_id(), Some(second));
    }

    #[test]
    fn test_window_tree_close_focused() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        tree.add_buffer_window(buffer_id);
        tree.split_horizontal(buffer_id);

        assert_eq!(tree.count(), 2);
        tree.close_focused();
        assert_eq!(tree.count(), 1);
    }

    #[test]
    fn test_window_tree_close_others() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        tree.add_buffer_window(buffer_id);
        tree.split_horizontal(buffer_id);
        tree.split_vertical(buffer_id);

        assert_eq!(tree.count(), 3);
        tree.close_others();
        assert_eq!(tree.count(), 1);
    }

    #[test]
    fn test_window_tree_focus_next() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        let first = tree.add_buffer_window(buffer_id);
        let second = tree.split_horizontal(buffer_id);

        // Currently on second, focus next should wrap to first.
        tree.focus_next();
        assert_eq!(tree.focused_id(), Some(first));
        // And again to second.
        tree.focus_next();
        assert_eq!(tree.focused_id(), Some(second));
    }

    #[test]
    fn test_window_tree_focus_prev() {
        let mut tree = WindowTree::new();
        let buffer_id = BufferId::new(0);
        let first = tree.add_buffer_window(buffer_id);
        let second = tree.split_horizontal(buffer_id);

        // Currently on second, focus prev goes to first.
        tree.focus_prev();
        assert_eq!(tree.focused_id(), Some(first));
        // Prev again wraps to second.
        tree.focus_prev();
        assert_eq!(tree.focused_id(), Some(second));
    }
}
