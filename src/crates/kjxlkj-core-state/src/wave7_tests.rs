//! Wave 7 tests: text objects, format, mark motions,
//! numbered register rotation, macro append,
//! last-search shorthand in ranges.

use kjxlkj_core_types::{Key, Mode};

use crate::editor::EditorState;

fn ed() -> EditorState {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "hello world\nfoo bar\nbaz qux\n");
    e
}

fn buf_text(e: &EditorState) -> String {
    let id = e.current_buffer_id();
    e.buffers.get(id).unwrap().content.to_string()
}

// --- Text object tests ---

#[test]
fn diw_deletes_inner_word() {
    let mut e = ed();
    // cursor on 'h': diw should delete "hello"
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('w'));
    assert!(matches!(e.mode, Mode::Normal));
    let text = buf_text(&e);
    assert!(text.starts_with(" world"));
}

#[test]
fn daw_deletes_around_word() {
    let mut e = ed();
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('a'));
    e.handle_key(Key::char('w'));
    assert!(matches!(e.mode, Mode::Normal));
    let text = buf_text(&e);
    // Should delete "hello " (word + trailing space)
    assert!(text.starts_with("world"));
}

#[test]
fn dip_deletes_inner_paragraph() {
    let mut e = ed();
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('p'));
    assert!(matches!(e.mode, Mode::Normal));
}

#[test]
fn ciw_changes_inner_word() {
    let mut e = ed();
    e.handle_key(Key::char('c'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('w'));
    assert!(matches!(e.mode, Mode::Insert));
}

// --- Delimiter text objects ---

#[test]
fn di_paren_deletes_inner_parens() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "foo(bar baz)end\n");
    // Move cursor inside parens.
    for _ in 0..4 {
        e.handle_key(Key::char('l'));
    }
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('i'));
    e.handle_key(Key::char('('));
    let text = buf_text(&e);
    assert!(text.contains("()"), "got: {}", text);
}

// --- gq format ---

#[test]
fn gqq_formats_current_line() {
    let mut e = EditorState::new(80, 24);
    e.open_file("t.txt", "hello\n");
    e.handle_key(Key::char('g'));
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('q'));
    assert!(matches!(e.mode, Mode::Normal));
}

// --- Numbered register rotation ---

#[test]
fn delete_rotates_numbered_registers() {
    let mut e = ed();
    // dd first line → "hello world\n" into reg 1
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('d'));
    // dd second line → "foo bar\n" into reg 1, old goes to 2
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('d'));
    use kjxlkj_core_edit::RegisterName;
    let r1 = e.registers.get(RegisterName::Numbered(1));
    assert!(r1.is_some(), "register 1 should have content");
    let r2 = e.registers.get(RegisterName::Numbered(2));
    assert!(r2.is_some(), "register 2 should have content");
}

// --- Macro append ---

#[test]
fn q_uppercase_appends_macro() {
    let mut e = ed();
    // Record macro @a: just 'j'
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('a'));
    e.handle_key(Key::char('j'));
    e.handle_key(Key::char('q'));
    assert!(e.macro_store.contains_key(&'a'));
    let len_before = e.macro_store[&'a'].len();
    // Append to macro @a with qA: 'k'
    e.handle_key(Key::char('q'));
    e.handle_key(Key::char('A'));
    e.handle_key(Key::char('k'));
    e.handle_key(Key::char('q'));
    let len_after = e.macro_store[&'a'].len();
    assert_eq!(len_after, len_before + 1);
}

// --- Last-search shorthand ---

#[test]
fn backslash_slash_uses_last_search() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["hello", "foo", "bar", "baz"];
    let ctx = RangeContext {
        current_line: 0,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
        last_search: Some("bar"),
        vars: None, call_fn: None,
    };
    let (range, rest) = parse_range_ctx("\\/d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 2);
    assert_eq!(rest, "d");
}

#[test]
fn backslash_question_uses_last_search() {
    use crate::ex_parse_ranges::{parse_range_ctx, RangeContext};
    let lines = vec!["hello", "foo", "bar", "baz"];
    let ctx = RangeContext {
        current_line: 3,
        total_lines: 4,
        lines: &lines,
        mark_line: None,
        last_search: Some("foo"),
        vars: None, call_fn: None,
    };
    let (range, _) = parse_range_ctx("\\?d", &ctx);
    let r = range.unwrap();
    assert_eq!(r.start, 1);
}

// --- Mark motions in op-pending ---

#[test]
fn d_backtick_mark_deletes_to_mark() {
    let mut e = ed();
    // Set mark 'a' at position (0,0)
    e.handle_key(Key::char('m'));
    e.handle_key(Key::char('a'));
    // Move down
    e.handle_key(Key::char('j'));
    // d`a should delete from current pos to mark
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('`'));
    e.handle_key(Key::char('a'));
    assert!(matches!(e.mode, Mode::Normal));
}

// --- f/t in op-pending ---

#[test]
fn df_deletes_to_char() {
    let mut e = ed();
    // df<space> on "hello world" should delete "hello "
    e.handle_key(Key::char('d'));
    e.handle_key(Key::char('f'));
    e.handle_key(Key::char(' '));
    let text = buf_text(&e);
    assert!(text.starts_with("world"), "got: {}", text);
}
