//! Command dispatch: parse and execute ex commands.

use kjxlkj_core_types::Action;

/// Parse and dispatch an ex command string.
///
/// Returns an action to execute, or None if the command is unknown.
pub fn dispatch_command(cmd: &str) -> Option<Action> {
    let cmd = cmd.trim();
    if cmd.is_empty() {
        return None;
    }

    // Split command into name and arguments.
    let (name, args) = split_command(cmd);
    let force = name.ends_with('!');
    let name_clean = if force {
        &name[..name.len() - 1]
    } else {
        &name
    };

    match name_clean {
        "q" | "quit" => {
            if force {
                Some(Action::ForceQuit)
            } else {
                Some(Action::Quit)
            }
        }
        "qa" | "qall" => Some(Action::QuitAll),
        "w" | "write" => {
            if args.is_empty() {
                Some(Action::Write)
            } else {
                Some(Action::Write)
            }
        }
        "wq" | "x" | "xit" => Some(Action::WriteQuit),
        "wa" | "wall" => Some(Action::WriteAll),
        "wqa" | "xall" => Some(Action::WriteAllQuit),
        "e" | "edit" => {
            if !args.is_empty() {
                Some(Action::OpenFile(
                    std::path::PathBuf::from(args),
                ))
            } else {
                None
            }
        }
        "bn" | "bnext" => Some(Action::NextBuffer),
        "bp" | "bprevious" | "bprev" => Some(Action::PrevBuffer),
        "bd" | "bdelete" => Some(Action::DeleteBuffer),
        "b" | "buffer" => {
            if !args.is_empty() {
                Some(Action::SwitchBuffer(args.to_string()))
            } else {
                None
            }
        }
        "sp" | "split" => Some(Action::SplitHorizontal),
        "vs" | "vsplit" => Some(Action::SplitVertical),
        "close" => Some(Action::CloseWindow),
        "terminal" | "term" => Some(Action::SpawnTerminal),
        "noh" | "nohlsearch" => Some(Action::Nop),
        "ls" | "buffers" | "files" => {
            // Buffer listing — Nop in this implementation;
            // a real impl would populate a scratch buffer.
            Some(Action::Nop)
        }
        "on" | "only" => Some(Action::Nop),
        "s" | "substitute" => {
            // :s/pat/repl/flags parsed and executed.
            Some(Action::Substitute(args.to_string()))
        }
        "g" | "global" => Some(Action::GlobalCommand(args.to_string())),
        "v" | "vglobal" => Some(Action::VglobalCommand(args.to_string())),
        "sort" => Some(Action::SortLines(args.to_string())),
        "set" => Some(Action::Nop),
        "reg" | "registers" => Some(Action::Nop),
        "marks" => Some(Action::Nop),
        "history" => Some(Action::Nop),
        "read" | "r" => Some(Action::Nop),
        "source" | "so" => Some(Action::Nop),
        "new" => Some(Action::SplitHorizontal),
        "vnew" => Some(Action::SplitVertical),
        "tab" | "tabnew" | "tabe" => {
            Some(Action::Nop)
        }
        "cq" | "cquit" => {
            Some(Action::ForceQuit)
        }
        "SessionSave" => Some(Action::SessionSave),
        "SessionLoad" => Some(Action::SessionLoad),
        _ => {
            // Check for line number command.
            if let Ok(line) = name.parse::<usize>() {
                Some(Action::MoveCursor(
                    kjxlkj_core_types::Motion::GotoLine(
                        line.saturating_sub(1),
                    ),
                    1,
                ))
            } else {
                None
            }
        }
    }
}

fn split_command(cmd: &str) -> (String, &str) {
    let cmd = cmd.trim();

    // Handle line-number-only commands.
    if cmd.chars().next().map_or(false, |c| c.is_ascii_digit()) {
        let end = cmd
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(cmd.len());
        let (num, rest) = cmd.split_at(end);
        return (num.to_string(), rest.trim());
    }

    // Find end of command name: whitespace or `/` (for :s/pat/).
    let name_end = cmd
        .find(|c: char| c.is_whitespace() || c == '/')
        .unwrap_or(cmd.len());
    let (name, rest) = cmd.split_at(name_end);
    (name.to_string(), rest.trim_start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quit_command() {
        assert!(matches!(dispatch_command("q"), Some(Action::Quit)));
        assert!(matches!(
            dispatch_command("q!"),
            Some(Action::ForceQuit)
        ));
    }

    #[test]
    fn write_command() {
        assert!(matches!(dispatch_command("w"), Some(Action::Write)));
    }

    #[test]
    fn edit_command() {
        let action = dispatch_command("e test.txt");
        assert!(matches!(action, Some(Action::OpenFile(_))));
    }

    #[test]
    fn line_number_command() {
        let action = dispatch_command("42");
        assert!(matches!(
            action,
            Some(Action::MoveCursor(
                kjxlkj_core_types::Motion::GotoLine(41),
                1
            ))
        ));
    }

    #[test]
    fn substitute_command() {
        let action = dispatch_command("s/foo/bar/g");
        assert!(matches!(
            action,
            Some(Action::Substitute(_))
        ));
    }

    #[test]
    fn unknown_command() {
        assert!(dispatch_command("foobar").is_none());
    }
}
