//! Action dispatch: process editor actions by delegating
//! to the appropriate mode/edit/state handlers.

use crate::EditorState;
use kjxlkj_core_types::{Action, Mode, Operator};

impl EditorState {
    /// Dispatch a single action, mutating editor state.
    pub fn dispatch(&mut self, action: Action) {
        self.store_repeatable(&action);

        match action {
            Action::Nop => {}
            Action::Quit => self.should_quit = true,
            Action::ForceQuit => self.should_quit = true,
            Action::QuitAll => self.should_quit = true,
            Action::QuitSignal => self.should_quit = true,
            Action::Write => self.do_write(),
            Action::WriteQuit => {
                self.do_write();
                self.should_quit = true;
            }
            Action::WriteAll => self.do_write_all(),
            Action::WriteAllQuit => {
                self.do_write_all();
                self.should_quit = true;
            }
            Action::Resize(c, r) => self.handle_resize(c, r),
            Action::MoveCursor(motion, count) => {
                self.do_motion(motion, count);
            }
            Action::Scroll(dir, count) => self.do_scroll(dir, count),
            Action::EnterInsert(pos) => self.enter_insert(pos),
            Action::EnterVisual(kind) => self.enter_visual(kind),
            Action::EnterOperatorPending(op) => self.enter_op_pending(op),
            Action::EnterCommand(kind) => self.enter_command(kind),
            Action::EnterReplace => {
                self.mode = Mode::Replace;
            }
            Action::ReturnToNormal => {
                if self.mode == Mode::Insert {
                    self.update_caret_mark();
                }
                // Save visual marks when leaving visual mode
                if let Mode::Visual(_) = self.mode {
                    if let Some(vs) = &self.visual_state {
                        let (al, ac) = vs.anchor;
                        let (cl, cc) = self.cursor_pos();
                        let (sl, sc, el, ec) = if al < cl || (al == cl && ac <= cc) {
                            (al, ac, cl, cc)
                        } else {
                            (cl, cc, al, ac)
                        };
                        self.update_visual_marks(sl, sc, el, ec);
                    }
                }
                self.mode = Mode::Normal;
                self.visual_state = None;
                self.command_state = None;
                if let Some(w) = self.focused_window_mut() {
                    if w.cursor.grapheme_offset > 0 {
                        w.cursor.grapheme_offset -= 1;
                    }
                }
            }
            Action::InsertChar(ch) => self.insert_char(ch),
            Action::DeleteCharForward => self.delete_char_forward(),
            Action::DeleteCharBackward => self.do_backspace(),
            Action::Delete(motion, count) => {
                self.do_operator_motion_action(Operator::Delete, motion, count);
            }
            Action::Change(motion, count) => {
                self.do_operator_motion_action(Operator::Change, motion, count);
            }
            Action::Yank(motion, count) => {
                self.do_operator_motion_action(Operator::Yank, motion, count);
            }
            Action::DoubleOperator(op, count) => self.do_double_operator(op, count),
            Action::SubstituteChar => self.do_substitute_char(),
            Action::SubstituteLine => self.do_substitute_line(),
            Action::ChangeToEnd => self.do_change_to_end(),
            Action::JoinLines => self.do_join(),
            Action::JoinLinesNoSpace => self.do_join_no_space(),
            Action::ToggleCaseChar => self.do_toggle_case(),
            Action::DotRepeat => self.do_dot_repeat(),
            Action::Undo => self.do_undo(),
            Action::Redo => self.do_redo(),
            Action::Put(before) => self.do_put(before),
            Action::ReplaceChar(c) => self.do_replace_char(c),
            Action::Increment(n) => self.do_increment(n),
            other => self.dispatch_extended(other),
        }
    }
}
