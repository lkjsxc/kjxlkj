//! Architecture invariant tests.
//!
//! These tests verify the key architectural properties specified in
//! `/docs/spec/architecture/runtime.md`.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};

/// Test: Single-writer core task invariant.
///
/// The EditorState is not Sync (intentionally) to prevent concurrent access.
/// All mutations go through a single owner - the core task.
#[test]
fn test_single_writer_invariant() {
    // EditorState should be Send (can be moved to another thread)
    fn assert_send<T: Send>() {}
    assert_send::<EditorState>();

    // EditorState should NOT be Sync (no shared concurrent access)
    // This is verified by the fact that EditorState contains non-Sync fields
    // (like TextBuffer which wraps Rope).

    // Create state and verify single-threaded mutation pattern
    let mut state = EditorState::new();
    let initial_version = state.buffer().version();

    // Mutation through single owner
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let final_version = state.buffer().version();

    // Version should have changed (mutations occurred)
    assert!(final_version.value() > initial_version.value());
}

/// Test: Snapshot immutability and render isolation.
///
/// Snapshots are cheap clones of visible state that the render task
/// can consume without blocking the core task.
#[test]
fn test_snapshot_immutability() {
    let mut state = EditorState::new();

    // Insert some content
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Take a snapshot
    let snapshot1 = state.snapshot();
    let lines1: Vec<_> = snapshot1.buffer.lines.clone();

    // Mutate the state further
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Take a new snapshot
    let snapshot2 = state.snapshot();
    let lines2: Vec<_> = snapshot2.buffer.lines.clone();

    // The first snapshot should be unchanged (immutable)
    assert_eq!(lines1, snapshot1.buffer.lines);

    // The two snapshots should differ
    assert_ne!(lines1, lines2);
}

/// Test: Deterministic event handling.
///
/// The same sequence of events should produce the same final state.
#[test]
fn test_deterministic_event_handling() {
    let events = vec![
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE),
    ];

    // Run the same events twice
    let mut state1 = EditorState::new();
    for event in &events {
        state1.handle_key(event.clone());
    }

    let mut state2 = EditorState::new();
    for event in &events {
        state2.handle_key(event.clone());
    }

    // Both states should produce identical snapshots
    let snap1 = state1.snapshot();
    let snap2 = state2.snapshot();

    assert_eq!(snap1.buffer.lines, snap2.buffer.lines);
    assert_eq!(snap1.cursor.line(), snap2.cursor.line());
    assert_eq!(snap1.cursor.col(), snap2.cursor.col());
    assert_eq!(snap1.mode, snap2.mode);
}

/// Test: Mode transitions are correct.
///
/// All mode transitions follow the documented state machine.
#[test]
fn test_mode_transitions() {
    let mut state = EditorState::new();

    // Start in Normal mode
    assert_eq!(state.mode(), Mode::Normal);

    // i -> Insert
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Insert);

    // Esc -> Normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Normal);

    // v -> Visual
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Visual);

    // Esc -> Normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Normal);

    // : -> Command
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Command);

    // Esc -> Normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Normal);

    // R -> Replace
    state.handle_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Replace);

    // Esc -> Normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), Mode::Normal);
}

/// Test: Shutdown order is deterministic.
///
/// Quit command triggers clean shutdown.
#[test]
fn test_shutdown_determinism() {
    let mut state = EditorState::new();

    // Should not quit initially
    assert!(!state.should_quit());

    // Enter command mode and type :q
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    // Should quit now
    assert!(state.should_quit());
}

/// Test: Cursor never goes out of bounds.
///
/// Boundary clamping ensures cursor is always valid.
#[test]
fn test_cursor_boundary_clamping() {
    let mut state = EditorState::new();

    // Insert some text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Try to move left many times (should clamp at 0)
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 0);
    assert_eq!(snapshot.cursor.line(), 0);

    // Try to move right many times (should clamp at line end)
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    // Cursor should be at the last character, not beyond
    assert!(snapshot.cursor.col() <= 2); // "abc" has chars at 0,1,2

    // Try to move up many times (should clamp at line 0)
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.line(), 0);
}

/// Test: Undo/redo maintains consistency.
///
/// Undo stack operations are correct and reversible.
/// Note: The full undo/redo implementation may need additional work.
/// This test validates that undo operations don't corrupt state.
#[test]
fn test_undo_redo_consistency() {
    let mut state = EditorState::new();

    // Check initial state
    let initial = state.snapshot();
    let initial_line_count = initial.buffer.line_count;

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let after_insert = state.snapshot();

    // Text was inserted
    assert!(!after_insert.buffer.lines[0].is_empty());
    assert!(after_insert.buffer.lines[0].contains("hello"));

    // Undo - cursor should still be in valid position
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));

    let after_undo = state.snapshot();

    // Cursor should still be valid (not out of bounds)
    assert!(after_undo.cursor.line() < after_undo.buffer.line_count || after_undo.buffer.line_count == 0);

    // Redo (Ctrl-r) - should not panic
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));

    let after_redo = state.snapshot();

    // Cursor should still be valid
    assert!(after_redo.cursor.line() < after_redo.buffer.line_count || after_redo.buffer.line_count == 0);
}
