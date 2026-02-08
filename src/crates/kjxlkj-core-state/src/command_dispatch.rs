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

    // Shell command: `!{cmd}`
    if cmd.starts_with('!') {
        let shell_cmd = &cmd[1..];
        return Some(Action::ShellCommand(
            shell_cmd.to_string(),
        ));
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
        "sp" | "split" => {
            if !args.is_empty() {
                Some(Action::SplitOpen(args.to_string()))
            } else {
                Some(Action::SplitHorizontal)
            }
        }
        "vs" | "vsplit" => {
            if !args.is_empty() {
                Some(Action::VsplitOpen(args.to_string()))
            } else {
                Some(Action::SplitVertical)
            }
        }
        "close" => Some(Action::CloseWindow),
        "on" | "only" => Some(Action::OnlyWindow),
        "hide" => Some(Action::HideWindow),
        "terminal" | "term" => Some(Action::SpawnTerminal),
        "noh" | "nohlsearch" => Some(Action::Nop),
        "ls" | "buffers" | "files" => {
            // Buffer listing — Nop in this implementation;
            // a real impl would populate a scratch buffer.
            Some(Action::Nop)
        }
        "s" | "substitute" => {
            // :s/pat/repl/flags parsed and executed.
            Some(Action::Substitute(args.to_string()))
        }
        "g" | "global" => Some(Action::GlobalCommand(args.to_string())),
        "v" | "vglobal" => Some(Action::VglobalCommand(args.to_string())),
        "sort" => Some(Action::SortLines(args.to_string())),
        "d" | "delete" => {
            Some(Action::RangeDelete(
                full_with_range(cmd),
            ))
        }
        "y" | "yank" => {
            Some(Action::RangeYank(
                full_with_range(cmd),
            ))
        }
        "t" | "copy" => {
            Some(Action::RangeCopy(
                full_with_range(cmd),
            ))
        }
        "m" | "move" => {
            Some(Action::RangeMove(
                full_with_range(cmd),
            ))
        }
        "normal" | "norm" => {
            Some(Action::RangeNormal(
                full_with_range(cmd),
            ))
        }
        "set" => Some(Action::Nop),
        "reg" | "registers" => Some(Action::Nop),
        "marks" => Some(Action::Nop),
        "history" => Some(Action::Nop),
        "read" | "r" => {
            if !args.is_empty() {
                Some(Action::ReadFile(args.to_string()))
            } else {
                Some(Action::Nop)
            }
        }
        "source" | "so" => Some(Action::Nop),
        "filetype" | "ft" => Some(Action::Nop),
        "autocmd" | "au" => Some(Action::Nop),
        "cnext" | "cn" => Some(Action::Nop),
        "cprev" | "cp" | "cprevious" => Some(Action::Nop),
        "cfirst" | "cfir" => Some(Action::Nop),
        "clast" | "cla" => Some(Action::Nop),
        "copen" => Some(Action::Nop),
        "cclose" | "ccl" => Some(Action::Nop),
        "grep" | "vimgrep" => {
            if !args.is_empty() {
                Some(Action::ShellCommand(
                    format!("grep -rn {}", args),
                ))
            } else {
                Some(Action::Nop)
            }
        }
        "make" => {
            Some(Action::ShellCommand(
                if args.is_empty() {
                    "make".to_string()
                } else {
                    format!("make {}", args)
                },
            ))
        }
        "new" => {
            if !args.is_empty() {
                Some(Action::SplitOpen(args.to_string()))
            } else {
                Some(Action::NewSplit)
            }
        }
        "vnew" => {
            if !args.is_empty() {
                Some(Action::VsplitOpen(args.to_string()))
            } else {
                Some(Action::NewVsplit)
            }
        }
        "tab" | "tabnew" | "tabe" => {
            Some(Action::Nop)
        }
        "resize" => {
            Some(Action::ResizeCmd(args.to_string()))
        }
        "vertical" => {
            // `:vertical resize N` etc.
            if args.starts_with("resize") {
                let rest = args
                    .strip_prefix("resize")
                    .unwrap_or("")
                    .trim();
                Some(Action::ResizeCmd(
                    format!("v {}", rest),
                ))
            } else if args.starts_with("split") {
                let rest = args
                    .strip_prefix("split")
                    .unwrap_or("")
                    .trim();
                if rest.is_empty() {
                    Some(Action::SplitVertical)
                } else {
                    Some(Action::VsplitOpen(
                        rest.to_string(),
                    ))
                }
            } else if args.starts_with("new") {
                Some(Action::NewVsplit)
            } else {
                Some(Action::Nop)
            }
        }
        "execute" | "exe" => {
            Some(Action::ExecuteExpr(args.to_string()))
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

/// Return the full command string (preserving range prefix).
fn full_with_range(cmd: &str) -> String {
    cmd.to_string()
}

fn split_command(cmd: &str) -> (String, &str) {
    let cmd = cmd.trim();

    // Skip range prefix: digits, commas, dots, $, %, '
    // to find the actual command name.
    let mut pos = 0;
    let bytes = cmd.as_bytes();
    while pos < bytes.len() {
        let b = bytes[pos];
        if b == b'%'
            || b == b'.'
            || b == b'$'
            || b == b','
            || b == b'\''
            || b.is_ascii_digit()
        {
            pos += 1;
        } else {
            break;
        }
    }

    // If we skipped some range prefix and there's still
    // an alpha command name, extract it.
    let range_end = pos;
    if range_end > 0 && range_end < cmd.len() {
        let rest = &cmd[range_end..];
        let name_end = rest
            .find(|c: char| {
                c.is_whitespace() || c == '/'
            })
            .unwrap_or(rest.len());
        let name = &rest[..name_end];
        if !name.is_empty()
            && name.chars().next().unwrap().is_alphabetic()
        {
            let args = rest[name_end..].trim_start();
            return (name.to_string(), args);
        }
    }

    // Handle line-number-only commands (no alpha after).
    if cmd
        .chars()
        .next()
        .map_or(false, |c| c.is_ascii_digit())
        && range_end == cmd.len()
    {
        let end = cmd
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(cmd.len());
        let (num, rest) = cmd.split_at(end);
        return (num.to_string(), rest.trim());
    }

    // Find end of command name: whitespace or `/`.
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

    #[test]
    fn range_delete_command() {
        let action = dispatch_command("1,5d");
        assert!(matches!(
            action,
            Some(Action::RangeDelete(_))
        ));
    }

    #[test]
    fn range_normal_command() {
        let action = dispatch_command("%normal @a");
        assert!(matches!(
            action,
            Some(Action::RangeNormal(_))
        ));
    }
}
