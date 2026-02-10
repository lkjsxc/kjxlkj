//! Action dispatch: applies actions to EditorState.
//!
//! This is a thin facade. Implementation is split across:
//! - editing_ops.rs: buffer editing, undo/redo, motions
//! - cmdline_ops.rs: command-line, ex commands, notifications
//! - service_dispatch.rs: service response handling

use kjxlkj_core_mode::DispatchResult;
use kjxlkj_core_types::{Action, Key, KeyAction, Mode};

use crate::cursor_ops;
use crate::editor::EditorState;

impl EditorState {
    /// Process a top-level action.
    pub fn process_action(&mut self, action: Action) {
        match action {
            Action::Resize(cols, rows) => {
                self.terminal_size = (cols, rows);
            }
            Action::Paste(text) => {
                self.paste_text(&text);
            }
            Action::FocusGained | Action::FocusLost => {}
            Action::Quit => {
                self.try_quit(false);
            }
            Action::ForceQuit => {
                self.try_quit(true);
            }
            Action::KeyAction(ka) => {
                self.process_key_action(ka);
            }
            Action::ServiceResponse(resp) => {
                self.process_service_response(resp);
            }
        }
    }

    /// Process a decoded key.
    pub fn process_key(&mut self, key: Key) {
        if let Mode::Command(_) = &self.mode {
            self.process_cmdline_key(&key);
            return;
        }

        let result = self.dispatcher.dispatch(&key, &self.mode);
        match result {
            DispatchResult::Action(ka) => {
                self.process_key_action(ka);
            }
            DispatchResult::ModeChange(new_mode) => {
                self.change_mode(new_mode);
            }
            DispatchResult::Pending => {}
            DispatchResult::Noop => {}
        }
    }

    pub(crate) fn process_key_action(&mut self, ka: KeyAction) {
        match ka {
            KeyAction::InsertChar(c) => {
                self.do_insert_char(c);
            }
            KeyAction::DeleteCharForward => {
                self.do_delete_forward();
            }
            KeyAction::DeleteCharBackward => {
                self.do_delete_backward();
            }
            KeyAction::EnterMode(mode) => {
                self.enter_mode_with_cursor(mode);
            }
            KeyAction::InsertAppend => {
                self.do_insert_append();
            }
            KeyAction::InsertAppendEol => {
                self.do_insert_append_eol();
            }
            KeyAction::InsertFirstNonBlank => {
                self.do_insert_first_nonblank();
            }
            KeyAction::Motion(motion) => {
                self.do_motion(motion);
            }
            KeyAction::OperatorMotion { op, motion, count } => {
                self.do_operator(op, motion, count);
            }
            KeyAction::Undo => self.do_undo(),
            KeyAction::Redo => self.do_redo(),
            KeyAction::OpenLineBelow => {
                self.do_open_line_below();
            }
            KeyAction::OpenLineAbove => {
                self.do_open_line_above();
            }
            KeyAction::JoinLines => {
                self.do_join_lines();
            }
            KeyAction::PutAfter => {
                self.do_put(true);
            }
            KeyAction::PutBefore => {
                self.do_put(false);
            }
            KeyAction::ExCommand(cmd) => {
                self.execute_ex(&cmd);
            }
            KeyAction::WindowNext => {
                self.windows.active_tab_mut().next_window();
            }
            KeyAction::WindowPrev => {
                self.windows.active_tab_mut().prev_window();
            }
            KeyAction::WindowSplitH => {
                self.do_window_split_h();
            }
            KeyAction::WindowSplitV => {
                self.do_window_split_v();
            }
            KeyAction::WindowClose => {
                self.do_window_close();
            }
            KeyAction::ViewportCenter => {
                self.do_viewport_center();
            }
            KeyAction::ViewportTop => {
                self.do_viewport_top();
            }
            KeyAction::ViewportBottom => {
                self.do_viewport_bottom();
            }
            KeyAction::TerminalOpen => {
                self.do_terminal_open();
            }
            KeyAction::TerminalSplitH => {
                self.do_terminal_split(false);
            }
            KeyAction::TerminalSplitV => {
                self.do_terminal_split(true);
            }
            KeyAction::ExplorerToggle => {
                self.do_explorer_toggle();
            }
            KeyAction::ExplorerReveal => {
                self.do_explorer_reveal();
            }
            KeyAction::Noop => {}
        }
    }

    pub(crate) fn change_mode(&mut self, new_mode: Mode) {
        let old_mode = self.mode.clone();
        match (&old_mode, &new_mode) {
            (Mode::Insert, Mode::Normal) => {
                self.save_undo_checkpoint();
                self.with_active_buffer(|buf, cur| {
                    cursor_ops::cursor_leave_insert(cur, buf);
                });
            }
            (_, Mode::Command(kind)) => {
                let prefix = match kind {
                    kjxlkj_core_types::CommandKind::Ex => ":",
                    kjxlkj_core_types::CommandKind::SearchForward => "/",
                    kjxlkj_core_types::CommandKind::SearchBackward => "?",
                };
                self.cmdline.visible = true;
                self.cmdline.prefix = prefix.to_string();
                self.cmdline.content.clear();
                self.cmdline.cursor_pos = 0;
            }
            (Mode::Command(_), Mode::Normal) => {
                self.cmdline.visible = false;
            }
            _ => {}
        }
        self.mode = new_mode;
    }

    pub(crate) fn enter_mode_with_cursor(&mut self, mode: Mode) {
        if mode == Mode::Insert {
            self.save_undo_checkpoint();
        }
        self.mode = mode;
    }
}
