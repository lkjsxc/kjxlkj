/// Delete, undo/redo, and join-lines operations.
use kjxlkj_core_text::RopeExt;
use kjxlkj_core_types::{CursorPosition, Mode};

use crate::editor::EditorState;

impl EditorState {
    pub(crate) fn delete_char_backward(&mut self) {
        let cursor = self.windows.focused().cursor;
        if cursor.grapheme == 0 && cursor.line == 0 {
            return;
        }
        let buf_id = self.current_buffer_id();
        if cursor.grapheme > 0 {
            if let Some(buf) = self.buffers.get_mut(buf_id) {
                buf.content.delete_grapheme_range(
                    cursor.line,
                    cursor.grapheme - 1,
                    cursor.line,
                    cursor.grapheme,
                );
                buf.increment_version();
            }
            self.windows.focused_mut().cursor.grapheme -= 1;
        } else {
            // Join with previous line
            if let Some(buf) = self.buffers.get_mut(buf_id) {
                let prev_line = cursor.line - 1;
                let prev_line_g = buf.content.line_grapheme_count(prev_line);
                let prev_len = prev_line_g.saturating_sub(1);
                let line_start = buf.content.line_to_byte(cursor.line);
                let newline_start = line_start - 1;
                let start_char = buf.content.byte_to_char(newline_start);
                let end_char = buf.content.byte_to_char(line_start);
                buf.content.remove(start_char..end_char);
                buf.increment_version();
                self.windows.focused_mut().cursor = CursorPosition::new(prev_line, prev_len);
            }
        }
    }

    pub(crate) fn delete_char_forward(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let max_g = buf.content.line_grapheme_count(cursor.line);
            if cursor.grapheme < max_g {
                if !matches!(self.mode, Mode::Insert) {
                    buf.save_undo_checkpoint(cursor);
                }
                buf.content.delete_grapheme_range(
                    cursor.line,
                    cursor.grapheme,
                    cursor.line,
                    cursor.grapheme + 1,
                );
                buf.increment_version();
            }
        }
        self.clamp_cursor();
    }

    pub(crate) fn undo(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let current_version = buf.version;
            let current_content = buf.content.clone();
            if let Some(entry) = buf.undo_tree.pop_undo() {
                buf.undo_tree.redo(
                    current_version,
                    current_content,
                    cursor.line,
                    cursor.grapheme,
                );
                buf.content = entry.content_before;
                buf.version = entry.version_before;
                self.windows.focused_mut().cursor =
                    CursorPosition::new(entry.cursor_line, entry.cursor_grapheme);
            }
        }
    }

    pub(crate) fn redo(&mut self) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let current_version = buf.version;
            let current_content = buf.content.clone();
            if let Some(entry) = buf.undo_tree.redo(
                current_version,
                current_content,
                cursor.line,
                cursor.grapheme,
            ) {
                buf.content = entry.content_before;
                buf.version = entry.version_before;
                self.windows.focused_mut().cursor =
                    CursorPosition::new(entry.cursor_line, entry.cursor_grapheme);
            }
        }
    }

    pub(crate) fn join_lines(&mut self, with_space: bool) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            if cursor.line + 1 < buf.content.len_lines() {
                buf.save_undo_checkpoint(cursor);
                let next_line_start = buf.content.line_to_byte(cursor.line + 1);
                let current_line_end = next_line_start - 1;
                let start_char = buf.content.byte_to_char(current_line_end);
                let end_char = buf.content.byte_to_char(next_line_start);
                buf.content.remove(start_char..end_char);
                if with_space {
                    buf.content.insert(start_char, " ");
                }
                buf.increment_version();
            }
        }
    }
}
