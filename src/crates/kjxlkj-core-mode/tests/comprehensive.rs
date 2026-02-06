//! Comprehensive tests for core-mode state and parser.

use kjxlkj_core_mode::*;
use kjxlkj_core_types::*;

// ═══════════ ModeState tests ═══════════

#[test]
fn mode_state_initial() {
    let s = ModeState::new();
    assert_eq!(s.current(), Mode::Normal);
    assert!(s.is_normal());
    assert!(!s.is_insert());
    assert!(!s.is_visual());
    assert!(!s.is_command());
}

#[test]
fn mode_state_transition_to_insert() {
    let mut s = ModeState::new();
    s.transition(Mode::Insert);
    assert!(s.is_insert());
    assert_eq!(s.previous(), Mode::Normal);
}

#[test]
fn mode_state_transition_chain() {
    let mut s = ModeState::new();
    s.transition(Mode::Insert);
    s.transition(Mode::Normal);
    s.transition(Mode::Visual);
    assert!(s.is_visual());
    assert_eq!(s.previous(), Mode::Normal);
}

#[test]
fn mode_state_command_mode() {
    let mut s = ModeState::new();
    s.transition(Mode::Command);
    assert!(s.is_command());
}

#[test]
fn mode_state_visual_line() {
    let mut s = ModeState::new();
    s.transition(Mode::VisualLine);
    assert!(s.is_visual());
}

#[test]
fn mode_state_visual_block() {
    let mut s = ModeState::new();
    s.transition(Mode::VisualBlock);
    assert!(s.is_visual());
}

#[test]
fn mode_state_replace() {
    let mut s = ModeState::new();
    s.transition(Mode::Replace);
    assert!(!s.is_normal());
    assert!(!s.is_insert());
}

#[test]
fn mode_state_default() {
    let s = ModeState::default();
    assert_eq!(s.current(), Mode::Normal);
}

// ═══════════ KeyParser Normal Mode tests ═══════════

#[test]
fn parser_h_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('h')),
        Intent::Motion(MotionKind::Left, 1)
    );
}

#[test]
fn parser_j_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 1)
    );
}

#[test]
fn parser_k_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('k')),
        Intent::Motion(MotionKind::Up, 1)
    );
}

#[test]
fn parser_l_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('l')),
        Intent::Motion(MotionKind::Right, 1)
    );
}

#[test]
fn parser_w_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('w')),
        Intent::Motion(MotionKind::WordForward, 1)
    );
}

#[test]
fn parser_big_w_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('W')),
        Intent::Motion(MotionKind::WORDForward, 1)
    );
}

#[test]
fn parser_b_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('b')),
        Intent::Motion(MotionKind::WordBackward, 1)
    );
}

#[test]
fn parser_e_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('e')),
        Intent::Motion(MotionKind::WordForwardEnd, 1)
    );
}

#[test]
fn parser_dollar_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('$')),
        Intent::Motion(MotionKind::LineEnd, 1)
    );
}

#[test]
fn parser_caret_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('^')),
        Intent::Motion(MotionKind::FirstNonBlank, 1)
    );
}

#[test]
fn parser_zero_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('0')),
        Intent::Motion(MotionKind::LineStart, 1)
    );
}

#[test]
fn parser_big_g_motion() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('G')),
        Intent::Motion(MotionKind::FileEnd, 1)
    );
}

#[test]
fn parser_percent_matching() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('%')),
        Intent::Motion(MotionKind::MatchingBracket, 1)
    );
}

#[test]
fn parser_curly_brace_paragraphs() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('{')),
        Intent::Motion(MotionKind::PrevParagraph, 1)
    );
    assert_eq!(
        p.parse_normal(&KeyEvent::char('}')),
        Intent::Motion(MotionKind::NextParagraph, 1)
    );
}

#[test]
fn parser_pipe_goto_column() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('|')),
        Intent::Motion(MotionKind::GotoColumn(1), 1)
    );
}

// ──────────── Count prefix ────────────

#[test]
fn parser_count_single_digit() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('5')), Intent::Noop);
    assert_eq!(
        p.parse_normal(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 5)
    );
}

#[test]
fn parser_count_multi_digit() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('1'));
    p.parse_normal(&KeyEvent::char('2'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('l')),
        Intent::Motion(MotionKind::Right, 12)
    );
}

#[test]
fn parser_count_with_g_motion() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('5'));
    p.parse_normal(&KeyEvent::char('G'));
    // 5G = go to line 5
}

#[test]
fn parser_count_with_percent() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('5'));
    p.parse_normal(&KeyEvent::char('0'));
    let result = p.parse_normal(&KeyEvent::char('G'));
    assert_eq!(result, Intent::Motion(MotionKind::GotoLine(50), 1));
}

// ──────────── Mode entry ────────────

#[test]
fn parser_i_enters_insert() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('i')),
        Intent::EnterInsert(InsertPosition::BeforeCursor)
    );
}

#[test]
fn parser_big_i_enters_insert_first_nonblank() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('I')),
        Intent::EnterInsert(InsertPosition::FirstNonBlank)
    );
}

#[test]
fn parser_a_enters_insert_after() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::EnterInsert(InsertPosition::AfterCursor)
    );
}

#[test]
fn parser_big_a_enters_insert_eol() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('A')),
        Intent::EnterInsert(InsertPosition::EndOfLine)
    );
}

#[test]
fn parser_v_enters_visual() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('v')),
        Intent::EnterMode(Mode::Visual)
    );
}

#[test]
fn parser_big_v_enters_visual_line() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('V')),
        Intent::EnterMode(Mode::VisualLine)
    );
}

#[test]
fn parser_ctrl_v_enters_visual_block() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('v')),
        Intent::EnterMode(Mode::VisualBlock)
    );
}

#[test]
fn parser_colon_enters_command() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char(':')),
        Intent::EnterCommandLine(':')
    );
}

#[test]
fn parser_big_r_enters_replace() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('R')),
        Intent::EnterMode(Mode::Replace)
    );
}

#[test]
fn parser_o_opens_line_below() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('o')),
        Intent::OpenLine(true)
    );
}

#[test]
fn parser_big_o_opens_line_above() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('O')),
        Intent::OpenLine(false)
    );
}

// ──────────── Operators (d, y, c) ────────────

#[test]
fn parser_dd_line_delete() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('d')),
        Intent::LineOperator(OperatorKind::Delete, 1)
    );
}

#[test]
fn parser_yy_line_yank() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('y'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('y')),
        Intent::LineOperator(OperatorKind::Yank, 1)
    );
}

#[test]
fn parser_cc_line_change() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('c'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('c')),
        Intent::LineOperator(OperatorKind::Change, 1)
    );
}

#[test]
fn parser_indent_double() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('>'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('>')),
        Intent::LineOperator(OperatorKind::Indent, 1)
    );
}

#[test]
fn parser_outdent_double() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('<'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('<')),
        Intent::LineOperator(OperatorKind::Outdent, 1)
    );
}

#[test]
fn parser_dw_delete_word() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('w')),
        Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1)
    );
}

#[test]
fn parser_d_dollar_delete_to_eol() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('$')),
        Intent::Operator(OperatorKind::Delete, MotionKind::LineEnd, 1)
    );
}

#[test]
fn parser_cw_change_word() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('c'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('w')),
        Intent::Operator(OperatorKind::Change, MotionKind::WordForward, 1)
    );
}

#[test]
fn parser_yw_yank_word() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('y'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('w')),
        Intent::Operator(OperatorKind::Yank, MotionKind::WordForward, 1)
    );
}

#[test]
fn parser_3dd_counted_line_delete() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('3'));
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('d')),
        Intent::LineOperator(OperatorKind::Delete, 3)
    );
}

#[test]
fn parser_operator_escape_cancels() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d'));
    assert_eq!(
        p.parse_normal(&KeyEvent::special(KeyCode::Escape)),
        Intent::Noop
    );
    // Parser should be reset; next 'j' should work normally
    assert_eq!(
        p.parse_normal(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 1)
    );
}

// ──────────── Delete / change shortcuts ────────────

#[test]
fn parser_x_delete_char() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('x')), Intent::DeleteCharAt);
}

#[test]
fn parser_big_x_delete_char_before() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('X')),
        Intent::DeleteCharBefore
    );
}

#[test]
fn parser_big_d_delete_to_end() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('D')),
        Intent::DeleteToEnd
    );
}

#[test]
fn parser_big_c_change_to_end() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('C')),
        Intent::ChangeToEnd
    );
}

#[test]
fn parser_s_substitute_char() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('s')),
        Intent::SubstituteChar
    );
}

#[test]
fn parser_big_s_substitute_line() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('S')),
        Intent::SubstituteLine
    );
}

// ──────────── g-sequences ────────────

#[test]
fn parser_gg_file_start() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('g')),
        Intent::Motion(MotionKind::FileStart, 1)
    );
}

#[test]
fn parser_g_underscore_last_nonblank() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('_')),
        Intent::Motion(MotionKind::LastNonBlank, 1)
    );
}

#[test]
fn parser_gm_middle_of_line() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('m')),
        Intent::Motion(MotionKind::MiddleOfLine, 1)
    );
}

#[test]
fn parser_g_big_j_join_no_space() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('J')),
        Intent::JoinLines(false, 1)
    );
}

#[test]
fn parser_gp_paste_after_cursor_end() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('p')),
        Intent::Paste(RegisterName::Unnamed, PastePosition::AfterCursorEnd)
    );
}

// ──────────── z-sequences ────────────

#[test]
fn parser_zz_cursor_center() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('z'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('z')),
        Intent::Scroll(ScrollKind::CursorCenter)
    );
}

#[test]
fn parser_zt_cursor_top() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('z'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('t')),
        Intent::Scroll(ScrollKind::CursorTop)
    );
}

#[test]
fn parser_zb_cursor_bottom() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('z'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('b')),
        Intent::Scroll(ScrollKind::CursorBottom)
    );
}

// ──────────── find char motions ────────────

#[test]
fn parser_f_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('f'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::FindChar('a', FindCharKind::Forward)
    );
}

#[test]
fn parser_big_f_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('F'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('x')),
        Intent::FindChar('x', FindCharKind::Backward)
    );
}

#[test]
fn parser_t_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('t'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('z')),
        Intent::FindChar('z', FindCharKind::TillForward)
    );
}

#[test]
fn parser_big_t_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('T'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('q')),
        Intent::FindChar('q', FindCharKind::TillBackward)
    );
}

#[test]
fn parser_semicolon_repeat_find() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char(';')),
        Intent::RepeatFindChar
    );
}

#[test]
fn parser_comma_reverse_find() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char(',')),
        Intent::RepeatFindCharReverse
    );
}

// ──────────── register selection ────────────

#[test]
fn parser_register_selection() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('"'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::SelectRegister(RegisterName::Named('a'))
    );
}

#[test]
fn parser_register_clipboard() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('"'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('+')),
        Intent::SelectRegister(RegisterName::Clipboard)
    );
}

// ──────────── marks ────────────

#[test]
fn parser_set_mark() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('m'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::SetMark('a')
    );
}

#[test]
fn parser_jump_mark_exact() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('`'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::JumpToMark('a')
    );
}

#[test]
fn parser_jump_mark_line() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('\''));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::JumpToMarkLine('a')
    );
}

// ──────────── macros ────────────

#[test]
fn parser_macro_record() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('q'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::MacroToggleRecord('a')
    );
}

#[test]
fn parser_macro_play() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('@'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('a')),
        Intent::MacroPlay('a')
    );
}

#[test]
fn parser_macro_repeat_last() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('@'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('@')),
        Intent::MacroRepeatLast
    );
}

// ──────────── replace char ────────────

#[test]
fn parser_replace_char() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('r'));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('X')),
        Intent::ReplaceChar('X')
    );
}

// ──────────── undo / redo / repeat ────────────

#[test]
fn parser_undo() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('u')), Intent::Undo);
}

#[test]
fn parser_redo() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::ctrl('r')), Intent::Redo);
}

#[test]
fn parser_repeat_last_change() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('.')),
        Intent::RepeatLastChange
    );
}

// ──────────── misc normal mode ────────────

#[test]
fn parser_paste_p() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('p')),
        Intent::Paste(RegisterName::Unnamed, PastePosition::After)
    );
}

#[test]
fn parser_paste_big_p() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('P')),
        Intent::Paste(RegisterName::Unnamed, PastePosition::Before)
    );
}

#[test]
fn parser_tilde_toggle_case() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('~')), Intent::ToggleCase);
}

#[test]
fn parser_big_j_join() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('J')),
        Intent::JoinLines(true, 1)
    );
}

#[test]
fn parser_big_y_yank_line() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('Y')),
        Intent::YankLine(1)
    );
}

#[test]
fn parser_search_star() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('*')),
        Intent::SearchWordForward
    );
}

#[test]
fn parser_search_hash() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('#')),
        Intent::SearchWordBackward
    );
}

#[test]
fn parser_n_search_next() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('n')), Intent::SearchNext);
}

#[test]
fn parser_big_n_search_prev() {
    let mut p = KeyParser::new();
    assert_eq!(p.parse_normal(&KeyEvent::char('N')), Intent::SearchPrev);
}

// ──────────── ctrl keys ────────────

#[test]
fn parser_ctrl_d_half_page_down() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('d')),
        Intent::Scroll(ScrollKind::HalfPageDown)
    );
}

#[test]
fn parser_ctrl_u_half_page_up() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('u')),
        Intent::Scroll(ScrollKind::HalfPageUp)
    );
}

#[test]
fn parser_ctrl_f_full_page_down() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('f')),
        Intent::Scroll(ScrollKind::FullPageDown)
    );
}

#[test]
fn parser_ctrl_b_full_page_up() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('b')),
        Intent::Scroll(ScrollKind::FullPageUp)
    );
}

#[test]
fn parser_ctrl_e_line_down() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('e')),
        Intent::Scroll(ScrollKind::LineDown)
    );
}

#[test]
fn parser_ctrl_y_line_up() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('y')),
        Intent::Scroll(ScrollKind::LineUp)
    );
}

#[test]
fn parser_ctrl_o_jumplist_back() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('o')),
        Intent::JumpListBack
    );
}

#[test]
fn parser_ctrl_i_jumplist_forward() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_normal(&KeyEvent::ctrl('i')),
        Intent::JumpListForward
    );
}

// ──────────── leader key ────────────

#[test]
fn parser_leader_e_explorer() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char(' '));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('e')),
        Intent::ExCommand(":explorer".into())
    );
}

#[test]
fn parser_leader_t_terminal() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char(' '));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('t')),
        Intent::ExCommand(":terminal".into())
    );
}

#[test]
fn parser_leader_f_find() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char(' '));
    assert_eq!(
        p.parse_normal(&KeyEvent::char('f')),
        Intent::ExCommand(":find".into())
    );
}

// ═══════════ Insert Mode tests ═══════════

#[test]
fn parser_insert_char() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::char('x')),
        Intent::InsertChar('x')
    );
}

#[test]
fn parser_insert_escape() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Escape)),
        Intent::EnterMode(Mode::Normal)
    );
}

#[test]
fn parser_insert_enter() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Enter)),
        Intent::InsertNewline
    );
}

#[test]
fn parser_insert_backspace() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Backspace)),
        Intent::DeleteCharBefore
    );
}

#[test]
fn parser_insert_delete() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Delete)),
        Intent::DeleteCharAt
    );
}

#[test]
fn parser_insert_tab() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Tab)),
        Intent::InsertChar('\t')
    );
}

#[test]
fn parser_insert_arrow_keys() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Left)),
        Intent::Motion(MotionKind::Left, 1)
    );
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Right)),
        Intent::Motion(MotionKind::Right, 1)
    );
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Up)),
        Intent::Motion(MotionKind::Up, 1)
    );
    assert_eq!(
        p.parse_insert(&KeyEvent::special(KeyCode::Down)),
        Intent::Motion(MotionKind::Down, 1)
    );
}

#[test]
fn parser_insert_ctrl_h_backspace() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::ctrl('h')),
        Intent::DeleteCharBefore
    );
}

#[test]
fn parser_insert_ctrl_j_newline() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_insert(&KeyEvent::ctrl('j')),
        Intent::InsertNewline
    );
}

// ═══════════ Visual Mode tests ═══════════

#[test]
fn parser_visual_motions() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('h')),
        Intent::Motion(MotionKind::Left, 1)
    );
    assert_eq!(
        p.parse_visual(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 1)
    );
    assert_eq!(
        p.parse_visual(&KeyEvent::char('k')),
        Intent::Motion(MotionKind::Up, 1)
    );
    assert_eq!(
        p.parse_visual(&KeyEvent::char('l')),
        Intent::Motion(MotionKind::Right, 1)
    );
}

#[test]
fn parser_visual_d_deletes() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('d')),
        Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1)
    );
}

#[test]
fn parser_visual_y_yanks() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('y')),
        Intent::Operator(OperatorKind::Yank, MotionKind::Right, 1)
    );
}

#[test]
fn parser_visual_c_changes() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('c')),
        Intent::Operator(OperatorKind::Change, MotionKind::Right, 1)
    );
}

#[test]
fn parser_visual_escape() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::special(KeyCode::Escape)),
        Intent::EnterMode(Mode::Normal)
    );
}

#[test]
fn parser_visual_indent_outdent() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('>')),
        Intent::Indent(true, 1)
    );
    assert_eq!(
        p.parse_visual(&KeyEvent::char('<')),
        Intent::Indent(false, 1)
    );
}

#[test]
fn parser_visual_o_swap() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_visual(&KeyEvent::char('o')),
        Intent::VisualSwapEnd
    );
}

// ═══════════ Command Mode tests ═══════════

#[test]
fn parser_command_escape() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_command(&KeyEvent::special(KeyCode::Escape)),
        Intent::EnterMode(Mode::Normal)
    );
}

// ═══════════ Replace Mode tests ═══════════

#[test]
fn parser_replace_char_input() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_replace(&KeyEvent::char('a')),
        Intent::ReplaceInsert('a')
    );
}

#[test]
fn parser_replace_escape() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_replace(&KeyEvent::special(KeyCode::Escape)),
        Intent::EnterMode(Mode::Normal)
    );
}

#[test]
fn parser_replace_backspace() {
    let mut p = KeyParser::new();
    assert_eq!(
        p.parse_replace(&KeyEvent::special(KeyCode::Backspace)),
        Intent::Motion(MotionKind::Left, 1)
    );
}

// ═══════════ Reset tests ═══════════

#[test]
fn parser_reset_clears_state() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d')); // Enter operator pending
    p.reset(); // Clear
    assert_eq!(
        p.parse_normal(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 1) // Should be plain motion
    );
}

#[test]
fn parser_reset_clears_count() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('5')); // Start count
    p.reset();
    assert_eq!(
        p.parse_normal(&KeyEvent::char('j')),
        Intent::Motion(MotionKind::Down, 1) // count should be 1 not 5
    );
}
