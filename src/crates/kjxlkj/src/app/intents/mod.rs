//! Intent application.

mod helpers;

use kjxlkj_core::{EditorState, Intent, Mode};

use super::{apply_motion, apply_operator};
use helpers::*;

/// Apply an intent to the editor state.
pub fn apply_intent(state: &mut EditorState, intent: Intent) {
    match intent {
        Intent::None => {}
        Intent::Move(motion) => apply_motion(state, motion),
        Intent::ChangeMode(mode) => state.set_mode(mode),
        Intent::EnterInsert { at_line_end, after_cursor } => {
            apply_enter_insert(state, at_line_end, after_cursor);
        }
        Intent::EnterCommand => state.set_mode(Mode::Command),
        Intent::EnterReplace => state.set_mode(Mode::Replace),
        Intent::StartVisual(kind) => apply_start_visual(state, kind),
        Intent::OpenLineBelow => apply_open_line_below(state),
        Intent::OpenLineAbove => apply_open_line_above(state),
        Intent::DeleteChar => apply_delete_char(state),
        Intent::DeleteCharBefore => apply_delete_char_before(state),
        Intent::Execute(op) => apply_operator(state, op),
        Intent::Undo => apply_undo(state),
        Intent::Redo => apply_redo(state),
        Intent::Paste { before, cursor_at_end } => {
            apply_paste(state, before, cursor_at_end);
        }
        Intent::JoinLines { with_space } => apply_join_lines(state, with_space),
        Intent::ReplaceChar(c) => apply_replace_char(state, c),
        Intent::ToggleCaseChar => apply_toggle_case(state),
        Intent::SearchForward => apply_search_forward(state),
        Intent::SearchBackward => apply_search_backward(state),
        Intent::Quit { .. } => state.should_quit = true,
        Intent::Write { path } => apply_write(state, path),
        Intent::WriteQuit { path } => {
            apply_write(state, path);
            state.should_quit = true;
        }
        _ => {}
    }
}
