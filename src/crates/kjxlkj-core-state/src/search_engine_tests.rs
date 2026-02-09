//! Tests for the search engine.
use crate::search_types::{CaseMode, SearchDirection, SearchMatch, SearchState};

#[test]
fn test_forward_search() {
    let mut state = SearchState::new();
    state.set_pattern("hello".into(), SearchDirection::Forward);
    let lines = vec!["foo", "hello world", "bar"];
    let found = state.find_next(&lines, 0, 0);
    assert_eq!(
        found,
        Some(SearchMatch {
            line: 1,
            col_start: 0,
            col_end: 5,
        })
    );
}

#[test]
fn test_backward_search() {
    let mut state = SearchState::new();
    state.set_pattern("foo".into(), SearchDirection::Backward);
    let lines = vec!["foo bar", "hello", "baz"];
    let found = state.find_next(&lines, 2, 0);
    assert_eq!(
        found,
        Some(SearchMatch {
            line: 0,
            col_start: 0,
            col_end: 3,
        })
    );
}

#[test]
fn test_smartcase_lowercase() {
    let mut state = SearchState::new();
    state.case_mode = CaseMode::SmartCase;
    state.set_pattern("hello".into(), SearchDirection::Forward);
    assert!(!state.is_case_sensitive());
}

#[test]
fn test_smartcase_uppercase() {
    let mut state = SearchState::new();
    state.case_mode = CaseMode::SmartCase;
    state.set_pattern("Hello".into(), SearchDirection::Forward);
    assert!(state.is_case_sensitive());
}

#[test]
fn test_search_wraps() {
    let mut state = SearchState::new();
    state.set_pattern("foo".into(), SearchDirection::Forward);
    state.wrap_scan = true;
    let lines = vec!["foo", "bar", "baz"];
    let found = state.find_next(&lines, 1, 0);
    assert_eq!(
        found,
        Some(SearchMatch {
            line: 0,
            col_start: 0,
            col_end: 3,
        })
    );
}

#[test]
fn test_no_wrap() {
    let mut state = SearchState::new();
    state.set_pattern("foo".into(), SearchDirection::Forward);
    state.wrap_scan = false;
    let lines = vec!["foo", "bar", "baz"];
    let found = state.find_next(&lines, 1, 0);
    assert_eq!(found, None);
}

#[test]
fn test_count_matches() {
    let mut state = SearchState::new();
    state.set_pattern("a".into(), SearchDirection::Forward);
    let lines = vec!["aaa", "bbb", "aba"];
    assert_eq!(state.count_matches(&lines), 5);
}

#[test]
fn test_history() {
    let mut state = SearchState::new();
    state.set_pattern("first".into(), SearchDirection::Forward);
    state.set_pattern("second".into(), SearchDirection::Forward);
    assert_eq!(state.history.len(), 2);
    assert_eq!(state.history[0], "second");
}

#[test]
fn test_build_all_matches() {
    let mut state = SearchState::new();
    state.set_pattern("ab".into(), SearchDirection::Forward);
    let lines = vec!["ab cd ab", "ef", "ab"];
    state.build_all_matches(&lines);
    assert_eq!(state.matches.len(), 3);
}
