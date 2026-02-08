//! Tests for escape sequence parser.

use crate::escape_parser::{EscapeParser, ParseAction};

#[test]
fn parse_plain_text() {
    let mut parser = EscapeParser::new();
    let mut actions = Vec::new();
    for b in b"hello" {
        parser.feed(*b, &mut actions);
    }
    assert_eq!(actions.len(), 5);
    assert!(matches!(actions[0], ParseAction::Print('h')));
}

#[test]
fn parse_csi_sequence() {
    let mut parser = EscapeParser::new();
    let mut actions = Vec::new();
    // ESC [ 1 ; 2 H
    for b in b"\x1b[1;2H" {
        parser.feed(*b, &mut actions);
    }
    assert!(actions.iter().any(|a| matches!(
        a,
        ParseAction::CsiDispatch('H', _)
    )));
}

#[test]
fn parse_c0_control() {
    let mut parser = EscapeParser::new();
    let mut actions = Vec::new();
    parser.feed(0x0A, &mut actions); // LF
    assert!(matches!(
        actions[0],
        ParseAction::Execute(0x0A)
    ));
}
