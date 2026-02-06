//! Intent dispatch: process parsed intents against editor state.

use crate::dispatch_editing::*;
use crate::dispatch_navigation::*;
use crate::dispatch_operators::*;
use crate::EditorState;
use kjxlkj_core_types::{Intent, Mode, OperatorKind};

/// Process a single intent, mutating editor state.
pub fn dispatch_intent(state: &mut EditorState, intent: Intent) {
    match intent {
        Intent::Noop => {}
        Intent::Motion(kind, count) => {
            dispatch_motion(state, kind, count)
        }
        Intent::EnterMode(mode) => {
            state.mode.transition(mode);
            state.parser.reset();
        }
        Intent::EnterInsert(pos) => {
            dispatch_enter_insert(state, pos)
        }
        Intent::InsertChar(c) => dispatch_insert_char(state, c),
        Intent::InsertNewline => dispatch_insert_newline(state),
        Intent::DeleteCharBefore => {
            dispatch_delete_char_before(state)
        }
        Intent::DeleteCharAt => dispatch_delete_char_at(state),
        Intent::Operator(op, motion, count) => {
            dispatch_operator(state, op, motion, count);
        }
        Intent::OperatorTextObject(op, kind, inner) => {
            dispatch_operator_text_object(state, op, kind, inner);
        }
        Intent::LineOperator(op, count) => {
            dispatch_line_operator(state, op, count);
        }
        Intent::Undo => dispatch_undo(state),
        Intent::Redo => dispatch_redo(state),
        Intent::Paste(_, paste_pos) => {
            dispatch_paste(state, paste_pos)
        }
        Intent::YankLine(count) => {
            dispatch_yank_line(state, count)
        }
        Intent::DeleteToEnd => dispatch_delete_to_end(state),
        Intent::ChangeToEnd => {
            dispatch_delete_to_end(state);
            state.mode.transition(Mode::Insert);
        }
        Intent::OpenLine(below) => {
            dispatch_open_line(state, below)
        }
        Intent::JoinLines(spaces, count) => {
            dispatch_join_lines(state, spaces, count);
        }
        Intent::ReplaceChar(c) => {
            dispatch_replace_char(state, c)
        }
        Intent::Scroll(kind) => dispatch_scroll(state, kind),
        Intent::ExCommand(cmd) => {
            crate::commands::dispatch_ex_command(state, &cmd)
        }
        Intent::ToggleCase => dispatch_toggle_case(state),
        Intent::SubstituteChar => {
            dispatch_delete_char_at(state);
            state.mode.transition(Mode::Insert);
        }
        Intent::SubstituteLine => {
            dispatch_line_operator(
                state,
                OperatorKind::Change,
                1,
            );
        }
        Intent::ReplaceInsert(c) => {
            dispatch_replace_insert(state, c)
        }
        Intent::Indent(indent, count) => {
            dispatch_indent(state, indent, count);
        }
        _ => {} // Remaining intents stubbed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EditorState;
    use kjxlkj_core_types::{
        InsertPosition, Intent, Mode, MotionKind, OperatorKind, Size,
    };

    fn setup_editor(text: &str) -> EditorState {
        let mut s = EditorState::new(Size::new(80, 24));
        let bid = s.create_buffer_from_text(text);
        s.create_window(bid);
        s
    }

    #[test]
    fn insert_char() {
        let mut s = setup_editor("hello");
        s.mode.transition(Mode::Insert);
        dispatch_intent(&mut s, Intent::InsertChar('x'));
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('x'));
    }

    #[test]
    fn motion_down() {
        let mut s = setup_editor("line1\nline2\nline3");
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
        assert_eq!(s.cursor().line, 1);
    }

    #[test]
    fn dd_deletes_line() {
        let mut s = setup_editor("aaa\nbbb\nccc");
        dispatch_intent(
            &mut s,
            Intent::LineOperator(OperatorKind::Delete, 1),
        );
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('b'));
    }

    #[test]
    fn quit_command() {
        let mut s = setup_editor("hello");
        dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
        assert!(s.should_quit);
    }

    #[test]
    fn enter_insert_mode() {
        let mut s = setup_editor("hello");
        dispatch_intent(
            &mut s,
            Intent::EnterInsert(InsertPosition::BeforeCursor),
        );
        assert_eq!(s.current_mode(), Mode::Insert);
    }

    #[test]
    fn open_line_below() {
        let mut s = setup_editor("hello\nworld");
        dispatch_intent(&mut s, Intent::OpenLine(true));
        assert_eq!(s.current_mode(), Mode::Insert);
        assert_eq!(s.cursor().line, 1);
    }

    #[test]
    fn toggle_case() {
        let mut s = setup_editor("Hello");
        dispatch_intent(&mut s, Intent::ToggleCase);
        let buf = s.active_buffer().unwrap();
        assert!(buf.text.line_to_string(0).starts_with('h'));
    }

    #[test]
    fn scroll_half_page_down() {
        let mut s = setup_editor(
            &(0..50).map(|i| format!("line{}", i)).collect::<Vec<_>>().join("\n"),
        );
        dispatch_intent(
            &mut s,
            Intent::Scroll(kjxlkj_core_types::ScrollKind::HalfPageDown),
        );
        let win = s.active_window_state().unwrap();
        assert!(win.cursor_line > 0);
    }
}
