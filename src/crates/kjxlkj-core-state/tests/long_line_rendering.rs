//! Long-line rendering protection tests.
//! Ensures viewport-bounded snapshot creation and line slicing work
//! correctly with extremely long lines.

use kjxlkj_core_text::{TextBuffer, BufferSnapshot};
use kjxlkj_core_types::Position;

fn make_long_line(len: usize) -> String {
    "x".repeat(len)
}

fn make_long_line_buffer(line_len: usize, line_count: usize) -> TextBuffer {
    let line = make_long_line(line_len);
    let text: String = (0..line_count).map(|_| line.as_str()).collect::<Vec<_>>().join("\n");
    TextBuffer::from_text(&text)
}

// ── Snapshot does not materialize the full buffer ─────────────────────

#[test]
fn snapshot_only_viewport_lines() {
    let buf = make_long_line_buffer(10_000, 1000);
    let snap = BufferSnapshot::from_buffer(&buf, 500, 24, Position::new(500, 0));
    assert_eq!(snap.line_count(), 24);
    // Each line is 10k chars but we only have 24 lines
    assert_eq!(snap.total_lines, 1000);
}

#[test]
fn snapshot_line_length_preserved() {
    let buf = make_long_line_buffer(50_000, 10);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 10, Position::new(0, 0));
    // The snapshot does store the full line content (but only viewport lines)
    assert_eq!(snap.line(0).unwrap().len(), 50_000);
}

// ── Line slicing bounds horizontal viewport ──────────────────────────

#[test]
fn line_slice_start_of_long_line() {
    let buf = make_long_line_buffer(10_000, 5);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 5, Position::new(0, 0));
    let slice = snap.line_slice(0, 0, 80).unwrap();
    assert_eq!(slice.len(), 80);
    assert!(slice.chars().all(|c| c == 'x'));
}

#[test]
fn line_slice_middle_of_long_line() {
    let buf = make_long_line_buffer(10_000, 5);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 5, Position::new(0, 0));
    let slice = snap.line_slice(0, 5000, 80).unwrap();
    assert_eq!(slice.len(), 80);
}

#[test]
fn line_slice_at_end_of_long_line() {
    let buf = make_long_line_buffer(100, 3);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 3, Position::new(0, 0));
    let slice = snap.line_slice(0, 90, 80).unwrap();
    // Only 10 chars remaining (100 - 90)
    assert_eq!(slice.len(), 10);
}

#[test]
fn line_slice_beyond_line_returns_empty() {
    let buf = make_long_line_buffer(50, 3);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 3, Position::new(0, 0));
    let slice = snap.line_slice(0, 100, 80).unwrap();
    assert!(slice.is_empty());
}

#[test]
fn line_slice_invalid_offset_returns_none() {
    let buf = make_long_line_buffer(50, 3);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 3, Position::new(0, 0));
    assert!(snap.line_slice(99, 0, 80).is_none());
}

// ── Unicode long lines ───────────────────────────────────────────────

#[test]
fn long_unicode_line_slicing() {
    // 5000 multi-byte characters
    let line: String = std::iter::repeat('日').take(5000).collect();
    let buf = TextBuffer::from_text(&line);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 1, Position::new(0, 0));
    let slice = snap.line_slice(0, 100, 80).unwrap();
    assert_eq!(slice.chars().count(), 80);
    assert!(slice.chars().all(|c| c == '日'));
}

// ── Multiple long lines don't explode memory ─────────────────────────

#[test]
fn viewport_of_many_long_lines() {
    // 100 lines of 100k chars each — but viewport is only 24 lines
    let buf = make_long_line_buffer(100_000, 100);
    let snap = BufferSnapshot::from_buffer(&buf, 50, 24, Position::new(50, 0));
    assert_eq!(snap.line_count(), 24);
    // Slice each visible line to 80 cols
    for i in 0..24 {
        let slice = snap.line_slice(i, 0, 80).unwrap();
        assert_eq!(slice.len(), 80);
    }
}

// ── Mixed line lengths ───────────────────────────────────────────────

#[test]
fn mixed_short_and_long_lines() {
    let mut text = String::new();
    for i in 0..50 {
        if i % 5 == 0 {
            text.push_str(&make_long_line(10_000));
        } else {
            text.push_str(&format!("short line {}", i));
        }
        text.push('\n');
    }
    let buf = TextBuffer::from_text(&text);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 24, Position::new(0, 0));
    // Line 0 is long, line 1 is short
    let slice0 = snap.line_slice(0, 0, 80).unwrap();
    assert_eq!(slice0.len(), 80);
    let slice1 = snap.line_slice(1, 0, 80).unwrap();
    assert!(slice1.len() < 80); // short line is less than 80 chars
}

// ── Wrap toggling with long lines ────────────────────────────────────

#[test]
fn nowrap_slice_deterministic() {
    let buf = make_long_line_buffer(500, 10);
    let snap = BufferSnapshot::from_buffer(&buf, 0, 10, Position::new(0, 0));
    // Same parameters should produce identical slices
    let s1 = snap.line_slice(0, 100, 80).unwrap();
    let s2 = snap.line_slice(0, 100, 80).unwrap();
    assert_eq!(s1, s2);
}

// ── Stress: very large line count + long lines ───────────────────────

#[test]
fn stress_large_buffer_long_lines() {
    // 5000 lines of 1000 chars each
    let buf = make_long_line_buffer(1000, 5000);
    let snap = BufferSnapshot::from_buffer(&buf, 2500, 24, Position::new(2500, 0));
    assert_eq!(snap.line_count(), 24);
    assert_eq!(snap.first_line, 2500);
    for i in 0..24 {
        let slice = snap.line_slice(i, 0, 80).unwrap();
        assert_eq!(slice.len(), 80);
    }
}
