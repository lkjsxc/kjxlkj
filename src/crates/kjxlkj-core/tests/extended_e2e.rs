//! Additional E2E integration tests for the editor core.

use kjxlkj_core::state::*;
use kjxlkj_core::types::*;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn parse_key(s: &mut EditorState, key: KeyEvent) -> Intent {
    match s.current_mode() {
        Mode::Normal | Mode::OperatorPending | Mode::InsertNormal => s.parser.parse_normal(&key),
        Mode::Insert => s.parser.parse_insert(&key),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => s.parser.parse_visual(&key),
        Mode::Command => s.parser.parse_command(&key),
        Mode::Replace | Mode::Terminal => s.parser.parse_replace(&key),
    }
}

fn feed(s: &mut EditorState, key: KeyEvent) {
    let intent = parse_key(s, key);
    dispatch_intent(s, intent);
}

// ──────────── Multi-key editing sequences ────────────

#[test]
fn e2e_insert_word_then_normal() {
    let mut s = setup("");
    feed(&mut s, KeyEvent::char('i'));
    assert_eq!(s.current_mode(), Mode::Insert);
    for c in "rust".chars() {
        feed(&mut s, KeyEvent::char(c));
    }
    feed(&mut s, KeyEvent::special(KeyCode::Escape));
    assert_eq!(s.current_mode(), Mode::Normal);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "rust");
}

#[test]
fn e2e_navigate_and_delete() {
    let mut s = setup("abcdef");
    feed(&mut s, KeyEvent::char('l'));
    feed(&mut s, KeyEvent::char('l'));
    feed(&mut s, KeyEvent::char('x'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "abdef");
}

#[test]
fn e2e_dd_on_multiline() {
    let mut s = setup("first\nsecond\nthird");
    feed(&mut s, KeyEvent::char('j'));
    feed(&mut s, KeyEvent::char('d'));
    feed(&mut s, KeyEvent::char('d'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn e2e_yy_p() {
    let mut s = setup("only line");
    feed(&mut s, KeyEvent::char('Y'));
    feed(&mut s, KeyEvent::char('p'));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 2);
}

#[test]
fn e2e_o_insert_line() {
    let mut s = setup("above");
    feed(&mut s, KeyEvent::char('o'));
    assert_eq!(s.current_mode(), Mode::Insert);
    for c in "below".chars() {
        feed(&mut s, KeyEvent::char(c));
    }
    feed(&mut s, KeyEvent::special(KeyCode::Escape));
    assert_eq!(s.cursor().line, 1);
}

#[test]
fn e2e_big_o_insert_above() {
    let mut s = setup("below");
    feed(&mut s, KeyEvent::char('O'));
    assert_eq!(s.current_mode(), Mode::Insert);
    for c in "above".chars() {
        feed(&mut s, KeyEvent::char(c));
    }
    feed(&mut s, KeyEvent::special(KeyCode::Escape));
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn e2e_gg_goes_to_top() {
    let mut s = setup("a\nb\nc\nd\ne");
    feed(&mut s, KeyEvent::char('G'));
    assert!(s.cursor().line >= 4);
    feed(&mut s, KeyEvent::char('g'));
    feed(&mut s, KeyEvent::char('g'));
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn e2e_dollar_then_zero() {
    let mut s = setup("hello world");
    feed(&mut s, KeyEvent::char('$'));
    assert!(s.cursor().col >= 10);
    feed(&mut s, KeyEvent::char('0'));
    assert_eq!(s.cursor().col, 0);
}

#[test]
fn e2e_w_word_forward() {
    let mut s = setup("one two three");
    feed(&mut s, KeyEvent::char('w'));
    assert_eq!(s.cursor().col, 4);
    feed(&mut s, KeyEvent::char('w'));
    assert_eq!(s.cursor().col, 8);
}

#[test]
fn e2e_b_word_backward() {
    let mut s = setup("one two three");
    feed(&mut s, KeyEvent::char('$'));
    feed(&mut s, KeyEvent::char('b'));
    assert_eq!(s.cursor().col, 8);
}

#[test]
fn e2e_ctrl_d_scroll() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    feed(&mut s, KeyEvent::ctrl('d'));
    assert!(s.cursor().line > 0);
}

#[test]
fn e2e_ctrl_u_scroll() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    feed(&mut s, KeyEvent::ctrl('d'));
    feed(&mut s, KeyEvent::ctrl('u'));
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn e2e_join_lines_j() {
    let mut s = setup("hello\nworld");
    feed(&mut s, KeyEvent::char('J'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 1);
}

#[test]
fn e2e_tilde_toggle() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('~'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('A'));
}

#[test]
fn e2e_replace_mode() {
    let mut s = setup("abc");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Replace));
    assert_eq!(s.current_mode(), Mode::Replace);
    feed(&mut s, KeyEvent::char('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('X'));
}

#[test]
fn e2e_colon_q() {
    let mut s = setup("abc");
    dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
    assert!(s.should_quit);
}

#[test]
fn e2e_indent_line() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "    hello");
}

#[test]
fn e2e_substitute_char() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('s'));
    assert_eq!(s.current_mode(), Mode::Insert);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "bc");
}

#[test]
fn e2e_big_d_delete_to_end() {
    let mut s = setup("hello world");
    feed(&mut s, KeyEvent::char('l'));
    feed(&mut s, KeyEvent::char('l'));
    feed(&mut s, KeyEvent::char('D'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "he");
}

#[test]
fn e2e_big_c_change_to_end() {
    let mut s = setup("hello world");
    feed(&mut s, KeyEvent::char('C'));
    assert_eq!(s.current_mode(), Mode::Insert);
}

#[test]
fn e2e_paste_before() {
    let mut s = setup("xyz");
    s.registers.set(RegisterName::Unnamed, RegisterContent::charwise("AB"));
    feed(&mut s, KeyEvent::char('P'));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.contains("AB"));
    assert!(line.contains("xyz"));
}

#[test]
fn e2e_visual_mode_enter_exit() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('v'));
    assert_eq!(s.current_mode(), Mode::Visual);
    feed(&mut s, KeyEvent::special(KeyCode::Escape));
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_visual_line_mode() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('V'));
    assert_eq!(s.current_mode(), Mode::VisualLine);
}

#[test]
fn e2e_command_mode_enter_exit() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char(':'));
    assert_eq!(s.current_mode(), Mode::Command);
    feed(&mut s, KeyEvent::special(KeyCode::Escape));
    assert_eq!(s.current_mode(), Mode::Normal);
}

#[test]
fn e2e_backspace_in_insert() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('i'));
    feed(&mut s, KeyEvent::char('x'));
    feed(&mut s, KeyEvent::special(KeyCode::Backspace));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "abc");
}

#[test]
fn e2e_enter_in_insert() {
    let mut s = setup("abc");
    feed(&mut s, KeyEvent::char('i'));
    feed(&mut s, KeyEvent::special(KeyCode::Enter));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn e2e_5j_count_motions() {
    let text = (0..20).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    feed(&mut s, KeyEvent::char('5'));
    feed(&mut s, KeyEvent::char('j'));
    assert_eq!(s.cursor().line, 5);
}

#[test]
fn e2e_multiple_buffers() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid1 = s.create_buffer_from_text("buffer1");
    let bid2 = s.create_buffer_from_text("buffer2");
    s.create_window(bid1);
    assert_ne!(bid1, bid2);
}
