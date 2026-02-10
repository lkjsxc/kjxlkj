//! Undo, redo, and register operations.

use kjxlkj_core_edit::insert_char_at;
use kjxlkj_core_undo::UndoEntry;

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn do_undo(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let entry = self
            .undo_trees
            .get_mut(&buf_id)
            .and_then(|t| t.undo().cloned());
        if let Some(entry) = entry {
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                buf.content = ropey::Rope::from_str(&entry.content);
                buf.version.increment();
            }
            let win = self.windows.active_tab_mut().active_mut();
            win.cursor_line = entry.cursor_line;
            win.cursor_offset = entry.cursor_offset;
        }
    }

    pub(crate) fn do_redo(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let entry = self
            .undo_trees
            .get_mut(&buf_id)
            .and_then(|t| t.redo().cloned());
        if let Some(entry) = entry {
            if let Some(buf) = self.buffers.get_mut(&buf_id) {
                buf.content = ropey::Rope::from_str(&entry.content);
                buf.version.increment();
            }
            let win = self.windows.active_tab_mut().active_mut();
            win.cursor_line = entry.cursor_line;
            win.cursor_offset = entry.cursor_offset;
        }
    }

    pub(crate) fn do_put(&mut self, after: bool) {
        let reg_name = self.pending_register.take().unwrap_or('"');
        let text = self.registers.read(reg_name).to_string();
        if text.is_empty() {
            return;
        }
        self.save_undo_checkpoint();
        self.with_active_buffer(|buf, cur| {
            if after {
                cur.grapheme_offset += 1;
            }
            for c in text.chars() {
                insert_char_at(buf, cur, c);
            }
        });
    }

    pub(crate) fn paste_text(&mut self, text: &str) {
        self.save_undo_checkpoint();
        for c in text.chars() {
            self.do_insert_char(c);
        }
    }

    pub(crate) fn save_undo_checkpoint(&mut self) {
        let buf_id = match self.active_buffer_id() {
            Some(id) => id,
            None => return,
        };
        let content = self
            .buffers
            .get(&buf_id)
            .map(|b| b.to_string_content())
            .unwrap_or_default();
        let win = self.windows.active_tab().active();
        let entry = UndoEntry {
            content,
            cursor_line: win.cursor_line,
            cursor_offset: win.cursor_offset,
        };
        if let Some(tree) = self.undo_trees.get_mut(&buf_id) {
            tree.push(entry);
        }
    }
}
