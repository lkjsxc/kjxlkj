//! Jumplist and changelist navigation for EditorState.

use kjxlkj_core_types::{Action, ContentKind};

use crate::editor::EditorState;
use crate::navlist::Position;

impl EditorState {
    /// Navigate jumplist (Ctrl-o / Ctrl-i).
    pub(crate) fn navigate_jumplist(&mut self, action: &Action) {
        let pos = match action {
            Action::JumpOlder => self.jumplist.go_older(),
            Action::JumpNewer => self.jumplist.go_newer(),
            _ => return,
        };
        if let Some(p) = pos {
            let wid = self.focus.focused;
            if let Some(win) = self.windows.get_mut(&wid) {
                if let ContentKind::Buffer(buf_id) = win.content {
                    if let Some(buf) = self.buffers.get(&buf_id) {
                        let lines = buf.line_count();
                        let line = p.line.min(lines.saturating_sub(1));
                        let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                        win.cursor.line = line;
                        win.cursor.col = p.col.min(cols.saturating_sub(1));
                    }
                }
            }
        }
    }

    /// Navigate changelist (g; / g,).
    pub(crate) fn navigate_changelist(&mut self, action: &Action) {
        let pos = match action {
            Action::ChangeOlder => self.changelist.go_older(),
            Action::ChangeNewer => self.changelist.go_newer(),
            _ => return,
        };
        if let Some(p) = pos {
            let wid = self.focus.focused;
            if let Some(win) = self.windows.get_mut(&wid) {
                if let ContentKind::Buffer(buf_id) = win.content {
                    if let Some(buf) = self.buffers.get(&buf_id) {
                        let lines = buf.line_count();
                        let line = p.line.min(lines.saturating_sub(1));
                        let cols = buf.line(line).map(|l| l.len()).unwrap_or(0);
                        win.cursor.line = line;
                        win.cursor.col = p.col.min(cols.saturating_sub(1));
                    }
                }
            }
        }
    }

    /// Record current cursor position in the jumplist.
    pub(crate) fn record_jump(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.jumplist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }

    /// Record current cursor position in the changelist.
    pub(crate) fn record_change(&mut self) {
        let wid = self.focus.focused;
        if let Some(win) = self.windows.get(&wid) {
            self.changelist.push(Position { line: win.cursor.line, col: win.cursor.col });
        }
    }
}
