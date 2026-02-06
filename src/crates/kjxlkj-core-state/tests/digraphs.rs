//! Tests for digraph insertion (Ctrl-K c1 c2).

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Mode, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s.mode.transition(Mode::Insert);
    s
}

#[test]
fn digraph_accented_a() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('a', '\''));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "á");
}

#[test]
fn digraph_euro_sign() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('E', 'u'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "€");
}

#[test]
fn digraph_infinity() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('0', '0'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "∞");
}

#[test]
fn digraph_arrow_right() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('-', '>'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "→");
}

#[test]
fn digraph_greek_alpha() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('a', '*'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "α");
}

#[test]
fn digraph_copyright() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('c', 'O'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "©");
}

#[test]
fn digraph_fraction_half() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('1', '2'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "½");
}

#[test]
fn digraph_reverse_order() {
    let mut s = setup("");
    // Digraph for arrow is (-, >) but reverse should also work
    dispatch_intent(&mut s, Intent::InsertDigraph('>', '-'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "→");
}

#[test]
fn digraph_unknown_shows_error() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('z', 'z'));
    // Unknown digraph should not insert anything
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "");
    assert!(s.message.is_some());
    assert!(s.message.unwrap().contains("Unknown digraph"));
}

#[test]
fn digraph_not_equal() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('!', '='));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "≠");
}

#[test]
fn digraph_eszett() {
    let mut s = setup("");
    dispatch_intent(&mut s, Intent::InsertDigraph('s', 's'));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "ß");
}

#[test]
fn digraph_inserts_at_cursor() {
    let mut s = setup("hello");
    s.mode.transition(Mode::Insert);
    // Move cursor to col 3
    if let Some(wid) = s.active_window {
        if let Some(win) = s.windows.get_mut(&wid) { win.cursor_col = 3; }
    }
    dispatch_intent(&mut s, Intent::InsertDigraph('a', '\''));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "helálo");
}
