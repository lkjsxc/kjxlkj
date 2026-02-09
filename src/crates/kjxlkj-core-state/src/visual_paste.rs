//! Visual paste: replace selection with register content.
//! Block mode inserts column-wise.

use kjxlkj_core_types::{CursorPosition, Mode, VisualKind};

use crate::editor::EditorState;

impl EditorState {
    /// Paste in visual mode: delete selection, then put.
    pub(crate) fn visual_paste(&mut self, kind: VisualKind) {
        let anchor = match self.visual_anchor.take() {
            Some(a) => a,
            None => {
                self.mode = Mode::Normal;
                return;
            }
        };
        let cursor = self.windows.focused().cursor;
        let (start, end) = if (anchor.line, anchor.grapheme) <= (cursor.line, cursor.grapheme) {
            (anchor, cursor)
        } else {
            (cursor, anchor)
        };
        // Read register content before deleting.
        let reg = self
            .pending_register
            .take()
            .map(|rn| self.read_special_register(rn))
            .unwrap_or_else(|| self.registers.get_unnamed().cloned());
        let reg = match reg {
            Some(r) => r,
            None => {
                self.mode = Mode::Normal;
                return;
            }
        };
        match kind {
            VisualKind::Block => self.visual_block_paste(start, end, &reg.content),
            _ => {
                let linewise = kind == VisualKind::Line;
                self.apply_visual_op(kjxlkj_core_types::Operator::Delete, start, end, linewise);
                self.insert_text(&reg.content);
            }
        }
        self.mode = Mode::Normal;
        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    /// Block paste: insert each line of register content at the
    /// corresponding row of the block selection.
    fn visual_block_paste(&mut self, start: CursorPosition, end: CursorPosition, content: &str) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
        }
        let col = start.grapheme.min(end.grapheme);
        let lines: Vec<&str> = content.lines().collect();
        for (i, row) in (start.line..=end.line).enumerate() {
            let text = lines.get(i).copied().unwrap_or("");
            let s = CursorPosition::new(row, col);
            let col_end = start.grapheme.max(end.grapheme);
            let e = CursorPosition::new(row, col_end);
            self.delete_range_raw(buf_id, s.line, s.grapheme, e.line, e.grapheme + 1);
            // Insert replacement text at column position.
            if let Some(buf) = self.buffers.get_mut(buf_id) {
                let line_len = buf.content.line(row).chars().count();
                let ins_col = col.min(line_len);
                let byte_off = buf.content.line_to_byte(row) + ins_col;
                let char_idx = buf.content.byte_to_char(byte_off);
                buf.content.insert(char_idx, text);
                buf.increment_version();
            }
        }
        self.windows.focused_mut().cursor = start;
    }
}
