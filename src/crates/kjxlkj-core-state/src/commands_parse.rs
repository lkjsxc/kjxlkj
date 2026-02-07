//! Ex command parsing: convert string input into ExCommand.

use kjxlkj_core_types::EditorError;

use crate::commands::ExCommand;
use crate::commands_parse_ext::{parse_extended, parse_global, parse_substitute};

/// Parse an Ex command string into an ExCommand.
pub fn parse_command(input: &str) -> Result<ExCommand, EditorError> {
    let input = input.trim();
    let input = input.strip_prefix(':').unwrap_or(input);
    let input = input.trim();
    if input.is_empty() {
        return Err(EditorError::InvalidCommand("empty command".into()));
    }
    // Line number only
    if let Ok(n) = input.parse::<usize>() {
        return Ok(ExCommand::GoToLine(n));
    }
    // Shell command
    if let Some(rest) = input.strip_prefix('!') {
        return Ok(ExCommand::ShellCommand(rest.trim().to_string()));
    }
    // Substitute :s/p/r/f
    if input.starts_with("s/") || input.starts_with("s!") {
        return parse_substitute(input);
    }
    // Global :g/p/cmd and :v/p/cmd
    if input.starts_with("g/") || input.starts_with("g!") {
        return parse_global(input, false);
    }
    if input.starts_with("v/") {
        return parse_global(input, true);
    }
    // Split into command and args
    let (cmd, args) = split_cmd_args(input);
    match cmd {
        "q" | "quit" => Ok(ExCommand::Quit),
        "q!" | "quit!" => Ok(ExCommand::ForceQuit),
        "qa" | "qall" => Ok(ExCommand::QuitAll),
        "qa!" | "qall!" => Ok(ExCommand::ForceQuitAll),
        "w" | "write" => Ok(ExCommand::Write(opt_string(args))),
        "wa" | "wall" => Ok(ExCommand::WriteAll),
        "wq" | "x" | "exit" => Ok(ExCommand::WriteQuit(opt_string(args))),
        "e" | "edit" => {
            let path = args.unwrap_or_default().to_string();
            Ok(ExCommand::Edit(path, false))
        }
        "e!" | "edit!" => {
            let path = args.unwrap_or_default().to_string();
            Ok(ExCommand::Edit(path, true))
        }
        "ls" | "buffers" | "files" => Ok(ExCommand::BufferList),
        "bn" | "bnext" => Ok(ExCommand::BufferNext),
        "bp" | "bprev" | "bprevious" => Ok(ExCommand::BufferPrev),
        "bd" | "bdelete" => Ok(ExCommand::BufferDelete(false)),
        "bd!" | "bdelete!" => Ok(ExCommand::BufferDelete(true)),
        "sp" | "split" => Ok(ExCommand::Split),
        "vsp" | "vsplit" => Ok(ExCommand::VSplit),
        "new" => Ok(ExCommand::New),
        "vnew" => Ok(ExCommand::VNew),
        "only" | "on" => Ok(ExCommand::Only),
        "set" | "se" => Ok(ExCommand::Set(args.unwrap_or_default().to_string())),
        "enew" => Ok(ExCommand::Enew),
        "saveas" => Ok(ExCommand::SaveAs(req_arg(args, "saveas")?)),
        "b" | "buffer" => parse_buffer_switch(args),
        "scratch" => Ok(ExCommand::ScratchBuffer),
        _ => parse_extended(cmd, args),
    }
}

fn parse_buffer_switch(args: Option<&str>) -> Result<ExCommand, EditorError> {
    let arg = args.unwrap_or("");
    if arg == "#" {
        return Ok(ExCommand::SwitchBuffer(0)); // 0 = alternate
    }
    let id: u64 = arg
        .parse()
        .map_err(|_| EditorError::InvalidCommand(format!("invalid buffer number: {arg}")))?;
    Ok(ExCommand::SwitchBuffer(id))
}

fn split_cmd_args(input: &str) -> (&str, Option<&str>) {
    if let Some(idx) = input.find(|c: char| c.is_whitespace()) {
        let cmd = &input[..idx];
        let args = input[idx..].trim_start();
        if args.is_empty() {
            (cmd, None)
        } else {
            (cmd, Some(args))
        }
    } else {
        (input, None)
    }
}

fn opt_string(args: Option<&str>) -> Option<String> {
    args.map(|s| s.to_string())
}

fn req_arg(args: Option<&str>, cmd_name: &str) -> Result<String, EditorError> {
    args.map(|s| s.to_string())
        .ok_or_else(|| EditorError::InvalidCommand(format!("{cmd_name}: argument required")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quit() {
        assert_eq!(parse_command(":q").unwrap(), ExCommand::Quit);
        assert_eq!(parse_command(":q!").unwrap(), ExCommand::ForceQuit);
        assert_eq!(parse_command(":qa").unwrap(), ExCommand::QuitAll);
    }

    #[test]
    fn parse_write() {
        assert_eq!(parse_command(":w").unwrap(), ExCommand::Write(None));
        assert_eq!(
            parse_command(":w foo.txt").unwrap(),
            ExCommand::Write(Some("foo.txt".into()))
        );
    }

    #[test]
    fn parse_line_number() {
        assert_eq!(parse_command(":42").unwrap(), ExCommand::GoToLine(42));
    }

    #[test]
    fn parse_edit() {
        assert!(
            matches!(parse_command(":e main.rs").unwrap(), ExCommand::Edit(p, false) if p == "main.rs")
        );
        assert!(
            matches!(parse_command(":e! main.rs").unwrap(), ExCommand::Edit(p, true) if p == "main.rs")
        );
    }

    #[test]
    fn parse_set() {
        assert!(
            matches!(parse_command(":set number").unwrap(), ExCommand::Set(s) if s == "number")
        );
    }

    #[test]
    fn parse_shell() {
        assert!(matches!(parse_command(":!ls").unwrap(), ExCommand::ShellCommand(s) if s == "ls"));
    }

    #[test]
    fn parse_buffer() {
        assert!(matches!(
            parse_command(":b 3").unwrap(),
            ExCommand::SwitchBuffer(3)
        ));
    }
}
