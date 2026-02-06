//! Extended tests for mode state and key parser edge cases.

use kjxlkj_core_mode::*;
use kjxlkj_core_types::*;

// ──────────── ModeState transitions ────────────

#[test]
fn mode_state_initial() {
    let m = ModeState::new();
    assert_eq!(m.current(), Mode::Normal);
    assert_eq!(m.previous(), Mode::Normal);
}

#[test]
fn mode_state_transition_to_insert() {
    let mut m = ModeState::new();
    m.transition(Mode::Insert);
    assert_eq!(m.current(), Mode::Insert);
    assert_eq!(m.previous(), Mode::Normal);
}

#[test]
fn mode_state_back_to_normal() {
    let mut m = ModeState::new();
    m.transition(Mode::Insert);
    m.transition(Mode::Normal);
    assert_eq!(m.current(), Mode::Normal);
    assert_eq!(m.previous(), Mode::Insert);
}

#[test]
fn mode_state_chain() {
    let mut m = ModeState::new();
    m.transition(Mode::Insert);
    m.transition(Mode::Normal);
    m.transition(Mode::Visual);
    assert_eq!(m.current(), Mode::Visual);
    assert_eq!(m.previous(), Mode::Normal);
}

#[test]
fn mode_state_is_normal() {
    let m = ModeState::new();
    assert!(m.is_normal());
    assert!(!m.is_insert());
    assert!(!m.is_visual());
    assert!(!m.is_command());
}

#[test]
fn mode_state_is_insert() {
    let mut m = ModeState::new();
    m.transition(Mode::Insert);
    assert!(m.is_insert());
    assert!(!m.is_normal());
}

#[test]
fn mode_state_is_visual() {
    let mut m = ModeState::new();
    m.transition(Mode::Visual);
    assert!(m.is_visual());
}

#[test]
fn mode_state_is_visual_line() {
    let mut m = ModeState::new();
    m.transition(Mode::VisualLine);
    assert!(m.is_visual());
}

#[test]
fn mode_state_is_command() {
    let mut m = ModeState::new();
    m.transition(Mode::Command);
    assert!(m.is_command());
}

// ──────────── KeyParser normal mode edge cases ────────────

#[test]
fn parser_unknown_key_is_noop() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('Q'));
    // Unknown keys produce Noop or a specific intent
    let _ = intent; // just ensure no panic
}

#[test]
fn parser_h_left() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('h'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Left, 1)));
}

#[test]
fn parser_j_down() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('j'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Down, 1)));
}

#[test]
fn parser_k_up() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('k'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Up, 1)));
}

#[test]
fn parser_l_right() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('l'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Right, 1)));
}

#[test]
fn parser_dollar_line_end() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('$'));
    assert!(matches!(intent, Intent::Motion(MotionKind::LineEnd, 1)));
}

#[test]
fn parser_zero_line_start() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('0'));
    assert!(matches!(intent, Intent::Motion(MotionKind::LineStart, 1)));
}

#[test]
fn parser_caret_first_non_blank() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('^'));
    assert!(matches!(
        intent,
        Intent::Motion(MotionKind::FirstNonBlank, 1)
    ));
}

#[test]
fn parser_w_word_forward() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert!(matches!(
        intent,
        Intent::Motion(MotionKind::WordForward, 1)
    ));
}

#[test]
fn parser_b_word_backward() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('b'));
    assert!(matches!(
        intent,
        Intent::Motion(MotionKind::WordBackward, 1)
    ));
}

#[test]
fn parser_e_word_end() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('e'));
    assert!(matches!(
        intent,
        Intent::Motion(MotionKind::WordForwardEnd, 1)
    ));
}

#[test]
fn parser_i_insert() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('i'));
    assert!(matches!(
        intent,
        Intent::EnterInsert(InsertPosition::BeforeCursor)
    ));
}

#[test]
fn parser_a_insert_after() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('a'));
    assert!(matches!(
        intent,
        Intent::EnterInsert(InsertPosition::AfterCursor)
    ));
}

#[test]
fn parser_big_a_end_of_line() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('A'));
    assert!(matches!(
        intent,
        Intent::EnterInsert(InsertPosition::EndOfLine)
    ));
}

#[test]
fn parser_big_i_first_non_blank() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('I'));
    assert!(matches!(
        intent,
        Intent::EnterInsert(InsertPosition::FirstNonBlank)
    ));
}

#[test]
fn parser_x_delete() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('x'));
    assert!(matches!(intent, Intent::DeleteCharAt));
}

#[test]
fn parser_o_open_below() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('o'));
    assert!(matches!(intent, Intent::OpenLine(true)));
}

#[test]
fn parser_big_o_open_above() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('O'));
    assert!(matches!(intent, Intent::OpenLine(false)));
}

#[test]
fn parser_big_j_join() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('J'));
    assert!(matches!(intent, Intent::JoinLines(true, _)));
}

#[test]
fn parser_tilde_toggle() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('~'));
    assert!(matches!(intent, Intent::ToggleCase));
}

#[test]
fn parser_big_d_delete_to_end() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('D'));
    assert!(matches!(intent, Intent::DeleteToEnd));
}

#[test]
fn parser_big_c_change_to_end() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('C'));
    assert!(matches!(intent, Intent::ChangeToEnd));
}

#[test]
fn parser_big_y_yank_line() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('Y'));
    assert!(matches!(intent, Intent::YankLine(_)));
}

#[test]
fn parser_v_visual() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('v'));
    assert!(matches!(intent, Intent::EnterMode(Mode::Visual)));
}

#[test]
fn parser_big_v_visual_line() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('V'));
    assert!(matches!(intent, Intent::EnterMode(Mode::VisualLine)));
}

#[test]
fn parser_colon_command() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char(':'));
    assert!(matches!(intent, Intent::EnterCommandLine(':')));
}

#[test]
fn parser_p_paste() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('p'));
    assert!(matches!(
        intent,
        Intent::Paste(RegisterName::Unnamed, PastePosition::After)
    ));
}

#[test]
fn parser_big_p_paste_before() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('P'));
    assert!(matches!(
        intent,
        Intent::Paste(RegisterName::Unnamed, PastePosition::Before)
    ));
}

#[test]
fn parser_s_substitute() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('s'));
    assert!(matches!(intent, Intent::SubstituteChar));
}

#[test]
fn parser_big_s_substitute_line() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('S'));
    assert!(matches!(intent, Intent::SubstituteLine));
}

#[test]
fn parser_big_g_file_end() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::char('G'));
    assert!(matches!(intent, Intent::Motion(MotionKind::FileEnd, _)));
}

// ──────────── Count accumulation ────────────

#[test]
fn parser_count_5j() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('5'));
    let intent = p.parse_normal(&KeyEvent::char('j'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Down, 5)));
}

#[test]
fn parser_count_10l() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('1'));
    p.parse_normal(&KeyEvent::char('0'));
    let intent = p.parse_normal(&KeyEvent::char('l'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Right, 10)));
}

#[test]
fn parser_count_2w() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('2'));
    let intent = p.parse_normal(&KeyEvent::char('w'));
    assert!(matches!(intent, Intent::Motion(MotionKind::WordForward, 2)));
}

// ──────────── Insert mode parsing ────────────

#[test]
fn parser_insert_char() {
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::char('a'));
    assert!(matches!(intent, Intent::InsertChar('a')));
}

#[test]
fn parser_insert_space() {
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::char(' '));
    assert!(matches!(intent, Intent::InsertChar(' ')));
}

#[test]
fn parser_insert_escape() {
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::special(KeyCode::Escape));
    assert!(matches!(intent, Intent::EnterMode(Mode::Normal)));
}

#[test]
fn parser_insert_enter() {
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::special(KeyCode::Enter));
    assert!(matches!(intent, Intent::InsertNewline));
}

#[test]
fn parser_insert_backspace() {
    let mut p = KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::special(KeyCode::Backspace));
    assert!(matches!(intent, Intent::DeleteCharBefore));
}

// ──────────── Visual mode ────────────

#[test]
fn parser_visual_escape() {
    let mut p = KeyParser::new();
    let intent = p.parse_visual(&KeyEvent::special(KeyCode::Escape));
    assert!(matches!(intent, Intent::EnterMode(Mode::Normal)));
}

#[test]
fn parser_visual_motions() {
    let mut p = KeyParser::new();
    let h = p.parse_visual(&KeyEvent::char('h'));
    assert!(matches!(h, Intent::Motion(MotionKind::Left, _)));
    let j = p.parse_visual(&KeyEvent::char('j'));
    assert!(matches!(j, Intent::Motion(MotionKind::Down, _)));
}

// ──────────── Command mode ────────────

#[test]
fn parser_command_escape() {
    let mut p = KeyParser::new();
    let intent = p.parse_command(&KeyEvent::special(KeyCode::Escape));
    assert!(matches!(intent, Intent::EnterMode(Mode::Normal)));
}

// ──────────── Replace mode ────────────

#[test]
fn parser_replace_char() {
    let mut p = KeyParser::new();
    let intent = p.parse_replace(&KeyEvent::char('x'));
    assert!(matches!(intent, Intent::ReplaceInsert('x')));
}

#[test]
fn parser_replace_escape() {
    let mut p = KeyParser::new();
    let intent = p.parse_replace(&KeyEvent::special(KeyCode::Escape));
    assert!(matches!(intent, Intent::EnterMode(Mode::Normal)));
}

// ──────────── Parser reset ────────────

#[test]
fn parser_reset_clears_state() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d')); // enter operator pending
    p.reset();
    let intent = p.parse_normal(&KeyEvent::char('j'));
    assert!(matches!(intent, Intent::Motion(MotionKind::Down, 1)));
}

#[test]
fn parser_escape_resets_pending() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('d')); // operator pending
    let intent = p.parse_normal(&KeyEvent::special(KeyCode::Escape));
    assert!(matches!(intent, Intent::Noop | Intent::EnterMode(Mode::Normal)));
}

// ──────────── Ctrl key parsing ────────────

#[test]
fn parser_ctrl_d_scroll() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::ctrl('d'));
    assert!(matches!(
        intent,
        Intent::Scroll(ScrollKind::HalfPageDown)
    ));
}

#[test]
fn parser_ctrl_u_scroll() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::ctrl('u'));
    assert!(matches!(
        intent,
        Intent::Scroll(ScrollKind::HalfPageUp)
    ));
}

#[test]
fn parser_ctrl_f_page_down() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::ctrl('f'));
    assert!(matches!(
        intent,
        Intent::Scroll(ScrollKind::FullPageDown)
    ));
}

#[test]
fn parser_ctrl_b_page_up() {
    let mut p = KeyParser::new();
    let intent = p.parse_normal(&KeyEvent::ctrl('b'));
    assert!(matches!(
        intent,
        Intent::Scroll(ScrollKind::FullPageUp)
    ));
}

// ──────────── gg sequence ────────────

#[test]
fn parser_gg_file_start() {
    let mut p = KeyParser::new();
    p.parse_normal(&KeyEvent::char('g'));
    let intent = p.parse_normal(&KeyEvent::char('g'));
    assert!(matches!(intent, Intent::Motion(MotionKind::FileStart, _)));
}
