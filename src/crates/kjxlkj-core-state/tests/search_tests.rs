//! Tests for search forward/backward, pattern compilation, Vim regex translation.

use kjxlkj_core_state::search::{search_backward, search_forward, SearchState};
use kjxlkj_core_state::search_regex::{
    compile_pattern, find_all_matches, find_next, find_prev, translate_vim_pattern,
};
use kjxlkj_core_text::TextBuffer;
use kjxlkj_core_types::{BufferId, Position};

fn buf(text: &str) -> TextBuffer {
    TextBuffer::from_text(BufferId(1), "t".into(), text)
}

// --- search_forward / search_backward ---

#[test]
fn search_forward_finds_match() {
    let b = buf("hello world\nfoo bar\n");
    let pos = search_forward(&b, "foo", Position::ZERO).unwrap();
    assert_eq!(pos, Position::new(1, 0));
}

#[test]
fn search_forward_wraps_around() {
    let b = buf("abc\ndef\n");
    let pos = search_forward(&b, "abc", Position::new(1, 0)).unwrap();
    assert_eq!(pos, Position::new(0, 0));
}

#[test]
fn search_forward_no_match() {
    let b = buf("hello world\n");
    assert!(search_forward(&b, "zzz", Position::ZERO).is_none());
}

#[test]
fn search_backward_finds_match() {
    let b = buf("abc\ndef\nabc\n");
    let pos = search_backward(&b, "abc", Position::new(2, 2)).unwrap();
    assert_eq!(pos, Position::new(2, 0));
}

#[test]
fn search_backward_wraps() {
    let b = buf("xyz\nabc\n");
    let pos = search_backward(&b, "abc", Position::new(0, 0)).unwrap();
    assert_eq!(pos, Position::new(1, 0));
}

// --- Pattern compilation ---

#[test]
fn compile_case_insensitive() {
    let re = compile_pattern("hello", false).unwrap();
    assert!(re.is_match("HELLO"));
    assert!(re.is_match("hello"));
}

#[test]
fn compile_case_sensitive() {
    let re = compile_pattern("hello", true).unwrap();
    assert!(!re.is_match("HELLO"));
    assert!(re.is_match("hello"));
}

#[test]
fn compile_invalid_regex_error() {
    assert!(compile_pattern("[invalid", true).is_err());
}

// --- Vim pattern translation ---

#[test]
fn translate_word_boundary() {
    assert_eq!(translate_vim_pattern("\\<word\\>"), "\\bword\\b");
}

#[test]
fn translate_groups_and_alternation() {
    assert_eq!(translate_vim_pattern("\\(a\\|b\\)"), "(a|b)");
}

#[test]
fn translate_quantifier_plus() {
    assert_eq!(translate_vim_pattern("a\\+"), "a+");
}

#[test]
fn translate_passthrough() {
    assert_eq!(translate_vim_pattern("abc"), "abc");
}

// --- SmartCase ---

#[test]
fn smart_case_lower_is_insensitive() {
    let ss = SearchState {
        smart_case: true,
        ignore_case: true,
        ..SearchState::new()
    };
    assert!(!ss.effective_case_sensitive("hello"));
}

#[test]
fn smart_case_upper_is_sensitive() {
    let ss = SearchState {
        smart_case: true,
        ignore_case: true,
        ..SearchState::new()
    };
    assert!(ss.effective_case_sensitive("Hello"));
}

#[test]
fn default_is_case_sensitive() {
    let ss = SearchState::new();
    assert!(ss.effective_case_sensitive("hello"));
}

// --- find_all / find_next / find_prev ---

#[test]
fn find_all_matches_count() {
    let re = compile_pattern("ab", true).unwrap();
    let m = find_all_matches("ababab", &re);
    assert_eq!(m.len(), 3);
}

#[test]
fn find_next_from_offset() {
    let re = compile_pattern("x", true).unwrap();
    let m = find_next("..x..x", &re, 3).unwrap();
    assert_eq!(m.0, 5);
}

#[test]
fn find_prev_before_offset() {
    let re = compile_pattern("x", true).unwrap();
    let m = find_prev("x..x..", &re, 4).unwrap();
    assert_eq!(m.0, 3);
}
