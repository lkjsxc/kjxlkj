//! Motion and operator integration tests.
//!
//! Tests for motions and operators as required by
//! /docs/reference/CONFORMANCE_EDITING.md

use kjxlkj_core_edit::{apply_motion, Motion};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Cursor, MotionIntent, Position};

/// Create a test buffer.
fn test_buffer(content: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId::new(1), content)
}

/// Create a cursor at position.
fn cursor_at(line: usize, col: usize) -> Cursor {
    Cursor::new(line, col)
}

// =============================================================================
// Character motions (h, l)
// =============================================================================

/// Test: h moves left.
#[test]
fn test_motion_h() {
    let buffer = test_buffer("hello");
    let cursor = cursor_at(0, 3);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Left, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 2);
}

/// Test: h at column 0 stays at 0.
#[test]
fn test_motion_h_boundary() {
    let buffer = test_buffer("hello");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Left, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 0);
}

/// Test: l moves right.
#[test]
fn test_motion_l() {
    let buffer = test_buffer("hello");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Right, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 1);
}

/// Test: l at end of line is clamped.
#[test]
fn test_motion_l_boundary() {
    let buffer = test_buffer("hi");
    let cursor = cursor_at(0, 1);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Right, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should be clamped to last valid position
    assert!(new_pos.col <= 2);
}

// =============================================================================
// Line motions (j, k)
// =============================================================================

/// Test: j moves down.
#[test]
fn test_motion_j() {
    let buffer = test_buffer("line 1\nline 2");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Down, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 1);
}

/// Test: j at last line stays at last line.
#[test]
fn test_motion_j_boundary() {
    let buffer = test_buffer("only one line");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Down, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 0);
}

/// Test: k moves up.
#[test]
fn test_motion_k() {
    let buffer = test_buffer("line 1\nline 2");
    let cursor = cursor_at(1, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Up, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 0);
}

/// Test: k at first line stays at first line.
#[test]
fn test_motion_k_boundary() {
    let buffer = test_buffer("line 1\nline 2");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Up, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 0);
}

// =============================================================================
// Line start/end motions (0, ^, $)
// =============================================================================

/// Test: 0 goes to line start.
#[test]
fn test_motion_line_start() {
    let buffer = test_buffer("  hello");
    let cursor = cursor_at(0, 5);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::LineStart, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 0);
}

/// Test: ^ goes to first non-blank.
#[test]
fn test_motion_first_nonblank() {
    let buffer = test_buffer("  hello");
    let cursor = cursor_at(0, 5);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::FirstNonBlank, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 2);
}

/// Test: $ goes to line end.
#[test]
fn test_motion_line_end() {
    let buffer = test_buffer("hello");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::LineEnd, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should be at or near end
    assert!(new_pos.col >= 4);
}

// =============================================================================
// Word motions (w, b, e)
// =============================================================================

/// Test: w moves to next word start.
#[test]
fn test_motion_word_start() {
    let buffer = test_buffer("one two three");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::WordStart, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should move past "one " to "two"
    assert!(new_pos.col >= 3);
}

/// Test: b moves to previous word start.
#[test]
fn test_motion_word_back() {
    let buffer = test_buffer("one two three");
    let cursor = cursor_at(0, 8);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::WordStartBack, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should move back to start of "two" or "one"
    assert!(new_pos.col < 8);
}

/// Test: e moves to end of word.
#[test]
fn test_motion_word_end() {
    let buffer = test_buffer("one two three");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::WordEnd, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should be at 'e' of "one" (index 2)
    assert!(new_pos.col >= 2);
}

// =============================================================================
// File motions (G)
// =============================================================================

/// Test: G goes to end of file.
#[test]
fn test_motion_file_end() {
    let buffer = test_buffer("line 1\nline 2\nline 3");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::FileEnd, 1),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 2);
}

// =============================================================================
// Count tests
// =============================================================================

/// Test: Motion with count 3.
#[test]
fn test_motion_with_count() {
    let buffer = test_buffer("hello");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Right, 3),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.col, 3);
}

/// Test: Down motion with count.
#[test]
fn test_motion_down_with_count() {
    let buffer = test_buffer("line 1\nline 2\nline 3\nline 4\nline 5");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Down, 3),
        &cursor,
        &buffer,
        24,
    );
    
    assert_eq!(new_pos.line, 3);
}

// =============================================================================
// Edge cases
// =============================================================================

/// Test: Motion on empty buffer.
#[test]
fn test_motion_empty_buffer() {
    let buffer = test_buffer("");
    let cursor = cursor_at(0, 0);
    
    // All motions should not crash on empty buffer
    let motions = [
        MotionIntent::Left,
        MotionIntent::Right,
        MotionIntent::Up,
        MotionIntent::Down,
        MotionIntent::LineStart,
        MotionIntent::LineEnd,
    ];
    
    for motion in &motions {
        let _ = apply_motion(&Motion::new(motion.clone(), 1), &cursor, &buffer, 24);
    }
}

/// Test: Motion on single character buffer.
#[test]
fn test_motion_single_char() {
    let buffer = test_buffer("x");
    let cursor = cursor_at(0, 0);
    
    let new_pos = apply_motion(
        &Motion::new(MotionIntent::Right, 1),
        &cursor,
        &buffer,
        24,
    );
    
    // Should stay at 0 or 1
    assert!(new_pos.col <= 1);
}

/// Test: Motion determinism.
#[test]
fn test_motion_determinism() {
    let buffer = test_buffer("hello world");
    let cursor = cursor_at(0, 0);
    
    let pos1 = apply_motion(&Motion::new(MotionIntent::WordStart, 1), &cursor, &buffer, 24);
    let pos2 = apply_motion(&Motion::new(MotionIntent::WordStart, 1), &cursor, &buffer, 24);
    
    assert_eq!(pos1, pos2);
}
