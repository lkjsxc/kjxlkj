//! Tests for undo tree.

#[cfg(test)]
mod tests {
    use crate::tree::UndoTree;
    use crate::tree_types::UndoEntry;

    fn make_entry(old: &str, new: &str) -> UndoEntry {
        UndoEntry {
            start: 0,
            old_text: old.into(),
            new_text: new.into(),
            cursor_before: (0, 0),
            cursor_after: (0, new.len()),
        }
    }

    #[test]
    fn basic_undo_redo() {
        let mut tree = UndoTree::new();
        tree.record(make_entry("", "hello"));
        assert_eq!(tree.undo_count(), 1);
        let group = tree.undo().unwrap();
        assert_eq!(group.entries[0].new_text, "hello");
        assert_eq!(tree.undo_count(), 0);
        let group = tree.redo().unwrap();
        assert_eq!(group.entries[0].new_text, "hello");
    }

    #[test]
    fn grouped_undo() {
        let mut tree = UndoTree::new();
        tree.begin_group();
        tree.record(make_entry("", "a"));
        tree.record(make_entry("a", "ab"));
        tree.end_group();
        assert_eq!(tree.undo_count(), 1);
        let group = tree.undo().unwrap();
        assert_eq!(group.entries.len(), 2);
    }

    #[test]
    fn branching() {
        let mut tree = UndoTree::new();
        tree.record(make_entry("", "a"));
        tree.record(make_entry("a", "ab"));
        tree.undo(); // back to "a"
        tree.record(make_entry("a", "ac"));
        assert_eq!(tree.total_nodes(), 3);
    }
}
