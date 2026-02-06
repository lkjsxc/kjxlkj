//! Tests for append-at-EOL, viewport wrapping, and memory/latency regression.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{InsertPosition, Intent, Mode, MotionKind, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// --- Append at EOL regression ---

#[test]
fn append_at_eol_cursor_at_last_char() {
    let mut s = setup("hello");
    // Move cursor to end ('$')
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    // Cursor should be on last char (col 4)
    assert_eq!(s.cursor().col, 4);
    // 'a' should place insert cursor at col 5 (true EOL)
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::AfterCursor));
    assert_eq!(s.current_mode(), Mode::Insert);
    assert_eq!(s.cursor().col, 5);
}

#[test]
fn append_at_eol_type_produces_correct_text() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::Motion(MotionKind::LineEnd, 1));
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::AfterCursor));
    dispatch_intent(&mut s, Intent::InsertChar('X'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim_end(), "helloX");
}

#[test]
fn eol_append_with_capital_a() {
    let mut s = setup("world");
    // 'A' should move cursor to end-of-line and enter insert
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::EndOfLine));
    assert_eq!(s.cursor().col, 5);
    dispatch_intent(&mut s, Intent::InsertChar('!'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim_end(), "world!");
}

#[test]
fn append_at_eol_empty_line() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::AfterCursor));
    assert_eq!(s.cursor().col, 1);
    dispatch_intent(&mut s, Intent::InsertChar('A'));
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_to_string(0).contains('A'));
}

// --- Viewport wrapping ---

#[test]
fn wrap_true_by_default() {
    let s = setup("hello world this is a long line");
    let win = s.active_window_state().unwrap();
    assert!(win.wrap, "wrap should be true by default");
}

#[test]
fn wrap_true_means_left_col_zero() {
    let mut s = setup("hello world this is a long line");
    // Move cursor right past viewport
    for _ in 0..50 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    }
    let win = s.active_window_state().unwrap();
    if win.wrap {
        assert_eq!(win.left_col, 0, "with wrap on, left_col must be 0");
    }
}

#[test]
fn nowrap_allows_horizontal_scroll() {
    let mut s = setup(&"x".repeat(200));
    if let Some(win) = s.active_window_mut() {
        win.wrap = false;
    }
    // Move to column 100
    for _ in 0..100 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Right, 1));
    }
    let win = s.active_window_state().unwrap();
    assert!(!win.wrap);
    // With nowrap, left_col should scroll
    assert!(win.left_col > 0, "nowrap should allow horizontal scrolling");
}

// --- Large file viewport bounded ---

#[test]
fn viewport_bounded_large_buffer() {
    let lines: String = (0..10000).map(|i| format!("line {}\n", i)).collect();
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(&lines);
    s.create_window(bid);
    let win = s.active_window_state().unwrap();
    assert_eq!(win.top_line, 0);
    assert!(win.height <= 24);
    // Viewport only shows height lines, not all 10000
}

#[test]
fn navigate_large_file_preserves_viewport() {
    let lines: String = (0..1000).map(|i| format!("line {}\n", i)).collect();
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(&lines);
    s.create_window(bid);
    // Move down 500 lines
    dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 500));
    let win = s.active_window_state().unwrap();
    assert_eq!(win.cursor_line, 500);
    assert!(win.top_line > 0);
    assert!(win.top_line <= 500);
}

// --- Typing burst ordering ---

#[test]
fn typing_burst_200_chars_in_order() {
    let mut s = setup("");
    s.mode.transition(Mode::Insert);
    let chars: Vec<char> = (0..200).map(|i| (b'a' + (i % 26) as u8) as char).collect();
    for &c in &chars {
        dispatch_intent(&mut s, Intent::InsertChar(c));
    }
    let buf = s.active_buffer().unwrap();
    let text = buf.text.line_to_string(0);
    let expected: String = chars.into_iter().collect();
    assert!(text.starts_with(&expected));
}

// --- Scroll burst ordering ---

#[test]
fn scroll_burst_200_lines() {
    let lines: String = (0..500).map(|i| format!("line {}\n", i)).collect();
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(&lines);
    s.create_window(bid);
    for _ in 0..200 {
        dispatch_intent(&mut s, Intent::Motion(MotionKind::Down, 1));
    }
    assert_eq!(s.cursor().line, 200);
}

// --- Mode switch burst ---

#[test]
fn mode_switch_burst_10() {
    let mut s = setup("hello");
    for _ in 0..10 {
        dispatch_intent(&mut s, Intent::EnterInsert(InsertPosition::BeforeCursor));
        assert_eq!(s.current_mode(), Mode::Insert);
        dispatch_intent(&mut s, Intent::EnterMode(Mode::Normal));
        assert_eq!(s.current_mode(), Mode::Normal);
    }
}

// --- Input ordering preserved ---

#[test]
fn input_ordering_insert_sequence() {
    let mut s = setup("");
    s.mode.transition(Mode::Insert);
    let sequence = "abcdefghij";
    for c in sequence.chars() {
        dispatch_intent(&mut s, Intent::InsertChar(c));
    }
    let buf = s.active_buffer().unwrap();
    let text = buf.text.line_to_string(0);
    assert!(text.starts_with(sequence));
}

#[test]
fn deterministic_snapshot_same_input() {
    let mut s1 = setup("hello\nworld");
    let mut s2 = setup("hello\nworld");
    dispatch_intent(&mut s1, Intent::Motion(MotionKind::Down, 1));
    dispatch_intent(&mut s2, Intent::Motion(MotionKind::Down, 1));
    assert_eq!(s1.cursor(), s2.cursor());
    assert_eq!(s1.current_mode(), s2.current_mode());
}
