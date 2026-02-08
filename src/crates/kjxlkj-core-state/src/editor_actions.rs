//! Editor actions: motions, insert mode, visual/command
//! mode entry, and character-level editing.

use kjxlkj_core_edit::{execute_motion, CursorPosition};
use kjxlkj_core_types::{
    ActionCommandKind, CommandKind, InsertPosition, Mode,
    Motion, Operator, VisualKind,
};

use crate::EditorState;

impl EditorState {
    // -- helpers -----------------------------------------------

    /// Read cursor position without borrowing self mutably.
    pub(crate) fn cursor_pos(&self) -> (usize, usize) {
        self.focused_window()
            .map(|w| (w.cursor.line, w.cursor.grapheme_offset))
            .unwrap_or((0, 0))
    }

    // -- motions -----------------------------------------------

    /// Execute a cursor motion.
    pub(crate) fn do_motion(
        &mut self,
        motion: Motion,
        count: u32,
    ) {
        let (line, col) = self.cursor_pos();
        let bid = match self.active_buffer_id() {
            Some(b) => b,
            None => return,
        };
        let new_pos = {
            let buf = match self.buffers.get(&bid) {
                Some(b) => b,
                None => return,
            };
            let mut cursor = CursorPosition::new(line, col);
            execute_motion(
                &mut cursor, &motion, count, &buf.content,
            );
            (cursor.line, cursor.grapheme_offset)
        };
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line = new_pos.0;
            w.cursor.grapheme_offset = new_pos.1;
            w.viewport.follow_cursor(new_pos.0, 3, 0);
        }
    }

    // -- insert mode -------------------------------------------

    /// Enter insert mode at the given position.
    pub(crate) fn enter_insert(
        &mut self,
        pos: InsertPosition,
    ) {
        use InsertPosition::*;
        match pos {
            NewLineBelow => {
                self.open_line_below();
                return;
            }
            NewLineAbove => {
                self.open_line_above();
                return;
            }
            EndOfLine => {
                if let Some(w) = self.focused_window_mut() {
                    w.cursor.grapheme_offset = usize::MAX;
                }
            }
            FirstNonBlank => {
                self.do_motion(Motion::FirstNonBlank, 1);
            }
            AfterCursor => {
                if let Some(w) = self.focused_window_mut() {
                    w.cursor.grapheme_offset += 1;
                }
            }
            _ => {}
        }
        self.mode = Mode::Insert;
        self.insert_state.reset();
    }

    fn open_line_below(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            let end = buf.content.line_end_offset(line);
            buf.content.insert_char(end, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line += 1;
            w.cursor.grapheme_offset = 0;
        }
        self.mode = Mode::Insert;
        self.insert_state.reset();
    }

    fn open_line_above(&mut self) {
        let line = self.cursor_pos().0;
        if let Some(buf) = self.active_buffer_mut() {
            let start = buf.content.line_start_offset(line);
            buf.content.insert_char(start, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset = 0;
        }
        self.mode = Mode::Insert;
        self.insert_state.reset();
    }

    // -- visual / command / op-pending -------------------------

    /// Enter visual mode.
    pub(crate) fn enter_visual(
        &mut self,
        kind: VisualKind,
    ) {
        let anchor = self.cursor_pos();
        self.visual_state = Some(
            kjxlkj_core_mode::VisualModeState::new(
                kind, anchor,
            ),
        );
        self.mode = Mode::Visual(kind);
    }

    /// Enter operator-pending mode.
    pub(crate) fn enter_op_pending(&mut self, op: Operator) {
        self.mode = Mode::OperatorPending(op);
    }

    /// Enter command mode.
    pub(crate) fn enter_command(
        &mut self,
        kind: ActionCommandKind,
    ) {
        let mode_kind = match kind {
            ActionCommandKind::Ex => CommandKind::Ex,
            ActionCommandKind::SearchForward => {
                CommandKind::SearchForward
            }
            ActionCommandKind::SearchBackward => {
                CommandKind::SearchBackward
            }
        };
        self.command_state = Some(
            kjxlkj_core_mode::CommandModeState::new(kind),
        );
        self.mode = Mode::Command(mode_kind);
    }

    // -- character-level insert editing ------------------------

    /// Insert a character in insert mode.
    pub(crate) fn insert_char(&mut self, ch: char) {
        if ch == '\n' {
            self.insert_newline_impl();
            return;
        }
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off =
                buf.content.line_grapheme_to_offset(line, col);
            buf.content.insert_char(off, ch);
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.grapheme_offset += 1;
        }
    }

    fn insert_newline_impl(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off =
                buf.content.line_grapheme_to_offset(line, col);
            buf.content.insert_char(off, '\n');
            buf.modified = true;
        }
        if let Some(w) = self.focused_window_mut() {
            w.cursor.line += 1;
            w.cursor.grapheme_offset = 0;
        }
    }

    /// Backspace in insert mode.
    pub(crate) fn do_backspace(&mut self) {
        let (line, col) = self.cursor_pos();
        if col == 0 && line == 0 {
            return;
        }
        // Need prev-line grapheme count if joining lines.
        let prev_gc = if col == 0 && line > 0 {
            self.active_buffer()
                .map(|b| b.content.line_grapheme_count(line - 1))
                .unwrap_or(0)
        } else {
            0
        };
        if let Some(buf) = self.active_buffer_mut() {
            if col > 0 {
                let off = buf
                    .content
                    .line_grapheme_to_offset(line, col - 1);
                buf.content.delete_range(off, off + 1);
                buf.modified = true;
            } else {
                let off = buf.content.line_start_offset(line);
                if off > 0 {
                    buf.content.delete_range(off - 1, off);
                    buf.modified = true;
                }
            }
        }
        if let Some(w) = self.focused_window_mut() {
            if col > 0 {
                w.cursor.grapheme_offset -= 1;
            } else if line > 0 {
                w.cursor.line -= 1;
                w.cursor.grapheme_offset = prev_gc;
            }
        }
    }

    /// Delete character forward (`x`).
    pub(crate) fn delete_char_forward(&mut self) {
        let (line, col) = self.cursor_pos();
        if let Some(buf) = self.active_buffer_mut() {
            let off =
                buf.content.line_grapheme_to_offset(line, col);
            if off < buf.content.len_chars() {
                buf.content.delete_range(off, off + 1);
                buf.modified = true;
            }
        }
    }
}
