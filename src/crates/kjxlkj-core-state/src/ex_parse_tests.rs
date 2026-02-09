//! Tests for ex command range and substitute parsers.
use crate::ex_parse::{parse_range, ExRange};
use crate::ex_parse_substitute::parse_substitute;

#[test]
fn test_parse_range_percent() {
    let (range, rest) = parse_range("%s/foo/bar/", 5, 100);
    assert_eq!(range, Some(ExRange { start: 0, end: 99 }));
    assert_eq!(rest, "s/foo/bar/");
}

#[test]
fn test_parse_range_single_line() {
    let (range, rest) = parse_range("5d", 0, 100);
    assert_eq!(range, Some(ExRange { start: 4, end: 4 }));
    assert_eq!(rest, "d");
}

#[test]
fn test_parse_range_line_range() {
    let (range, rest) = parse_range("10,20s/a/b/g", 0, 100);
    assert_eq!(range, Some(ExRange { start: 9, end: 19 }));
    assert_eq!(rest, "s/a/b/g");
}

#[test]
fn test_parse_range_dot_dollar() {
    let (range, rest) = parse_range(".,$d", 5, 100);
    assert_eq!(range, Some(ExRange { start: 5, end: 99 }));
    assert_eq!(rest, "d");
}

#[test]
fn test_parse_range_offset() {
    let (range, _) = parse_range(".-2,.+3d", 10, 100);
    assert_eq!(range, Some(ExRange { start: 8, end: 13 }));
}

#[test]
fn test_parse_range_none() {
    let (range, rest) = parse_range("d", 0, 100);
    assert!(range.is_none());
    assert_eq!(rest, "d");
}

#[test]
fn test_parse_substitute_basic() {
    let cmd = parse_substitute("/foo/bar/").unwrap();
    assert_eq!(cmd.pattern, "foo");
    assert_eq!(cmd.replacement, "bar");
    assert!(!cmd.global);
}

#[test]
fn test_parse_substitute_global() {
    let cmd = parse_substitute("/foo/bar/g").unwrap();
    assert!(cmd.global);
}

#[test]
fn test_parse_substitute_flags() {
    let cmd = parse_substitute("/foo/bar/gin").unwrap();
    assert!(cmd.global);
    assert!(cmd.case_insensitive);
    assert!(cmd.count_only);
}

#[test]
fn test_parse_substitute_alt_delim() {
    let cmd = parse_substitute("#foo#bar#g").unwrap();
    assert_eq!(cmd.pattern, "foo");
    assert_eq!(cmd.replacement, "bar");
    assert!(cmd.global);
}

#[test]
fn test_parse_substitute_escaped_delim() {
    let cmd = parse_substitute("/foo\\/bar/baz/").unwrap();
    assert_eq!(cmd.pattern, "foo\\/bar");
    assert_eq!(cmd.replacement, "baz");
}
