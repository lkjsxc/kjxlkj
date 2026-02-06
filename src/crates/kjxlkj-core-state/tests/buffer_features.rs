//! Tests for buffer features: alternate buffer, scratch, quickfix, session, config.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, KeyEvent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

// ── Alternate buffer ──

#[test]
fn ctrl_caret_no_alternate() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::SwitchAlternate);
    assert!(s.message.as_ref().unwrap().contains("No alternate"));
}

#[test]
fn alternate_set_by_enew() {
    let mut s = setup("original");
    let original_bid = s.active_buffer().unwrap().id;
    dispatch_intent(&mut s, Intent::ExCommand(":enew".into()));
    assert_eq!(s.alternate_file, Some(original_bid));
}

#[test]
fn switch_alternate_round_trip() {
    let mut s = setup("first");
    let bid1 = s.active_buffer().unwrap().id;
    dispatch_intent(&mut s, Intent::ExCommand(":enew".into()));
    let bid2 = s.active_buffer().unwrap().id;
    assert_ne!(bid1, bid2);
    // Switch to alternate (bid1)
    dispatch_intent(&mut s, Intent::SwitchAlternate);
    assert_eq!(s.active_buffer().unwrap().id, bid1);
    // Switch back
    dispatch_intent(&mut s, Intent::SwitchAlternate);
    assert_eq!(s.active_buffer().unwrap().id, bid2);
}

#[test]
fn b_hash_switches_alternate() {
    let mut s = setup("first");
    let bid1 = s.active_buffer().unwrap().id;
    dispatch_intent(&mut s, Intent::ExCommand(":enew".into()));
    dispatch_intent(&mut s, Intent::ExCommand(":b#".into()));
    assert_eq!(s.active_buffer().unwrap().id, bid1);
}

#[test]
fn b_number_switches() {
    let mut s = setup("first");
    let bid1 = s.active_buffer().unwrap().id;
    let bid2 = s.create_buffer_from_text("second");
    dispatch_intent(&mut s, Intent::ExCommand(format!(":b {}", bid2.0).as_str().into()));
    assert_eq!(s.active_buffer().unwrap().id, bid2);
    assert_eq!(s.alternate_file, Some(bid1));
}

#[test]
fn b_nonexistent_buffer() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":b 999".into()));
    assert!(s.message.as_ref().unwrap().contains("does not exist"));
}

#[test]
fn ctrl_caret_parser() {
    let mut s = setup("hello");
    let key = KeyEvent::ctrl('6');
    let intent = s.parser.parse_normal(&key);
    assert_eq!(intent, Intent::SwitchAlternate);
}

// ── Scratch buffer ──

#[test]
fn scratch_buffer_created() {
    let mut s = setup("original");
    dispatch_intent(&mut s, Intent::ExCommand(":scratch".into()));
    let buf = s.active_buffer().unwrap();
    assert!(buf.scratch);
    assert!(!buf.listed);
    assert!(s.message.as_ref().unwrap().contains("Scratch"));
}

// ── Config options ──

#[test]
fn set_cursorline() {
    let mut s = setup("hello");
    assert!(!s.options.cursorline);
    dispatch_intent(&mut s, Intent::ExCommand(":set cursorline".into()));
    assert!(s.options.cursorline);
    dispatch_intent(&mut s, Intent::ExCommand(":set nocursorline".into()));
    assert!(!s.options.cursorline);
}

#[test]
fn set_list() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":set list".into()));
    assert!(s.options.list);
}

#[test]
fn set_hidden() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":set hidden".into()));
    assert!(s.options.hidden);
    dispatch_intent(&mut s, Intent::ExCommand(":set nohidden".into()));
    assert!(!s.options.hidden);
}

#[test]
fn set_showmode_default() {
    let s = setup("hello");
    assert!(s.options.showmode);
    assert!(s.options.showcmd);
}

#[test]
fn query_option() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":set cursorline?".into()));
    assert!(s.message.as_ref().unwrap().contains("false"));
}

// ── Quickfix ──

#[test]
fn quickfix_empty_list() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::ExCommand(":copen".into()));
    assert!(s.message.as_ref().unwrap().contains("empty"));
}

#[test]
fn quickfix_cnext_cprev() {
    let mut s = setup("hello");
    use kjxlkj_core_state::{QuickfixEntry, QuickfixKind};
    s.quickfix.set(vec![
        QuickfixEntry { file: "a.rs".into(), line: 1, col: 0, text: "err1".into(), kind: QuickfixKind::Error },
        QuickfixEntry { file: "b.rs".into(), line: 5, col: 0, text: "err2".into(), kind: QuickfixKind::Error },
    ], "test");
    dispatch_intent(&mut s, Intent::ExCommand(":cnext".into()));
    assert!(s.message.as_ref().unwrap().contains("b.rs"));
    dispatch_intent(&mut s, Intent::ExCommand(":cprev".into()));
    assert!(s.message.as_ref().unwrap().contains("a.rs"));
}

// ── Session ──

#[test]
fn recent_files_tracking() {
    let mut s = setup("hello");
    s.recent_files.push("/test.rs", 5, 0);
    assert_eq!(s.recent_files.entries.len(), 1);
    let found = s.recent_files.find("/test.rs").unwrap();
    assert_eq!(found.line, 5);
}

#[test]
fn autosave_default_disabled() {
    let s = setup("hello");
    assert!(!s.autosave.enabled);
}
