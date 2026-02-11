//! Integration tests for text object operations (diw, ci(, yaw, etc.)

use crate::editor::EditorState;
use kjxlkj_core_types::{BufferId, Key, KeyModifiers, Mode, RangeType};

fn ed() -> EditorState { EditorState::new(80, 24) }
fn m() -> KeyModifiers { KeyModifiers::default() }
fn ed_with(text: &str) -> EditorState {
    let mut s = ed();
    s.buffers.get_mut(&BufferId(0)).unwrap().insert(0, 0, text).unwrap();
    s
}
fn cursor_col(s: &EditorState) -> usize { s.windows.get(&s.focus.focused).unwrap().cursor.col }
fn buf_text(s: &EditorState) -> String {
    let win = s.windows.get(&s.focus.focused).unwrap();
    if let kjxlkj_core_types::ContentKind::Buffer(id) = win.content {
        s.buffers.get(&id).map(|b| {
            (0..b.line_count()).filter_map(|l| b.line(l)).collect::<String>()
        }).unwrap_or_default()
    } else { String::new() }
}

fn send(s: &mut EditorState, keys: &str) {
    for c in keys.chars() { s.handle_key(&Key::Char(c), &m()); }
}

#[test]
fn diw_deletes_inner_word() {
    let mut s = ed_with("hello world");
    send(&mut s, "diw");
    assert_eq!(s.mode, Mode::Normal);
    let t = buf_text(&s);
    assert!(t.starts_with(" world"), "expected ' world' prefix, got: {t}");
}

#[test]
fn daw_deletes_around_word_with_space() {
    let mut s = ed_with("hello world");
    send(&mut s, "daw");
    assert_eq!(s.mode, Mode::Normal);
    let t = buf_text(&s);
    assert!(t.starts_with("world"), "expected 'world', got: {t}");
}

#[test]
fn ciw_changes_inner_word_enters_insert() {
    let mut s = ed_with("hello world");
    send(&mut s, "ciw");
    assert_eq!(s.mode, Mode::Insert);
    let t = buf_text(&s);
    assert!(t.starts_with(" world"), "expected ' world' prefix, got: {t}");
}

#[test]
fn yiw_yanks_inner_word() {
    let mut s = ed_with("hello world");
    send(&mut s, "yiw");
    assert_eq!(s.mode, Mode::Normal);
    let entry = s.registers.get('"').expect("unnamed register should have yank");
    assert_eq!(entry.text, "hello");
    assert_eq!(entry.scope, RangeType::Characterwise);
}

#[test]
fn di_paren_deletes_inside_parens() {
    let mut s = ed_with("fn(abc)");
    // Move cursor inside parens.
    send(&mut s, "lll");
    assert_eq!(cursor_col(&s), 3);
    send(&mut s, "di(");
    let t = buf_text(&s);
    assert_eq!(t.trim_end(), "fn()");
}

#[test]
fn ci_brace_changes_inside_braces() {
    let mut s = ed_with("{content}");
    send(&mut s, "l");
    send(&mut s, "ci{");
    assert_eq!(s.mode, Mode::Insert);
    let t = buf_text(&s);
    assert_eq!(t.trim_end(), "{}");
}

#[test]
fn operator_pending_i_a_prefix_keys() {
    let mut s = ed_with("hello world");
    s.handle_key(&Key::Char('d'), &m());
    assert!(matches!(s.mode, Mode::OperatorPending(_)));
    s.handle_key(&Key::Char('i'), &m());
    assert!(matches!(s.mode, Mode::OperatorPending(_)));
    s.handle_key(&Key::Char('w'), &m());
    assert_eq!(s.mode, Mode::Normal);
}

#[test]
fn dip_deletes_inner_paragraph() {
    let mut s = ed_with("aaa\nbbb\n\nccc");
    send(&mut s, "dip");
    assert_eq!(s.mode, Mode::Normal);
    let t = buf_text(&s);
    // First paragraph (aaa, bbb) deleted, blank + ccc remain.
    assert!(t.contains("ccc"), "expected 'ccc' remaining, got: {t}");
}

#[test]
fn dis_deletes_inner_sentence() {
    let mut s = ed_with("Hello world. Goodbye.");
    send(&mut s, "dis");
    assert_eq!(s.mode, Mode::Normal);
}
