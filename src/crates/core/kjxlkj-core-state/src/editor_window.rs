//! Window management operations: split, close.
//!
//! See /docs/spec/features/window/splits-windows.md.

use kjxlkj_core_types::WindowId;

use crate::editor::EditorState;
use crate::window_state::WindowState;

impl EditorState {
    pub(crate) fn next_id(&mut self) -> u64 {
        let id = self.id_counter;
        self.id_counter += 1;
        id
    }

    pub(crate) fn split_vertical(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        self.layout
            .split_vertical(focused, new_wid, content);
        self.windows.insert(
            new_wid,
            WindowState::new(new_wid, content),
        );
        self.focus.set_focus(new_wid);
    }

    pub(crate) fn split_horizontal(&mut self) {
        let focused = self.focus.focused;
        let win = self.windows.get(&focused).unwrap();
        let content = win.content;
        let new_wid = WindowId(self.next_id());
        self.layout
            .split_horizontal(focused, new_wid, content);
        self.windows.insert(
            new_wid,
            WindowState::new(new_wid, content),
        );
        self.focus.set_focus(new_wid);
    }

    pub(crate) fn close_window(&mut self) {
        let focused = self.focus.focused;
        let ids = self.layout.window_ids();
        if ids.len() <= 1 {
            return;
        }
        if self.layout.close_window(focused) {
            self.windows.remove(&focused);
            let remaining = self.layout.window_ids();
            let fallback = remaining
                .first()
                .copied()
                .unwrap_or(WindowId(0));
            self.focus.on_window_closed(focused, fallback);
        }
    }
}
