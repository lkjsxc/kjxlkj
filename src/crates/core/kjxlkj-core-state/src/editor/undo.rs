use kjxlkj_core_edit::Edit;

use super::{BufferState, EditorState};

impl EditorState {
    pub(super) fn undo(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let Some(tx) = self.undo.pop_undo() else {
            return;
        };
        for edit in tx.iter().rev() {
            let Some(inv) = edit.inverse() else {
                continue;
            };
            apply_edit(buf, &inv);
        }
        buf.modified = buf.text.to_string() != buf.saved_text;
        buf.version = buf.version.next();
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }

    pub(super) fn redo(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        let Some(tx) = self.undo.pop_redo() else {
            return;
        };
        for edit in &tx {
            apply_edit(buf, edit);
        }
        buf.modified = buf.text.to_string() != buf.saved_text;
        buf.version = buf.version.next();
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }
}

fn apply_edit(buf: &mut BufferState, edit: &Edit) {
    match edit {
        Edit::Insert { at, text } => {
            let _ = buf.text.insert(*at, text);
        }
        Edit::Delete { range, .. } => {
            let _ = buf.text.remove(*range);
        }
    }
}
