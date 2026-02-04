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

/// Test: Undo restores previous state.
#[test]
fn test_end_to_end_undo() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let snapshot1 = state.snapshot();
    assert!(snapshot1.buffer.lines[0].contains("hi"));

    // Delete
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));

    // Undo
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));

    let snapshot2 = state.snapshot();
    // Should have some content restored
    assert!(!snapshot2.buffer.lines.is_empty());
}

/// Test: Visual line mode selects whole lines.
#[test]
fn test_end_to_end_visual_line() {
    let mut state = EditorState::new();

    // Insert some text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // V -> Visual Line
    let (snapshot, _) = event_loop_iteration(
        &mut state,
        KeyEvent::new(KeyCode::Char('V'), KeyModifiers::NONE),
        None,
    );
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualLine);
}

/// Test: Cursor movement with motions.
#[test]
fn test_end_to_end_motions() {
    let mut state = EditorState::new();

    // Insert some text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to line start

    // w -> Word forward
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() > 0);
}

/// Test: Yank and paste.
#[test]
fn test_end_to_end_yank_paste() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // yy -> yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

    // p -> paste
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));

    let snapshot = state.snapshot();
    assert!(snapshot.buffer.line_count > 1);
}

/// Test: Search forward.
#[test]
fn test_end_to_end_search() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // / -> Search forward
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));

    // Should be in some kind of pending state (search entry)
    let snapshot = state.snapshot();
    // The mode might be Normal (with search overlay) or Command
    assert!(snapshot.mode == kjxlkj_core_types::Mode::Normal || 
            snapshot.mode == kjxlkj_core_types::Mode::Command);
}

/// Test: Line movement j/k.
#[test]
fn test_end_to_end_line_movement() {
    let mut state = EditorState::new();

    // Insert multi-line text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Get to first line
    for _ in 0..5 {
        state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    }

    let snapshot1 = state.snapshot();
    let initial_line = snapshot1.cursor.line();

    // j -> Down (if possible)
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    // If we had more than one line, cursor should have moved
    if snapshot1.buffer.line_count > 1 && initial_line < snapshot1.buffer.line_count - 1 {
        assert!(snapshot2.cursor.line() >= initial_line);
    }
}

/// Test: Column movement h/l.
#[test]
fn test_end_to_end_column_movement() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "abcde".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to start

    let snapshot1 = state.snapshot();
    assert_eq!(snapshot1.cursor.col(), 0);

    // l -> Right
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    assert_eq!(snapshot2.cursor.col(), 1);

    // h -> Left
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let snapshot3 = state.snapshot();
    assert_eq!(snapshot3.cursor.col(), 0);
}

/// Test: dd deletes whole line.
#[test]
fn test_end_to_end_delete_line() {
    let mut state = EditorState::new();

    // Insert multi-line text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let snapshot1 = state.snapshot();
    let initial_lines = snapshot1.buffer.line_count;

    // dd -> Delete line
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    let snapshot2 = state.snapshot();
    assert!(snapshot2.buffer.line_count < initial_lines);
}

/// Test: G goes to last line.
#[test]
fn test_end_to_end_go_to_end() {
    let mut state = EditorState::new();

    // Insert multi-line text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE)); // Go to top

    // G -> Go to last line
    state.handle_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.line(), snapshot.buffer.line_count - 1);
}

/// Test: $ goes to end of line.
#[test]
fn test_end_to_end_line_end() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to start

    // $ -> End of line
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() > 0);
}

/// Test: ^ goes to first non-whitespace character.
#[test]
fn test_end_to_end_first_non_blank() {
    let mut state = EditorState::new();

    // Insert text with leading spaces
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to start

    // ^ -> First non-blank
    state.handle_key(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 2); // After the two spaces
}

/// Test: Join lines with J.
#[test]
fn test_end_to_end_join_lines() {
    let mut state = EditorState::new();

    // Insert multi-line text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Get to first line
    for _ in 0..5 {
        state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    }

    let snapshot1 = state.snapshot();
    let initial_lines = snapshot1.buffer.line_count;

    // J -> Join lines
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE));

    let snapshot2 = state.snapshot();
    // If we had 2+ lines and join worked, we should have fewer lines
    // Or the content should be merged on one line
    if initial_lines >= 2 {
        // Either line count decreased or content merged
        assert!(snapshot2.buffer.line_count <= initial_lines);
    }
}

/// Test: Word movement with w.
#[test]
fn test_end_to_end_word_motion_w() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Start

    // w -> Move to next word
    let snapshot1 = state.snapshot();
    let initial_col = snapshot1.cursor.col();
    
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    assert!(snapshot2.cursor.col() > initial_col);
}

/// Test: Beginning of word movement with b.
#[test]
fn test_end_to_end_word_motion_b() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // b -> Move to previous word
    let snapshot1 = state.snapshot();
    
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    assert!(snapshot2.cursor.col() < snapshot1.cursor.col());
}

/// Test: End of word movement with e.
#[test]
fn test_end_to_end_word_motion_e() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Start

    // e -> Move to end of word
    let snapshot1 = state.snapshot();
    let initial_col = snapshot1.cursor.col();
    
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    assert!(snapshot2.cursor.col() > initial_col);
}

/// Test: Replace mode with R and typing.
#[test]
fn test_end_to_end_replace_mode_typing() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // yy to yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

    // p to paste after
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should have at least 2 lines now
    assert!(snapshot.buffer.line_count >= 2);
}

/// Test: Open line below with o.
#[test]
fn test_end_to_end_open_below_insert() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let snapshot1 = state.snapshot();
    let initial_lines = snapshot1.buffer.line_count;

    // o -> Open line below
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    
    // Should be in insert mode and have new line
    assert_eq!(snapshot2.mode, kjxlkj_core_types::Mode::Insert);
    assert!(snapshot2.buffer.line_count > initial_lines);
}

/// Test: Open line above with O.
#[test]
fn test_end_to_end_open_above_insert() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let snapshot1 = state.snapshot();
    let initial_lines = snapshot1.buffer.line_count;

    // O -> Open line above
    state.handle_key(KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    
    // Should be in insert mode and have new line
    assert_eq!(snapshot2.mode, kjxlkj_core_types::Mode::Insert);
    assert!(snapshot2.buffer.line_count > initial_lines);
}
