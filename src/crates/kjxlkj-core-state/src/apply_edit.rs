//! Edit and command intent application.

use kjxlkj_core_edit::{clamp_cursor, execute_motion, EditOp};
use kjxlkj_core_mode::parse_command;
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{
    Cursor, EditorError, EditorResult, Intent, Mode, RegisterContent, RegisterName,
};
use kjxlkj_core_undo::UndoHistory;
use kjxlkj_core_ui::MessageLevel;

use crate::EditorState;

impl EditorState {
    pub(crate) fn apply_edit_intent(&mut self, intent: Intent) -> EditorResult<()> {
        let window = self.windows.get_mut(&self.active_window)
            .ok_or(EditorError::WindowNotFound(self.active_window))?;
        let buffer = self.buffers.get_mut(&window.buffer_id)
            .ok_or(EditorError::BufferNotFound(window.buffer_id))?;

        match intent {
            Intent::InsertText(text) => {
                kjxlkj_core_edit::insert_text(buffer, &mut window.cursor, &text)?;
            }
            Intent::InsertNewline => {
                let line_ending = buffer.meta().line_ending.as_str();
                if self.mode_state.mode == Mode::Normal {
                    kjxlkj_core_edit::open_line_below(buffer, &mut window.cursor, line_ending)?;
                } else {
                    kjxlkj_core_edit::insert_newline(buffer, &mut window.cursor, line_ending)?;
                }
            }
            Intent::DeleteBackward => {
                kjxlkj_core_edit::delete_backward(buffer, &mut window.cursor)?;
            }
            Intent::DeleteForward => {
                kjxlkj_core_edit::delete_forward(buffer, &window.cursor)?;
            }
            Intent::DeleteChar => {
                let deleted = kjxlkj_core_edit::delete_char(buffer, &window.cursor)?;
                self.registers.set(RegisterName::Unnamed, RegisterContent::char(deleted));
            }
            Intent::DeleteLine => {
                let content = kjxlkj_core_edit::delete_line(buffer, &window.cursor)?;
                self.registers.set(RegisterName::Unnamed, content);
                clamp_cursor(buffer, &mut window.cursor);
            }
            Intent::DeleteRange(_) => {
                if let Some(range) = self.mode_state.visual_range(window.cursor.position) {
                    let deleted = buffer.delete(range)?;
                    self.registers.set(RegisterName::Unnamed, RegisterContent::char(deleted));
                    window.cursor.move_to(range.start.line, range.start.col);
                    clamp_cursor(buffer, &mut window.cursor);
                }
            }
            Intent::YankLine => {
                if let Some(content) = kjxlkj_core_edit::yank_line(buffer, &window.cursor) {
                    self.registers.set(RegisterName::Unnamed, content);
                    self.set_message("1 line yanked", MessageLevel::Info);
                }
            }
            Intent::PasteAfter => {
                if let Some(content) = self.registers.get(RegisterName::Unnamed) {
                    kjxlkj_core_edit::paste_after(buffer, &mut window.cursor, content)?;
                }
            }
            Intent::PasteBefore => {
                if let Some(content) = self.registers.get(RegisterName::Unnamed) {
                    kjxlkj_core_edit::paste_before(buffer, &mut window.cursor, content)?;
                }
            }
            _ => {
                return self.apply_command_intent(intent);
            }
        }
        Ok(())
    }

    pub(crate) fn apply_command_intent(&mut self, intent: Intent) -> EditorResult<()> {
        let window = self.windows.get_mut(&self.active_window)
            .ok_or(EditorError::WindowNotFound(self.active_window))?;
        let buffer = self.buffers.get_mut(&window.buffer_id)
            .ok_or(EditorError::BufferNotFound(window.buffer_id))?;

        match intent {
            Intent::Undo => {
                if let Some(history) = self.undo_histories.get_mut(&window.buffer_id) {
                    if let Some(tx) = history.undo() {
                        for op in tx.ops {
                            match op {
                                EditOp::Insert { pos, text } => { buffer.insert(pos, &text)?; }
                                EditOp::Delete { range, .. } => { buffer.delete(range)?; }
                            }
                        }
                        window.cursor.move_to(tx.cursor_after.line, tx.cursor_after.col);
                    }
                }
            }
            Intent::Redo => {
                if let Some(history) = self.undo_histories.get_mut(&window.buffer_id) {
                    if let Some(tx) = history.redo() {
                        for op in tx.ops {
                            match op {
                                EditOp::Insert { pos, text } => { buffer.insert(pos, &text)?; }
                                EditOp::Delete { range, .. } => { buffer.delete(range)?; }
                            }
                        }
                        window.cursor.move_to(tx.cursor_after.line, tx.cursor_after.col);
                    }
                }
            }
            Intent::ExecuteCommand(cmd) => {
                let parsed = parse_command(&cmd);
                return self.apply_intent(parsed);
            }
            Intent::WriteBuffer { path, force: _ } => {
                let write_path = path.or_else(|| buffer.path().cloned());
                if let Some(p) = write_path {
                    buffer.mark_clean();
                    self.set_message(&format!("Written: {}", p.display()), MessageLevel::Info);
                } else {
                    self.set_message("No file name", MessageLevel::Error);
                }
            }
            Intent::OpenFile(path) => {
                let new_buffer = TextBuffer::from_file(path, "");
                let new_id = new_buffer.id();
                self.buffers.insert(new_id, new_buffer);
                self.undo_histories.insert(new_id, UndoHistory::new());
                window.buffer_id = new_id;
                window.cursor = Cursor::default();
            }
            Intent::CloseBuffer { force } => {
                if !force && buffer.is_modified() {
                    self.set_message("No write since last change", MessageLevel::Error);
                } else {
                    self.should_quit = true;
                }
            }
            Intent::Quit { force } => {
                if !force && buffer.is_modified() {
                    self.set_message("No write since last change (add ! to override)", MessageLevel::Error);
                } else {
                    self.should_quit = true;
                }
            }
            Intent::ClearSelection => { self.mode_state.visual_anchor = None; }
            Intent::ExtendSelection(dir) => { execute_motion(buffer, &mut window.cursor, dir, 1); }
            Intent::CenterCursor => {
                let height = window.viewport.height as u32;
                let cursor_line = window.cursor.line();
                window.viewport.top_line = cursor_line.saturating_sub(height / 2);
            }
            _ => {}
        }
        Ok(())
    }
}
