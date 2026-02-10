//! Action dispatch – facade routing actions to ops modules.

use kjxlkj_core_mode::DispatchResult;
use kjxlkj_core_types::{Action, Key, KeyAction, Mode};
use kjxlkj_core_ui::WindowContent;
use kjxlkj_input::ime_route::{route_ime_key, ImeResult};

use super::cursor_ops;
use crate::editor::EditorState;

impl EditorState {
    /// Process a top-level action.
    pub fn process_action(&mut self, action: Action) {
        match action {
            Action::Resize(c, r) => self.terminal_size = (c, r),
            Action::Paste(text) => self.paste_text(&text),
            Action::FocusGained | Action::FocusLost => {}
            Action::Quit => self.try_quit(false),
            Action::ForceQuit => self.try_quit(true),
            Action::KeyAction(ka) => self.process_key_action(ka),
            Action::ServiceResponse(r) => self.process_service_response(r),
        }
    }

    /// Process a decoded key.
    pub fn process_key(&mut self, key: Key) {
        if let Mode::Command(_) = &self.mode {
            self.process_cmdline_key(&key);
            return;
        }
        // IME routing: in Insert mode, route through IME before dispatch.
        if self.mode == Mode::Insert && self.ime.is_composing() {
            match route_ime_key(&mut self.ime, &key.code) {
                ImeResult::Consumed => return,
                ImeResult::Commit(text) => {
                    for c in text.chars() {
                        self.do_insert_char(c);
                    }
                    return;
                }
                ImeResult::Cancelled => return,
                ImeResult::PassThrough => {} // fall through to normal dispatch
            }
        }
        // Explorer-focused window: use explorer dispatch in Normal mode
        if self.mode == Mode::Normal {
            let is_explorer = matches!(
                self.windows.active_tab().active().content,
                WindowContent::Explorer
            );
            if is_explorer {
                self.dispatch_explorer_key(&key);
                return;
            }
        }
        let result = self.dispatcher.dispatch(&key, &self.mode);
        if let Some(reg) = self.dispatcher.register.take() {
            self.pending_register = Some(reg);
        }
        match result {
            DispatchResult::Action(ka) => self.process_key_action(ka),
            DispatchResult::ModeChange(m) => self.change_mode(m),
            DispatchResult::Pending | DispatchResult::Noop => {}
        }
    }

    pub(crate) fn process_key_action(&mut self, ka: KeyAction) {
        match ka {
            KeyAction::InsertChar(c) => self.do_insert_char(c),
            KeyAction::DeleteCharForward => self.do_delete_forward(),
            KeyAction::DeleteCharBackward => self.do_delete_backward(),
            KeyAction::EnterMode(mode) => self.enter_mode_with_cursor(mode),
            KeyAction::InsertAppend => self.do_insert_append(),
            KeyAction::InsertAppendEol => self.do_insert_append_eol(),
            KeyAction::InsertFirstNonBlank => self.do_insert_first_nonblank(),
            KeyAction::Motion(m) => self.do_motion(m),
            KeyAction::OperatorMotion { op, motion, count } => self.do_operator(op, motion, count),
            KeyAction::Undo => self.do_undo(),
            KeyAction::Redo => self.do_redo(),
            KeyAction::OpenLineBelow => self.do_open_line_below(),
            KeyAction::OpenLineAbove => self.do_open_line_above(),
            KeyAction::JoinLines => self.do_join_lines(),
            KeyAction::PutAfter => self.do_put(true),
            KeyAction::PutBefore => self.do_put(false),
            KeyAction::ExCommand(cmd) => self.execute_ex(&cmd),
            KeyAction::WindowNext => self.windows.active_tab_mut().next_window(),
            KeyAction::WindowPrev => self.windows.active_tab_mut().prev_window(),
            KeyAction::WindowFocusLeft => self
                .windows
                .active_tab_mut()
                .focus_direction(crate::focus::FocusDir::Left),
            KeyAction::WindowFocusRight => self
                .windows
                .active_tab_mut()
                .focus_direction(crate::focus::FocusDir::Right),
            KeyAction::WindowFocusUp => self
                .windows
                .active_tab_mut()
                .focus_direction(crate::focus::FocusDir::Up),
            KeyAction::WindowFocusDown => self
                .windows
                .active_tab_mut()
                .focus_direction(crate::focus::FocusDir::Down),
            KeyAction::WindowSplitH => self.do_window_split_h(),
            KeyAction::WindowSplitV => self.do_window_split_v(),
            KeyAction::WindowClose => self.do_window_close(),
            KeyAction::ViewportCenter => self.do_viewport_center(),
            KeyAction::ViewportTop => self.do_viewport_top(),
            KeyAction::ViewportBottom => self.do_viewport_bottom(),
            KeyAction::TerminalOpen => self.do_terminal_open(),
            KeyAction::TerminalSplitH => self.do_terminal_split(false),
            KeyAction::TerminalSplitV => self.do_terminal_split(true),
            KeyAction::ExplorerToggle => self.do_explorer_toggle(),
            KeyAction::ExplorerReveal => self.do_explorer_reveal(),
            KeyAction::ReplaceChar(c) => self.do_replace_char(c),
            KeyAction::ReplaceBackspace => self.do_replace_backspace(),
            KeyAction::VisualOperator(op) => self.do_visual_operator(op),
            KeyAction::Noop => {}
        }
    }

    pub(crate) fn change_mode(&mut self, new_mode: Mode) {
        match (&self.mode, &new_mode) {
            (Mode::Insert, Mode::Normal) | (Mode::Replace, Mode::Normal) => {
                self.save_undo_checkpoint();
                if matches!(self.mode, Mode::Replace) {
                    self.replace_stack.clear();
                }
                self.with_active_buffer(|buf, cur| cursor_ops::cursor_leave_insert(cur, buf));
            }
            (_, Mode::Replace) => {
                self.save_undo_checkpoint();
                self.replace_stack.clear();
            }
            (Mode::Visual(_), Mode::Normal) => {
                self.visual_anchor = None;
            }
            (_, Mode::Visual(_)) => {
                let win = self.windows.active_tab().active();
                self.visual_anchor = Some((win.cursor_line, win.cursor_offset));
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
