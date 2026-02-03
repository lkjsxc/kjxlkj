use kjxlkj_core_edit::Edit;
use kjxlkj_core_types::CursorPos;

use super::EditorState;

impl EditorState {
    pub(super) fn open_line_below(&mut self) {
        let Some((buf_idx, win_idx)) = self.active_indices() else {
            return;
        };
        let buf = &mut self.buffers[buf_idx];
        let win = &mut self.windows[win_idx];
        win.cursor = buf.text.clamp_cursor(win.cursor);
        let line = win.cursor.line;

        let insert_at = buf.text.line_char_range(line).map(|r| r.end).unwrap_or_else(|| buf.text.len_chars());
        if buf.text.insert(insert_at, "\n").is_err() {
            return;
        }
        buf.modified = true;
        buf.version = buf.version.next();
        self.undo.push_transaction(vec![Edit::Insert { at: insert_at, text: "\n".to_string() }]);

        win.cursor = CursorPos {
            line: line.saturating_add(1),
            col: 0,
        };
        win.cursor = buf.text.clamp_cursor(win.cursor);
    }
}

