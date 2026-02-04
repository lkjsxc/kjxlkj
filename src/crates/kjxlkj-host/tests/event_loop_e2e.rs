//! End-to-end event loop tests.
//!
//! These tests verify the minimal "event → core → snapshot → render" loop
//! as specified in `/docs/spec/architecture/runtime.md`.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers};
use kjxlkj_core_ui::EditorSnapshot;
use kjxlkj_render::RenderDiff;

/// Simulate a minimal event loop iteration without terminal I/O.
fn event_loop_iteration(
    state: &mut EditorState,
    event: KeyEvent,
    prev_snapshot: Option<&EditorSnapshot>,
) -> (EditorSnapshot, Option<RenderDiff>) {
    // 1. Handle the event (core processing)
    state.handle_key(event);

    // 2. Generate a snapshot
    let snapshot = state.snapshot();

    // 3. Compute render diff (if we have a previous snapshot)
    let diff = prev_snapshot.map(|prev| RenderDiff::compute(prev, &snapshot));

    (snapshot, diff)
}

/// Test: Minimal end-to-end event loop is deterministic.
///
/// The same sequence of events produces the same final state and render output.
#[test]
fn test_end_to_end_determinism() {
    let events = vec![
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
    ];

    // Run twice and compare
    let mut state1 = EditorState::new();
    let mut snapshots1 = Vec::new();
    let mut prev = None;
    for event in &events {
        let (snapshot, _diff) = event_loop_iteration(&mut state1, event.clone(), prev.as_ref());
        snapshots1.push(snapshot.clone());
        prev = Some(snapshot);
    }

    let mut state2 = EditorState::new();
    let mut snapshots2 = Vec::new();
    let mut prev = None;
    for event in &events {
        let (snapshot, _diff) = event_loop_iteration(&mut state2, event.clone(), prev.as_ref());
        snapshots2.push(snapshot.clone());
        prev = Some(snapshot);
    }

    // All snapshots should be identical
    assert_eq!(snapshots1.len(), snapshots2.len());
    for (s1, s2) in snapshots1.iter().zip(snapshots2.iter()) {
        assert_eq!(s1.buffer.lines, s2.buffer.lines);
        assert_eq!(s1.cursor.line(), s2.cursor.line());
        assert_eq!(s1.cursor.col(), s2.cursor.col());
        assert_eq!(s1.mode, s2.mode);
    }
}

/// Test: Event loop handles mode transitions correctly.
#[test]
fn test_end_to_end_mode_transitions() {
    let mut state = EditorState::new();

    // Start in Normal
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);

    // i -> Insert
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);

    // Type some chars
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert!(snapshot.buffer.lines[0].contains('x'));

    // Escape -> Normal
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Render diff is compatible with the event loop.
#[test]
fn test_end_to_end_render_diff() {
    let mut state = EditorState::new();

    // Initial snapshot (no diff yet)
    let (snapshot1, diff1) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        None,
    );

    // First iteration has no previous, so no diff
    assert!(diff1.is_none());

    // Make a change
    let (snapshot2, diff2) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        Some(&snapshot1),
    );

    // Should show cursor/content changes
    let diff = diff2.expect("Should have diff");
    assert!(diff.needs_redraw());

    // Compare same snapshot - no changes needed
    let diff3 = RenderDiff::compute(&snapshot2, &snapshot2);
    assert!(!diff3.needs_redraw());
}

/// Test: Quit command ends the event loop cleanly.
#[test]
fn test_end_to_end_quit() {
    let mut state = EditorState::new();

    // Enter command mode
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

    // Before Enter, should not quit
    assert!(!state.should_quit());

    // After Enter, should quit
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(state.should_quit());
}

/// Test: Multiple iterations maintain state consistency.
#[test]
fn test_end_to_end_state_consistency() {
    let mut state = EditorState::new();
    let mut prev_snapshot: Option<EditorSnapshot> = None;

    // Simulate 100 iterations
    for i in 0..100 {
        let event = if i % 10 == 0 {
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE)
        } else if i % 10 == 9 {
            KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE)
        } else {
            KeyEvent::new(KeyCode::Char(((i % 26) as u8 + b'a') as char), KeyModifiers::NONE)
        };

        let (snapshot, _diff) = event_loop_iteration(&mut state, event, prev_snapshot.as_ref());

        // Invariants must hold after every iteration
        assert!(snapshot.cursor.line() < snapshot.buffer.line_count.max(1));

        prev_snapshot = Some(snapshot);
    }
}

/// Test: Visual mode is entered and exited correctly.
#[test]
fn test_end_to_end_visual_mode() {
    let mut state = EditorState::new();

    // Start in Normal
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);

    // Escape to normal
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);

    // v -> Visual
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);

    // Escape -> Normal
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE),
        Some(&snapshot),
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Replace mode overwrites text correctly.
#[test]
fn test_end_to_end_replace_mode() {
    let mut state = EditorState::new();

    // Insert some text first
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // R -> Replace mode
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Replace);

    // Type to overwrite
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE),
        Some(&snapshot),
    );

    // First char should be replaced
    assert!(snapshot.buffer.lines[0].starts_with("X"));
}

/// Test: Delete in normal mode works correctly.
#[test]
fn test_end_to_end_delete() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // x should delete char under cursor
    let snapshot1 = state.snapshot();
    let initial_len = snapshot1.buffer.lines[0].len();

    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();

    assert!(snapshot2.buffer.lines[0].len() < initial_len);
}

/// Test: Append mode enters insert after cursor.
#[test]
fn test_end_to_end_append() {
    let mut state = EditorState::new();

    // a -> Append (Insert after cursor)
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: o opens new line below.
#[test]
fn test_end_to_end_open_below() {
    let mut state = EditorState::new();

    // o -> Insert on new line below
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: O opens new line above.
#[test]
fn test_end_to_end_open_above() {
    let mut state = EditorState::new();

    // O -> Insert on new line above
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}
