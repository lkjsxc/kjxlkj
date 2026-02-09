//! Tests for user command registry and parsing.
use crate::user_commands::{Nargs, RangeMode, UserCommand, UserCommandRegistry};
use crate::user_commands_parse::parse_command_def;

#[test]
fn test_define_user_command() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Greet".to_string(),
        replacement: "echo <args>".to_string(),
        nargs: Nargs::One,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    assert!(reg.define(cmd, false).is_ok());
    assert!(reg.contains("Greet"));
    assert_eq!(reg.len(), 1);
}

#[test]
fn test_reject_lowercase_name() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "greet".to_string(),
        replacement: "".to_string(),
        nargs: Nargs::Zero,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    assert!(reg.define(cmd, false).is_err());
}

#[test]
fn test_no_overwrite_duplicate() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Foo".to_string(),
        replacement: "bar".to_string(),
        nargs: Nargs::Zero,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    reg.define(cmd.clone(), false).unwrap();
    assert!(reg.define(cmd, false).is_err());
}

#[test]
fn test_overwrite_duplicate() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Foo".to_string(),
        replacement: "bar".to_string(),
        nargs: Nargs::Zero,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    reg.define(cmd.clone(), false).unwrap();
    assert!(reg.define(cmd, true).is_ok());
}

#[test]
fn test_expand_with_args() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Hello".to_string(),
        replacement: "echo <args>".to_string(),
        nargs: Nargs::One,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    reg.define(cmd, false).unwrap();
    let expanded = reg.expand("Hello", "world", false).unwrap();
    assert_eq!(expanded, "echo world");
}

#[test]
fn test_expand_with_bang() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Do".to_string(),
        replacement: "exec<bang> thing".to_string(),
        nargs: Nargs::Zero,
        range: RangeMode::None,
        bang: true,
        complete: None,
    };
    reg.define(cmd, false).unwrap();
    assert_eq!(reg.expand("Do", "", true).unwrap(), "exec! thing");
    assert_eq!(reg.expand("Do", "", false).unwrap(), "exec thing");
}

#[test]
fn test_remove_command() {
    let mut reg = UserCommandRegistry::new();
    let cmd = UserCommand {
        name: "Foo".to_string(),
        replacement: "bar".to_string(),
        nargs: Nargs::Zero,
        range: RangeMode::None,
        bang: false,
        complete: None,
    };
    reg.define(cmd, false).unwrap();
    assert!(reg.remove("Foo").is_ok());
    assert!(!reg.contains("Foo"));
}

#[test]
fn test_parse_command_def() {
    let (cmd, overwrite) =
        parse_command_def("-nargs=1 Greet echo <args>").unwrap();
    assert_eq!(cmd.name, "Greet");
    assert_eq!(cmd.replacement, "echo <args>");
    assert_eq!(cmd.nargs, Nargs::One);
    assert!(!overwrite);
}

#[test]
fn test_parse_command_def_overwrite() {
    let (cmd, overwrite) = parse_command_def("! Foo bar").unwrap();
    assert_eq!(cmd.name, "Foo");
    assert!(overwrite);
}
