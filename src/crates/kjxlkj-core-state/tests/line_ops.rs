//! Tests for :t/:copy, :m/:move, :r/:read commands.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn dispatch_ex(state: &mut EditorState, cmd: &str) {
    dispatch_intent(state, Intent::ExCommand(cmd.into()));
}

fn line(state: &EditorState, idx: usize) -> String {
    state.active_buffer().unwrap().text.line_to_string(idx)
}

fn line_count(state: &EditorState) -> usize {
    state.active_buffer().unwrap().text.line_count()
}

// ── :t / :copy ─────────────────────────────────

#[test]
fn copy_current_line_below() {
    let mut s = setup("aaa\nbbb\nccc");
    // cursor on line 0, copy to after line 2
    dispatch_ex(&mut s, ":t 3");
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 3).trim(), "aaa");
}

#[test]
fn copy_range_to_destination() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_ex(&mut s, ":1,2t 4");
    assert_eq!(line_count(&s), 6);
    assert_eq!(line(&s, 4).trim(), "aaa");
    assert_eq!(line(&s, 5).trim(), "bbb");
}

#[test]
fn copy_single_line_with_address() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":2t 1");
    // Line 2 (0-indexed: 1 → "bbb") copied after line 1 (0-indexed: 0)
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 1).trim(), "bbb");
}

#[test]
fn copy_to_zero_inserts_at_top() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":3t 0");
    // Line 3 (0-indexed: 2 → "ccc") copied after line 0 (before line 1)
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 1).trim(), "ccc");
}

#[test]
fn copy_sets_message() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":1,3t 3");
    assert!(s.message.as_deref().unwrap().contains("3 lines copied"));
}

#[test]
fn copy_without_dest_shows_usage() {
    let mut s = setup("aaa\nbbb");
    dispatch_ex(&mut s, ":t");
    assert!(s.message.as_deref().unwrap().contains("Usage"));
}

// ── :m / :move ─────────────────────────────────

#[test]
fn move_current_line_down() {
    let mut s = setup("aaa\nbbb\nccc");
    // cursor on line 0, move to after line 2
    dispatch_ex(&mut s, ":m 3");
    assert_eq!(line_count(&s), 3);
    assert_eq!(line(&s, 0).trim(), "bbb");
    assert_eq!(line(&s, 1).trim(), "ccc");
    assert_eq!(line(&s, 2).trim(), "aaa");
}

#[test]
fn move_range_down() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_ex(&mut s, ":1,2m 4");
    // lines "aaa","bbb" moved after "ddd"
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 0).trim(), "ccc");
    assert_eq!(line(&s, 1).trim(), "ddd");
    assert_eq!(line(&s, 2).trim(), "aaa");
    assert_eq!(line(&s, 3).trim(), "bbb");
}

#[test]
fn move_range_up() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    dispatch_ex(&mut s, ":3,4m 1");
    // lines "ccc","ddd" moved after line 1 (0-indexed: 0 → "aaa")
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 0).trim(), "aaa");
    assert_eq!(line(&s, 1).trim(), "ccc");
    assert_eq!(line(&s, 2).trim(), "ddd");
    assert_eq!(line(&s, 3).trim(), "bbb");
}

#[test]
fn move_sets_modified() {
    let mut s = setup("aaa\nbbb");
    dispatch_ex(&mut s, ":m 2");
    assert!(s.active_buffer().unwrap().modified);
}

#[test]
fn move_sets_message() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":1,2m 3");
    assert!(s.message.as_deref().unwrap().contains("2 lines moved"));
}

#[test]
fn move_without_dest_shows_usage() {
    let mut s = setup("aaa\nbbb");
    dispatch_ex(&mut s, ":m");
    assert!(s.message.as_deref().unwrap().contains("Usage"));
}

// ── :r / :read ─────────────────────────────────

#[test]
fn read_nonexistent_file() {
    let mut s = setup("aaa");
    dispatch_ex(&mut s, ":r /nonexistent/path/xyz");
    assert!(s.message.as_deref().unwrap().contains("Can't open"));
}

#[test]
fn read_without_filename() {
    let mut s = setup("aaa");
    dispatch_ex(&mut s, ":r");
    assert!(s.message.as_deref().unwrap().contains("Usage"));
}

#[test]
fn read_file_inserts_content() {
    use std::io::Write;
    let dir = std::env::temp_dir().join("kjxlkj_test_read");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("read_test.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    write!(f, "inserted_line_1\ninserted_line_2\n").unwrap();
    drop(f);

    let mut s = setup("aaa\nbbb");
    dispatch_ex(&mut s, &format!(":r {}", path.to_string_lossy()));
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 1).trim(), "inserted_line_1");
    assert_eq!(line(&s, 2).trim(), "inserted_line_2");
    assert!(s.message.as_deref().unwrap().contains("2 line"));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn read_file_sets_modified() {
    use std::io::Write;
    let dir = std::env::temp_dir().join("kjxlkj_test_read_mod");
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("read_mod.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    write!(f, "hello\n").unwrap();
    drop(f);

    let mut s = setup("aaa");
    dispatch_ex(&mut s, &format!(":r {}", path.to_string_lossy()));
    assert!(s.active_buffer().unwrap().modified);

    let _ = std::fs::remove_dir_all(&dir);
}

// ── :copy alias ────────────────────────────────

#[test]
fn copy_alias_works() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":copy 3");
    assert_eq!(line_count(&s), 4);
    assert_eq!(line(&s, 3).trim(), "aaa");
}

// ── :move alias ────────────────────────────────

#[test]
fn move_alias_works() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_ex(&mut s, ":move 3");
    assert_eq!(line_count(&s), 3);
    assert_eq!(line(&s, 2).trim(), "aaa");
}
