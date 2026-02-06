//! Tests for regex search, case-insensitive search, and word-boundary search.

use kjxlkj_core_state::{dispatch_intent, EditorState};
use kjxlkj_core_types::{Intent, Size};

fn setup(text: &str) -> EditorState {
    let mut s = EditorState::new(Size::new(80, 24));
    let bid = s.create_buffer_from_text(text);
    s.create_window(bid);
    s
}

fn cursor(s: &EditorState) -> (usize, usize) {
    let pos = s.cursor();
    (pos.line, pos.col)
}

// ──────── Literal search ────────

#[test]
fn search_forward_literal() {
    let mut s = setup("hello world\nfoo bar\nhello again");
    dispatch_intent(&mut s, Intent::SearchForward("hello".into()));
    // Should find second occurrence (forward from 0,0 starts at 0,1+)
    assert_eq!(cursor(&s), (2, 0));
}

#[test]
fn search_backward_literal() {
    let mut s = setup("hello world\nfoo bar\nhello again");
    // Move to last line first
    dispatch_intent(&mut s, Intent::Motion(kjxlkj_core_types::MotionKind::FileEnd, 1));
    dispatch_intent(&mut s, Intent::SearchBackward("foo".into()));
    assert_eq!(cursor(&s).0, 1);
}

#[test]
fn search_next_repeats() {
    let mut s = setup("aaa\naaa\naaa");
    dispatch_intent(&mut s, Intent::SearchForward("aaa".into()));
    assert_eq!(cursor(&s).0, 1); // first match forward from line 0
    dispatch_intent(&mut s, Intent::SearchNext);
    assert_eq!(cursor(&s).0, 2);
}

#[test]
fn search_prev_reverses() {
    let mut s = setup("aaa\nbbb\naaa");
    dispatch_intent(&mut s, Intent::SearchForward("aaa".into()));
    assert_eq!(cursor(&s).0, 2);
    dispatch_intent(&mut s, Intent::SearchPrev);
    assert_eq!(cursor(&s).0, 0);
}

// ──────── Case-insensitive search ────────

#[test]
fn search_case_insensitive() {
    let mut s = setup("Hello World\nhello world");
    s.options.ignorecase = true;
    s.options.smartcase = false;
    dispatch_intent(&mut s, Intent::SearchForward("hello".into()));
    // With ignorecase, should find "Hello" on line 0 then wrap or match line 1
    assert_eq!(cursor(&s).0, 1);
}

#[test]
fn search_smartcase_uppercase_literal() {
    let mut s = setup("hello world\nHello World");
    s.options.ignorecase = true;
    s.options.smartcase = true;
    // SmartCase: "Hello" has uppercase, so match exact case
    dispatch_intent(&mut s, Intent::SearchForward("Hello".into()));
    assert_eq!(cursor(&s).0, 1);
}

// ──────── Regex search ────────

#[test]
fn search_regex_dot() {
    let mut s = setup("cat\ncar\ncap\ncab");
    dispatch_intent(&mut s, Intent::SearchForward("ca.".into()));
    assert_eq!(cursor(&s).0, 1); // first match: "car" on line 1
}

#[test]
fn search_regex_anchors() {
    let mut s = setup("  foo\nfoo bar\n  bar foo");
    dispatch_intent(&mut s, Intent::SearchForward("^foo".into()));
    assert_eq!(cursor(&s).0, 1);
}

#[test]
fn search_regex_character_class() {
    let mut s = setup("abc\na1c\na2c\nabc");
    dispatch_intent(&mut s, Intent::SearchForward("a[0-9]c".into()));
    assert_eq!(cursor(&s).0, 1);
}

// ──────── Word-under-cursor search ────────

#[test]
fn search_word_forward() {
    let mut s = setup("foo bar\nbaz foo\nqux");
    dispatch_intent(&mut s, Intent::SearchWordForward);
    assert_eq!(cursor(&s).0, 1); // finds "foo" on line 1
}

#[test]
fn search_word_backward() {
    let mut s = setup("foo bar\nbaz\nfoo qux");
    // Move to last line
    dispatch_intent(&mut s, Intent::Motion(kjxlkj_core_types::MotionKind::FileEnd, 1));
    dispatch_intent(&mut s, Intent::SearchWordBackward);
    assert_eq!(cursor(&s).0, 0); // finds "foo" on line 0
}

// ──────── No match ────────

#[test]
fn search_no_match_sets_message() {
    let mut s = setup("hello world");
    dispatch_intent(&mut s, Intent::SearchForward("zzzzz".into()));
    assert!(s.message.as_ref().unwrap().contains("not found"));
}

#[test]
fn search_no_pattern_message() {
    let mut s = setup("hello");
    dispatch_intent(&mut s, Intent::SearchNext);
    assert!(s.message.as_ref().unwrap().contains("No previous search"));
}

// ──────── Regex: Vim metacharacters ────────

#[test]
fn search_vim_word_boundary() {
    let mut s = setup("foobar\nfoo bar\nbarfoo");
    // \< and \> are word boundaries in Vim
    dispatch_intent(&mut s, Intent::SearchForward(r"\<foo\>".into()));
    assert_eq!(cursor(&s).0, 1); // "foo" as whole word on line 1
}

#[test]
fn search_vim_digit_class() {
    let mut s = setup("abc\na1b\nc2d");
    dispatch_intent(&mut s, Intent::SearchForward(r"\d".into()));
    assert_eq!(cursor(&s).0, 1);
}
