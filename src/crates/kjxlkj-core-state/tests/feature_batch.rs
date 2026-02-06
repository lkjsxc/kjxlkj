//! Tests for features added in batch: filter, Ctrl-V, gR, :saveas, :enew,
//! :syntax, :highlight, :digraphs commands.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, KeyEvent, Mode, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ── Filter through external command ──

#[test]
fn filter_sort_lines() {
    let mut s = setup("cherry\napple\nbanana");
    dispatch_intent(&mut s, Intent::ExCommand(":%!sort".into()));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "apple");
    assert_eq!(buf.text.line_to_string(1).trim(), "banana");
    assert_eq!(buf.text.line_to_string(2).trim(), "cherry");
}

#[test]
fn filter_rev_lines() {
    let mut s = setup("aaa\nbbb\nccc");
    dispatch_intent(&mut s, Intent::ExCommand(":%!rev".into()));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "aaa");
    assert_eq!(buf.text.line_to_string(1).trim(), "bbb");
    assert_eq!(buf.text.line_to_string(2).trim(), "ccc");
}

#[test]
fn filter_cat() {
    let mut s = setup("hello\nworld");
    dispatch_intent(&mut s, Intent::ExCommand(":%!cat".into()));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.line_to_string(0).trim(), "hello");
    assert_eq!(buf.text.line_to_string(1).trim(), "world");
}

// ── Ctrl-V literal insert ──

#[test]
fn ctrl_v_literal_insert() {
    let mut s = setup("");
    s.mode.transition(Mode::Insert);
    // Ctrl-V sets pending, then next char is inserted literally
    let key_cv = KeyEvent::ctrl('v');
    let intent1 = s.parser.parse_insert(&key_cv);
    assert_eq!(intent1, Intent::Noop); // pending state set
    let key_tab = KeyEvent::char('\t');
    let intent2 = s.parser.parse_insert(&key_tab);
    assert_eq!(intent2, Intent::InsertChar('\t'));
}

// ── gR enters replace mode ──

#[test]
fn gr_enters_replace_mode() {
    let mut s = setup("hello");
    let key_g = KeyEvent::char('g');
    let intent1 = s.parser.parse_normal(&key_g);
    assert_eq!(intent1, Intent::Noop); // g sets pending
    let key_r = KeyEvent::char('R');
    let intent2 = s.parser.parse_normal(&key_r);
    assert_eq!(intent2, Intent::EnterMode(Mode::Replace));
}

// ── :enew creates new buffer ──

#[test]
fn enew_creates_empty_buffer() {
    let mut s = setup("old content");
    dispatch_intent(&mut s, Intent::ExCommand(":enew".into()));
    let buf = s.active_buffer().unwrap();
    assert_eq!(buf.text.text().trim(), "");
}

// ── :syntax on/off ──

#[test]
fn syntax_on_off() {
    let mut s = setup("hello");
    assert!(s.syntax_enabled);
    dispatch_intent(&mut s, Intent::ExCommand(":syntax off".into()));
    assert!(!s.syntax_enabled);
    dispatch_intent(&mut s, Intent::ExCommand(":syntax on".into()));
    assert!(s.syntax_enabled);
}

// ── :highlight shows info ──

#[test]
fn highlight_group_info() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":highlight Comment".into()));
    assert!(s.message.as_ref().unwrap().contains("Comment"));
}

#[test]
fn highlight_unknown_group() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":highlight FooBar".into()));
    assert!(s.message.as_ref().unwrap().contains("Unknown"));
}

// ── :digraphs shows table ──

#[test]
fn digraphs_command_shows_entries() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":digraphs".into()));
    let msg = s.message.as_ref().unwrap();
    assert!(msg.contains("á")); // should contain at least accented a
}

// ── HighlightGroup::from_name ──

#[test]
fn highlight_group_from_name() {
    use kjxlkj_core_types::HighlightGroup;
    assert_eq!(HighlightGroup::from_name("Comment"), Some(HighlightGroup::Comment));
    assert_eq!(HighlightGroup::from_name("Keyword"), Some(HighlightGroup::Keyword));
    assert_eq!(HighlightGroup::from_name("Nonexistent"), None);
}

#[test]
fn highlight_group_default_style() {
    use kjxlkj_core_types::HighlightGroup;
    let s = HighlightGroup::Comment.default_style();
    assert!(s.italic);
    assert!(s.fg.is_some());
}

// ── :saveas reuses :write path ──

#[test]
fn saveas_no_args() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":saveas".into()));
    assert!(s.message.as_ref().unwrap().contains("Usage"));
}
