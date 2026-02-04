//! Editing invariant tests.
//!
//! These tests verify the editing semantics specified in
//! `/docs/spec/editing/README.md`.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers};

/// Test: Cursor boundary clamping - never panic.
#[test]
fn test_boundary_clamping_left() {
    let mut state = EditorState::new();

    // Move left many times on empty buffer - should not panic
    for _ in 0..1000 {
        state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 0);
}

/// Test: Cursor boundary clamping - up on first line.
#[test]
fn test_boundary_clamping_up() {
    let mut state = EditorState::new();

    // Move up many times - should clamp to first line
    for _ in 0..1000 {
        state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.line(), 0);
}

/// Test: Cursor boundary clamping - down on last line.
#[test]
fn test_boundary_clamping_down() {
    let mut state = EditorState::new();

    // Move down many times - should clamp to last line
    for _ in 0..1000 {
        state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    // Should be clamped to valid line
    assert!(snapshot.cursor.line() < snapshot.buffer.line_count.max(1));
}

/// Test: Cursor boundary clamping - right at end of line.
#[test]
fn test_boundary_clamping_right() {
    let mut state = EditorState::new();

    // Insert some text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Move right many times - should clamp to line end
    for _ in 0..1000 {
        state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    }

    let snapshot = state.snapshot();
    // "abc" has 3 chars, cursor should be at max col 2 in normal mode
    assert!(snapshot.cursor.col() <= 2);
}

/// Test: Repeatability - same input sequence produces same result.
#[test]
fn test_repeatability() {
    let actions = vec![
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
    ];

    // Run 10 times
    let mut results = Vec::new();
    for _ in 0..10 {
        let mut state = EditorState::new();
        for action in &actions {
            state.handle_key(action.clone());
        }
        let snapshot = state.snapshot();
        results.push((
            snapshot.buffer.lines.clone(),
            snapshot.cursor.line(),
            snapshot.cursor.col(),
        ));
    }

    // All results should be identical
    for result in &results {
        assert_eq!(result, &results[0]);
    }
}

/// Test: Determinism - operations produce predictable outcomes.
#[test]
fn test_determinism() {
    // Test insert produces predictable content
    let mut state = EditorState::new();

    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let after_insert = state.snapshot();
    assert!(after_insert.buffer.lines[0].contains("hello"));

    // Delete character with x
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));

    let after_delete = state.snapshot();
    // Should have deleted first char
    assert!(!after_delete.buffer.lines[0].starts_with('h'));
}

/// Test: Visual mode selection operations.
#[test]
fn test_visual_mode_operations() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // Enter visual mode
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Visual);

    // Select word
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));

    // Yank
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

    // Should be back in normal mode
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
}

/// Test: Undo invariant - undo never corrupts state.
#[test]
fn test_undo_invariant() {
    let mut state = EditorState::new();

    // Perform various edits
    for i in 0..50 {
        state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        state.handle_key(KeyEvent::new(
            KeyCode::Char(((i % 26) as u8 + b'a') as char),
            KeyModifiers::NONE,
        ));
        state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    }

    // Undo many times
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));

        // State should always be valid
        let snapshot = state.snapshot();
        assert!(snapshot.cursor.line() < snapshot.buffer.line_count.max(1));
    }

    // Redo many times
    for _ in 0..100 {
        state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));

        // State should always be valid
        let snapshot = state.snapshot();
        assert!(snapshot.cursor.line() < snapshot.buffer.line_count.max(1));
    }
}

/// Test: Mode transitions are consistent.
#[test]
fn test_mode_transition_consistency() {
    let mut state = EditorState::new();

    // Cycle through modes many times
    for _ in 0..100 {
        // Normal -> Insert
        state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Insert);

        // Insert -> Normal
        state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);

        // Normal -> Visual
        state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Visual);

        // Visual -> Normal
        state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);

        // Normal -> Command
        state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Command);

        // Command -> Normal
        state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
        assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
    }
}

/// Test: Basic movement works.
#[test]
fn test_basic_movement() {
    let mut state = EditorState::new();

    // Insert multiple lines
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for _ in 0..5 {
        state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
        state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
        state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let initial = state.snapshot();

    // Move left (h)
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));

    // Move up (k)
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));

    let after_move = state.snapshot();
    // Should have moved up (different line or at least valid)
    assert!(after_move.cursor.line() < initial.cursor.line() || after_move.cursor.line() == 0);
}

/// Test: Home motion (0) goes to column 0.
#[test]
fn test_home_motion() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Cursor should be at end of "hello world"
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() > 0);

    // Press 0 to go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 0);
}
