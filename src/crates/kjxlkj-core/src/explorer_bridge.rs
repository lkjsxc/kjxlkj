//! Explorer–editor integration: wire ExplorerAction into editor core intents.
//!
//! Connects the file explorer's actions (open file, create file, refresh)
//! to editor-state mutations, primarily via `:e {path}` dispatch.

use kjxlkj_service_fs::explorer::ExplorerAction;
use kjxlkj_service_fs::{ExplorerState, TreeNode, DirEntry, EntryKind};
use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Intent;

/// Dispatch an explorer action against editor state.
/// Returns true if the action was handled.
pub fn dispatch_explorer_action(
    editor: &mut EditorState,
    explorer: &mut ExplorerState,
    action: ExplorerAction,
) -> bool {
    match action {
        ExplorerAction::OpenFile(path) => {
            let path_str = path.to_string_lossy().to_string();
            kjxlkj_core_state::dispatch_intent(
                editor,
                Intent::ExCommand(format!(":e {path_str}")),
            );
            true
        }
        ExplorerAction::ToggleExpand(idx) => {
            toggle_expand_at(&mut explorer.tree, idx);
            true
        }
        ExplorerAction::CreateFile(name) => {
            let path_str = if let Some(ref root) = explorer.root {
                root.join(&name).to_string_lossy().to_string()
            } else { name.clone() };
            kjxlkj_core_state::dispatch_intent(
                editor,
                Intent::ExCommand(format!(":e {path_str}")),
            );
            true
        }
        ExplorerAction::Close => { explorer.visible = false; true }
        ExplorerAction::Refresh => true, // refresh is handled by the FS service
        ExplorerAction::SelectUp | ExplorerAction::SelectDown => true,
    }
}

/// Toggle expand/collapse of a tree node at flat index.
fn toggle_expand_at(tree: &mut [TreeNode], target: usize) {
    let mut idx = 0;
    toggle_recursive(tree, target, &mut idx);
}

fn toggle_recursive(nodes: &mut [TreeNode], target: usize, idx: &mut usize) {
    for node in nodes.iter_mut() {
        if *idx == target {
            if matches!(node.entry.kind, EntryKind::Directory) {
                node.expanded = !node.expanded;
            }
            return;
        }
        *idx += 1;
        if node.expanded { toggle_recursive(&mut node.children, target, idx); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kjxlkj_core_types::Size;

    fn make_node(name: &str, kind: EntryKind, expanded: bool) -> TreeNode {
        TreeNode {
            entry: DirEntry { path: name.into(), name: name.into(), kind, size: 0, hidden: false },
            children: Vec::new(), expanded, depth: 0,
        }
    }

    #[test]
    fn open_file_dispatches_edit_command() {
        let mut editor = EditorState::new(Size::new(80, 24));
        let bid = editor.create_buffer();
        editor.create_window(bid);
        let mut explorer = ExplorerState::new();
        let action = ExplorerAction::OpenFile("src/main.rs".into());
        let handled = dispatch_explorer_action(&mut editor, &mut explorer, action);
        assert!(handled);
        // The :e command for a non-existent file creates a new buffer
        assert!(editor.message.is_some());
    }

    #[test]
    fn close_action_hides_explorer() {
        let mut editor = EditorState::new(Size::new(80, 24));
        let bid = editor.create_buffer();
        editor.create_window(bid);
        let mut explorer = ExplorerState::new();
        explorer.visible = true;
        dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::Close);
        assert!(!explorer.visible);
    }

    #[test]
    fn toggle_expand_directory() {
        let mut editor = EditorState::new(Size::new(80, 24));
        let bid = editor.create_buffer();
        editor.create_window(bid);
        let mut explorer = ExplorerState::new();
        let mut dir = make_node("src", EntryKind::Directory, false);
        dir.children.push(make_node("main.rs", EntryKind::File, false));
        explorer.tree = vec![dir];
        dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::ToggleExpand(0));
        assert!(explorer.tree[0].expanded);
        dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::ToggleExpand(0));
        assert!(!explorer.tree[0].expanded);
    }

    #[test]
    fn create_file_with_root() {
        let mut editor = EditorState::new(Size::new(80, 24));
        let bid = editor.create_buffer();
        editor.create_window(bid);
        let mut explorer = ExplorerState::new();
        explorer.root = Some("/project".into());
        let action = ExplorerAction::CreateFile("new.rs".into());
        let handled = dispatch_explorer_action(&mut editor, &mut explorer, action);
        assert!(handled);
        assert!(editor.message.is_some());
    }

    #[test]
    fn select_actions_are_handled() {
        let mut editor = EditorState::new(Size::new(80, 24));
        let bid = editor.create_buffer();
        editor.create_window(bid);
        let mut explorer = ExplorerState::new();
        assert!(dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::SelectUp));
        assert!(dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::SelectDown));
        assert!(dispatch_explorer_action(&mut editor, &mut explorer, ExplorerAction::Refresh));
    }
}
