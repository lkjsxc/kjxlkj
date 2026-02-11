//! Explorer key routing for windows with ContentKind::Explorer.
//!
//! See /docs/spec/features/navigation/file_explorer.md — Navigation.

use kjxlkj_core_types::{ContentKind, ExplorerStateId, Key, KeyModifiers};
use kjxlkj_service_explorer::ExplorerAction;

use crate::editor::EditorState;

impl EditorState {
    /// Check if the focused window is an explorer window.
    pub(crate) fn focused_explorer_id(&self) -> Option<ExplorerStateId> {
        let ws = self.windows.get(&self.focus.focused)?;
        match ws.content {
            ContentKind::Explorer(eid) => Some(eid),
            _ => None,
        }
    }

    /// Handle a key event when focused on an explorer window.
    /// Returns true if the key was consumed by the explorer.
    pub(crate) fn handle_explorer_key(
        &mut self, key: &Key, _mods: &KeyModifiers
    ) -> bool {
        let eid = match self.focused_explorer_id() {
            Some(id) => id,
            None => return false,
        };
        let action = match key {
            Key::Char('j') => ExplorerAction::MoveDown,
            Key::Char('k') => ExplorerAction::MoveUp,
            Key::Char('h') => ExplorerAction::CollapseOrParent,
            Key::Char('l') | Key::Enter => ExplorerAction::ExpandOrOpen,
            Key::Char('o') => ExplorerAction::ExpandOrOpen,
            Key::Char('q') => ExplorerAction::Close,
            _ => return false,
        };
        let should_close = {
            let estate = match self.explorer_states.get_mut(&eid) {
                Some(s) => s,
                None => return false,
            };
            estate.apply_action(action)
        };
        if should_close {
            self.close_explorer();
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_service_explorer::{ExplorerAction, ExplorerNode, ExplorerState, NodeId};
    use std::path::PathBuf;

    fn ed_with_explorer() -> EditorState {
        let mut s = EditorState::new(80, 24);
        s.open_explorer();
        // Set up a synthetic tree in the explorer.
        let eid = s.focused_explorer_id().unwrap();
        let estate = s.explorer_states.get_mut(&eid).unwrap();
        let ids: Vec<NodeId> = (0..4).map(|_| estate.alloc_node_id()).collect();
        let mut src = ExplorerNode::dir(ids[1], "src".into(), 1, PathBuf::from("/proj/src"));
        src.children.push(ExplorerNode::file(ids[3], "main.rs".into(), 2, PathBuf::from("/proj/src/main.rs")));
        let root = ExplorerNode {
            id: ids[0], name: "proj".into(), is_dir: true, depth: 0,
            path: PathBuf::from("/proj"),
            children: vec![
                src,
                ExplorerNode::file(ids[2], "README.md".into(), 1, PathBuf::from("/proj/README.md")),
            ],
        };
        estate.set_root(root);
        estate.expansion_set.insert(ids[0]);
        estate.rebuild_visible_rows();
        s
    }

    fn m() -> KeyModifiers { KeyModifiers::default() }

    #[test]
    fn explorer_j_moves_down() {
        let mut s = ed_with_explorer();
        let eid = s.focused_explorer_id().unwrap();
        assert_eq!(s.explorer_states[&eid].selected_index, 0);
        s.handle_explorer_key(&Key::Char('j'), &m());
        assert_eq!(s.explorer_states[&eid].selected_index, 1);
    }

    #[test]
    fn explorer_k_moves_up() {
        let mut s = ed_with_explorer();
        let eid = s.focused_explorer_id().unwrap();
        s.handle_explorer_key(&Key::Char('j'), &m());
        s.handle_explorer_key(&Key::Char('k'), &m());
        assert_eq!(s.explorer_states[&eid].selected_index, 0);
    }

    #[test]
    fn explorer_l_expands() {
        let mut s = ed_with_explorer();
        let eid = s.focused_explorer_id().unwrap();
        s.handle_explorer_key(&Key::Char('j'), &m()); // select "src"
        s.handle_explorer_key(&Key::Char('l'), &m()); // expand
        assert_eq!(s.explorer_states[&eid].row_count(), 4); // proj, src, ?, README
    }

    #[test]
    fn explorer_q_closes() {
        let mut s = ed_with_explorer();
        assert_eq!(s.layout.window_ids().len(), 2);
        s.handle_explorer_key(&Key::Char('q'), &m());
        assert_eq!(s.layout.window_ids().len(), 1);
    }

    #[test]
    fn non_explorer_returns_false() {
        let mut s = EditorState::new(80, 24);
        assert!(!s.handle_explorer_key(&Key::Char('j'), &m()));
    }
}
