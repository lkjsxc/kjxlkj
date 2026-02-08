//! Action dispatch: process editor actions by delegating
//! to the appropriate mode/edit/state handlers.

use kjxlkj_core_types::{Action, Mode, Operator};
use crate::EditorState;

impl EditorState {
    /// Dispatch a single action, mutating editor state.
    pub fn dispatch(&mut self, action: Action) {
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
            Action::WriteAll => {
                self.do_write_all();
            }
            Action::WriteAllQuit => {
                self.do_write_all();
                self.should_quit = true;
            }
            Action::Resize(c, r) => {
                self.handle_resize(c, r);
            }
            Action::MoveCursor(motion, count) => {
                self.do_motion(motion, count);
            }
            Action::EnterInsert(pos) => {
                self.enter_insert(pos);
            }
            Action::EnterVisual(kind) => {
                self.enter_visual(kind);
            }
            Action::EnterOperatorPending(op) => {
                self.enter_op_pending(op);
            }
            Action::EnterCommand(kind) => {
                self.enter_command(kind);
            }
            Action::EnterReplace => {
                self.mode = Mode::Replace;
            }
            Action::ReturnToNormal => {
                self.mode = Mode::Normal;
                self.visual_state = None;
                self.command_state = None;
                // Clamp cursor: in Normal mode cursor
                // must be on a character, not past end.
                if let Some(w) = self.focused_window_mut() {
                    if w.cursor.grapheme_offset > 0 {
                        w.cursor.grapheme_offset -= 1;
                    }
                }
            }
            Action::InsertChar(ch) => {
                self.insert_char(ch);
            }
            Action::DeleteCharForward => {
                self.delete_char_forward();
            }
            Action::DeleteCharBackward => {
                self.do_backspace();
            }
            Action::Delete(motion, count) => {
                self.do_operator_motion_action(
                    Operator::Delete,
                    motion,
                    count,
                );
            }
            Action::Change(motion, count) => {
                self.do_operator_motion_action(
                    Operator::Change,
                    motion,
                    count,
                );
            }
            Action::Yank(motion, count) => {
                self.do_operator_motion_action(
                    Operator::Yank,
                    motion,
                    count,
                );
            }
            Action::DoubleOperator(op, count) => {
                self.do_double_operator(op, count);
            }
            Action::Undo => self.do_undo(),
            Action::Redo => self.do_redo(),
            Action::Put(before) => {
                self.do_put(before);
            }
            Action::JoinLines => self.do_join(),
            Action::ToggleCaseChar => self.do_toggle_case(),
            Action::OpenFile(path) => {
                self.do_open_file(&path);
            }
            Action::NextBuffer => self.do_next_buffer(),
            Action::PrevBuffer => self.do_prev_buffer(),
            Action::DeleteBuffer => self.do_delete_buffer(),
            Action::SplitHorizontal => {
                self.do_split_horizontal();
            }
            Action::SplitVertical => {
                self.do_split_vertical();
            }
            Action::ReplaceChar(c) => {
                self.do_replace_char(c);
            }
            Action::Increment(n) => {
                self.do_increment(n);
            }
            Action::ExecuteCommand(cmd) => {
                if let Some(a) =
                    crate::dispatch_command(&cmd)
                {
                    self.dispatch(a);
                }
            }
            Action::SearchForward(pat) => {
                self.do_search_forward(pat);
            }
            Action::SearchBackward(pat) => {
                self.do_search_backward(pat);
            }
            Action::NextMatch => self.do_next_match(),
            Action::PrevMatch => self.do_prev_match(),
            Action::FocusWindow(dir) => {
                self.do_focus_window(dir);
            }
            Action::CycleWindow => {
                self.do_cycle_window();
            }
            Action::CloseWindow => {
                self.do_close_window();
            }
            _ => {}
        }
    }
}
