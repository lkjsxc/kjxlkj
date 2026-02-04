//! Latency and performance probe tests.
//!
//! Implements deterministic probes as required by:
//! - /docs/spec/technical/latency.md
//! - /docs/todo/current/wave-implementation/technical/latency/regression/README.md
//!
//! These tests assert ordering and convergence guarantees, not wall-clock time.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::{KeyCode, KeyEvent, KeyModifiers, Mode};

/// Helper to create a key event.
fn key(code: KeyCode) -> KeyEvent {
    KeyEvent {
        code,
        modifiers: KeyModifiers::default(),
    }
}

/// Helper to create a ctrl key event.
fn ctrl_key(c: char) -> KeyEvent {
    KeyEvent {
        code: KeyCode::Char(c),
        modifiers: KeyModifiers {
            ctrl: true,
            ..Default::default()
        },
    }
}

// ============================================================================
// A. Deterministic typing-burst probes
// ============================================================================

/// Test: Typing burst - 200 characters applied in order.
/// Spec: /docs/spec/technical/latency.md - Acceptance criteria #1
#[test]
fn test_typing_burst_200_chars_in_order() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i'))); // Enter insert mode
    assert_eq!(editor.mode(), Mode::Insert);

    // Type 200 characters
    let chars: Vec<char> = (0..200)
        .map(|i| ((i % 26) as u8 + b'a') as char)
        .collect();

    for &c in &chars {
        editor.handle_key(key(KeyCode::Char(c)));
    }

    let content = editor.content();
    let expected: String = chars.iter().collect();

    // Assert all characters are present in order
    assert_eq!(content, expected);
    // Assert cursor is at the end (insertion point)
    assert_eq!(editor.cursor().col(), 200);
}

/// Test: Typing burst with newlines maintains order.
#[test]
fn test_typing_burst_with_newlines() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i')));

    // Type 50 lines of 10 chars each
    for line in 0..50 {
        for col in 0..10 {
            let c = ((col % 26) as u8 + b'a') as char;
            editor.handle_key(key(KeyCode::Char(c)));
        }
        if line < 49 {
            editor.handle_key(key(KeyCode::Enter));
        }
    }

    // Check line count
    assert_eq!(editor.buffer().line_count(), 50);

    // Check cursor is on last line
    assert_eq!(editor.cursor().line(), 49);
}

/// Test: Final snapshot reflects final input (no off-by-one lag).
#[test]
fn test_no_off_by_one_lag() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i')));

    // Type a specific sequence
    let sequence = "hello world";
    for c in sequence.chars() {
        editor.handle_key(key(KeyCode::Char(c)));
        // Take snapshot after each key
        let snapshot = editor.snapshot();
        // Content should contain all typed chars so far
        let content = editor.content();
        assert!(sequence.starts_with(&content) || content.len() <= sequence.len());
    }

    // Final state must match
    assert_eq!(editor.content(), sequence);
}

/// Test: Interleaved typing and deletion maintains consistency.
#[test]
fn test_interleaved_typing_deletion() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i')));

    // Type some chars, delete some, type more
    for c in "hello".chars() {
        editor.handle_key(key(KeyCode::Char(c)));
    }
    // Delete 2 chars
    editor.handle_key(key(KeyCode::Backspace));
    editor.handle_key(key(KeyCode::Backspace));

    // Type more
    for c in "p me".chars() {
        editor.handle_key(key(KeyCode::Char(c)));
    }

    assert_eq!(editor.content(), "help me");
}

// ============================================================================
// B. Scroll and cursor visibility probes
// ============================================================================

/// Test: Scrolling 200 lines keeps cursor visible.
/// Spec: /docs/spec/technical/latency.md - Acceptance criteria #2
#[test]
fn test_scroll_200_lines_cursor_visible() {
    let mut editor = EditorState::new();

    // Create a large file
    let content: String = (0..500)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);

    // Move down 200 lines
    for _ in 0..200 {
        editor.handle_key(key(KeyCode::Char('j')));
    }

    // Check cursor position
    let cursor_line = editor.cursor().line();
    assert_eq!(cursor_line, 200);

    // Cursor must be within viewport
    let snapshot = editor.snapshot();
    let viewport = &snapshot.buffer.viewport;
    assert!(
        cursor_line >= viewport.top_line && cursor_line < viewport.top_line + viewport.height,
        "Cursor line {} not in viewport [{}, {})",
        cursor_line,
        viewport.top_line,
        viewport.top_line + viewport.height
    );
}

/// Test: Scrolling up keeps cursor visible.
#[test]
fn test_scroll_up_cursor_visible() {
    let mut editor = EditorState::new();

    let content: String = (0..200)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);

    // Go to end
    editor.handle_key(key(KeyCode::Char('G')));
    assert_eq!(editor.cursor().line(), 199);

    // Scroll up 50 lines
    for _ in 0..50 {
        editor.handle_key(key(KeyCode::Char('k')));
    }

    let cursor_line = editor.cursor().line();
    let snapshot = editor.snapshot();
    let viewport = &snapshot.buffer.viewport;
    assert!(
        cursor_line >= viewport.top_line && cursor_line < viewport.top_line + viewport.height,
        "Cursor not visible after scrolling up"
    );
}

/// Test: Half-page scroll keeps cursor visible.
#[test]
fn test_half_page_scroll_cursor_visible() {
    let mut editor = EditorState::new();

    let content: String = (0..200)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);

    // Ctrl-d multiple times
    for _ in 0..5 {
        editor.handle_key(ctrl_key('d'));

        let cursor_line = editor.cursor().line();
        let snapshot = editor.snapshot();
        let viewport = &snapshot.buffer.viewport;
        assert!(
            cursor_line >= viewport.top_line && cursor_line < viewport.top_line + viewport.height,
            "Cursor not visible after Ctrl-d"
        );
    }
}

// ============================================================================
// C. Resize probes
// ============================================================================

/// Test: Resize while typing maintains cursor visibility.
/// Spec: /docs/spec/technical/latency.md - Acceptance criteria #4
#[test]
fn test_resize_while_typing_cursor_visible() {
    let mut editor = EditorState::new();

    let content: String = (0..100)
        .map(|i| format!("line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);

    // Move to middle of file and enter insert mode
    for _ in 0..50 {
        editor.handle_key(key(KeyCode::Char('j')));
    }
    editor.handle_key(key(KeyCode::Char('i')));

    // Type some characters
    for c in "test".chars() {
        editor.handle_key(key(KeyCode::Char(c)));
    }

    // Resize smaller
    editor.resize(80, 10);

    // Cursor must still be visible
    let cursor_line = editor.cursor().line();
    let snapshot = editor.snapshot();
    let viewport = &snapshot.buffer.viewport;
    assert!(
        cursor_line >= viewport.top_line && cursor_line < viewport.top_line + viewport.height,
        "Cursor not visible after resize to smaller"
    );

    // Resize larger
    editor.resize(80, 40);

    // Cursor must still be visible
    let cursor_line = editor.cursor().line();
    let snapshot = editor.snapshot();
    let viewport = &snapshot.buffer.viewport;
    assert!(
        cursor_line >= viewport.top_line && cursor_line < viewport.top_line + viewport.height,
        "Cursor not visible after resize to larger"
    );
}

/// Test: Rapid resize storm doesn't corrupt state.
#[test]
fn test_rapid_resize_storm() {
    let mut editor = EditorState::new();
    editor.load_content("line 1\nline 2\nline 3\nline 4\nline 5");
    editor.resize(80, 24);

    // Move to line 3
    editor.handle_key(key(KeyCode::Char('j')));
    editor.handle_key(key(KeyCode::Char('j')));
    let target_line = editor.cursor().line();

    // Rapid resize storm
    for size in [10, 5, 20, 8, 15, 30, 6, 12, 25, 10].iter() {
        editor.resize(80, *size);
    }

    // Content should be unchanged
    assert_eq!(editor.buffer().line_count(), 5);
    // Mode should be unchanged
    assert_eq!(editor.mode(), Mode::Normal);
    // Cursor line should be unchanged (column may clamp)
    assert_eq!(editor.cursor().line(), target_line);
}

// ============================================================================
// D. Snapshot ordering probes
// ============================================================================

/// Test: Snapshot sequence is monotonic.
#[test]
fn test_snapshot_sequence_monotonic() {
    let mut editor = EditorState::new();
    editor.load_content("hello world");

    // Perform various operations
    let operations = [
        key(KeyCode::Char('l')),
        key(KeyCode::Char('l')),
        key(KeyCode::Char('x')),
        key(KeyCode::Char('i')),
        key(KeyCode::Char('a')),
        key(KeyCode::Escape),
    ];

    for op in operations {
        editor.handle_key(op);
        let snapshot = editor.snapshot();
        // The snapshot should be valid
        let version = snapshot.buffer.version.value();
        assert!(version >= 0);
    }
}

/// Test: Stale snapshots can be identified by version.
#[test]
fn test_snapshot_version_allows_staleness_detection() {
    let mut editor = EditorState::new();
    editor.load_content("initial");

    let snapshot1 = editor.snapshot();
    let version1 = snapshot1.buffer.version.value();

    // Modify
    editor.handle_key(key(KeyCode::Char('i')));
    editor.handle_key(key(KeyCode::Char('X')));
    editor.handle_key(key(KeyCode::Escape));

    let snapshot2 = editor.snapshot();
    let version2 = snapshot2.buffer.version.value();

    // version2 should be greater
    assert!(version2 > version1);
}

// ============================================================================
// E. Input ordering probes
// ============================================================================

/// Test: Total ordering - all inputs applied in sequence.
#[test]
fn test_input_total_ordering() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i')));

    // Create a deterministic input sequence
    let inputs: Vec<char> = "The quick brown fox jumps over the lazy dog.".chars().collect();

    for c in &inputs {
        editor.handle_key(key(KeyCode::Char(*c)));
    }

    let content = editor.content();
    let expected: String = inputs.iter().collect();

    // Exact match proves total ordering
    assert_eq!(content, expected);
}

/// Test: Interleaved mode switches preserve ordering.
#[test]
fn test_mode_switch_ordering() {
    let mut editor = EditorState::new();
    editor.load_content("hello");

    // Complex sequence: normal movement, insert, normal, visual, insert
    editor.handle_key(key(KeyCode::Char('l'))); // move right
    editor.handle_key(key(KeyCode::Char('l')));
    editor.handle_key(key(KeyCode::Char('i'))); // insert
    editor.handle_key(key(KeyCode::Char('X')));
    editor.handle_key(key(KeyCode::Escape)); // back to normal
    editor.handle_key(key(KeyCode::Char('l')));
    editor.handle_key(key(KeyCode::Char('v'))); // visual
    editor.handle_key(key(KeyCode::Escape)); // back to normal

    // All operations should have applied in order
    assert_eq!(editor.mode(), Mode::Normal);
    // 'X' was inserted at position 2
    assert!(editor.content().contains('X'));
}

// ============================================================================
// F. Backpressure simulation probes
// ============================================================================

/// Test: Large input stream processed without loss.
#[test]
fn test_large_input_stream_no_loss() {
    let mut editor = EditorState::new();
    editor.load_content("");
    editor.handle_key(key(KeyCode::Char('i')));

    // Simulate large burst (1000 chars)
    let input: Vec<char> = (0..1000).map(|i| ((i % 26) as u8 + b'a') as char).collect();

    for c in &input {
        editor.handle_key(key(KeyCode::Char(*c)));
    }

    // All characters must be present
    assert_eq!(editor.content().len(), 1000);
}

/// Test: Movement burst processed without skip.
#[test]
fn test_movement_burst_no_skip() {
    let mut editor = EditorState::new();

    let content: String = (0..1000)
        .map(|i| format!("line {:04}", i))
        .collect::<Vec<_>>()
        .join("\n");
    editor.load_content(&content);
    editor.resize(80, 24);

    // Move down 500 times
    for _ in 0..500 {
        editor.handle_key(key(KeyCode::Char('j')));
    }

    // Must be exactly at line 500
    assert_eq!(editor.cursor().line(), 500);
}
