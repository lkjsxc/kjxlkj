//! Tests for command parser.

use crate::{CommandKind, CommandParser, Range};
use crate::command::{BufferArg, SetArg, SubstituteArgs, SubstituteFlags};
use std::path::PathBuf;

#[test]
fn test_parse_write() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("w").unwrap();
    assert_eq!(cmd.kind, CommandKind::Write(None));
}

#[test]
fn test_parse_write_file() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("w foo.txt").unwrap();
    assert_eq!(cmd.kind, CommandKind::Write(Some(PathBuf::from("foo.txt"))));
}

#[test]
fn test_parse_quit() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("q").unwrap();
    assert_eq!(cmd.kind, CommandKind::Quit);
    assert!(!cmd.force);
}

#[test]
fn test_parse_quit_force() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("q!").unwrap();
    assert_eq!(cmd.kind, CommandKind::Quit);
    assert!(cmd.force);
}

#[test]
fn test_parse_wq() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("wq").unwrap();
    assert_eq!(cmd.kind, CommandKind::WriteQuit);
}

#[test]
fn test_parse_edit() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("e test.rs").unwrap();
    assert_eq!(cmd.kind, CommandKind::Edit(PathBuf::from("test.rs")));
}

#[test]
fn test_parse_buffer_number() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("b 3").unwrap();
    assert_eq!(cmd.kind, CommandKind::Buffer(BufferArg::Number(3)));
}

#[test]
fn test_parse_buffer_name() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("b main.rs").unwrap();
    assert_eq!(cmd.kind, CommandKind::Buffer(BufferArg::Name("main.rs".to_string())));
}

#[test]
fn test_parse_split() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("sp").unwrap();
    assert_eq!(cmd.kind, CommandKind::Split(None));
}

#[test]
fn test_parse_vsplit_file() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("vsp lib.rs").unwrap();
    assert_eq!(cmd.kind, CommandKind::VSplit(Some(PathBuf::from("lib.rs"))));
}

#[test]
fn test_parse_set_enable() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("set number").unwrap();
    assert_eq!(cmd.kind, CommandKind::Set(SetArg::Enable("number".to_string())));
}

#[test]
fn test_parse_set_disable() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("set nonumber").unwrap();
    assert_eq!(cmd.kind, CommandKind::Set(SetArg::Disable("number".to_string())));
}

#[test]
fn test_parse_set_value() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("set tabstop=4").unwrap();
    assert_eq!(cmd.kind, CommandKind::Set(SetArg::Value("tabstop".to_string(), "4".to_string())));
}

#[test]
fn test_parse_substitute() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("s/foo/bar/g").unwrap();
    let expected = SubstituteArgs {
        pattern: "foo".to_string(),
        replacement: "bar".to_string(),
        flags: SubstituteFlags { global: true, ignore_case: false, confirm: false },
    };
    assert_eq!(cmd.kind, CommandKind::Substitute(expected));
}

#[test]
fn test_parse_range_all() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("%d").unwrap();
    assert_eq!(cmd.range, Some(Range::All));
    assert_eq!(cmd.kind, CommandKind::Delete);
}

#[test]
fn test_parse_range_line() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("10d").unwrap();
    assert_eq!(cmd.range, Some(Range::Line(9))); // 0-indexed
    assert_eq!(cmd.kind, CommandKind::Delete);
}

#[test]
fn test_parse_range_from_to() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("10,20d").unwrap();
    assert_eq!(cmd.range, Some(Range::FromTo(9, 19))); // 0-indexed
    assert_eq!(cmd.kind, CommandKind::Delete);
}

#[test]
fn test_parse_list_buffers() {
    let mut parser = CommandParser::new();
    let cmd = parser.parse("ls").unwrap();
    assert_eq!(cmd.kind, CommandKind::ListBuffers);
}

#[test]
fn test_parse_bn_bp() {
    let mut parser = CommandParser::new();
    assert_eq!(parser.parse("bn").unwrap().kind, CommandKind::BufferNext);
    assert_eq!(parser.parse("bp").unwrap().kind, CommandKind::BufferPrev);
}
