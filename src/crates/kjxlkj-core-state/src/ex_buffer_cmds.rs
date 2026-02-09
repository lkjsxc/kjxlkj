/// Ex buffer commands — write, navigation, window management.
use kjxlkj_core_types::{Action, ContentSource};

use crate::editor::EditorState;
use crate::ex_parse::ExRange;

impl EditorState {
    pub(crate) fn write_current_buffer(&mut self) {
        let buf_id = self.current_buffer_id();
        if let Some(buf) = self.buffers.get_mut(buf_id) {
            if let Some(path) = &buf.path {
                let content = buf.content.to_string();
                let path = path.clone();
                if let Ok(()) = std::fs::write(&path, &content) {
                    buf.mark_saved();
                    self.notify_info(&format!("Written: {}", path.display()));
                } else {
                    self.notify_error(&format!("E212: Failed to write: {}", path.display()));
                }
            } else {
                self.notify_error("E32: No file name");
            }
        }
    }

    pub(crate) fn next_buffer(&mut self) {
        self.buffers.next();
        let buf_id = self.buffers.current_id();
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }

    pub(crate) fn prev_buffer(&mut self) {
        self.buffers.prev();
        let buf_id = self.buffers.current_id();
        self.windows.focused_mut().content = ContentSource::Buffer(buf_id);
    }

    pub(crate) fn split_horizontal(&mut self) {
        let buf_id = self.current_buffer_id();
        self.windows.split_horizontal(buf_id);
    }

    pub(crate) fn split_vertical(&mut self) {
        let buf_id = self.current_buffer_id();
        self.windows.split_vertical(buf_id);
    }

    pub(crate) fn close_window(&mut self) {
        if !self.windows.close_focused() {
            self.handle_action(Action::Quit);
        }
    }

    pub(crate) fn has_unsaved_buffers(&self) -> bool {
        self.buffers.iter().any(|b| b.modified)
    }

    pub(crate) fn delete_range(&mut self, range: ExRange) {
        let buf_id = self.current_buffer_id();
        let cursor = self.windows.focused().cursor;

        if let Some(buf) = self.buffers.get_mut(buf_id) {
            let max_line = buf.content.len_lines().saturating_sub(1);
            let start = range.start.min(max_line);
            let end = (range.end + 1).min(buf.content.len_lines());

            buf.save_undo_checkpoint(cursor);

            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end)
            };

            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
                buf.content.remove(start_char..end_char);
                buf.increment_version();
            }
        }

        self.clamp_cursor();
        self.ensure_cursor_visible();
    }

    pub(crate) fn yank_range(&mut self, range: ExRange) {
        let buf_id = self.current_buffer_id();

        if let Some(buf) = self.buffers.get(buf_id) {
            let max_line = buf.content.len_lines().saturating_sub(1);
            let start = range.start.min(max_line);
            let end = (range.end + 1).min(buf.content.len_lines());

            let start_byte = buf.content.line_to_byte(start);
            let end_byte = if end >= buf.content.len_lines() {
                buf.content.len_bytes()
            } else {
                buf.content.line_to_byte(end)
            };

            if start_byte < end_byte {
                let start_char = buf.content.byte_to_char(start_byte);
                let end_char = buf.content.byte_to_char(end_byte);
                let yanked = buf.content.slice(start_char..end_char).to_string();
                self.registers.set_unnamed(yanked, true);
                let count = range.line_count();
                self.notify_info(&format!("{count} line(s) yanked"));
            }
        }
    }
}
