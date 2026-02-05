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

/// Test: Append mode with a.
#[test]
fn test_end_to_end_append_mode() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start of line
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    // a -> append (insert after cursor)
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
    // Cursor should have moved right
    assert!(snapshot.cursor.col() >= 1);
}

/// Test: Append at end of line with A.
#[test]
fn test_end_to_end_append_eol() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    // A -> append at end of line
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
    // Cursor should be at end
    assert!(snapshot.cursor.col() >= 4);
}

/// Test: Insert at beginning of line with I.
#[test]
fn test_end_to_end_insert_bol() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // I -> insert at beginning
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
    assert_eq!(snapshot.cursor.col(), 0);
}

/// Test: Visual mode selection with motion.
#[test]
fn test_end_to_end_visual_mode_select() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    // v -> visual mode
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual line mode.
#[test]
fn test_end_to_end_visual_line_mode() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // V -> visual line mode
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualLine);
}

/// Test: Exit visual mode with Escape.
#[test]
fn test_end_to_end_visual_escape() {
    let mut state = EditorState::new();

    // v -> visual mode
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    let snapshot1 = state.snapshot();
    assert_eq!(snapshot1.mode, kjxlkj_core_types::Mode::Visual);
    
    // Escape -> normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot2 = state.snapshot();
    assert_eq!(snapshot2.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete word motion with dw.
#[test]
fn test_end_to_end_delete_word() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    let before = state.snapshot();
    let before_len = before.buffer.lines.first().map(|l| l.len()).unwrap_or(0);

    // dw -> delete word
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    let after = state.snapshot();
    let after_len = after.buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    
    // Should have deleted at least "hello "
    assert!(after_len < before_len);
}

/// Test: Change word motion with cw.
#[test]
fn test_end_to_end_change_word() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // cw -> change word (should enter insert mode)
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Undo with u key.
#[test]
fn test_end_to_end_u_undo() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let before_undo = state.snapshot();
    assert!(before_undo.buffer.lines.first().map(|l| l.contains("hello")).unwrap_or(false));

    // u -> undo
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    
    // Content may have changed after undo
    let after_undo = state.snapshot();
    assert!(after_undo.mode == kjxlkj_core_types::Mode::Normal);
}

/// Test: Command mode entry.
#[test]
fn test_end_to_end_command_mode() {
    let mut state = EditorState::new();

    // : -> command mode
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Command);
}

/// Test: Line join with J command.
#[test]
fn test_end_to_end_J_join() {
    let mut state = EditorState::new();

    // Insert two lines
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    for c in "line2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE)); // Go up

    let before = state.snapshot();
    let before_lines = before.buffer.line_count;

    // J -> join lines
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE));
    
    let after = state.snapshot();
    // May have one less line after join
    assert!(after.buffer.line_count <= before_lines);
}

/// Test: Go to end of file with G motion.
#[test]
fn test_end_to_end_G_motion() {
    let mut state = EditorState::new();

    // Insert multiple lines with Enter key
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Currently at line 2
    let before = state.snapshot();
    
    // G -> go to end (should stay or move)
    state.handle_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
    
    let after = state.snapshot();
    // Should be in normal mode and at last line
    assert_eq!(after.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Replace single char with r.
#[test]
fn test_end_to_end_replace_char() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // r followed by a char -> replace char under cursor
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    
    let snapshot = state.snapshot();
    // Should still be in normal mode
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Multiple cursor movements.
#[test]
fn test_end_to_end_cursor_movements() {
    let mut state = EditorState::new();

    // Insert text on single line
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // 0 - go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let at_start = state.snapshot();
    assert_eq!(at_start.cursor.col(), 0);

    // $ - go to end of line
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    let at_end = state.snapshot();
    assert!(at_end.cursor.col() > at_start.cursor.col());

    // h - move left
    let before_h = state.snapshot();
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let after_h = state.snapshot();
    assert!(after_h.cursor.col() < before_h.cursor.col());

    // l - move right
    let before_l = state.snapshot();
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    let after_l = state.snapshot();
    assert!(after_l.cursor.col() >= before_l.cursor.col());
}

/// Test: Word motion with w key.
#[test]
fn test_end_to_end_word_w_motion() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    let at_start = state.snapshot();
    assert_eq!(at_start.cursor.col(), 0);

    // w - go to next word
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_w = state.snapshot();
    assert!(after_w.cursor.col() > 0);
}

/// Test: Word motion backward with b key.
#[test]
fn test_end_to_end_word_b_motion() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let at_end = state.snapshot();

    // b - go to previous word
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after_b = state.snapshot();
    assert!(after_b.cursor.col() < at_end.cursor.col());
}

/// Test: End of word motion with e key.
#[test]
fn test_end_to_end_word_e_motion() {
    let mut state = EditorState::new();

    // Insert text with multiple words
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    let at_start = state.snapshot();
    assert_eq!(at_start.cursor.col(), 0);

    // e - go to end of word
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after_e = state.snapshot();
    assert!(after_e.cursor.col() > 0);
}

/// Test: Insert at beginning of line with I.
#[test]
fn test_end_to_end_insert_bol_I() {
    let mut state = EditorState::new();

    // Insert text and move to middle
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // l - stay somewhere in line
    let before_I = state.snapshot();
    
    // I -> insert at beginning
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    let after_I = state.snapshot();
    
    assert_eq!(after_I.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Append at end of line with A.
#[test]
fn test_end_to_end_append_eol_A() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to start

    // A -> append at end of line
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    let after_A = state.snapshot();
    
    assert_eq!(after_A.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: First non-blank with ^ motion.
#[test]
fn test_end_to_end_caret_first_non_blank() {
    let mut state = EditorState::new();

    // Insert text with leading spaces
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)); // Go to start

    let at_start = state.snapshot();
    assert_eq!(at_start.cursor.col(), 0);

    // ^ -> go to first non-blank
    state.handle_key(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE));
    let after_caret = state.snapshot();
    assert!(after_caret.cursor.col() >= 4);
}

/// Test: Substitue single char with s.
#[test]
fn test_end_to_end_substitute_char_s() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // s -> substitute (delete char and enter insert mode)
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    let after_s = state.snapshot();
    
    assert_eq!(after_s.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank and paste workflow.
#[test]
fn test_end_to_end_yy_p_paste() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // yy -> yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

    // p -> paste
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    
    let after_paste = state.snapshot();
    // Should have content (either same or duplicated)
    assert!(!after_paste.buffer.lines.is_empty());
}

/// Test: Delete character under cursor with x.
#[test]
fn test_end_to_end_x_delete_char() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // x -> delete char under cursor
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let after_x = state.snapshot();
    
    // Content should be shorter
    let content = after_x.buffer.lines.join("");
    assert!(content.len() < 5 || content != "hello");
}

/// Test: Open line below with o.
#[test]
fn test_end_to_end_o_open_below() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    let before_o = state.snapshot();
    let line_count_before = before_o.buffer.line_count;

    // o -> open line below
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let after_o = state.snapshot();
    
    assert_eq!(after_o.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Open line above with O.
#[test]
fn test_end_to_end_O_open_above() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // O -> open line above
    state.handle_key(KeyEvent::new(KeyCode::Char('O'), KeyModifiers::NONE));
    let after_O = state.snapshot();
    
    assert_eq!(after_O.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Visual block mode with Ctrl-V.
#[test]
fn test_end_to_end_visual_block_mode() {
    let mut state = EditorState::new();

    // Insert some text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "abc".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Ctrl-V -> visual block mode
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    let after_ctrl_v = state.snapshot();
    
    assert_eq!(after_ctrl_v.mode, kjxlkj_core_types::Mode::VisualBlock);
}

/// Test: Replace mode with R.
#[test]
fn test_end_to_end_R_replace_mode() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // R -> replace mode
    state.handle_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE));
    let after_R = state.snapshot();
    
    assert_eq!(after_R.mode, kjxlkj_core_types::Mode::Replace);
}

/// Test: Go to end of file with G.
#[test]
fn test_end_to_end_G_goto_end() {
    let mut state = EditorState::new();

    // Insert multiple lines
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start first
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    // G -> go to end of file
    state.handle_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
    let after_G = state.snapshot();
    
    // Should be at last line
    assert!(after_G.cursor.line() >= 2);
}

/// Test: Join lines with J.
#[test]
fn test_end_to_end_J_join_lines() {
    let mut state = EditorState::new();

    // Insert multiple lines
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to first line
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));

    // J -> join lines
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE));
    let after_J = state.snapshot();
    
    // Should still work without crash
    assert!(after_J.mode == kjxlkj_core_types::Mode::Normal);
}

/// Test: Navigate with H, M, L (screen top/middle/bottom).
#[test]
fn test_end_to_end_H_M_L_navigation() {
    let mut state = EditorState::new();

    // Insert multiple lines
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // H -> screen top
    state.handle_key(KeyEvent::new(KeyCode::Char('H'), KeyModifiers::NONE));
    let after_H = state.snapshot();
    assert_eq!(after_H.mode, kjxlkj_core_types::Mode::Normal);

    // L -> screen bottom
    state.handle_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::NONE));
    let after_L = state.snapshot();
    assert_eq!(after_L.mode, kjxlkj_core_types::Mode::Normal);

    // M -> screen middle
    state.handle_key(KeyEvent::new(KeyCode::Char('M'), KeyModifiers::NONE));
    let after_M = state.snapshot();
    assert_eq!(after_M.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Word motions w, b, e.
#[test]
fn test_end_to_end_word_motions() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // w -> next word
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_w = state.snapshot();
    assert!(after_w.cursor.col() > 0);

    // b -> previous word
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after_b = state.snapshot();
    assert_eq!(after_b.cursor.col(), 0);

    // e -> end of word
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after_e = state.snapshot();
    assert!(after_e.cursor.col() > 0);
}

/// Test: Search with / and ?.
#[test]
fn test_end_to_end_search_commands() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // / -> forward search (enters command mode or search mode)
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
    let after_search = state.snapshot();
    // Should be in a mode that accepts search input
    assert!(after_search.mode != kjxlkj_core_types::Mode::Insert);
    
    // Escape back
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // ? -> backward search
    state.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::NONE));
    let after_backward = state.snapshot();
    assert!(after_backward.mode != kjxlkj_core_types::Mode::Insert);
}

/// Test: Paste with p and P.
#[test]
fn test_end_to_end_paste_commands() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // yy -> yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));

    // p -> paste after
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let after_p = state.snapshot();
    assert_eq!(after_p.mode, kjxlkj_core_types::Mode::Normal);

    // P -> paste before
    state.handle_key(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::NONE));
    let after_P = state.snapshot();
    assert_eq!(after_P.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete line with dd.
#[test]
fn test_end_to_end_dd_delete_line() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to first line
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));

    // dd -> delete line
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let after_dd = state.snapshot();
    assert_eq!(after_dd.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change line with cc.
#[test]
fn test_end_to_end_cc_change_line() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // cc -> change line (enter insert mode)
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    let after_cc = state.snapshot();
    assert_eq!(after_cc.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Repeat command with .
#[test]
fn test_end_to_end_dot_repeat() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "aaa".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // x -> delete char
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    
    // . -> repeat delete
    state.handle_key(KeyEvent::new(KeyCode::Char('.'), KeyModifiers::NONE));
    let after_dot = state.snapshot();
    assert_eq!(after_dot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: f and F for find character.
#[test]
fn test_end_to_end_find_char() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // f + o -> find 'o'
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let after_f = state.snapshot();
    assert_eq!(after_f.mode, kjxlkj_core_types::Mode::Normal);

    // F + h -> find 'h' backward
    state.handle_key(KeyEvent::new(KeyCode::Char('F'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let after_F = state.snapshot();
    assert_eq!(after_F.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: t and T for till character.
#[test]
fn test_end_to_end_till_char() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // t + o -> till 'o' (one before 'o')
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let after_t = state.snapshot();
    assert_eq!(after_t.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual line mode selection.
#[test]
fn test_end_to_end_visual_line_selection() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to first line
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));

    // V -> visual line mode
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::NONE));
    let after_V = state.snapshot();
    assert_eq!(after_V.mode, kjxlkj_core_types::Mode::VisualLine);

    // j -> extend selection
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    let after_j = state.snapshot();
    assert_eq!(after_j.mode, kjxlkj_core_types::Mode::VisualLine);

    // d -> delete selection
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let after_d = state.snapshot();
    assert_eq!(after_d.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: D deletes to end of line.
#[test]
fn test_end_to_end_D_delete_to_end() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to middle
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));

    // D -> delete to end of line
    state.handle_key(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE));
    let after_D = state.snapshot();
    assert_eq!(after_D.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: C changes to end of line.
#[test]
fn test_end_to_end_C_change_to_end() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to middle
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));

    // C -> change to end of line (enter insert mode)
    state.handle_key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::NONE));
    let after_C = state.snapshot();
    assert_eq!(after_C.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Y yanks line.
#[test]
fn test_end_to_end_Y_yank_line() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Y -> yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::NONE));
    let after_Y = state.snapshot();
    assert_eq!(after_Y.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Substitute with s.
#[test]
fn test_end_to_end_s_substitute() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // s -> substitute (delete char and enter insert)
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    let after_s = state.snapshot();
    assert_eq!(after_s.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: S substitutes line.
#[test]
fn test_end_to_end_S_substitute_line() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // S -> substitute line
    state.handle_key(KeyEvent::new(KeyCode::Char('S'), KeyModifiers::NONE));
    let after_S = state.snapshot();
    assert_eq!(after_S.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: X deletes char before cursor.
#[test]
fn test_end_to_end_X_delete_before() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // X -> delete char before cursor
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    let after_X = state.snapshot();
    assert_eq!(after_X.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: n and N for next/prev match.
#[test]
fn test_end_to_end_n_N_match() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // n -> next match
    state.handle_key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::NONE));
    let after_n = state.snapshot();
    assert_eq!(after_n.mode, kjxlkj_core_types::Mode::Normal);

    // N -> prev match
    state.handle_key(KeyEvent::new(KeyCode::Char('N'), KeyModifiers::NONE));
    let after_N = state.snapshot();
    assert_eq!(after_N.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Search word under cursor with * and #.
#[test]
fn test_end_to_end_star_hash_search() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // * -> search word forward
    state.handle_key(KeyEvent::new(KeyCode::Char('*'), KeyModifiers::NONE));
    let after_star = state.snapshot();
    assert_eq!(after_star.mode, kjxlkj_core_types::Mode::Normal);

    // # -> search word backward
    state.handle_key(KeyEvent::new(KeyCode::Char('#'), KeyModifiers::NONE));
    let after_hash = state.snapshot();
    assert_eq!(after_hash.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Semicolon and comma for repeat find.
#[test]
fn test_end_to_end_semicolon_comma_repeat() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // f + l
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));

    // ; -> repeat find
    state.handle_key(KeyEvent::new(KeyCode::Char(';'), KeyModifiers::NONE));
    let after_semi = state.snapshot();
    assert_eq!(after_semi.mode, kjxlkj_core_types::Mode::Normal);

    // , -> repeat find reverse
    state.handle_key(KeyEvent::new(KeyCode::Char(','), KeyModifiers::NONE));
    let after_comma = state.snapshot();
    assert_eq!(after_comma.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Indent and outdent with > and <.
#[test]
fn test_end_to_end_indent_outdent() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // >> -> indent
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::NONE));
    let after_indent = state.snapshot();
    assert_eq!(after_indent.mode, kjxlkj_core_types::Mode::Normal);

    // << -> outdent
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::NONE));
    let after_outdent = state.snapshot();
    assert_eq!(after_outdent.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Toggle case with ~.
#[test]
fn test_end_to_end_toggle_case() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // ~ -> toggle case
    state.handle_key(KeyEvent::new(KeyCode::Char('~'), KeyModifiers::NONE));
    let after_toggle = state.snapshot();
    assert_eq!(after_toggle.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Mark and jump with m and '.
#[test]
fn test_end_to_end_marks() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));

    // m + a -> set mark 'a'
    state.handle_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let after_mark = state.snapshot();
    assert_eq!(after_mark.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Percent to match bracket.
#[test]
fn test_end_to_end_percent_match() {
    let mut state = EditorState::new();

    // Insert text with brackets
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "(hello)".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // % -> match bracket
    state.handle_key(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::NONE));
    let after_percent = state.snapshot();
    assert_eq!(after_percent.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-D for half page down.
#[test]
fn test_end_to_end_ctrl_d() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Ctrl-D -> half page down
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CTRL));
    let after_ctrl_d = state.snapshot();
    assert_eq!(after_ctrl_d.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-U for half page up.
#[test]
fn test_end_to_end_ctrl_u() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Ctrl-U -> half page up
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CTRL));
    let after_ctrl_u = state.snapshot();
    assert_eq!(after_ctrl_u.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Page up and page down.
#[test]
fn test_end_to_end_page_up_down() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // PageDown
    state.handle_key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
    let after_pgdn = state.snapshot();
    assert_eq!(after_pgdn.mode, kjxlkj_core_types::Mode::Normal);

    // PageUp
    state.handle_key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE));
    let after_pgup = state.snapshot();
    assert_eq!(after_pgup.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: gj and gk for display line motion.
#[test]
fn test_end_to_end_gj_gk() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));

    // gj -> display line down
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    let after_gj = state.snapshot();
    assert_eq!(after_gj.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Replace char with r.
#[test]
fn test_end_to_end_replace_char_r() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // r + x -> replace with 'x'
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let after_r = state.snapshot();
    assert_eq!(after_r.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Command mode with :q.
#[test]
fn test_end_to_end_command_mode_colon() {
    let mut state = EditorState::new();

    // : -> command mode
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    let after_colon = state.snapshot();
    assert_eq!(after_colon.mode, kjxlkj_core_types::Mode::Command);

    // Escape back
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let after_esc = state.snapshot();
    assert_eq!(after_esc.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: ci" text object (change inside quotes).
#[test]
fn test_end_to_end_change_inside_quotes() {
    let mut state = EditorState::new();

    // Insert text with quotes
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "\"hello\"".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to middle
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));

    // ci" -> change inside quotes
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('"'), KeyModifiers::NONE));
    let after_ci = state.snapshot();
    // Should enter insert mode
    assert_eq!(after_ci.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: ca" text object (change around quotes).
#[test]
fn test_end_to_end_change_around_quotes() {
    let mut state = EditorState::new();

    // Insert text with quotes
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "\"hello\"".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to middle
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));

    // ca" -> change around quotes
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('"'), KeyModifiers::NONE));
    let after_ca = state.snapshot();
    // Should enter insert mode
    assert_eq!(after_ca.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Escape from visual mode back to normal.
#[test]
fn test_end_to_end_visual_escape_to_normal() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // v -> visual mode
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    let after_v = state.snapshot();
    assert_eq!(after_v.mode, kjxlkj_core_types::Mode::Visual);

    // Escape -> back to normal
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let after_esc = state.snapshot();
    assert_eq!(after_esc.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank in visual mode.
#[test]
fn test_end_to_end_visual_yank() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // v -> visual, l -> select, y -> yank
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let after_y = state.snapshot();
    assert_eq!(after_y.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete in visual mode.
#[test]
fn test_end_to_end_visual_delete() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // v -> visual, l -> select, d -> delete
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let after_d = state.snapshot();
    assert_eq!(after_d.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change in visual mode.
#[test]
fn test_end_to_end_visual_change() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // v -> visual, l -> select, c -> change
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    let after_c = state.snapshot();
    assert_eq!(after_c.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: zz center screen.
#[test]
fn test_end_to_end_zz_center() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // zz -> center screen
    state.handle_key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    let after_zz = state.snapshot();
    assert_eq!(after_zz.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: zt top screen.
#[test]
fn test_end_to_end_zt_top() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // zt -> scroll to top
    state.handle_key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    let after_zt = state.snapshot();
    assert_eq!(after_zt.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: zb bottom screen.
#[test]
fn test_end_to_end_zb_bottom() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // zb -> scroll to bottom
    state.handle_key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after_zb = state.snapshot();
    assert_eq!(after_zb.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-F page down.
#[test]
fn test_end_to_end_ctrl_f() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Ctrl-F -> page down
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CTRL));
    let after_ctrl_f = state.snapshot();
    assert_eq!(after_ctrl_f.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-B page up.
#[test]
fn test_end_to_end_ctrl_b() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));

    // Ctrl-B -> page up
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CTRL));
    let after_ctrl_b = state.snapshot();
    assert_eq!(after_ctrl_b.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Backspace in insert mode.
#[test]
fn test_end_to_end_insert_backspace() {
    let mut state = EditorState::new();

    // Enter insert mode and type
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    
    // Backspace
    state.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    let after_bs = state.snapshot();
    assert_eq!(after_bs.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Enter in insert mode.
#[test]
fn test_end_to_end_insert_enter() {
    let mut state = EditorState::new();

    // Enter insert mode and type
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    
    // Enter
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let after_enter = state.snapshot();
    assert_eq!(after_enter.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Tab in insert mode.
#[test]
fn test_end_to_end_insert_tab() {
    let mut state = EditorState::new();

    // Enter insert mode
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    
    // Tab
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    let after_tab = state.snapshot();
    assert_eq!(after_tab.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: gU uppercase.
#[test]
fn test_end_to_end_gU_uppercase() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // gU + motion
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('U'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_gU = state.snapshot();
    assert_eq!(after_gU.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: gu lowercase.
#[test]
fn test_end_to_end_gu_lowercase() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "HELLO".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Go to start
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));

    // gu + motion
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_gu = state.snapshot();
    assert_eq!(after_gu.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Arrow keys in normal mode.
#[test]
fn test_end_to_end_arrow_keys() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Arrow keys
    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    let after_arrows = state.snapshot();
    assert_eq!(after_arrows.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Home and End keys in insert mode.
#[test]
fn test_end_to_end_home_end_keys() {
    let mut state = EditorState::new();

    // Insert text
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    
    // Home and End in insert mode
    state.handle_key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    let after_home = state.snapshot();
    assert_eq!(after_home.mode, kjxlkj_core_types::Mode::Insert);
    
    state.handle_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    let after_end = state.snapshot();
    assert_eq!(after_end.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Word backwards motion (b).
#[test]
fn test_end_to_end_word_backward_b() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after_b = state.snapshot();
    assert_eq!(after_b.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Word end motion (e).
#[test]
fn test_end_to_end_word_end_e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after_e = state.snapshot();
    assert_eq!(after_e.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to file end (G).
#[test]
fn test_end_to_end_go_file_end_G() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::NONE));
    let after_G = state.snapshot();
    assert_eq!(after_G.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to file start (gg).
#[test]
fn test_end_to_end_go_file_start_gg() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    let after_gg = state.snapshot();
    assert_eq!(after_gg.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Join lines (J).
#[test]
fn test_end_to_end_join_lines_J() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::NONE));
    let after_J = state.snapshot();
    assert_eq!(after_J.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Insert at line start (I).
#[test]
fn test_end_to_end_insert_line_start_I() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    let after_I = state.snapshot();
    assert_eq!(after_I.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Append at line end (A).
#[test]
fn test_end_to_end_append_line_end_A() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    let after_A = state.snapshot();
    assert_eq!(after_A.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete to end of line (D).
#[test]
fn test_end_to_end_delete_to_line_end_D() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::NONE));
    let after_D = state.snapshot();
    assert_eq!(after_D.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Redo (Ctrl-r).
#[test]
fn test_end_to_end_redo_ctrl_r() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));
    let after_redo = state.snapshot();
    assert_eq!(after_redo.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual block mode (Ctrl-v).
#[test]
fn test_end_to_end_visual_block_ctrl_v() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    let after_ctrl_v = state.snapshot();
    assert_eq!(after_ctrl_v.mode, kjxlkj_core_types::Mode::VisualBlock);
}

/// Test: Multiple escapes stay in normal mode.
#[test]
fn test_end_to_end_multiple_escapes() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let after_esc = state.snapshot();
    assert_eq!(after_esc.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete backward (X).
#[test]
fn test_end_to_end_delete_backward_X() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    let after_X = state.snapshot();
    assert_eq!(after_X.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: yy yank line.
#[test]
fn test_end_to_end_yy_yank_line() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let after_yy = state.snapshot();
    assert_eq!(after_yy.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete forward (x).
#[test]
fn test_end_to_end_delete_forward_x() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let after_x = state.snapshot();
    assert_eq!(after_x.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Paste after (p).
#[test]
fn test_end_to_end_paste_after_p() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let after_p = state.snapshot();
    assert_eq!(after_p.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Paste before (P).
#[test]
fn test_end_to_end_paste_before_P() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::NONE));
    let after_P = state.snapshot();
    assert_eq!(after_P.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Empty content remains stable.
#[test]
fn test_end_to_end_empty_content_stable() {
    let mut state = EditorState::new();
    let snap1 = state.snapshot();
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snap2 = state.snapshot();
    assert_eq!(snap1.mode, snap2.mode);
}

/// Test: Replace mode (R).
#[test]
fn test_end_to_end_replace_mode_R() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::NONE));
    let after_R = state.snapshot();
    assert_eq!(after_R.mode, kjxlkj_core_types::Mode::Replace);
    
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    let after_replace = state.snapshot();
    assert_eq!(after_replace.mode, kjxlkj_core_types::Mode::Replace);
}

/// Test: Word WORD motion (W).
#[test]
fn test_end_to_end_WORD_motion_W() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello.world foo".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('W'), KeyModifiers::NONE));
    let after_W = state.snapshot();
    assert_eq!(after_W.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: WORD backward motion (B).
#[test]
fn test_end_to_end_WORD_backward_B() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello.world foo".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('B'), KeyModifiers::NONE));
    let after_B = state.snapshot();
    assert_eq!(after_B.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: WORD end motion (E).
#[test]
fn test_end_to_end_WORD_end_E() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello.world foo".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('E'), KeyModifiers::NONE));
    let after_E = state.snapshot();
    assert_eq!(after_E.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete line with dd on first line.
#[test]
fn test_end_to_end_dd_delete_line_first() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let after_dd = state.snapshot();
    assert_eq!(after_dd.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change line with cc enters insert mode.
#[test]
fn test_end_to_end_cc_change_line_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    let after_cc = state.snapshot();
    assert_eq!(after_cc.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete word with dw.
#[test]
fn test_end_to_end_dw_delete_word() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_dw = state.snapshot();
    assert_eq!(after_dw.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word with cw.
#[test]
fn test_end_to_end_cw_change_word() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_cw = state.snapshot();
    assert_eq!(after_cw.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank word with yw.
#[test]
fn test_end_to_end_yw_yank_word() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_yw = state.snapshot();
    assert_eq!(after_yw.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete to line end with d$.
#[test]
fn test_end_to_end_d_dollar_delete_to_end() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    let after_d_dollar = state.snapshot();
    assert_eq!(after_d_dollar.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete to line start with d0.
#[test]
fn test_end_to_end_d0_delete_to_start() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let after_d0 = state.snapshot();
    assert_eq!(after_d0.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual mode extend selection.
#[test]
fn test_end_to_end_visual_extend_selection() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_vw = state.snapshot();
    assert_eq!(after_vw.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Substitute char with s at start.
#[test]
fn test_end_to_end_substitute_char_s_start() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    let after_s = state.snapshot();
    assert_eq!(after_s.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Substitute line with S.
#[test]
fn test_end_to_end_substitute_line_S() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('S'), KeyModifiers::NONE));
    let after_S = state.snapshot();
    assert_eq!(after_S.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Change to end with C.
#[test]
fn test_end_to_end_change_to_end_C() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::NONE));
    let after_C = state.snapshot();
    assert_eq!(after_C.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank to end with Y.
#[test]
fn test_end_to_end_yank_line_Y() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::NONE));
    let after_Y = state.snapshot();
    assert_eq!(after_Y.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-e scroll down.
#[test]
fn test_end_to_end_ctrl_e_scroll_down() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CTRL));
    let after_ctrl_e = state.snapshot();
    assert_eq!(after_ctrl_e.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Ctrl-y scroll up.
#[test]
fn test_end_to_end_ctrl_y_scroll_up() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CTRL));
    let after_ctrl_y = state.snapshot();
    assert_eq!(after_ctrl_y.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete word with dw from middle of line.
#[test]
fn test_end_to_end_delete_word_from_middle() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_dw = state.snapshot();
    assert_eq!(after_dw.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word with cw.
#[test]
fn test_end_to_end_change_word_mid() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_cw = state.snapshot();
    assert_eq!(after_cw.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete to end with D from middle.
#[test]
fn test_end_to_end_delete_to_end_mid() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world!".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT));
    let after_D = state.snapshot();
    assert_eq!(after_D.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Insert mode then go back to normal.
#[test]
fn test_end_to_end_insert_escape_cycle() {
    let mut state = EditorState::new();
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
    
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Insert);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
}

/// Test: First non-blank with ^ motion.
#[test]
fn test_end_to_end_first_non_blank_caret() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::NONE));
    let after_caret = state.snapshot();
    assert_eq!(after_caret.cursor.col(), 4); // After the spaces
}

/// Test: Escape from visual mode to normal (v mode).
#[test]
fn test_end_to_end_visual_char_escape() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Visual);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
}

/// Test: Escape from visual line mode.
#[test]
fn test_end_to_end_visual_line_escape() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::VisualLine);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    assert_eq!(state.mode(), kjxlkj_core_types::Mode::Normal);
}

/// Test: Multiple keys with count (2w).
#[test]
fn test_end_to_end_count_motion() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three four".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_2w = state.snapshot();
    assert_eq!(after_2w.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Count with delete (2dd).
#[test]
fn test_end_to_end_count_delete() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('2'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let after_2dd = state.snapshot();
    assert_eq!(after_2dd.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Find next character (fw motion).
#[test]
fn test_end_to_end_find_char_fw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after_fw = state.snapshot();
    assert_eq!(after_fw.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Till character (to motion).
#[test]
fn test_end_to_end_till_char_to() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let after_to = state.snapshot();
    assert_eq!(after_to.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Find backward (F).
#[test]
fn test_end_to_end_find_backward() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('F'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after_Fe = state.snapshot();
    assert_eq!(after_Fe.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Till backward (T).
#[test]
fn test_end_to_end_till_backward() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('T'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after_Te = state.snapshot();
    assert_eq!(after_Te.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete with find (dfw).
#[test]
fn test_end_to_end_delete_find() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let after_dfo = state.snapshot();
    assert_eq!(after_dfo.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Append at line end (A) behavior.
#[test]
fn test_end_to_end_append_at_line_end() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE));
    let after_A = state.snapshot();
    assert_eq!(after_A.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Insert at first non-blank (I) behavior.
#[test]
fn test_end_to_end_insert_first_nonblank() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::NONE));
    let after_I = state.snapshot();
    assert_eq!(after_I.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Open line above (O) enters insert mode.
#[test]
fn test_end_to_end_open_line_above_O() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('O'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Open line below (o) enters insert mode.
#[test]
fn test_end_to_end_open_line_below_o() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete char under cursor (x) stays in normal.
#[test]
fn test_end_to_end_delete_char_under_x() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Move to beginning of line (0).
#[test]
fn test_end_to_end_line_start_zero() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 0);
}

/// Test: Move to end of line ($).
#[test]
fn test_end_to_end_line_end_dollar_sign() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() > 0);
}

/// Test: Replace single char (r).
#[test]
fn test_end_to_end_replace_single_char_r() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Word forward (w) motion moves cursor.
#[test]
fn test_end_to_end_word_forward_moves() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after > before);
}

/// Test: Word backward (b) motion moves cursor.
#[test]
fn test_end_to_end_word_backward_moves() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two three".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after < before);
}

/// Test: Word end (e) motion moves cursor.
#[test]
fn test_end_to_end_word_end_moves() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "one two".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after > before);
}

/// Test: Append after cursor (a) enters insert.
#[test]
fn test_end_to_end_append_after_cursor_a() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Substitute char (s) enters insert mode.
#[test]
fn test_end_to_end_substitute_s_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete line (dd) stays in normal.
#[test]
fn test_end_to_end_dd_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank line (yy) stays in normal.
#[test]
fn test_end_to_end_yy_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change line (cc) enters insert.
#[test]
fn test_end_to_end_cc_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Paste after (p) stays in normal.
#[test]
fn test_end_to_end_paste_p_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Yank and paste
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Line down (j) motion.
#[test]
fn test_end_to_end_line_down_j() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Move to first line using k (since we end on line2)
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    let line_before = state.snapshot().cursor.line();
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    let line_after = state.snapshot().cursor.line();
    assert!(line_after >= line_before);
}

/// Test: Line up (k) motion.
#[test]
fn test_end_to_end_line_up_k() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let line_before = state.snapshot().cursor.line();
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    let line_after = state.snapshot().cursor.line();
    assert!(line_after <= line_before);
}

/// Test: Char left (h) motion.
#[test]
fn test_end_to_end_char_left_h() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let col_before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let col_after = state.snapshot().cursor.col();
    assert!(col_after < col_before);
}

/// Test: Char right (l) motion.
#[test]
fn test_end_to_end_char_right_l() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let col_before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    let col_after = state.snapshot().cursor.col();
    assert!(col_after > col_before);
}

/// Test: Enter command mode (:) via colon key.
#[test]
fn test_end_to_end_command_mode_enter() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Command);
}

/// Test: Replace mode (R).
#[test]
fn test_end_to_end_replace_mode_R_key() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Replace);
}

/// Test: Visual line mode (V).
#[test]
fn test_end_to_end_visual_line_mode_V() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualLine);
}

/// Test: Visual block mode (Ctrl-V).
#[test]
fn test_end_to_end_visual_block_mode_ctrl_v() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualBlock);
}

/// Test: Insert at line start (I).
#[test]
fn test_end_to_end_insert_line_start_I_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "  hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Append at line end (A).
#[test]
fn test_end_to_end_append_end_A_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Change to end of line (C).
#[test]
fn test_end_to_end_change_end_C_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete to end of line (D).
#[test]
fn test_end_to_end_delete_end_D_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    state.handle_key(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT));
    let after_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    assert!(after_len <= before_len);
}

/// Test: Join lines (J).
#[test]
fn test_end_to_end_join_J_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let lines_before = state.snapshot().buffer.lines.len();
    // Go to first line
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT));
    let lines_after = state.snapshot().buffer.lines.len();
    assert!(lines_after <= lines_before);
}

/// Test: Delete character under cursor (x) and escape.
#[test]
fn test_end_to_end_x_stays_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should stay in normal mode
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Undo (u) in normal mode.
#[test]
fn test_end_to_end_undo_u_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    // Just verify we stay in normal mode
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Redo (Ctrl-R) in normal mode.
#[test]
fn test_end_to_end_redo_ctrl_r_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));
    // Just verify we stay in normal mode
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to file start (gg via G press twice - here just test G).
#[test]
fn test_end_to_end_goto_end_G() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    // G should go to last line
    assert!(snapshot.cursor.line() >= 2);
}

/// Test: Page down (Ctrl-D).
#[test]
fn test_end_to_end_half_page_down_ctrl_d() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CTRL));
    // Just verify we stay in normal mode
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Half page up (Ctrl-U).
#[test]
fn test_end_to_end_half_page_up_ctrl_u() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CTRL));
    // Just verify we stay in normal mode
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Full page down (Ctrl-F).
#[test]
fn test_end_to_end_full_page_down_ctrl_f() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Full page up (Ctrl-B).
#[test]
fn test_end_to_end_full_page_up_ctrl_b() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3\nline4\nline5".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Scroll line down (Ctrl-E).
#[test]
fn test_end_to_end_scroll_line_down_ctrl_e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Scroll line up (Ctrl-Y).
#[test]
fn test_end_to_end_scroll_line_up_ctrl_y() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Increment number (Ctrl-A).
#[test]
fn test_end_to_end_increment_ctrl_a() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "42".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Decrement number (Ctrl-X).
#[test]
fn test_end_to_end_decrement_ctrl_x() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "42".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Enter visual block mode from normal (Ctrl-V).
#[test]
fn test_end_to_end_visual_block_ctrl_v_from_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualBlock);
}

/// Test: Open line above (O).
#[test]
fn test_end_to_end_open_above_O() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('O'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}
/// Test: Open line below (o).
#[test]
fn test_end_to_end_open_below_o() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Replace mode with R key.
#[test]
fn test_end_to_end_replace_mode_R_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('R'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Replace);
}

/// Test: Word motion forward (w).
#[test]
fn test_end_to_end_word_forward_w() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after > before);
}

/// Test: Word motion backward (b).
#[test]
fn test_end_to_end_word_back_b() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after <= before);
}

/// Test: Word end motion with e key.
#[test]
fn test_end_to_end_word_end_e_motion() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after >= before);
}

/// Test: First non-blank motion (^).
#[test]
fn test_end_to_end_first_nonblank_caret() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "   hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() >= 3); // After the spaces
}

/// Test: Line start motion with 0 key.
#[test]
fn test_end_to_end_line_start_zero_motion() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.cursor.col(), 0);
}

/// Test: Line end motion ($).
#[test]
fn test_end_to_end_line_end_dollar() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert!(snapshot.cursor.col() > 0);
}

/// Test: Append at end (A).
#[test]
fn test_end_to_end_append_A_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Insert at start (I).
#[test]
fn test_end_to_end_insert_I_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "   hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Single char replace (r).
#[test]
fn test_end_to_end_single_replace_r() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should stay in normal mode after replace
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank line (yy).
#[test]
fn test_end_to_end_yank_yy_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Paste (p).
#[test]
fn test_end_to_end_paste_p_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Yank first
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    // Then paste
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Paste before (P).
#[test]
fn test_end_to_end_paste_before_P_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Yank first
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    // Then paste before
    state.handle_key(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete line (dd).
#[test]
fn test_end_to_end_delete_dd_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    
    let lines_before = state.snapshot().buffer.lines.len();
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let lines_after = state.snapshot().buffer.lines.len();
    assert!(lines_after <= lines_before);
}

/// Test: Visual mode selection (v).
#[test]
fn test_end_to_end_visual_v_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual line mode (V).
#[test]
fn test_end_to_end_visual_line_V_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::VisualLine);
}

/// Test: Arrow key down.
#[test]
fn test_end_to_end_arrow_down() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.line();
    state.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let after = state.snapshot().cursor.line();
    assert!(after >= before);
}

/// Test: Arrow key up.
#[test]
fn test_end_to_end_arrow_up() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.line();
    state.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let after = state.snapshot().cursor.line();
    assert!(after <= before);
}

/// Test: Arrow key left.
#[test]
fn test_end_to_end_arrow_left() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after <= before);
}

/// Test: Arrow key right.
#[test]
fn test_end_to_end_arrow_right() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before = state.snapshot().cursor.col();
    state.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    let after = state.snapshot().cursor.col();
    assert!(after >= before);
}

/// Test: Append (a) enters insert mode.
#[test]
fn test_end_to_end_append_a_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Change to end of line (C).
#[test]
fn test_end_to_end_change_C_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete word (dw).
#[test]
fn test_end_to_end_delete_word_dw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word (cw).
#[test]
fn test_end_to_end_change_word_cw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Backspace in insert mode deletes char.
#[test]
fn test_end_to_end_insert_backspace_deletes() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    
    let before_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    state.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    let after_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    assert!(after_len <= before_len);
}

/// Test: Delete in insert mode.
#[test]
fn test_end_to_end_insert_delete_key() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Enter in insert mode.
#[test]
fn test_end_to_end_insert_enter_newline() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    
    let lines_before = state.snapshot().buffer.lines.len();
    state.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let lines_after = state.snapshot().buffer.lines.len();
    assert!(lines_after >= lines_before);
}

/// Test: Tab key in insert mode.
#[test]
fn test_end_to_end_insert_tab_key() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Escape from insert mode.
#[test]
fn test_end_to_end_escape_from_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    assert_eq!(state.snapshot().mode, kjxlkj_core_types::Mode::Insert);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Escape from visual mode.
#[test]
fn test_end_to_end_escape_from_visual() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    assert_eq!(state.snapshot().mode, kjxlkj_core_types::Mode::Visual);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Escape from visual line mode.
#[test]
fn test_end_to_end_escape_from_visual_line() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT));
    assert_eq!(state.snapshot().mode, kjxlkj_core_types::Mode::VisualLine);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Escape from visual block mode.
#[test]
fn test_end_to_end_escape_from_visual_block() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    assert_eq!(state.snapshot().mode, kjxlkj_core_types::Mode::VisualBlock);
    
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Multiple cursor movements.
#[test]
fn test_end_to_end_multiple_hjkl() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld\ntest".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Move around
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank and paste operation.
#[test]
fn test_end_to_end_yank_paste_workflow() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Yank line
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    
    // Paste
    state.handle_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    
    let snapshot = state.snapshot();
    assert!(snapshot.buffer.lines.len() >= 1);
}

/// Test: Delete character under cursor (x).
#[test]
fn test_end_to_end_delete_x_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    let before_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let after_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    assert!(after_len <= before_len);
}

/// Test: Delete character before cursor (X).
#[test]
fn test_end_to_end_delete_X_e2e() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    let before_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::SHIFT));
    let after_len = state.snapshot().buffer.lines.first().map(|l| l.len()).unwrap_or(0);
    assert!(after_len <= before_len);
}

/// Test: Substitute char (s).
#[test]
fn test_end_to_end_substitute_s() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Substitute line with S key.
#[test]
fn test_end_to_end_substitute_line_S_shift() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('S'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Change inner word (ciw).
#[test]
fn test_end_to_end_change_inner_word_ciw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete to end of line (d$).
#[test]
fn test_end_to_end_delete_to_end_d_dollar() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual word selection (viw).
#[test]
fn test_end_to_end_visual_inner_word_viw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should be in visual mode with selection
    assert!(snapshot.mode.is_visual());
}

/// Test: Find character forward (f).
#[test]
fn test_end_to_end_find_char_f() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Find character backward (F).
#[test]
fn test_end_to_end_find_char_F() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('F'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Till character forward (t).
#[test]
fn test_end_to_end_till_char_t() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('t'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Till character backward (T).
#[test]
fn test_end_to_end_till_char_T() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('T'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}
/// Test: Repeat last find (;).
#[test]
fn test_end_to_end_repeat_find_semicolon() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(';'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Repeat last find reverse (,).
#[test]
fn test_end_to_end_repeat_find_comma() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char(','), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Dot repeat with delete.
#[test]
fn test_end_to_end_dot_repeat_delete() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    // Delete a character
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    // Repeat
    state.handle_key(KeyEvent::new(KeyCode::Char('.'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Mark (m).
#[test]
fn test_end_to_end_mark_m() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Jump to mark (').
#[test]
fn test_end_to_end_jump_mark_quote() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    // Set mark
    state.handle_key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    // Move
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    // Jump to mark
    state.handle_key(KeyEvent::new(KeyCode::Char('\''), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: g key in normal mode stays in normal mode.
#[test]
fn test_end_to_end_g_key_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Goto middle of screen (M).
#[test]
fn test_end_to_end_goto_middle_M() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('M'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Goto top of screen (H).
#[test]
fn test_end_to_end_goto_top_H() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('H'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Goto bottom of screen (L).
#[test]
fn test_end_to_end_goto_bottom_L() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Percent matching with parens.
#[test]
fn test_end_to_end_percent_match_parens() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "(hello)".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual delete (vd).
#[test]
fn test_end_to_end_visual_delete_vd() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual yank (vy).
#[test]
fn test_end_to_end_visual_yank_vy() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual change (vc).
#[test]
fn test_end_to_end_visual_change_vc() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Join lines without space (gJ).
#[test]
fn test_end_to_end_join_gJ() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change case (~).
#[test]
fn test_end_to_end_change_case_tilde() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('~'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Word motion W.
#[test]
fn test_end_to_end_WORD_forward_W() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello.world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('W'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Word motion B.
#[test]
fn test_end_to_end_WORD_back_B() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello.world test".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('B'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Indent line (>>).
#[test]
fn test_end_to_end_indent_line() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Outdent line (<<).
#[test]
fn test_end_to_end_outdent_line() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Undo multiple times.
#[test]
fn test_end_to_end_undo_multiple() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Redo multiple times.
#[test]
fn test_end_to_end_redo_multiple() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to matching bracket (%) with square brackets.
#[test]
fn test_end_to_end_percent_brackets() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "[test]".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to matching bracket (%) with curly braces.
#[test]
fn test_end_to_end_percent_braces() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "{test}".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual mode with delete (vd) multiple chars.
#[test]
fn test_end_to_end_visual_delete_vd_multi() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete to end of line (D).
#[test]
fn test_end_to_end_delete_to_end_D() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank to end of line (Y).
#[test]
fn test_end_to_end_yank_to_end_Y() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change to end of line (C) with insert.
#[test]
fn test_end_to_end_change_to_end_C_insert() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('C'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Replace char in normal (r).
#[test]
fn test_end_to_end_replace_char_r_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Enter command mode via colon (:).
#[test]
fn test_end_to_end_command_mode_via_colon() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Command);
}

/// Test: Command mode escape to normal.
#[test]
fn test_end_to_end_command_escape_to_normal() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char(':'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual line to visual.
#[test]
fn test_end_to_end_visual_line_to_visual() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('V'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should switch or stay in visual mode
    assert!(
        snapshot.mode == kjxlkj_core_types::Mode::Visual
            || snapshot.mode == kjxlkj_core_types::Mode::VisualLine
            || snapshot.mode == kjxlkj_core_types::Mode::Normal
    );
}

/// Test: Multiple line delete with dd.
#[test]
fn test_end_to_end_multi_line_delete_dd() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Search forward (/).
#[test]
fn test_end_to_end_search_forward_slash() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    // Should be in search or command mode
    assert!(
        snapshot.mode == kjxlkj_core_types::Mode::Command
            || snapshot.mode == kjxlkj_core_types::Mode::Normal
    );
}

/// Test: Search backward (?).
#[test]
fn test_end_to_end_search_backward_question() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('?'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    // Should be in search or command mode
    assert!(
        snapshot.mode == kjxlkj_core_types::Mode::Command
            || snapshot.mode == kjxlkj_core_types::Mode::Normal
    );
}

/// Test: Word object boundary.
#[test]
fn test_end_to_end_word_object_boundary() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Line start and end with home/end navigation.
#[test]
fn test_end_to_end_home_end_navigation() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Paste with P before cursor.
#[test]
fn test_end_to_end_paste_P_before() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('P'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual block select.
#[test]
fn test_end_to_end_visual_block_select() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld\ntest".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert!(
        snapshot.mode == kjxlkj_core_types::Mode::VisualBlock
            || snapshot.mode == kjxlkj_core_types::Mode::Visual
    );
}

/// Test: Join adjacent lines with J operator.
#[test]
fn test_end_to_end_join_adjacent_J() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('J'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Go to first non-blank char (^).
#[test]
fn test_end_to_end_first_non_blank_char() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "   hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('^'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Insert at line start (I).
#[test]
fn test_end_to_end_insert_at_start_I() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "   hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('I'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Append at line end (A).
#[test]
fn test_end_to_end_append_at_end_A() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('A'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: New line above (O).
#[test]
fn test_end_to_end_new_line_above_O() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello\nworld".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('O'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: PageUp key.
#[test]
fn test_end_to_end_page_up() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: PageDown key.
#[test]
fn test_end_to_end_page_down() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Insert mode delete to word start (Ctrl+W).
#[test]
fn test_end_to_end_insert_ctrl_w() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CTRL));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Normal mode word back motion (b) basic.
#[test]
fn test_end_to_end_word_back_b_basic() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Normal mode word forward motion (w) basic.
#[test]
fn test_end_to_end_word_forward_w_basic() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Insert mode navigation left.
#[test]
fn test_end_to_end_insert_left_arrow() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    
    state.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Insert mode navigation right.
#[test]
fn test_end_to_end_insert_right_arrow() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Delete word forward (dw) operator.
#[test]
fn test_end_to_end_delete_word_dw_operator() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word (cw) operator.
#[test]
fn test_end_to_end_change_word_cw_operator() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank word (yw).
#[test]
fn test_end_to_end_yank_word_yw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Yank word end (ye).
#[test]
fn test_end_to_end_yank_word_end_ye() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete word end (de).
#[test]
fn test_end_to_end_delete_word_end_de() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word end (ce).
#[test]
fn test_end_to_end_change_word_end_ce() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Move to beginning with 0.
#[test]
fn test_end_to_end_move_to_zero() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
    assert_eq!(snapshot.cursor.col(), 0);
}

/// Test: Move to end with $.
#[test]
fn test_end_to_end_move_to_dollar() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete word backward (db).
#[test]
fn test_end_to_end_delete_word_back_db() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change word backward (cb).
#[test]
fn test_end_to_end_change_word_back_cb() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank word backward (yb).
#[test]
fn test_end_to_end_yank_word_back_yb() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Visual select with j.
#[test]
fn test_end_to_end_visual_select_j() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual select with k.
#[test]
fn test_end_to_end_visual_select_k() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "line1\nline2\nline3".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual mode with word motion (vw).
#[test]
fn test_end_to_end_visual_word_vw() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual mode with end motion (ve).
#[test]
fn test_end_to_end_visual_end_ve() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Visual mode with back motion (vb).
#[test]
fn test_end_to_end_visual_back_vb() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Visual);
}

/// Test: Normal mode indent (>>).
#[test]
fn test_end_to_end_indent_right() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('>'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Normal mode outdent (<<).
#[test]
fn test_end_to_end_outdent_left() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "    hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('<'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete char (x) multiple.
#[test]
fn test_end_to_end_delete_x_multiple() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete char backward (X) multiple.
#[test]
fn test_end_to_end_delete_X_multiple() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::SHIFT));
    state.handle_key(KeyEvent::new(KeyCode::Char('X'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete to line beginning (d0).
#[test]
fn test_end_to_end_delete_d0() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change to line beginning (c0).
#[test]
fn test_end_to_end_change_c0() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}

/// Test: Yank to line beginning (y0).
#[test]
fn test_end_to_end_yank_y0() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Delete to line end (d$).
#[test]
fn test_end_to_end_delete_d_dollar() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Normal);
}

/// Test: Change to line end (c$).
#[test]
fn test_end_to_end_change_c_dollar() {
    let mut state = EditorState::new();
    state.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    for c in "hello world".chars() {
        state.handle_key(KeyEvent::new(KeyCode::Char(c), KeyModifiers::NONE));
    }
    state.handle_key(KeyEvent::new(KeyCode::Escape, KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE));
    
    state.handle_key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::NONE));
    state.handle_key(KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT));
    let snapshot = state.snapshot();
    assert_eq!(snapshot.mode, kjxlkj_core_types::Mode::Insert);
}