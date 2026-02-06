//! Comprehensive integration tests for core-state dispatch.

use kjxlkj_core_state::*;
use kjxlkj_core_types::*;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ──────────── Buffer / Window creation ────────────

#[test]
fn create_empty_editor() {
    let s = EditorState::new(Size::new(80, 24));
    assert!(s.active_buffer().is_none());
    assert!(s.active_window.is_none());
    assert_eq!(s.current_mode(), Mode::Normal);
    assert!(!s.should_quit);
}

#[test]
fn create_buffer_and_window() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("hello");
    let wid = s.create_window(bid);
    assert_eq!(s.active_window, Some(wid));
    assert!(s.active_buffer().is_some());
}

#[test]
fn multiple_buffers() {
    let mut s = EditorState::new(Size::new(80, 24));
    let b1 = s.create_buffer_from_text("first");
    let b2 = s.create_buffer_from_text("second");
    assert_ne!(b1, b2);
    assert!(s.buffers.contains_key(&b1));
    assert!(s.buffers.contains_key(&b2));
}

#[test]
fn cursor_default_position() {
    let s = setup("hello");
    assert_eq!(s.cursor(), Position::new(0, 0));
}

// ──────────── Motion dispatch ────────────

#[test]
fn dispatch_motion_right() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    assert_eq!(s.cursor(), Position::new(0, 1));
}

#[test]
fn dispatch_motion_right_count() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    assert_eq!(s.cursor(), Position::new(0, 3));
}

#[test]
fn dispatch_motion_left() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Left, 1));
    assert_eq!(s.cursor(), Position::new(0, 2));
}

#[test]
fn dispatch_motion_down_up() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 2));
    assert_eq!(s.cursor().line, 2);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Up, 1));
    assert_eq!(s.cursor().line, 1);
}

#[test]
fn dispatch_motion_line_start() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineStart, 1));
    assert_eq!(s.cursor().col, 0);
}

#[test]
fn dispatch_motion_line_end() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    assert_eq!(s.cursor().col, 4);
}

#[test]
fn dispatch_motion_first_non_blank() {
    let mut s = setup("   hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::FirstNonBlank, 1));
    assert_eq!(s.cursor().col, 3);
}

#[test]
fn dispatch_motion_word_forward() {
    let mut s = setup("hello world test");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::WordForward, 1));
    assert_eq!(s.cursor().col, 6);
}

#[test]
fn dispatch_motion_word_backward() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 8));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::WordBackward, 1));
    assert_eq!(s.cursor().col, 6);
}

#[test]
fn dispatch_motion_file_start() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 2));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::FileStart, 1));
    assert_eq!(s.cursor(), Position::new(0, 0));
}

#[test]
fn dispatch_motion_file_end() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::FileEnd, 1));
    assert_eq!(s.cursor().line, 2);
}

// ──────────── Mode transition dispatch ────────────

#[test]
fn dispatch_enter_insert_before() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 0);
}

#[test]
fn dispatch_enter_insert_after() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::AfterCursor));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 1);
}

#[test]
fn dispatch_enter_insert_eol() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::EndOfLine));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 5);
}

#[test]
fn dispatch_enter_insert_first_nonblank() {
    let mut s = setup("   hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(
        &mut s,
        Intent::EnterInsert(InsertPosition::FirstNonBlank),
    );
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 3);
}

#[test]
fn dispatch_enter_mode_visual() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    assert_eq!(s.current_mode(), Mode::Visual);
}

#[test]
fn dispatch_enter_mode_command() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Command));
    assert_eq!(s.current_mode(), Mode::Command);
}

#[test]
fn dispatch_enter_mode_replace() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Replace));
    assert_eq!(s.current_mode(), Mode::Replace);
}

#[test]
fn dispatch_escape_to_normal() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Insert));
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Normal));
    assert_eq!(s.current_mode(), Mode::Normal);
}

// ──────────── Insert operations ────────────

#[test]
fn dispatch_insert_char() {
    let mut s = setup("ello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('h'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
    assert_eq!(s.cursor().col, 1);
}

#[test]
fn dispatch_insert_multiple_chars() {
    let mut s = setup("");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::InsertChar('a'));
    dispatch_intent(&mut s, Intent::InsertChar('b'));
    dispatch_intent(&mut s, Intent::InsertChar('c'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "abc");
    assert_eq!(s.cursor().col, 3);
}

#[test]
fn dispatch_insert_newline() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::InsertNewline);
    assert_eq!(s.cursor(), Position::new(1, 0));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2);
}

#[test]
fn dispatch_delete_char_before() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Insert);
    // Move cursor to col 3
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::DeleteCharBefore);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "helo");
    assert_eq!(s.cursor().col, 2);
}

#[test]
fn dispatch_delete_char_before_at_start() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Insert);
    dispatch_intent(&mut s, Intent::DeleteCharBefore);
    // Should be a no-op
    assert_eq!(s.cursor(), Position::new(0, 0));
}

#[test]
fn dispatch_delete_char_before_joins_lines() {
    let mut s = setup("ab\ncd");
    s.mode.transition(Mode::Insert);
    // Move to start of line 2
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineStart, 1));
    dispatch_intent(&mut s, Intent::DeleteCharBefore);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "abcd");
    assert_eq!(s.cursor(), Position::new(0, 2));
}

#[test]
fn dispatch_delete_char_at() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::DeleteCharAt);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "ello");
}

// ──────────── Operator dispatch ────────────

#[test]
fn dispatch_dw_delete_word() {
    let mut s = setup("hello world");
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1),
    );
    let buf = s.active_buffer().unwrap();
    let text = buf.text.text();
    assert!(!text.starts_with("hello"));
}

#[test]
fn dispatch_dd_delete_line() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Delete, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "bbb");
}

#[test]
fn dispatch_dd_last_line() {
    let mut s = setup("aaa\nbbb");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Delete, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 2); // trailing line
}

#[test]
fn dispatch_yy_yank_line() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Yank, 1));
    // Buffer should be unchanged
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "aaa");
    // Register should contain yanked text
    assert!(s.registers.unnamed_text().is_some());
}

#[test]
fn dispatch_cc_change_line() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::LineOperator(OperatorKind::Change, 1));
    assert_eq!(s.current_mode(), Mode::Insert);
}

// ──────────── Delete to end / Change to end ────────────

#[test]
fn dispatch_delete_to_end() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(&mut s, Intent::DeleteToEnd);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

#[test]
fn dispatch_change_to_end() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(&mut s, Intent::ChangeToEnd);
    assert_eq!(s.current_mode(), Mode::Insert);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

// ──────────── Open line ────────────

#[test]
fn dispatch_open_line_below() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::OpenLine(true));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().line, 1);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 3);
}

#[test]
fn dispatch_open_line_above() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::OpenLine(false));
    assert_eq!(s.current_mode(), Mode::Insert);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_count(), 3);
}

// ──────────── Join lines ────────────

#[test]
fn dispatch_join_lines_with_space() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::JoinLines(true, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello world");
}

#[test]
fn dispatch_join_lines_without_space() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::JoinLines(false, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "helloworld");
}

// ──────────── Replace char ────────────

#[test]
fn dispatch_replace_char() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ReplaceChar('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "Xello");
    assert_eq!(s.current_mode(), Mode::Normal);
}

// ──────────── Toggle case ────────────

#[test]
fn dispatch_toggle_case_lower_to_upper() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ToggleCase);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('H'));
    assert_eq!(s.cursor().col, 1); // cursor moves right
}

#[test]
fn dispatch_toggle_case_upper_to_lower() {
    let mut s = setup("Hello");
    dispatch_intent(&mut s, Intent::ToggleCase);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('h'));
}

// ──────────── Substitute ────────────

#[test]
fn dispatch_substitute_char() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::SubstituteChar);
    assert_eq!(s.current_mode(), Mode::Insert);
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "ello");
}

#[test]
fn dispatch_substitute_line() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::SubstituteLine);
    assert_eq!(s.current_mode(), Mode::Insert);
}

// ──────────── Replace insert mode ────────────

#[test]
fn dispatch_replace_insert() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Replace);
    dispatch_intent(&mut s, Intent::ReplaceInsert('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.char_at(Position::new(0, 0)), Some('X'));
    assert_eq!(s.cursor().col, 1);
}

// ──────────── Indent / Outdent ────────────

#[test]
fn dispatch_indent() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "    hello");
}

#[test]
fn dispatch_outdent() {
    let mut s = setup("    hello");
    dispatch_intent(&mut s, Intent::Indent(false, 1));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

// ──────────── Scroll ────────────

#[test]
fn dispatch_scroll_half_page_down() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::HalfPageDown));
    let win = s.active_window_state().unwrap();
    assert!(win.cursor_line > 0);
    assert!(win.top_line > 0);
}

#[test]
fn dispatch_scroll_half_page_up() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::HalfPageDown));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::HalfPageUp));
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 0);
}

#[test]
fn dispatch_scroll_cursor_center() {
    let text = (0..50).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 25));
    dispatch_intent(&mut s, Intent::Scroll(ScrollKind::CursorCenter));
    let win = s.active_window_state().unwrap();
    assert!(win.top_line > 0);
    assert!(win.top_line < 25);
}

// ──────────── Ex commands ────────────

#[test]
fn dispatch_quit() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":q".into()));
    assert!(s.should_quit);
}

#[test]
fn dispatch_quit_force() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":q!".into()));
    assert!(s.should_quit);
}

#[test]
fn dispatch_wq() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":wq".into()));
    assert!(s.should_quit);
}

#[test]
fn dispatch_unknown_command() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":foobar".into()));
    assert!(s.message.is_some());
    assert!(s.message.unwrap().contains("unknown"));
}

// ──────────── Noop ────────────

#[test]
fn dispatch_noop() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Noop);
    assert_eq!(s.cursor(), Position::new(0, 0));
}

// ──────────── Yank line ────────────

#[test]
fn dispatch_big_y_yank_line() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::YankLine(1));
    assert!(s.registers.unnamed_text().is_some());
    // Buffer should be unchanged
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello");
}

// ──────────── Paste ────────────

#[test]
fn dispatch_paste_after() {
    let mut s = setup("hello\nworld");
    // Yank first
    dispatch_intent(&mut s, Intent::YankLine(1));
    // Move down and paste
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(
        &mut s,
        Intent::Paste(RegisterName::Unnamed, PastePosition::After),
    );
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 3);
}

// ──────────── Window state ────────────

#[test]
fn window_ensure_visible_scrolls() {
    let text = (0..100).map(|i| format!("L{}", i)).collect::<Vec<_>>().join("\n");
    let mut s = setup(&text);
    // Move cursor far down
    for _ in 0..30 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    let win = s.active_window_state().unwrap();
    // Cursor should be visible in viewport
    assert!(win.cursor_line >= win.top_line);
    assert!(win.cursor_line < win.top_line + win.height);
}

// ──────────── RegisterFile ────────────

#[test]
fn register_file_set_get() {
    let mut rf = RegisterFile::new();
    rf.set(RegisterName::Named('a'), RegisterContent::charwise("test"));
    assert_eq!(rf.get(RegisterName::Named('a')).map(|r| r.text.as_str()), Some("test"));
}

#[test]
fn register_file_yank_sets_unnamed() {
    let mut rf = RegisterFile::new();
    rf.yank("hello", false);
    assert_eq!(rf.unnamed_text(), Some("hello"));
}

#[test]
fn register_file_delete_cascades() {
    let mut rf = RegisterFile::new();
    rf.delete("first", false);
    rf.delete("second", false);
    // Unnamed should be most recent
    assert_eq!(rf.unnamed_text(), Some("second"));
    // Numbered 1 should be most recent
    assert_eq!(
        rf.get(RegisterName::Numbered(1)).map(|r| r.text.as_str()),
        Some("second")
    );
    // Numbered 2 should be previous
    assert_eq!(
        rf.get(RegisterName::Numbered(2)).map(|r| r.text.as_str()),
        Some("first")
    );
}
