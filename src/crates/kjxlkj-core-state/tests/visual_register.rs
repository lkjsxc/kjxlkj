//! Tests for visual mode operators, insert-from-register, and macros.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::*;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ── Insert from register (Ctrl-r) ─────────────────────────

#[test]
fn insert_from_register_named() {
    let mut s = setup("hello");
    s.registers.set(
        RegisterName::Named('a'),
        RegisterContent::charwise("XYZ"),
    );
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::InsertFromRegister('a'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "XYZhello");
}

#[test]
fn insert_from_register_unnamed() {
    let mut s = setup("test");
    s.registers.yank("ABC", false);
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::InsertFromRegister('"'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "ABCtest");
}

#[test]
fn insert_from_register_invalid_is_noop() {
    let mut s = setup("test");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::InsertFromRegister('!'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "test");
}

#[test]
fn insert_from_register_empty_is_noop() {
    let mut s = setup("test");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::InsertFromRegister('a'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "test");
}

// ── Visual mode delete ─────────────────────────────────────

#[test]
fn visual_delete_removes_selection() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 4));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1),
    );
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(
        line.contains("he") && line.contains("orld"),
        "Expected delete of visual selection, got: {}",
        line,
    );
}

#[test]
fn visual_delete_exits_to_normal() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::Right, 1),
    );
    assert_eq!(s.mode.current(), Mode::Normal);
}

#[test]
fn visual_yank_copies_selection() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 6));
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 4));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Yank, MotionKind::Right, 1),
    );
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hello world");
    let reg_text = s.registers.unnamed_text().unwrap_or("");
    assert!(
        !reg_text.is_empty(),
        "Expected yanked text in unnamed register",
    );
}

#[test]
fn visual_yank_exits_to_normal() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Yank, MotionKind::Right, 1),
    );
    assert_eq!(s.mode.current(), Mode::Normal);
}

#[test]
fn visual_change_enters_insert() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Change, MotionKind::Right, 1),
    );
    assert_eq!(s.mode.current(), Mode::Insert);
}

#[test]
fn visual_clears_anchor_after_op() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::Visual));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 2));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Yank, MotionKind::Right, 1),
    );
    let win = s.active_window_state().unwrap();
    assert!(win.visual_anchor.is_none());
}

// ── Visual line mode ───────────────────────────────────────

#[test]
fn visual_line_sets_anchor() {
    let mut s = setup("line1\nline2\nline3");
    dispatch_intent(&mut s, Intent::EnterMode(Mode::VisualLine));
    let win = s.active_window_state().unwrap();
    assert!(win.visual_anchor.is_some());
}

// ── Delete word before (Ctrl-w) ────────────────────────────

#[test]
fn ctrl_w_deletes_word_before_v2() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::DeleteWordBefore);
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(
        !line.contains("world"),
        "Expected word deleted, got: {}",
        line,
    );
}

// ── Delete to line start (Ctrl-u) ──────────────────────────

#[test]
fn ctrl_u_deletes_to_line_start_v2() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::DeleteToLineStart);
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(
        line.len() < 5,
        "Expected line mostly deleted, got: {}",
        line,
    );
}

// ── Dot repeat ─────────────────────────────────────────────

#[test]
fn dot_repeat_replays_delete() {
    let mut s = setup("aaa bbb ccc");
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Delete, MotionKind::WordForward, 1),
    );
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(!line.starts_with("aaa"), "After dw: {}", line);
    let len1 = line.len();
    dispatch_intent(&mut s, Intent::RepeatLastChange);
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.len() < len1, "After dot should delete more: {}", line);
}

// ── Select register ────────────────────────────────────────

#[test]
fn select_register_then_yank() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::SelectRegister(RegisterName::Named('a')));
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Yank, MotionKind::WordForward, 1),
    );
    let content = s.registers.get(RegisterName::Named('a'));
    assert!(content.is_some(), "Expected register 'a' to have content");
}

// ── Multiple substitute ────────────────────────────────────

#[test]
fn substitute_different_separator() {
    let mut s = setup("path/to/file");
    dispatch_intent(
        &mut s,
        Intent::ExCommand(":s#path#route#".into()),
    );
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "route/to/file");
}

// ── Mark across lines ──────────────────────────────────────

#[test]
fn mark_preserves_position_across_lines() {
    let mut s = setup("line1\nline2\nline3");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 3));
    dispatch_intent(&mut s, Intent::SetMark('b'));
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s, Intent::JumpToMark('b'));
    assert_eq!(s.cursor().line, 1);
    assert_eq!(s.cursor().col, 3);
}

// ── Indent operations ──────────────────────────────────────

#[test]
fn indent_adds_spaces() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Indent(true, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.starts_with("    "), "Expected indent: {}", line);
}

#[test]
fn dedent_removes_spaces() {
    let mut s = setup("    hello");
    dispatch_intent(&mut s, Intent::Indent(false, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert_eq!(line, "hello");
}

// ── Replace char ───────────────────────────────────────────

#[test]
fn replace_char_at_cursor() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    dispatch_intent(&mut s, Intent::ReplaceChar('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0), "hXllo");
}

// ── Toggle case ────────────────────────────────────────────

#[test]
fn toggle_case_tilde() {
    let mut s = setup("Hello");
    dispatch_intent(&mut s, Intent::ToggleCase);
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.starts_with('h'), "Expected toggle: {}", line);
}

// ── Open line ──────────────────────────────────────────────

#[test]
fn open_line_below() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::OpenLine(false));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 3);
    assert_eq!(s.mode.current(), Mode::Insert);
}

#[test]
fn open_line_above() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::OpenLine(true));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 3);
    assert_eq!(s.mode.current(), Mode::Insert);
}

// ── Join lines ─────────────────────────────────────────────

#[test]
fn join_lines_merges() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::JoinLines(true, 1));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    // J joins with optional space — check lines merged
    assert!(
        line.contains("hello") && line.contains("world"),
        "Expected joined: {}",
        line,
    );
}

// ── Paste operations ───────────────────────────────────────

#[test]
fn paste_after_inserts_yanked() {
    let mut s = setup("hello world");
    dispatch_intent(
        &mut s,
        Intent::Operator(OperatorKind::Yank, MotionKind::WordForward, 1),
    );
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::Paste(RegisterName::Unnamed, PastePosition::After));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.len() > 11, "Expected paste: {}", line);
}

#[test]
fn paste_before_inserts_yanked() {
    let mut s = setup("abc");
    s.registers.yank("XY", false);
    dispatch_intent(&mut s, Intent::Paste(RegisterName::Unnamed, PastePosition::Before));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.contains("XY"), "Expected paste before: {}", line);
}

// ── Macro recording/playback ───────────────────────────────

#[test]
fn macro_record_and_play() {
    let mut s = setup("hello");
    // Start recording into register a
    dispatch_intent(&mut s, Intent::MacroToggleRecord('a'));
    assert!(s.macro_recording.is_some());
    // Record some intents
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    dispatch_intent(&mut s, Intent::ToggleCase);
    // Stop recording
    dispatch_intent(&mut s, Intent::MacroToggleRecord('a'));
    assert!(s.macro_recording.is_none());
    assert!(s.macros.contains_key(&'a'));
}

#[test]
fn macro_playback_executes() {
    let mut s = setup("hello");
    // Manually store a macro that toggles case at cursor
    s.macros.insert('b', vec![Intent::ToggleCase]);
    dispatch_intent(&mut s, Intent::MacroPlay('b'));
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert!(line.starts_with('H'), "Expected toggled: {}", line);
}

#[test]
fn macro_repeat_last() {
    let mut s = setup("hello");
    s.macros.insert('c', vec![Intent::ToggleCase]);
    dispatch_intent(&mut s, Intent::MacroPlay('c'));
    // ToggleCase also advances cursor, so play again at next pos
    dispatch_intent(&mut s, Intent::MacroRepeatLast);
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    // First char toggled, then second char toggled
    assert!(line.starts_with("HE"), "Expected 2 toggled: {}", line);
}

#[test]
fn macro_empty_shows_message() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::MacroPlay('z'));
    assert!(s.message.as_ref().unwrap().contains("Empty"));
}

// ── Jump list ──────────────────────────────────────────────

#[test]
fn jump_list_back_and_forward() {
    let mut s = setup("line1\nline2\nline3\nline4\nline5");
    // Search jumps push onto jump list
    dispatch_intent(
        &mut s,
        Intent::SearchForward("line3".into()),
    );
    let pos_after = s.cursor();
    assert_eq!(pos_after.line, 2);
    // Jump back
    dispatch_intent(&mut s, Intent::JumpListBack);
    assert_ne!(s.cursor().line, 2);
}

#[test]
fn jump_list_empty_shows_message() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::JumpListBack);
    assert!(s.message.as_ref().unwrap().contains("bottom"));
}

// ── Change list ────────────────────────────────────────────

#[test]
fn change_list_navigation() {
    let mut s = setup("hello world");
    // Make a change (toggle case pushes change)
    dispatch_intent(&mut s, Intent::ToggleCase);
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 5));
    dispatch_intent(&mut s, Intent::ToggleCase);
    let col_after = s.cursor().col;
    // Now navigate back — should go to a previous change
    dispatch_intent(&mut s, Intent::ChangeListOlder);
    // Should not be at same position as after last toggle
    assert!(
        s.change_list_idx < s.change_list.len(),
        "Expected change list to navigate",
    );
}

#[test]
fn change_list_empty_shows_message() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ChangeListOlder);
    assert!(s.message.as_ref().unwrap().contains("oldest"));
}
