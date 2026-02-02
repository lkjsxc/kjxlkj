//! Intent processing for core task.

use kjxlkj_core_mode::{Intent, IntentKind};
use kjxlkj_core_state::EditorState;

use crate::ex_command::execute_ex_command;
use crate::operator_exec::{operator_line, operator_motion, operator_text_object};
use crate::register_ops;
use crate::search_ops;
use crate::text_ops::{backspace, delete_char, execute_motion_on_state, insert_text, redo, undo};
use crate::window_ops;
use crate::ActionResult;

/// Processes an intent and updates editor state.
pub fn process_intent(state: &mut EditorState, intent: Intent) -> ActionResult {
    match intent.kind {
        IntentKind::Noop => ActionResult::Ok,

        IntentKind::ChangeMode(mode) => {
            state.mode.transition(mode);
            ActionResult::ModeChanged(mode)
        }

        IntentKind::InsertText { text } => {
            insert_text(state, &text);
            ActionResult::Ok
        }

        IntentKind::InsertNewline => {
            insert_text(state, "\n");
            ActionResult::Ok
        }

        IntentKind::Backspace => {
            backspace(state);
            ActionResult::Ok
        }

        IntentKind::DeleteChar => {
            delete_char(state);
            ActionResult::Ok
        }

        IntentKind::Motion(motion) => {
            let count = intent.count.max(1);
            execute_motion_on_state(state, &motion, count);
            ActionResult::Ok
        }

        IntentKind::OperatorMotion { op, motion } => {
            operator_motion(state, op, motion);
            ActionResult::Ok
        }

        IntentKind::OperatorTextObject { op, text_object } => {
            operator_text_object(state, op, text_object);
            ActionResult::Ok
        }

        IntentKind::OperatorLine { op } => {
            operator_line(state, op);
            ActionResult::Ok
        }

        IntentKind::SplitHorizontal => {
            window_ops::split_horizontal(state);
            ActionResult::Ok
        }

        IntentKind::SplitVertical => {
            window_ops::split_vertical(state);
            ActionResult::Ok
        }

        IntentKind::CloseWindow => {
            window_ops::close_window(state);
            ActionResult::Ok
        }

        IntentKind::OnlyWindow => {
            window_ops::only_window(state);
            ActionResult::Ok
        }

        IntentKind::NextWindow => {
            window_ops::next_window(state);
            ActionResult::Ok
        }

        IntentKind::PrevWindow => {
            window_ops::prev_window(state);
            ActionResult::Ok
        }

        IntentKind::WindowDirection(dir) => {
            window_ops::window_direction(state, dir);
            ActionResult::Ok
        }

        IntentKind::PutAfter { register } => {
            register_ops::put_after(state, register);
            ActionResult::Ok
        }

        IntentKind::PutBefore { register } => {
            register_ops::put_before(state, register);
            ActionResult::Ok
        }

        IntentKind::YankLine => {
            register_ops::yank_line(state);
            ActionResult::Ok
        }

        IntentKind::SearchForward { pattern } => {
            search_ops::search_forward(state, &pattern);
            ActionResult::Ok
        }

        IntentKind::SearchBackward { pattern } => {
            search_ops::search_backward(state, &pattern);
            ActionResult::Ok
        }

        IntentKind::NextMatch => {
            search_ops::next_match(state);
            ActionResult::Ok
        }

        IntentKind::PrevMatch => {
            search_ops::prev_match(state);
            ActionResult::Ok
        }

        IntentKind::ExCommand { command } => {
            execute_ex_command(state, &command)
        }

        IntentKind::Quit => ActionResult::Quit,

        IntentKind::Save => ActionResult::Save,

        IntentKind::SaveQuit => ActionResult::SaveQuit,

        IntentKind::Undo => {
            undo(state);
            ActionResult::Ok
        }

        IntentKind::Redo => {
            redo(state);
            ActionResult::Ok
        }

        IntentKind::ToggleFileExplorer => {
            // TODO: Implement file explorer toggle
            state.set_message("File explorer not yet implemented");
            ActionResult::Ok
        }

        IntentKind::FocusFileExplorer => {
            // TODO: Implement file explorer focus
            state.set_message("File explorer not yet implemented");
            ActionResult::Ok
        }

        IntentKind::ToggleTerminal => {
            // TODO: Implement terminal toggle
            state.set_message("Terminal not yet implemented");
            ActionResult::Ok
        }

        IntentKind::Repeat => {
            // TODO: Implement repeat
            ActionResult::Ok
        }
    }
}
