//! Persistence for the undo tree (save/load as JSON).
//!
//! Supports saving undo history alongside session data.

use std::path::Path;

use crate::tree::UndoTree;

/// Save an undo tree to a JSON file.
pub fn save_undo_tree(tree: &UndoTree, path: &Path) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(tree)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    std::fs::write(path, json)
}

/// Load an undo tree from a JSON file.
pub fn load_undo_tree(path: &Path) -> std::io::Result<UndoTree> {
    let json = std::fs::read_to_string(path)?;
    let tree: UndoTree = serde_json::from_str(&json)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(tree)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::UndoEntry;

    #[test]
    fn round_trip() {
        let mut tree = UndoTree::new();
        tree.record(UndoEntry {
            start: 0,
            old_text: "".into(),
            new_text: "hello".into(),
            cursor_before: (0, 0),
            cursor_after: (0, 5),
        });

        let dir = std::env::temp_dir();
        let path = dir.join("test_undo.json");
        save_undo_tree(&tree, &path).unwrap();
        let loaded = load_undo_tree(&path).unwrap();
        assert_eq!(loaded.total_nodes(), 1);

        // Cleanup
        let _ = std::fs::remove_file(&path);
    }
}
