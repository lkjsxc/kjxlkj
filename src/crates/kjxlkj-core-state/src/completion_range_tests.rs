//! Tests for command-line completion and pattern ranges.

use kjxlkj_core_types::{Key, KeyCode, Modifier};

use crate::editor::EditorState;

fn ed() -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("test.txt", "hello world\nfoo bar\nbaz qux\n");
    e
}

fn enter_key() -> Key {
    Key::new(KeyCode::Enter, Modifier::NONE)
}

fn tab_key() -> Key {
    Key::new(KeyCode::Tab, Modifier::NONE)
}

fn backtab_key() -> Key {
    Key::new(KeyCode::BackTab, Modifier::NONE)
}

// --- Completion tests ---

#[test]
fn tab_completes_command() {
    let mut e = ed();
    e.handle_key(Key::char(':'));
    e.handle_key(Key::char('w'));
    e.handle_key(tab_key());
    // Should complete to a command starting with 'w'
    assert!(e.cmdline.content.starts_with('w'));
    assert!(e.cmdline.content.len() > 1); // more than just 'w'
}

#[test]
fn tab_cycles_candidates() {
    let mut e = ed();
    e.handle_key(Key::char(':'));
    e.handle_key(Key::char('w'));
    e.handle_key(tab_key());
    let first = e.cmdline.content.clone();
    e.handle_key(tab_key());
    let second = e.cmdline.content.clone();
    // Should cycle to a different candidate
    assert_ne!(first, second);
}

#[test]
fn backtab_cycles_backward() {
    let mut e = ed();
    e.handle_key(Key::char(':'));
    e.handle_key(Key::char('w'));
    e.handle_key(tab_key());
    let first = e.cmdline.content.clone();
    e.handle_key(tab_key());
    e.handle_key(backtab_key());
    // Should be back to first
    assert_eq!(e.cmdline.content, first);
}

#[test]
fn typing_resets_completion() {
    let mut e = ed();
    e.handle_key(Key::char(':'));
    e.handle_key(Key::char('w'));
    e.handle_key(tab_key());
    assert!(!e.cmdline.completion.candidates.is_empty());
    e.handle_key(Key::char('x'));
    assert!(e.cmdline.completion.candidates.is_empty());
}

// --- Pattern range tests ---

#[test]
fn pattern_forward_range() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["hello", "foo", "bar", "baz"];
    let ctx = RangeContext {
        current_line: 0,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
    };
    let (range, rest) = parse_range_ctx("/bar/d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 2);
    assert_eq!(r.end, 2);
    assert_eq!(rest, "d");
}

#[test]
fn pattern_backward_range() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["hello", "foo", "bar", "baz"];
    let ctx = RangeContext {
        current_line: 3,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
    };
    let (range, _) = parse_range_ctx("?foo?d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 1);
}

#[test]
fn mark_range() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["hello", "foo", "bar"];
    let mark_fn = |ch: char| -> Option<usize> {
        if ch == 'a' {
            Some(1)
        } else {
            None
        }
    };
    let ctx = RangeContext {
        current_line: 0,
        total_lines: 3,
        lines: &lines,
        mark_line: Some(&mark_fn),
    };
    let (range, rest) = parse_range_ctx("'a,'ad", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 1);
    assert_eq!(r.end, 1);
    assert_eq!(rest, "d");
}

#[test]
fn pattern_comma_pattern() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["aaa", "bbb", "ccc", "ddd"];
    let ctx = RangeContext {
        current_line: 0,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
    };
    let (range, rest) = parse_range_ctx("/bbb/,/ddd/d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 1);
    assert_eq!(r.end, 3);
    assert_eq!(rest, "d");
}
