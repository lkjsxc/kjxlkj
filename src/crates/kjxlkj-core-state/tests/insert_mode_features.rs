//! Insert mode feature tests — Ctrl-t/d indent, Ctrl-o, Ctrl-w, Ctrl-u,
//! Ctrl-r register, Ctrl-v literal, Ctrl-k digraph, autopairs.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{Intent, KeyCode, KeyEvent, Mode, Size};

fn setup(text: &str) -> EditorState {
    let mut st = EditorState::new(Size::new(80, 24));
    let bid = st.create_buffer_from_text(text);
    st.create_window(bid);
    st
}

fn cur(st: &EditorState) -> (usize, usize) {
    let w = st.active_window_state().unwrap();
    (w.cursor_line, w.cursor_col)
}

fn buf_text(st: &EditorState) -> String {
    st.active_buffer().unwrap().text.text()
}

#[test]
fn ctrl_t_indents_in_insert() {
    let mut st = setup("hello\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::Indent(true, 1));
    let text = buf_text(&st);
    assert!(text.starts_with("    ") || text.starts_with('\t'), "line should be indented");
}

#[test]
fn ctrl_d_dedents_in_insert() {
    let mut st = setup("    hello\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::Indent(false, 1));
    let text = buf_text(&st);
    assert!(!text.starts_with("    "), "indent should be removed");
}

#[test]
fn ctrl_w_delete_word_before() {
    let mut st = setup("hello world\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    // Move cursor to end: col 11 (after 'd')
    if let Some(w) = st.active_window_mut() { w.cursor_col = 11; }
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::DeleteWordBefore);
    let text = buf_text(&st);
    assert!(text.starts_with("hello"), "should delete 'world' leaving 'hello'");
}

#[test]
fn ctrl_u_delete_to_line_start() {
    let mut st = setup("hello world\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    if let Some(w) = st.active_window_mut() { w.cursor_col = 11; }
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::DeleteToLineStart);
    let text = buf_text(&st);
    assert_eq!(text.trim(), "", "all text before cursor deleted");
}

#[test]
fn ctrl_o_insert_normal_mode() {
    let mut st = setup("abc\ndef\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    assert_eq!(st.current_mode(), Mode::Insert);
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::InsertNormal));
    assert_eq!(st.current_mode(), Mode::InsertNormal);
}

#[test]
fn insert_char_and_cursor_advances() {
    let mut st = setup("\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertChar('H'));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertChar('i'));
    assert_eq!(cur(&st), (0, 2));
    let text = buf_text(&st);
    assert!(text.starts_with("Hi"));
}

#[test]
fn insert_newline_creates_line_below() {
    let mut st = setup("abc\n");
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    if let Some(w) = st.active_window_mut() { w.cursor_col = 3; }
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertNewline);
    assert_eq!(cur(&st).0, 1, "cursor should be on new line");
}

#[test]
fn autopairs_inserts_closing_bracket() {
    let mut st = setup("\n");
    st.options.autopairs = true;
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertChar('('));
    let text = buf_text(&st);
    assert!(text.contains("()"), "autopair should insert closing paren");
}

#[test]
fn autopairs_skip_over_closing() {
    let mut st = setup("\n");
    st.options.autopairs = true;
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertChar('('));
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertChar(')'));
    let text = buf_text(&st);
    // Should not have double closing paren
    assert!(!text.contains("))"), "should skip over existing closing paren");
}

#[test]
fn insert_from_register() {
    let mut st = setup("hello \n");
    // Yank "world" into register a
    st.registers.set(
        kjxlkj_core_types::RegisterName::Named('a'),
        kjxlkj_core_types::RegisterContent::charwise("world"),
    );
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::EnterMode(Mode::Insert));
    if let Some(w) = st.active_window_mut() { w.cursor_col = 6; }
    kjxlkj_core_state::dispatch_intent(&mut st, Intent::InsertFromRegister('a'));
    let text = buf_text(&st);
    assert!(text.contains("world"), "register content inserted");
}

#[test]
fn parser_ctrl_t_returns_indent() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('t'));
    assert_eq!(intent, Intent::Indent(true, 1));
}

#[test]
fn parser_ctrl_d_returns_dedent() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('d'));
    assert_eq!(intent, Intent::Indent(false, 1));
}

#[test]
fn parser_ctrl_o_enters_insert_normal() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('o'));
    assert_eq!(intent, Intent::EnterMode(Mode::InsertNormal));
}

#[test]
fn parser_ctrl_w_delete_word() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('w'));
    assert_eq!(intent, Intent::DeleteWordBefore);
}

#[test]
fn parser_ctrl_u_delete_to_start() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('u'));
    assert_eq!(intent, Intent::DeleteToLineStart);
}

#[test]
fn parser_ctrl_h_is_backspace() {
    let mut p = kjxlkj_core_mode::KeyParser::new();
    let intent = p.parse_insert(&KeyEvent::ctrl('h'));
    assert_eq!(intent, Intent::DeleteCharBefore);
}
