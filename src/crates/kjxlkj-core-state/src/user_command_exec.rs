//! User command execution — argument substitution and dispatch for :command definitions.

use crate::scripting::{UserCommand, NArgs};

/// Result of executing a user command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecResult {
    Ok(String),
    Error(String),
    NoSuchCommand(String),
}

/// Validate argument count against NArgs spec.
pub fn validate_nargs(nargs: NArgs, arg_count: usize) -> Result<(), String> {
    match nargs {
        NArgs::Zero if arg_count > 0 => Err("no arguments allowed".into()),
        NArgs::One if arg_count != 1 => Err("exactly one argument required".into()),
        NArgs::AtLeastOne if arg_count == 0 => Err("at least one argument required".into()),
        _ => Ok(()),
    }
}

/// Substitute `<args>`, `<bang>`, `<line1>`, `<line2>`, `<count>`, `<q-args>` in replacement text.
pub fn substitute_args(template: &str, args: &str, bang: bool, line1: usize, line2: usize) -> String {
    template
        .replace("<args>", args)
        .replace("<q-args>", &format!("\"{}\"", args.replace('\\', "\\\\").replace('"', "\\\"")))
        .replace("<bang>", if bang { "!" } else { "" })
        .replace("<line1>", &(line1 + 1).to_string())
        .replace("<line2>", &(line2 + 1).to_string())
        .replace("<count>", &(line2.saturating_sub(line1) + 1).to_string())
}

/// Execute a user command, returning the expanded replacement string.
pub fn execute_user_command(cmd: &UserCommand, args: &str, bang: bool, line1: usize, line2: usize) -> ExecResult {
    if bang && !cmd.bang {
        return ExecResult::Error(format!("{}: bang not allowed", cmd.name));
    }
    let arg_count = if args.is_empty() { 0 } else { args.split_whitespace().count() };
    if let Err(e) = validate_nargs(cmd.nargs, arg_count) {
        return ExecResult::Error(format!("{}: {}", cmd.name, e));
    }
    let expanded = substitute_args(&cmd.replacement, args, bang, line1, line2);
    ExecResult::Ok(expanded)
}

/// Find and execute from a list of user commands.
pub fn dispatch_user_command(cmds: &[UserCommand], name: &str, args: &str, bang: bool, line1: usize, line2: usize) -> ExecResult {
    match cmds.iter().find(|c| c.name == name) {
        Some(cmd) => execute_user_command(cmd, args, bang, line1, line2),
        None => ExecResult::NoSuchCommand(name.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd(name: &str, repl: &str, nargs: NArgs, bang: bool) -> UserCommand {
        UserCommand { name: name.into(), replacement: repl.into(), nargs, bang, range: false, buffer_local: false }
    }

    #[test]
    fn execute_simple() {
        let c = cmd("Greet", "echo 'hello <args>'", NArgs::Any, false);
        let r = execute_user_command(&c, "world", false, 0, 0);
        assert_eq!(r, ExecResult::Ok("echo 'hello world'".into()));
    }

    #[test]
    fn execute_with_bang() {
        let c = cmd("Save", "w<bang>", NArgs::Zero, true);
        let r = execute_user_command(&c, "", true, 0, 0);
        assert_eq!(r, ExecResult::Ok("w!".into()));
    }

    #[test]
    fn execute_bang_not_allowed() {
        let c = cmd("NoBang", "echo", NArgs::Zero, false);
        let r = execute_user_command(&c, "", true, 0, 0);
        assert!(matches!(r, ExecResult::Error(_)));
    }

    #[test]
    fn validate_nargs_zero() {
        assert!(validate_nargs(NArgs::Zero, 0).is_ok());
        assert!(validate_nargs(NArgs::Zero, 1).is_err());
    }

    #[test]
    fn validate_nargs_one() {
        assert!(validate_nargs(NArgs::One, 1).is_ok());
        assert!(validate_nargs(NArgs::One, 0).is_err());
        assert!(validate_nargs(NArgs::One, 2).is_err());
    }

    #[test]
    fn substitute_line_range() {
        let r = substitute_args("echo <line1>-<line2> (<count> lines)", "", false, 4, 9);
        assert_eq!(r, "echo 5-10 (6 lines)");
    }

    #[test]
    fn dispatch_found() {
        let cmds = vec![cmd("Hello", "echo hi", NArgs::Zero, false)];
        let r = dispatch_user_command(&cmds, "Hello", "", false, 0, 0);
        assert_eq!(r, ExecResult::Ok("echo hi".into()));
    }

    #[test]
    fn dispatch_not_found() {
        let r = dispatch_user_command(&[], "Missing", "", false, 0, 0);
        assert!(matches!(r, ExecResult::NoSuchCommand(_)));
    }

    #[test]
    fn q_args_escaping() {
        let r = substitute_args("echo <q-args>", "he\"llo", false, 0, 0);
        assert_eq!(r, "echo \"he\\\"llo\"");
    }
}
