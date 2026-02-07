//! Tests for Ex command parsing.

use kjxlkj_core_state::commands::ExCommand;
use kjxlkj_core_state::commands_parse::parse_command;

// --- Basic commands ---

#[test]
fn parse_quit() {
    assert_eq!(parse_command(":q").unwrap(), ExCommand::Quit);
}

#[test]
fn parse_force_quit() {
    assert_eq!(parse_command(":q!").unwrap(), ExCommand::ForceQuit);
}

#[test]
fn parse_quit_all() {
    assert_eq!(parse_command(":qa").unwrap(), ExCommand::QuitAll);
}

#[test]
fn parse_force_quit_all() {
    assert_eq!(parse_command(":qa!").unwrap(), ExCommand::ForceQuitAll);
}

#[test]
fn parse_write_no_arg() {
    assert_eq!(parse_command(":w").unwrap(), ExCommand::Write(None));
}

#[test]
fn parse_write_with_file() {
    assert_eq!(parse_command(":w foo.txt").unwrap(), ExCommand::Write(Some("foo.txt".into())));
}

#[test]
fn parse_write_quit() {
    assert!(matches!(parse_command(":wq").unwrap(), ExCommand::WriteQuit(None)));
}

#[test]
fn parse_edit() {
    match parse_command(":e main.rs").unwrap() {
        ExCommand::Edit(p, false) => assert_eq!(p, "main.rs"),
        other => panic!("expected Edit, got {other:?}"),
    }
}

#[test]
fn parse_edit_force() {
    match parse_command(":e! main.rs").unwrap() {
        ExCommand::Edit(p, true) => assert_eq!(p, "main.rs"),
        other => panic!("expected Edit!, got {other:?}"),
    }
}

#[test]
fn parse_buffer_list() {
    assert_eq!(parse_command(":ls").unwrap(), ExCommand::BufferList);
}

#[test]
fn parse_buffer_next() {
    assert_eq!(parse_command(":bn").unwrap(), ExCommand::BufferNext);
}

#[test]
fn parse_buffer_prev() {
    assert_eq!(parse_command(":bp").unwrap(), ExCommand::BufferPrev);
}

#[test]
fn parse_buffer_delete() {
    assert!(matches!(parse_command(":bd").unwrap(), ExCommand::BufferDelete(false)));
}

#[test]
fn parse_split() {
    assert_eq!(parse_command(":sp").unwrap(), ExCommand::Split);
}

#[test]
fn parse_vsplit() {
    assert_eq!(parse_command(":vsp").unwrap(), ExCommand::VSplit);
}

#[test]
fn parse_set() {
    match parse_command(":set number").unwrap() {
        ExCommand::Set(s) => assert_eq!(s, "number"),
        other => panic!("expected Set, got {other:?}"),
    }
}

#[test]
fn parse_sort() {
    assert_eq!(parse_command(":sort").unwrap(), ExCommand::Sort);
}

#[test]
fn parse_nohlsearch() {
    assert_eq!(parse_command(":noh").unwrap(), ExCommand::NoHlSearch);
}

#[test]
fn parse_line_number() {
    assert_eq!(parse_command(":42").unwrap(), ExCommand::GoToLine(42));
}

// --- Substitute ---

#[test]
fn parse_substitute() {
    match parse_command(":s/old/new/g").unwrap() {
        ExCommand::Substitute(p, r, f) => {
            assert_eq!(p, "old");
            assert_eq!(r, "new");
            assert_eq!(f, "g");
        }
        other => panic!("expected Substitute, got {other:?}"),
    }
}

// --- Global ---

#[test]
fn parse_global() {
    match parse_command(":g/pattern/d").unwrap() {
        ExCommand::Global(p, c) => {
            assert_eq!(p, "pattern");
            assert_eq!(c, "d");
        }
        other => panic!("expected Global, got {other:?}"),
    }
}

// --- Error ---

#[test]
fn parse_empty_is_error() {
    assert!(parse_command(":").is_err());
}
