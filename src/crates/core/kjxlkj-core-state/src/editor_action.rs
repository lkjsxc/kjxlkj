//! Action dispatch for EditorState.
//!
//! Extracted from editor.rs to keep each file ≤ 200 lines.

use kjxlkj_core_edit::apply_motion;
use kjxlkj_core_types::{Action, ContentKind, Mode, Motion};

use crate::editor::EditorState;
use crate::search::SearchDirection;

impl EditorState {
    /// Apply a typed action to editor state.
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::InsertChar(c) => {
                self.insert_text.push(c);
                self.insert_char(c);
            }
            Action::DeleteCharForward => {
                self.delete_char_forward()
            }
            Action::DeleteCharBackward => {
                self.delete_char_backward()
            }
            Action::Motion(ref motion) => {
                match motion {
                    Motion::SearchNext => {
                        let d = self.search.direction;
                        self.jump_to_match(d);
                    }
                    Motion::SearchPrev => {
                        let d = match self.search.direction {
                            SearchDirection::Forward => SearchDirection::Backward,
                            SearchDirection::Backward => SearchDirection::Forward,
                        };
                        self.jump_to_match(d);
                    }
                    _ => self.apply_cursor_motion(motion),
                }
            }
            Action::Quit => self.quit_requested = true,
            Action::ForceQuit => {
                self.quit_requested = true
            }
            Action::WriteQuit => {
                self.quit_requested = true
            }
            Action::Resize(cols, rows) => {
                self.terminal_size = (cols, rows);
            }
            Action::AppendEndOfLine => {
                self.cursor_to_eol();
            }
            Action::InsertFirstNonBlank => {
                self.cursor_to_first_nonblank();
            }
            Action::OpenLineBelow => {
                self.open_line_below();
            }
            Action::OpenLineAbove => {
                self.open_line_above();
            }
            Action::SplitVertical => {
                self.split_vertical();
            }
            Action::SplitHorizontal => {
                self.split_horizontal();
            }
            Action::CloseWindow => {
                self.close_window();
            }
            Action::ExitToNormal => {
                self.mode = Mode::Normal;
                let win = self.focused_window_mut();
                if win.cursor.col > 0 {
                    win.cursor.col -= 1;
                }
            }
            Action::OperatorLine(op) => {
                self.apply_operator_line(op);
            }
            Action::OperatorMotion(op, motion, count) => {
                self.apply_operator_motion(
                    op, motion, count,
                );
            }
            Action::SubstituteChar => {
                self.delete_char_forward();
            }
            Action::SubstituteLine => {
                self.delete_current_line_content();
            }
            Action::ChangeToEnd => {
                self.delete_to_eol();
            }
            Action::DeleteWordBackward => {
                self.delete_word_backward();
            }
            Action::DeleteToLineStart => {
                self.delete_to_line_start();
            }
            Action::DeleteToEnd => {
                self.delete_to_eol();
            }
            Action::JoinLinesNoSpace => {
                self.join_lines_no_space();
            }
            Action::PutAfter => {
                self.put_after();
            }
            Action::PutBefore => {
                self.put_before();
            }
            Action::ShowRegisters => {
                // Stub: register display would feed to UI.
            }
            _ => {}
        }
    }

    fn apply_cursor_motion(&mut self, motion: &Motion) {
        let wid = self.focus.focused;
        let win = self.windows.get(&wid).unwrap();
        if let ContentKind::Buffer(buf_id) = win.content {
            if let Some(buf) =
                self.buffers.get(&buf_id)
            {
                let cur = win.cursor;
                let new_cur =
                    apply_motion(&cur, motion, buf);
                self.windows
                    .get_mut(&wid)
                    .unwrap()
                    .cursor = new_cur;
            }
        }
    }
}
