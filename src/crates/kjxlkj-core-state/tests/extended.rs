//! Extended dispatch and state management tests.

use kjxlkj_core_state::*;
use kjxlkj_core_types::*;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ──────────── Multiple motion sequences ────────────

#[test]
fn motion_right_left_right() {
    let mut s = setup("abcdef");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Left, 1));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    assert_eq!(s.cursor().col, 4);
}

#[test]
fn motion_navigate_grid() {
    let mut s = setup("abc\ndef\nghi");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 2));
    assert_eq!(s.cursor(), Position::new(2, 2));
}

#[test]
fn motion_line_end_then_down() {
    let mut s = setup("long line here\nshort");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    assert!(s.cursor().col > 5);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    // Cursor col should be clamped to shorter line
}

// ──────────── Insert with positions ────────────

#[test]
fn insert_at_beginning_of_buffer() {
    let mut s = setup("world");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('h'));
    dispatch_intent(&mut s, Intent::InsertChar('i'));
    dispatch_intent(&mut s, Intent::InsertChar(' '));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hi world");
}

#[test]
fn insert_at_end_of_line() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::EndOfLine));
    dispatch_intent(&mut s, Intent::InsertChar('!'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello!");
}

#[test]
fn insert_multiple_newlines() {
    let mut s = setup("abc");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertNewline);
    dispatch_intent(&mut s, Intent::InsertNewline);
    dispatch_intent(&mut s, Intent::InsertNewline);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 4);
}

// ──────────── Delete edge cases ────────────

#[test]
fn delete_char_at_in_middle() {
    let mut s = setup("abc");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    dispatch_intent(&mut s, Intent::DeleteCharAt);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "ac");
}

#[test]
fn delete_char_before_on_empty_line() {
    let mut s = setup("abc\n\ndef");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    // On empty line, backspace joins with previous
    dispatch_intent(&mut s, Intent::DeleteCharBefore);
}

#[test]
fn multiple_deletes_char_at() {
    let mut s = setup("abcde");
    dispatch_intent(&mut s, Intent::DeleteCharAt);
    dispatch_intent(&mut s, Intent::DeleteCharAt);
    dispatch_intent(&mut s, Intent::DeleteCharAt);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "de");
}

// ──────────── Line operator details ────────────

#[test]
fn line_operator_delete_count_2() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Delete, 2));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "ccc");
}

#[test]
fn line_operator_yank_count_3() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Yank, 3));
    // Buffer unchanged
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 4);
    assert!(s.registers.unnamed_text().is_some());
}

// ──────────── Paste behaviors ────────────

#[test]
fn paste_charwise_after() {
    let mut s = setup("hello world");
    s.registers
        .set(RegisterName::Unnamed, RegisterContent::charwise("XY"));
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Unnamed, PastePosition::After),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.line_to_string(0);
    assert!(text.contains("XY"));
}

#[test]
fn paste_charwise_before() {
    let mut s = setup("hello");
    s.registers
        .set(RegisterName::Unnamed, RegisterContent::charwise("AB"));
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Unnamed, PastePosition::Before),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.line_to_string(0);
    assert!(text.contains("AB"));
}

#[test]
fn paste_linewise_after() {
    let mut s = setup("aaa\nbbb");
    s.registers
        .set(RegisterName::Unnamed, RegisterContent::linewise("new\n"));
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Unnamed, PastePosition::After),
    );
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 3);
}

#[test]
fn paste_linewise_before() {
    let mut s = setup("aaa\nbbb");
    s.registers
        .set(RegisterName::Unnamed, RegisterContent::linewise("new\n"));
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Unnamed, PastePosition::Before),
    );
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 3);
}

#[test]
fn paste_empty_register() {
    let mut s = setup("hello");
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Named('z'), PastePosition::After),
    );
    // Should be a no-op (register empty)
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

// ──────────── Scroll variants ────────────

#[test]
fn scroll_full_page_down() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageDown));
    assert!(s.cursor().line > 0);
}

#[test]
fn scroll_full_page_up() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageDown));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageUp));
    assert_eq!(s.cursor().line, 0);
}

#[test]
fn scroll_line_down() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::LineDown));
    let win = s.active_window_state().unwrap();
    assert!(win.top_line >= 1);
}

#[test]
fn scroll_line_up() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::FullPageDown));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::LineUp));
}

#[test]
fn scroll_cursor_top() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 20));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::CursorTop));
    let win = s.active_window_state().unwrap();
    assert_eq!(win.top_line, win.cursor_line);
}

#[test]
fn scroll_cursor_bottom() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 20));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::CursorBottom));
}

// ──────────── Ex command variants ────────────

#[test]
fn ex_command_w() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":w".into()));
    assert!(s.message.is_some()); // write message or error
}

#[test]
fn ex_command_empty() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":".into()));
    // No-op or message
}

// ──────────── Open line details ────────────

#[test]
fn open_below_on_last_line() {
    let mut s = setup("only");
    dispatch_intent(&mut s, Intent::OpenLine(true));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().line, 1);
}

#[test]
fn open_above_on_first_line() {
    let mut s = setup("only");
    dispatch_intent(&mut s, Intent::OpenLine(false));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().line, 0);
}

// ──────────── Join lines edge cases ────────────

#[test]
fn join_last_line_is_noop() {
    let mut s = setup("only");
    dispatch_intent(&mut s, Intent::JoinLines(true, 1));
    // Should not crash, buffer unchanged
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 1);
}

#[test]
fn join_with_trailing_spaces() {
    let mut s = setup("hello  \nworld");
    dispatch_intent(&mut s, Intent::JoinLines(true, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.contains("world"));
}

// ──────────── Replace char on different positions ────────────

#[test]
fn replace_char_middle() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    dispatch_intent(&mut s, Intent::ReplaceChar('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 2)), Some('X'));
}

#[test]
fn replace_char_last() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::ReplaceChar('!'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 4)), Some('!'));
}

// ──────────── Toggle case sequences ────────────

#[test]
fn toggle_case_multiple_times() {
    let mut s = setup("aBc");
    dispatch_intent(&mut s, Intent::ToggleCase);
    dispatch_intent(&mut s, Intent::ToggleCase);
    dispatch_intent(&mut s, Intent::ToggleCase);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('A'));
    assert_eq!(buf.text.char_at(Position::new(0, 1)), Some('b'));
    assert_eq!(buf.text.char_at(Position::new(0, 2)), Some('C'));
}

// ──────────── Indent edge cases ────────────

#[test]
fn indent_empty_line() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "    ");
}

#[test]
fn outdent_no_indent() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(false, 1));
    // Should not remove chars from the word
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

#[test]
fn double_indent() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "        hello");
}

// ──────────── Substitute details ────────────

#[test]
fn substitute_char_enters_insert() {
    let mut s = setup("abc");
    dispatch_intent(&mut s, Intent::SubstituteChar);
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.active_buffer().unwrap().text.line_to_string(0), "bc");
}

#[test]
fn substitute_line_enters_insert() {
    let mut s = setup("abc\ndef");
    dispatch_intent(&mut s, Intent::SubstituteLine);
    assert_eq!(s.current_mode(), Mode::Insert);
}

// ──────────── Replace insert mode ────────────

#[test]
fn replace_insert_overwrites() {
    let mut s = setup("abcde");
    s.mode.transition(Mode::Replace);
    dispatch_intent(&mut s, Intent::ReplaceInsert('X'));
    dispatch_intent(&mut s, Intent::ReplaceInsert('Y'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('X'));
    assert_eq!(buf.text.char_at(Position::new(0, 1)), Some('Y'));
}

// ──────────── Window state details ────────────

#[test]
fn window_state_scrolloff_up() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    // Move down far
    for _ in 0..30 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    // Move back up
    for _ in 0..10 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Up, 1));
    }
    let win = s.active_window_state().unwrap();
    assert!(win.cursor_line >= win.top_line);
    assert!(win.cursor_line < win.top_line + win.height);
}

#[test]
fn window_state_cursor_visible_after_delete() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::FileEnd, 1));
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Delete, 1));
    let win = s.active_window_state().unwrap();
    assert!(win.cursor_line >= win.top_line);
}

// ──────────── Register interactions ────────────

#[test]
fn register_named_set_and_retrieve() {
    let mut rf = RegisterFile::new();
    rf.set(RegisterName::Named('a'), RegisterContent::charwise("alpha"));
    rf.set(RegisterName::Named('b'), RegisterContent::charwise("beta"));
    assert_eq!(rf.get(RegisterName::Named('a')).unwrap().text, "alpha");
    assert_eq!(rf.get(RegisterName::Named('b')).unwrap().text, "beta");
}

#[test]
fn register_named_also_sets_unnamed() {
    let mut rf = RegisterFile::new();
    rf.set(RegisterName::Named('a'), RegisterContent::charwise("test"));
    assert_eq!(rf.unnamed_text(), Some("test"));
}

#[test]
fn register_yank_type() {
    let mut rf = RegisterFile::new();
    rf.yank("line\n", true);
    assert_eq!(rf.unnamed_type(), Some(RegisterType::Linewise));
}

#[test]
fn register_delete_cascade_9() {
    let mut rf = RegisterFile::new();
    for i in 1..=10 {
        rf.delete(&format!("d{}", i), false);
    }
    // Numbered 1 should be most recent
    assert_eq!(rf.get(RegisterName::Numbered(1)).unwrap().text, "d10");
    // Numbered 9 should be d2 (d1 was pushed out)
    assert_eq!(rf.get(RegisterName::Numbered(9)).unwrap().text, "d2");
}
