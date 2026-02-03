//! Intent application logic.

use kjxlkj_core_edit::{clamp_cursor, clamp_cursor_for_mode, execute_motion};
use kjxlkj_core_types::{EditorError, EditorResult, Intent};

use crate::EditorState;

impl EditorState {
    /// Apply an intent to the editor state.
    pub fn apply_intent(&mut self, intent: Intent) -> EditorResult<()> {
        self.apply_intent_inner(intent)?;

        // Ensure cursor is valid (but allow past-end in insert mode)
        let window = self.windows.get_mut(&self.active_window).unwrap();
        let buffer = self.buffers.get(&window.buffer_id).unwrap();
        clamp_cursor_for_mode(buffer, &mut window.cursor, self.mode_state.mode);

        // Update viewport to follow cursor
        let scrolloff = 3;
        window.viewport.ensure_line_visible(window.cursor.line(), scrolloff);

        Ok(())
    }

    fn apply_intent_inner(&mut self, intent: Intent) -> EditorResult<()> {
        let window = self.windows.get_mut(&self.active_window)
            .ok_or(EditorError::WindowNotFound(self.active_window))?;
        let buffer = self.buffers.get_mut(&window.buffer_id)
            .ok_or(EditorError::BufferNotFound(window.buffer_id))?;

        match intent {
            Intent::CursorMove(dir) => {
                execute_motion(buffer, &mut window.cursor, dir, 1);
            }
            Intent::CursorGoto(pos) => {
                window.cursor.move_to(pos.line, pos.col);
                clamp_cursor(buffer, &mut window.cursor);
            }
            Intent::CursorLineStart => {
                kjxlkj_core_edit::move_to_line_start(&mut window.cursor);
            }
            Intent::CursorFirstNonBlank => {
                kjxlkj_core_edit::move_to_first_non_blank(buffer, &mut window.cursor);
            }
            Intent::CursorLineEnd => {
                kjxlkj_core_edit::move_to_line_end(buffer, &mut window.cursor);
            }
            Intent::CursorFileStart => {
                kjxlkj_core_edit::move_to_file_start(&mut window.cursor);
            }
            Intent::CursorFileEnd => {
                kjxlkj_core_edit::move_to_file_end(buffer, &mut window.cursor);
            }
            Intent::CursorGotoLine(line) => {
                kjxlkj_core_edit::move_to_line(buffer, &mut window.cursor, line);
            }
            Intent::EnterMode(mode) => {
                self.mode_state.enter_mode(mode);
                if mode.is_visual() {
                    let pos = self.windows.get(&self.active_window).unwrap().cursor.position;
                    self.mode_state.visual_anchor = Some(pos);
                }
            }
            Intent::ExitToNormal => {
                self.mode_state.exit_to_normal();
            }
            _ => {
                return self.apply_edit_intent(intent);
            }
        }
        Ok(())
    }
}

