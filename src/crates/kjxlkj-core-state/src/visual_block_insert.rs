use crate::editor::EditorState;
/// Visual block insert operations (I/A in visual block mode).
use kjxlkj_core_types::{CursorPosition, Mode};

impl EditorState {
    /// Insert text at block start (I) or end (A) of each line in block.
    /// Called after user exits insert mode, with the typed text.
    /// If `count > 1`, the text is repeated count times.
    pub(crate) fn visual_block_insert(
        &mut self,
        text: &str,
        start: CursorPosition,
        end: CursorPosition,
        at_end: bool,
    ) {
        let count = self.op_count.max(1);
        let effective_text = text.repeat(count);
        let col = if at_end {
            start.grapheme.max(end.grapheme) + 1
        } else {
            start.grapheme.min(end.grapheme)
        };
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            buf.save_undo_checkpoint(cursor);
            for line in start.line..=end.line {
                if line < buf.content.len_lines() {
                    let line_start = buf.content.line_to_byte(line);
                    let line_slice = buf.content.line(line);
                    let line_str: String = line_slice.chars().collect();
                    let insert_col = col.min(line_str.trim_end_matches('\n').len());
                    let byte_pos = line_start + insert_col;
                    let char_idx = buf.content.byte_to_char(byte_pos);
                    buf.content.insert(char_idx, &effective_text);
                }
            }
            buf.increment_version();
        }
    }

    /// Handle I or A key in visual block mode: start insert at column.
    pub(crate) fn handle_visual_block_ia(&mut self, at_end: bool) {
        if let Some(anchor) = self.visual_anchor {
            let cursor = self.windows.focused().cursor;
            let col = if at_end {
                anchor.grapheme.max(cursor.grapheme)
            } else {
                anchor.grapheme.min(cursor.grapheme)
            };
            // Store block info for later application.
            let start_line = anchor.line.min(cursor.line);
            let end_line = anchor.line.max(cursor.line);
            self.block_insert_pending = Some((start_line, end_line, col, at_end));
            self.windows.focused_mut().cursor.grapheme = col;
            self.visual_anchor = None;
            self.mode = Mode::Insert;
        }
    }
}
