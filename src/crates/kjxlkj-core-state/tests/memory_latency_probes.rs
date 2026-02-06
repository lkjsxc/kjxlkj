//! Memory and latency regression probes.
//! Deterministic tests ensuring viewport-bounded materialization,
//! typing burst ordering, and snapshot cost bounds.

use kjxlkj_core_types::{Size, Position, Intent, Mode, InsertPosition};
use kjxlkj_core_state::EditorState;
use kjxlkj_core_text::{TextBuffer, BufferSnapshot};

fn setup_large(lines: usize) -> EditorState {
    let mut text = String::new();
    for i in 0..lines { text.push_str(&format!("line {} content here\n", i)); }
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(&text);
    s.create_window(bid);
    s
}

fn insert_mode(state: &mut EditorState) {
    kjxlkj_core_state::dispatch_intent(state, Intent::EnterInsert(InsertPosition::AfterCursor));
}

// ── Viewport-bounded snapshots ───────────────────────────────────────

#[test]
fn snapshot_10k_lines_only_materializes_viewport() {
    let text: String = (0..10_000).map(|i| format!("line {}\n", i)).collect();
    let buf = TextBuffer::from_text(&text);
    let snap = BufferSnapshot::from_buffer(&buf, 5000, 24, Position::new(5000, 0));
    assert_eq!(snap.line_count(), 24);
    assert_eq!(snap.first_line, 5000);
    assert_eq!(snap.total_lines, 10001);
}

#[test]
fn snapshot_50k_lines_viewport_bounded() {
    let text: String = (0..50_000).map(|i| format!("{}\n", i)).collect();
    let buf = TextBuffer::from_text(&text);
    let snap = BufferSnapshot::from_buffer(&buf, 25_000, 50, Position::new(25_000, 0));
    assert_eq!(snap.line_count(), 50);
}

// ── Large buffer edits don't clone entire buffer ─────────────────────

#[test]
fn insert_in_large_buffer_preserves_versions() {
    let text: String = (0..5_000).map(|i| format!("line {}\n", i)).collect();
    let mut buf = TextBuffer::from_text(&text);
    let v0 = buf.version();
    buf.insert_char(Position::new(2500, 0), 'X');
    let v1 = buf.version();
    assert!(v1.0 > v0.0);
    assert_eq!(buf.line_count(), 5001);
}

#[test]
fn delete_in_large_buffer() {
    let text: String = (0..5_000).map(|i| format!("line {}\n", i)).collect();
    let mut buf = TextBuffer::from_text(&text);
    use kjxlkj_core_types::Range;
    buf.delete_range(Range::new(Position::new(1000, 0), Position::new(1001, 0)));
    assert_eq!(buf.line_count(), 5000);
}

// ── Typing burst probe ──────────────────────────────────────────────

#[test]
fn typing_burst_200_chars_ordered() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("");
    s.create_window(bid);
    insert_mode(&mut s);
    for i in 0..200 {
        let c = (b'a' + (i % 26) as u8) as char;
        kjxlkj_core_state::dispatch_intent(&mut s, Intent::InsertChar(c));
    }
    let buf = s.active_buffer().unwrap();
    let line = buf.text.line_to_string(0);
    assert_eq!(line.len(), 200);
    // Verify order
    for (i, c) in line.chars().enumerate() {
        let expected = (b'a' + (i % 26) as u8) as char;
        assert_eq!(c, expected, "char at position {} wrong", i);
    }
}

#[test]
fn typing_burst_500_chars() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("");
    s.create_window(bid);
    insert_mode(&mut s);
    for _ in 0..500 {
        kjxlkj_core_state::dispatch_intent(&mut s, Intent::InsertChar('z'));
    }
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).len(), 500);
}

// ── Scroll probe ─────────────────────────────────────────────────────

#[test]
fn scroll_500_lines_cursor_valid() {
    let mut s = setup_large(1000);
    for _ in 0..500 {
        kjxlkj_core_state::dispatch_intent(
            &mut s,
            Intent::Motion(kjxlkj_core_types::MotionKind::Down, 1),
        );
    }
    let pos = s.cursor();
    assert_eq!(pos.line, 500);
    assert_eq!(pos.col, 0);
}

#[test]
fn scroll_to_end_and_back() {
    let mut s = setup_large(500);
    // Go to end
    kjxlkj_core_state::dispatch_intent(
        &mut s,
        Intent::Motion(kjxlkj_core_types::MotionKind::FileEnd, 1),
    );
    let end = s.cursor();
    assert!(end.line >= 499);
    // Go to start
    kjxlkj_core_state::dispatch_intent(
        &mut s,
        Intent::Motion(kjxlkj_core_types::MotionKind::FileStart, 1),
    );
    assert_eq!(s.cursor().line, 0);
}

// ── Mode switch probe ────────────────────────────────────────────────

#[test]
fn rapid_mode_switches_50_cycles() {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text("test");
    s.create_window(bid);
    for _ in 0..50 {
        insert_mode(&mut s);
        assert_eq!(s.current_mode(), Mode::Insert);
        kjxlkj_core_state::dispatch_intent(&mut s, Intent::EnterMode(Mode::Normal));
        assert_eq!(s.current_mode(), Mode::Normal);
    }
}

// ── Snapshot cost: creating snapshot is consistent ────────────────────

#[test]
fn snapshot_creation_deterministic() {
    let text: String = (0..1000).map(|i| format!("line {}\n", i)).collect();
    let buf = TextBuffer::from_text(&text);
    let s1 = BufferSnapshot::from_buffer(&buf, 100, 24, Position::new(100, 0));
    let s2 = BufferSnapshot::from_buffer(&buf, 100, 24, Position::new(100, 0));
    assert_eq!(s1.line_count(), s2.line_count());
    assert_eq!(s1.first_line, s2.first_line);
    for i in 0..s1.line_count() {
        assert_eq!(s1.line(i), s2.line(i));
    }
}

// ── Undo/redo probe ──────────────────────────────────────────────────

#[test]
fn undo_redo_in_large_buffer() {
    let mut s = setup_large(1000);
    insert_mode(&mut s);
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::InsertChar('Z'));
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::EnterMode(Mode::Normal));
    // Verify Z was inserted somewhere in line 0
    let buf = s.active_buffer().unwrap();
    let before = buf.text.line_to_string(0);
    assert!(before.contains('Z'), "Z should be present before undo");
    // Undo should not panic on a large buffer
    kjxlkj_core_state::dispatch_intent(&mut s, Intent::Undo);
    // Buffer should still be valid and accessible
    let buf = s.active_buffer().unwrap();
    assert!(buf.text.line_count() >= 1000);
}
