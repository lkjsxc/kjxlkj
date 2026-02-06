//! Intent dispatch: process parsed intents against editor state.

use crate::{dispatch_case::*, dispatch_editing::*, dispatch_editing_extra::*};
use crate::{dispatch_insert::*, dispatch_jumps::*, dispatch_macros::*};
use crate::{dispatch_marks::*, dispatch_misc::*, dispatch_navigation::*};
use crate::{dispatch_operators::*, dispatch_search::*, dispatch_windows::*, dispatch_yank_paste::*};
use crate::EditorState;
use kjxlkj_core_types::{Intent, Mode, MotionKind, OperatorKind, Position};

/// Handle mode transition with visual anchor management.
fn dispatch_enter_mode(state: &mut EditorState, mode: Mode) {
    // Save last visual selection before leaving visual mode
    if state.mode.current().is_visual() && !mode.is_visual() {
        if let Some(win) = state.active_window_state() {
            if let Some(anchor) = win.visual_anchor {
                let cursor = Position::new(win.cursor_line, win.cursor_col);
                state.last_visual = Some((anchor, cursor, state.mode.current()));
            }
        }
    }
    if mode == Mode::Visual || mode == Mode::VisualLine {
        if let Some(win) = state.active_window_state() {
            let anchor = Position::new(win.cursor_line, win.cursor_col);
            if let Some(wid) = state.active_window {
                if let Some(w) = state.windows.get_mut(&wid) {
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

/// Enter command-line mode.
fn dispatch_enter_cmdline(state: &mut EditorState, prefix: char) {
    state.cmdline.text.clear();
    state.cmdline.cursor = 0;
    state.cmdline.prefix = prefix;
    state.cmdline.history_idx = None;
    state.cmdline.saved_text = None;
    state.mode.transition(Mode::Command);
    state.parser.reset();
}

fn dispatch_repeat_last(state: &mut EditorState) {
    if let Some(last) = state.last_change.clone() {
        let saved = state.last_change.take();
        dispatch_intent(state, last);
        state.last_change = saved;
    }
}

/// Process a single intent, mutating editor state.
pub fn dispatch_intent(state: &mut EditorState, intent: Intent) {
    let was_insert_normal = state.mode.current() == Mode::InsertNormal;
    let intent_ref = intent.clone();
    // Record intents for macro recording (except toggle-record itself).
    if state.macro_recording.is_some() && !matches!(intent, Intent::MacroToggleRecord(_)) {
        if let Some((_, ref mut intents)) = state.macro_recording { intents.push(intent.clone()); }
    }
    if is_repeatable(&intent) { state.last_change = Some(intent.clone()); push_change(state); }
    if is_jump(&intent) { push_jump(state); }
    match intent {
        Intent::Noop => {}
        Intent::Motion(kind, count) => dispatch_motion(state, kind, count),
        Intent::EnterMode(mode) => dispatch_enter_mode(state, mode),
        Intent::EnterInsert(pos) => dispatch_enter_insert(state, pos),
        Intent::InsertChar(c) => dispatch_insert_char(state, c),
        Intent::InsertNewline => dispatch_insert_newline(state),
        Intent::DeleteCharBefore => dispatch_delete_char_before(state),
        Intent::DeleteCharAt => dispatch_delete_char_at(state),
        Intent::DeleteWordBefore => dispatch_delete_word_before(state),
        Intent::DeleteToLineStart => dispatch_delete_to_line_start(state),
        Intent::InsertFromRegister(reg) => dispatch_insert_from_register(state, reg),
        Intent::InsertDigraph(c1, c2) => dispatch_insert_digraph(state, c1, c2),
        Intent::Operator(op, m, c) => dispatch_operator(state, op, m, c),
        Intent::OperatorTextObject(op, k, inner) => dispatch_operator_text_object(state, op, k, inner),
        Intent::LineOperator(op, c) => dispatch_line_operator(state, op, c),
        Intent::Undo => dispatch_undo(state),
        Intent::Redo => dispatch_redo(state),
        Intent::Paste(_, pos) => dispatch_paste(state, pos),
        Intent::YankLine(count) => dispatch_yank_line(state, count),
        Intent::DeleteToEnd => dispatch_delete_to_end(state),
        Intent::ChangeToEnd => { dispatch_delete_to_end(state); state.mode.transition(Mode::Insert); }
        Intent::OpenLine(below) => dispatch_open_line(state, below),
        Intent::JoinLines(sp, c) => dispatch_join_lines(state, sp, c),
        Intent::ReplaceChar(c) => dispatch_replace_char(state, c),
        Intent::Scroll(kind) => dispatch_scroll(state, kind),
        Intent::ExCommand(cmd) => crate::commands::dispatch_ex_command(state, &cmd),
        Intent::ToggleCase => dispatch_toggle_case(state),
        Intent::SubstituteChar => { dispatch_delete_char_at(state); state.mode.transition(Mode::Insert); }
        Intent::SubstituteLine => dispatch_line_operator(state, OperatorKind::Change, 1),
        Intent::ReplaceInsert(c) => dispatch_replace_insert(state, c),
        Intent::Indent(indent, c) => dispatch_indent(state, indent, c),
        Intent::SearchForward(pat) => dispatch_search_forward(state, &pat),
        Intent::SearchBackward(pat) => dispatch_search_backward(state, &pat),
        Intent::SearchNext => dispatch_search_next(state),
        Intent::SearchPrev => dispatch_search_prev(state),
        Intent::SearchWordForward => dispatch_search_word_forward(state),
        Intent::SearchWordBackward => dispatch_search_word_backward(state),
        Intent::SetMark(c) => dispatch_set_mark(state, c),
        Intent::JumpToMark(c) => dispatch_jump_to_mark(state, c),
        Intent::JumpToMarkLine(c) => dispatch_jump_to_mark_line(state, c),
        Intent::FindChar(c, kind) => dispatch_find_char(state, c, kind),
        Intent::RepeatFindChar => dispatch_repeat_find_char(state),
        Intent::RepeatFindCharReverse => dispatch_repeat_find_char_reverse(state),
        Intent::CaseOperator(op, m, c) => dispatch_case_operator(state, op, m, c),
        Intent::CaseOperatorLine(op) => dispatch_case_operator_line(state, op),
        Intent::VisualSwapEnd => dispatch_visual_swap_end(state),
        Intent::SelectRegister(reg) => dispatch_select_register(state, reg),
        Intent::IncrementNumber(d) => dispatch_increment_number(state, d),
        Intent::MacroToggleRecord(reg) => dispatch_macro_toggle(state, reg),
        Intent::MacroPlay(reg) => dispatch_macro_play(state, reg),
        Intent::MacroRepeatLast => dispatch_macro_repeat_last(state),
        Intent::RepeatLastChange => dispatch_repeat_last(state),
        Intent::JumpListBack => dispatch_jump_back(state),
        Intent::JumpListForward => dispatch_jump_forward(state),
        Intent::ChangeListOlder => dispatch_change_older(state),
        Intent::ChangeListNewer => dispatch_change_newer(state),
        Intent::WindowSplitHorizontal => dispatch_window_split_horizontal(state),
        Intent::WindowSplitVertical => dispatch_window_split_vertical(state),
        Intent::WindowClose => dispatch_window_close(state),
        Intent::WindowOnly => dispatch_window_only(state),
        Intent::WindowFocusNext => dispatch_window_focus_next(state),
        Intent::WindowFocusPrev => dispatch_window_focus_prev(state),
        Intent::WindowFocusDirection(d) => dispatch_window_focus_direction(state, d),
        Intent::WindowEqualSize => dispatch_window_equal_size(state),
        Intent::WindowRotate => dispatch_window_rotate(state),
        Intent::EnterCommandLine(prefix) => dispatch_enter_cmdline(state, prefix),
        Intent::ReselectVisual => dispatch_reselect_visual(state),
        Intent::ShellCommand(cmd) => dispatch_shell_command(state, &cmd),
        Intent::PutRegister(before) => dispatch_put_register(state, before),
        Intent::SwitchAlternate => crate::commands_buffer::dispatch_switch_alternate(state),
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

