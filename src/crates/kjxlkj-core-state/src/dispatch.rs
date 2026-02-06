//! Intent dispatch: process parsed intents against editor state.

use crate::dispatch_editing::*;
use crate::dispatch_editing_extra::*;
use crate::dispatch_jumps::*;
use crate::dispatch_misc::*;
use crate::dispatch_navigation::*;
use crate::dispatch_operators::*;
use crate::dispatch_search::*;
use crate::dispatch_windows::*;
use crate::EditorState;
use kjxlkj_core_types::{Intent, Mode, MotionKind, OperatorKind, Position};

/// Process a single intent, mutating editor state.
pub fn dispatch_intent(state: &mut EditorState, intent: Intent) {
    // Check if we're in InsertNormal mode (Ctrl-o from insert).
    let was_insert_normal =
        state.mode.current() == Mode::InsertNormal;
    let intent_ref = intent.clone();
    // Record intents for macro recording (except toggle-record itself).
    if state.macro_recording.is_some()
        && !matches!(intent, Intent::MacroToggleRecord(_))
    {
        if let Some((_, ref mut intents)) = state.macro_recording {
            intents.push(intent.clone());
        }
    }
    // Track repeatable changes for dot repeat.
    if is_repeatable(&intent) {
        state.last_change = Some(intent.clone());
        // Track change positions
        push_change(state);
    }
    // Track jump positions for search/mark jumps.
    if is_jump(&intent) {
        push_jump(state);
    }
    match intent {
        Intent::Noop => {}
        Intent::Motion(kind, count) => {
            dispatch_motion(state, kind, count)
        }
        Intent::EnterMode(mode) => {
            // Set/clear visual anchor on mode transitions
            if mode == Mode::Visual || mode == Mode::VisualLine {
                if let Some(win) = state.active_window_state() {
                    let anchor = Position::new(
                        win.cursor_line,
                        win.cursor_col,
                    );
                    if let Some(wid) = state.active_window {
                        if let Some(w) =
                            state.windows.get_mut(&wid)
                        {
                            w.visual_anchor = Some(anchor);
                        }
                    }
                }
            } else if let Some(wid) = state.active_window {
                if let Some(w) = state.windows.get_mut(&wid) {
                    w.visual_anchor = None;
                }
            }
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
        Intent::DeleteWordBefore => {
            dispatch_delete_word_before(state)
        }
        Intent::DeleteToLineStart => {
            dispatch_delete_to_line_start(state)
        }
        Intent::InsertFromRegister(reg) => {
            dispatch_insert_from_register(state, reg)
        }
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
        // Search
        Intent::SearchForward(pat) => {
            dispatch_search_forward(state, &pat)
        }
        Intent::SearchBackward(pat) => {
            dispatch_search_backward(state, &pat)
        }
        Intent::SearchNext => dispatch_search_next(state),
        Intent::SearchPrev => dispatch_search_prev(state),
        Intent::SearchWordForward => {
            dispatch_search_word_forward(state)
        }
        Intent::SearchWordBackward => {
            dispatch_search_word_backward(state)
        }
        // Marks
        Intent::SetMark(c) => dispatch_set_mark(state, c),
        Intent::JumpToMark(c) => {
            dispatch_jump_to_mark(state, c)
        }
        Intent::JumpToMarkLine(c) => {
            dispatch_jump_to_mark_line(state, c)
        }
        // Find-char
        Intent::FindChar(c, kind) => {
            dispatch_find_char(state, c, kind)
        }
        Intent::RepeatFindChar => {
            dispatch_repeat_find_char(state)
        }
        Intent::RepeatFindCharReverse => {
            dispatch_repeat_find_char_reverse(state)
        }
        // Case operators
        Intent::CaseOperator(op, motion, count) => {
            dispatch_case_operator(state, op, motion, count)
        }
        Intent::CaseOperatorLine(op) => {
            dispatch_case_operator_line(state, op)
        }
        // Visual
        Intent::VisualSwapEnd => {
            dispatch_visual_swap_end(state)
        }
        // Register
        Intent::SelectRegister(reg) => {
            dispatch_select_register(state, reg)
        }
        // Increment/decrement
        Intent::IncrementNumber(delta) => {
            dispatch_increment_number(state, delta)
        }
        // Macro recording/playback
        Intent::MacroToggleRecord(reg) => {
            if let Some((rec_reg, intents)) = state.macro_recording.take() {
                // Stop recording — save to macro storage
                state.macros.insert(rec_reg, intents);
                state.message =
                    Some(format!("Recorded @{}", rec_reg));
            } else {
                // Start recording into register
                state.macro_recording = Some((reg, Vec::new()));
                state.message =
                    Some(format!("Recording @{}...", reg));
            }
        }
        Intent::MacroPlay(reg) => {
            if let Some(intents) = state.macros.get(&reg).cloned() {
                state.last_macro = Some(reg);
                // Temporarily stop recording to avoid re-recording
                let was_recording = state.macro_recording.take();
                for intent in intents {
                    dispatch_intent(state, intent);
                }
                state.macro_recording = was_recording;
            } else {
                state.message =
                    Some(format!("Empty macro @{}", reg));
            }
        }
        Intent::MacroRepeatLast => {
            if let Some(reg) = state.last_macro {
                if let Some(intents) = state.macros.get(&reg).cloned() {
                    let was_recording = state.macro_recording.take();
                    for intent in intents {
                        dispatch_intent(state, intent);
                    }
                    state.macro_recording = was_recording;
                }
            } else {
                state.message =
                    Some("No previous macro".into());
            }
        }
        // Dot repeat
        Intent::RepeatLastChange => {
            if let Some(last) = state.last_change.clone() {
                // Prevent infinite recursion: don't re-set last_change
                let saved = state.last_change.take();
                dispatch_intent(state, last);
                state.last_change = saved;
            }
        }
        // Jump list
        Intent::JumpListBack => dispatch_jump_back(state),
        Intent::JumpListForward => dispatch_jump_forward(state),
        // Change list
        Intent::ChangeListOlder => dispatch_change_older(state),
        Intent::ChangeListNewer => dispatch_change_newer(state),
        // Window management
        Intent::WindowSplitHorizontal => {
            dispatch_window_split_horizontal(state)
        }
        Intent::WindowSplitVertical => {
            dispatch_window_split_vertical(state)
        }
        Intent::WindowClose => dispatch_window_close(state),
        Intent::WindowOnly => dispatch_window_only(state),
        Intent::WindowFocusNext => {
            dispatch_window_focus_next(state)
        }
        Intent::WindowFocusPrev => {
            dispatch_window_focus_prev(state)
        }
        Intent::WindowFocusDirection(dir) => {
            dispatch_window_focus_direction(state, dir)
        }
        Intent::WindowEqualSize => {
            dispatch_window_equal_size(state)
        }
        Intent::WindowRotate => dispatch_window_rotate(state),
    }
    // InsertNormal: return to Insert after one normal command.
    if was_insert_normal
        && !matches!(intent_ref, Intent::EnterMode(_))
    {
        state.mode.transition(Mode::Insert);
    }
}

/// Check if an intent is a repeatable change (for dot).
fn is_repeatable(intent: &Intent) -> bool {
    matches!(
        intent,
        Intent::InsertChar(_)
            | Intent::InsertNewline
            | Intent::DeleteCharBefore
            | Intent::DeleteCharAt
            | Intent::Operator(_, _, _)
            | Intent::OperatorTextObject(_, _, _)
            | Intent::LineOperator(_, _)
            | Intent::ReplaceChar(_)
            | Intent::ReplaceInsert(_)
            | Intent::OpenLine(_)
            | Intent::JoinLines(_, _)
            | Intent::ToggleCase
            | Intent::SubstituteChar
            | Intent::SubstituteLine
            | Intent::DeleteToEnd
            | Intent::ChangeToEnd
            | Intent::Indent(_, _)
            | Intent::CaseOperator(_, _, _)
            | Intent::CaseOperatorLine(_)
            | Intent::IncrementNumber(_)
    )
}

/// Check if an intent causes a position jump (for jump list).
fn is_jump(intent: &Intent) -> bool {
    matches!(
        intent,
        Intent::SearchForward(_)
            | Intent::SearchBackward(_)
            | Intent::SearchNext
            | Intent::SearchPrev
            | Intent::SearchWordForward
            | Intent::SearchWordBackward
            | Intent::JumpToMark(_)
            | Intent::JumpToMarkLine(_)
            | Intent::Motion(MotionKind::MatchingBracket, _)
            | Intent::Motion(MotionKind::FileStart, _)
            | Intent::Motion(MotionKind::FileEnd, _)
            | Intent::Motion(MotionKind::GotoLine(_), _)
            | Intent::Motion(MotionKind::GotoPercent(_), _)
    )
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
