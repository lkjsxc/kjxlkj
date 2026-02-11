//! Explorer navigation actions.
//!
//! See /docs/spec/features/navigation/file_explorer.md — Navigation.

use crate::ExplorerState;

/// Actions that can be performed on the explorer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerAction {
    /// Move selection down (j).
    MoveDown,
    /// Move selection up (k).
    MoveUp,
    /// Collapse directory or move to parent (h).
    CollapseOrParent,
    /// Expand directory or open file (l).
    ExpandOrOpen,
    /// Toggle directory expansion.
    Toggle,
    /// Close the explorer (q).
    Close,
}

impl ExplorerState {
    /// Apply a navigation action to the explorer state.
    /// Returns true if this action requests closing the explorer.
    pub fn apply_action(&mut self, action: ExplorerAction) -> bool {
        match action {
            ExplorerAction::MoveDown => self.move_down(),
            ExplorerAction::MoveUp => self.move_up(),
            ExplorerAction::CollapseOrParent => self.collapse_or_parent(),
            ExplorerAction::ExpandOrOpen => self.expand_or_open(),
            ExplorerAction::Toggle => self.toggle(),
            ExplorerAction::Close => return true,
        }
        false
    }

    fn move_down(&mut self) {
        if self.selected_index + 1 < self.row_count() {
            self.selected_index += 1;
        }
    }

    fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    fn collapse_or_parent(&mut self) {
        if self.cached_rows.is_empty() { return; }
        let row = &self.cached_rows[self.selected_index];
        let nid = row.node_id;
        if row.is_dir && row.expanded {
            // Collapse this directory.
            self.expansion_set.remove(&nid);
            self.rebuild_visible_rows();
        } else {
            // Move to parent.
            if let Some(ref root) = self.root {
                if let Some(parent_id) = root.parent_of(nid) {
                    // Find parent's row index.
                    if let Some(idx) = self.cached_rows.iter()
                        .position(|r| r.node_id == parent_id)
                    {
                        self.selected_index = idx;
                    }
                }
            }
        }
    }

    fn expand_or_open(&mut self) {
        if self.cached_rows.is_empty() { return; }
        let row = &self.cached_rows[self.selected_index];
        let nid = row.node_id;
        if row.is_dir {
            if !row.expanded {
                self.expansion_set.insert(nid);
                self.rebuild_visible_rows();
            }
            // If already expanded, do nothing (spec: open dirs just expand).
        }
        // For files: the caller should handle file-open routing.
    }

    fn toggle(&mut self) {
        if self.cached_rows.is_empty() { return; }
        let row = &self.cached_rows[self.selected_index];
        let nid = row.node_id;
        if row.is_dir {
            if row.expanded {
                self.expansion_set.remove(&nid);
            } else {
                self.expansion_set.insert(nid);
            }
            self.rebuild_visible_rows();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::explorer_tree::{ExplorerNode, NodeId};
    use std::path::PathBuf;

    fn setup() -> ExplorerState {
        let mut st = ExplorerState::new(PathBuf::from("/p"));
        let ids: Vec<NodeId> = (0..4).map(|_| st.alloc_node_id()).collect();
        let root = ExplorerNode {
            id: ids[0], name: "p".into(), is_dir: true, depth: 0,
            path: PathBuf::from("/p"),
            children: vec![
                ExplorerNode {
                    id: ids[1], name: "src".into(), is_dir: true, depth: 1,
                    path: PathBuf::from("/p/src"),
                    children: vec![ExplorerNode::file(
                        ids[2], "main.rs".into(), 2, PathBuf::from("/p/src/main.rs"),
                    )],
                },
                ExplorerNode::file(ids[3], "README.md".into(), 1, PathBuf::from("/p/README.md")),
            ],
        };
        st.set_root(root);
        st.expansion_set.insert(ids[0]);
        st.rebuild_visible_rows();
        st
    }

    #[test]
    fn move_down_up() {
        let mut st = setup();
        assert_eq!(st.selected_index, 0);
        st.apply_action(ExplorerAction::MoveDown);
        assert_eq!(st.selected_index, 1);
        st.apply_action(ExplorerAction::MoveDown);
        assert_eq!(st.selected_index, 2);
        st.apply_action(ExplorerAction::MoveDown); // clamp
        assert_eq!(st.selected_index, 2);
        st.apply_action(ExplorerAction::MoveUp);
        assert_eq!(st.selected_index, 1);
    }

    #[test]
    fn expand_collapse() {
        let mut st = setup();
        // rows: p, src, README.md (root expanded)
        assert_eq!(st.row_count(), 3);
        st.selected_index = 1; // src
        st.apply_action(ExplorerAction::ExpandOrOpen);
        assert_eq!(st.row_count(), 4); // p, src, main.rs, README.md
        st.apply_action(ExplorerAction::CollapseOrParent);
        assert_eq!(st.row_count(), 3); // src collapsed
    }

    #[test]
    fn collapse_moves_to_parent() {
        let mut st = setup();
        st.selected_index = 2; // README.md (file)
        st.apply_action(ExplorerAction::CollapseOrParent);
        assert_eq!(st.selected_index, 0); // moved to root (parent)
    }

    #[test]
    fn toggle_dir() {
        let mut st = setup();
        st.selected_index = 1; // src (collapsed)
        st.apply_action(ExplorerAction::Toggle);
        assert_eq!(st.row_count(), 4); // expanded
        st.apply_action(ExplorerAction::Toggle);
        assert_eq!(st.row_count(), 3); // collapsed again
    }

    #[test]
    fn close_returns_true() {
        let mut st = setup();
        assert!(st.apply_action(ExplorerAction::Close));
    }
}
