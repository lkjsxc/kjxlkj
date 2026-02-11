//! Ex command parser. Tokenizes `:` command strings
//! into typed Actions.
//! See /docs/spec/commands/syntax.md and essential.md.

use kjxlkj_core_types::Action;

/// Parse an ex command string and return a typed Action.
/// The input should NOT include the leading `:`.
pub fn parse_ex_command(input: &str) -> Action {
    let input = input.trim();
    if input.is_empty() {
        return Action::Noop;
    }
    // Check for force flag.
    let (cmd, bang) = if input.ends_with('!') {
        (&input[..input.len() - 1], true)
    } else {
        (input, false)
    };
    // Split into command name and arguments.
    let (name, args) = split_cmd_args(cmd);
    match_command(name, args, bang)
}

fn split_cmd_args(s: &str) -> (&str, &str) {
    // Find boundary between command name and arguments.
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && bytes[i].is_ascii_alphabetic() {
        i += 1;
    }
    if i == 0 {
        // Numeric or special commands.
        return (s, "");
    }
    let name = &s[..i];
    let args = s[i..].trim_start();
    (name, args)
}

fn match_command(name: &str, args: &str, bang: bool) -> Action {
    match name {
        // Quit commands.
        "q" | "quit" => {
            if bang { Action::ForceQuit } else { Action::Quit }
        }
        "qa" | "qall" => {
            if bang { Action::ForceQuit } else { Action::Quit }
        }
        "cq" => Action::ForceQuit,
        // Write commands.
        "w" | "write" => {
            if args.is_empty() {
                Action::Write
            } else {
                Action::Write // TODO: write to specific file
            }
        }
        "wa" | "wall" => Action::Write,
        "wq" => Action::WriteQuit,
        "x" | "xit" | "exi" | "exit" => Action::WriteQuit,
        // Edit/file commands.
        "e" | "edit" => {
            if args.is_empty() {
                Action::Noop // reload current
            } else {
                Action::OpenFile(args.to_string())
            }
        }
        // Buffer commands.
        "bn" | "bnext" => Action::NextBuffer,
        "bp" | "bprevious" | "bprev" => Action::PreviousBuffer,
        "bd" | "bdelete" => Action::DeleteBuffer,
        "b" | "buffer" => {
            if let Ok(n) = args.parse::<u64>() {
                Action::SwitchBuffer(
                    kjxlkj_core_types::BufferId(n),
                )
            } else {
                Action::Noop
            }
        }
        // Window commands.
        "sp" | "split" => {
            if args.is_empty() {
                Action::SplitHorizontal
            } else {
                Action::SplitHorizontal // TODO: open file
            }
        }
        "vsp" | "vsplit" => {
            if args.is_empty() {
                Action::SplitVertical
            } else {
                Action::SplitVertical // TODO: open file
            }
        }
        "clo" | "close" => Action::CloseWindow,
        "on" | "only" => Action::WindowOnly,
        "new" => Action::SplitHorizontal,
        "vnew" => Action::SplitVertical,
        // Explorer / terminal.
        "Explorer" | "Ex" => Action::OpenExplorer,
        "terminal" | "term" => Action::OpenTerminal,
        // Register display.
        "registers" | "reg" | "display" | "di" => Action::ShowRegisters,
        // Search highlight clear.
        "nohlsearch" | "noh" => Action::ClearSearchHighlight,
        // Set option.
        "set" | "se" => parse_set_option(args),
        "setlocal" => parse_set_option(args),
        _ => Action::Noop,
    }
}

fn parse_set_option(args: &str) -> Action {
    let args = args.trim();
    if args.is_empty() { return Action::Noop; }
    if let Some((k, v)) = args.split_once('=') {
        return Action::SetOption(k.to_string(), v.to_string());
    }
    if let Some(name) = args.strip_prefix("no") {
        return Action::SetOption(name.to_string(), "false".to_string());
    }
    Action::SetOption(args.to_string(), "true".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quit() {
        assert_eq!(parse_ex_command("q"), Action::Quit);
        assert_eq!(parse_ex_command("quit"), Action::Quit);
    }

    #[test]
    fn parse_force_quit() {
        assert_eq!(parse_ex_command("q!"), Action::ForceQuit);
    }

    #[test]
    fn parse_write() {
        assert_eq!(parse_ex_command("w"), Action::Write);
    }

    #[test]
    fn parse_wq() {
        assert_eq!(parse_ex_command("wq"), Action::WriteQuit);
    }

    #[test]
    fn parse_split() {
        assert_eq!(parse_ex_command("sp"), Action::SplitHorizontal);
        assert_eq!(parse_ex_command("vsp"), Action::SplitVertical);
    }

    #[test]
    fn parse_edit_file() {
        match parse_ex_command("e myfile.txt") {
            Action::OpenFile(f) => assert_eq!(f, "myfile.txt"),
            _ => panic!("expected OpenFile"),
        }
    }

    #[test]
    fn parse_buffer_cmds() {
        assert_eq!(parse_ex_command("bn"), Action::NextBuffer);
        assert_eq!(parse_ex_command("bp"), Action::PreviousBuffer);
    }

    #[test]
    fn parse_empty() {
        assert_eq!(parse_ex_command(""), Action::Noop);
        assert_eq!(parse_ex_command("  "), Action::Noop);
    }

    #[test]
    fn parse_registers() {
        assert_eq!(parse_ex_command("registers"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("reg"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("display"), Action::ShowRegisters);
        assert_eq!(parse_ex_command("di"), Action::ShowRegisters);
    }

    #[test]
    fn parse_nohlsearch() {
        assert_eq!(parse_ex_command("nohlsearch"), Action::ClearSearchHighlight);
        assert_eq!(parse_ex_command("noh"), Action::ClearSearchHighlight);
    }

    #[test]
    fn parse_set_option_forms() {
        assert_eq!(parse_ex_command("set ignorecase"), Action::SetOption("ignorecase".into(), "true".into()));
        assert_eq!(parse_ex_command("set noignorecase"), Action::SetOption("ignorecase".into(), "false".into()));
        assert_eq!(parse_ex_command("set tabstop=4"), Action::SetOption("tabstop".into(), "4".into()));
    }
}
