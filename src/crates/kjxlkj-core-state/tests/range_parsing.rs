//! Tests for command range/address parsing.

use kjxlkj_core_state::EditorState;
use kjxlkj_core_types::Size;

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// These tests exercise range parsing indirectly through the Ex command dispatch,
// since parse_range and LineRange are pub(crate). We verify the effects.

#[test]
fn range_percent_deletes_all() {
    let mut s = setup("aaa\nbbb\nccc");
    kjxlkj_core_state::dispatch_intent(&mut s, kjxlkj_core_types::Intent::ExCommand(":%d".into()));
    assert_eq!(s.active_buffer().unwrap().text.line_count(), 1);
}

#[test]
fn range_line_numbers_substitute() {
    let mut s = setup("aaa\nbbb\naaa\naaa");
    kjxlkj_core_state::dispatch_intent(&mut s, kjxlkj_core_types::Intent::ExCommand(":2,3s/aaa/xxx".into()));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "aaa");
    assert_eq!(buf.text.line_to_string(2).trim(), "xxx");
    assert_eq!(buf.text.line_to_string(3).trim(), "aaa");
}

#[test]
fn range_dot_dollar() {
    let mut s = setup("aaa\nbbb\nccc");
    // Cursor on line 0, :.,$d should delete all
    kjxlkj_core_state::dispatch_intent(&mut s, kjxlkj_core_types::Intent::ExCommand(":.,$d".into()));
    assert_eq!(s.active_buffer().unwrap().text.line_count(), 1);
}

#[test]
fn range_single_address() {
    let mut s = setup("aaa\nbbb\nccc");
    kjxlkj_core_state::dispatch_intent(&mut s, kjxlkj_core_types::Intent::ExCommand(":2d".into()));
    assert_eq!(s.active_buffer().unwrap().text.line_count(), 2);
    assert_eq!(s.active_buffer().unwrap().text.line_to_string(1).trim(), "ccc");
}

#[test]
fn range_goto_line() {
    let mut s = setup("aaa\nbbb\nccc\nddd");
    kjxlkj_core_state::dispatch_intent(&mut s, kjxlkj_core_types::Intent::ExCommand(":3".into()));
    let wid = s.active_window.unwrap();
    assert_eq!(s.windows.get(&wid).unwrap().cursor_line, 2);
}
