//! Session save and load operations on EditorState.

use kjxlkj_core_ui::WindowContent;

use crate::editor::EditorState;
use crate::session::{BufferRef, SessionData, SessionLayoutNode, SessionTab, WindowRef};
use crate::window_tree::Window;

impl EditorState {
    /// Build a SessionData snapshot from the current editor state.
    pub fn session_save(&self) -> SessionData {
        let cwd = std::env::current_dir().unwrap_or_default();
        let mut session = SessionData::new(cwd);

        // Buffers
        for (_id, buf) in self.buffers.iter() {
            if let Some(path) = &buf.path {
                session.buffers.push(BufferRef {
                    path: path.to_string_lossy().to_string(),
                    encoding: "utf-8".to_string(),
                    modified: buf.modified,
                });
            }
        }

        // Tabs
        for (tab_idx, tab) in self.windows.tabs.iter().enumerate() {
            let layout = self.serialize_layout_windows(tab);
            session.tabs.push(SessionTab {
                layout,
                focused_window: tab.active_window,
            });
            if tab_idx == self.windows.active_tab {
                session.active_tab = tab_idx;
            }
        }

        session
    }

    /// Serialize windows of a tab into a session layout tree.
    fn serialize_layout_windows(&self, tab: &crate::window_tree::TabPage) -> SessionLayoutNode {
        if tab.windows.len() == 1 {
            self.window_to_layout_node(&tab.windows[0])
        } else {
            let children: Vec<SessionLayoutNode> = tab
                .windows
                .iter()
                .map(|w| self.window_to_layout_node(w))
                .collect();
            let weights = vec![1.0; children.len()];
            SessionLayoutNode::hsplit(children, weights)
        }
    }

    fn window_to_layout_node(&self, w: &Window) -> SessionLayoutNode {
        let wref = match &w.content {
            WindowContent::Buffer(bid) => {
                let path = self
                    .buffers
                    .get(bid)
                    .and_then(|b| b.path.as_ref())
                    .map(|p| p.to_string_lossy().to_string());
                WindowRef::buffer(
                    path,
                    (w.cursor_line, w.cursor_offset),
                    (w.top_line, w.left_col),
                    w.wrap,
                )
            }
            WindowContent::Terminal(_) => WindowRef::terminal(),
            WindowContent::Explorer => WindowRef {
                content_type: "explorer".to_string(),
                buffer_path: None,
                cursor_line: 0,
                cursor_grapheme: 0,
                top_line: 0,
                left_col: 0,
                wrap: false,
            },
        };
        SessionLayoutNode::leaf(wref)
    }

    /// Load a session, restoring layout and buffers.
    pub fn session_load(&mut self, session: &SessionData) {
        // Open buffers
        for bref in &session.buffers {
            let path = std::path::PathBuf::from(&bref.path);
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            self.open_file(path, content);
        }
        // Restore active tab
        if session.active_tab < self.windows.tabs.len() {
            self.windows.active_tab = session.active_tab;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_roundtrip_json() {
        let state = EditorState::new(80, 24);
        let session = state.session_save();
        let json = session.to_json().expect("serialize");
        let loaded = SessionData::from_json(&json).expect("deserialize");
        assert_eq!(loaded.version, 1);
        assert!(!loaded.tabs.is_empty());
    }

    #[test]
    fn session_terminal_persisted_as_node() {
        let mut state = EditorState::new(80, 24);
        state.do_terminal_open();
        let session = state.session_save();
        let json = session.to_json().expect("serialize");
        assert!(json.contains("terminal"), "terminal window node in session");
    }

    #[test]
    fn session_explorer_persisted_as_node() {
        let mut state = EditorState::new(80, 24);
        state.do_explorer_toggle();
        let session = state.session_save();
        let json = session.to_json().expect("serialize");
        assert!(json.contains("explorer"), "explorer window node in session");
    }
}
