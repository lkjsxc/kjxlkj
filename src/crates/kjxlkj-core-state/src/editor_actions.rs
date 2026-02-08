//! Editor actions: motions, insert mode, visual/command
//! mode entry.

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
            .map(|w| {
                (w.cursor.line, w.cursor.grapheme_offset)
            })
            .unwrap_or((0, 0))
    }

    // -- motions -----------------------------------------------

    /// Execute a cursor motion.
    pub(crate) fn do_motion(
        &mut self,
        motion: Motion,
        count: u32,
    ) {
        // Handle state-level motions.
        match &motion {
            Motion::JumpListBackward => {
                for _ in 0..count {
                    self.do_jump_older();
                }
                return;
            }
            Motion::JumpListForward => {
                for _ in 0..count {
                    self.do_jump_newer();
                }
                return;
            }
            _ => {}
        }
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
            let mut cursor =
                CursorPosition::new(line, col);
            execute_motion(
                &mut cursor, &motion, count,
                &buf.content,
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
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.cursor.grapheme_offset =
                        usize::MAX;
                }
            }
            FirstNonBlank => {
                self.do_motion(
                    Motion::FirstNonBlank,
                    1,
                );
            }
            AfterCursor => {
                if let Some(w) =
                    self.focused_window_mut()
                {
                    w.cursor.grapheme_offset += 1;
                }
            }
            _ => {}
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
    pub(crate) fn enter_op_pending(
        &mut self,
        op: Operator,
    ) {
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
            kjxlkj_core_mode::CommandModeState::new(
                kind,
            ),
        );
        self.mode = Mode::Command(mode_kind);
    }
}
