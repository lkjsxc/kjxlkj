//! Ex command dispatch.

/// Result of parsing and executing an ex command.
#[allow(dead_code)]
pub enum ExResult {
    /// Write the current buffer.
    Write(Option<String>),
    /// Quit the current window.
    Quit,
    /// Force quit.
    ForceQuit,
    /// Write and quit.
    WriteQuit,
    /// Quit all windows.
    QuitAll,
    /// Force quit all.
    ForceQuitAll,
    /// Edit a file.
    Edit(String),
    /// Split horizontally.
    Split(Option<String>),
    /// Split vertically.
    VSplit(Option<String>),
    /// Next buffer.
    BNext,
    /// Previous buffer.
    BPrev,
    /// Delete buffer.
    BDelete,
    /// Set option.
    Set(String),
    /// Open terminal.
    Terminal,
    /// Open explorer.
    Explorer,
    /// Show message.
    Message(String),
    /// Error message.
    Error(String),
    /// No-op (empty command).
    Noop,
}

/// Parse an ex command string into an ExResult.
pub fn parse_ex_command(input: &str) -> ExResult {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return ExResult::Noop;
    }

    // Split command and arguments
    let (cmd, args) = match trimmed.find(|c: char| c.is_whitespace()) {
        Some(pos) => {
            let (c, a) = trimmed.split_at(pos);
            (c, a.trim())
        }
        None => (trimmed, ""),
    };

    match cmd {
        "w" | "write" => {
            if args.is_empty() {
                ExResult::Write(None)
            } else {
                ExResult::Write(Some(args.to_string()))
            }
        }
        "q" | "quit" => ExResult::Quit,
        "q!" | "quit!" => ExResult::ForceQuit,
        "wq" => ExResult::WriteQuit,
        "x" | "xit" => ExResult::WriteQuit,
        "qa" | "qall" => ExResult::QuitAll,
        "qa!" | "qall!" => ExResult::ForceQuitAll,
        "e" | "edit" => {
            if args.is_empty() {
                ExResult::Error("No file name".to_string())
            } else {
                ExResult::Edit(args.to_string())
            }
        }
        "sp" | "split" => {
            if args.is_empty() {
                ExResult::Split(None)
            } else {
                ExResult::Split(Some(args.to_string()))
            }
        }
        "vsp" | "vsplit" => {
            if args.is_empty() {
                ExResult::VSplit(None)
            } else {
                ExResult::VSplit(Some(args.to_string()))
            }
        }
        "bn" | "bnext" => ExResult::BNext,
        "bp" | "bprevious" => ExResult::BPrev,
        "bd" | "bdelete" => ExResult::BDelete,
        "set" => ExResult::Set(args.to_string()),
        "terminal" => ExResult::Terminal,
        "Explorer" => ExResult::Explorer,
        _ => ExResult::Error(format!("Not an editor command: {cmd}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_write() {
        assert!(matches!(parse_ex_command("w"), ExResult::Write(None)));
        assert!(matches!(
            parse_ex_command("write foo.txt"),
            ExResult::Write(Some(_))
        ));
    }

    #[test]
    fn test_parse_quit() {
        assert!(matches!(parse_ex_command("q"), ExResult::Quit));
        assert!(matches!(parse_ex_command("q!"), ExResult::ForceQuit));
    }

    #[test]
    fn test_parse_edit() {
        assert!(matches!(parse_ex_command("e test.rs"), ExResult::Edit(_)));
    }

    #[test]
    fn test_parse_unknown() {
        assert!(matches!(parse_ex_command("foobar"), ExResult::Error(_)));
    }
}
