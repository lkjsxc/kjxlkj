//! Ex command parser.

use crate::command::{BufferArg, SetArg, SubstituteArgs, SubstituteFlags};
use crate::parser_helpers::ParserState;
use crate::{Command, CommandKind, Range};
use std::path::PathBuf;

pub use crate::parser_types::ParseError;

/// Parser for Ex commands.
#[derive(Debug, Default)]
pub struct CommandParser {
    state: ParserState,
}

impl CommandParser {
    /// Creates a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a command string.
    pub fn parse(&mut self, input: &str) -> Result<Command, ParseError> {
        self.state = ParserState::new(input);
        self.state.skip_whitespace();

        let range = self.state.parse_range();
        let (name, force) = self.state.parse_command_name();
        self.state.skip_whitespace();
        let args = self.state.remaining();

        let kind = self.resolve_command(&name, &args)?;
        let mut cmd = Command::new(kind);
        if let Some(r) = range {
            cmd = cmd.with_range(r);
        }
        if force {
            cmd = cmd.with_force();
        }

        Ok(cmd)
    }

    fn resolve_command(&self, name: &str, args: &str) -> Result<CommandKind, ParseError> {
        match name {
            "w" | "write" => Ok(CommandKind::Write(parse_optional_path(args))),
            "wa" | "wall" => Ok(CommandKind::WriteAll),
            "e" | "edit" => {
                if args.is_empty() {
                    Err(ParseError::MissingArgument("filename"))
                } else {
                    Ok(CommandKind::Edit(PathBuf::from(args)))
                }
            }
            "q" | "quit" => Ok(CommandKind::Quit),
            "qa" | "qall" => Ok(CommandKind::QuitAll),
            "wq" | "x" | "exit" => Ok(CommandKind::WriteQuit),
            "ls" | "buffers" => Ok(CommandKind::ListBuffers),
            "b" | "buffer" => parse_buffer_arg(args),
            "bn" | "bnext" => Ok(CommandKind::BufferNext),
            "bp" | "bprev" => Ok(CommandKind::BufferPrev),
            "bd" | "bdelete" => Ok(CommandKind::BufferDelete),
            "bw" | "bwipe" => Ok(CommandKind::BufferWipe),
            "sp" | "split" => Ok(CommandKind::Split(parse_optional_path(args))),
            "vsp" | "vsplit" => Ok(CommandKind::VSplit(parse_optional_path(args))),
            "clo" | "close" => Ok(CommandKind::Close),
            "on" | "only" => Ok(CommandKind::Only),
            "set" => parse_set_arg(args),
            "d" | "delete" => Ok(CommandKind::Delete),
            "y" | "yank" => Ok(CommandKind::Yank),
            "pu" | "put" => Ok(CommandKind::Put),
            "s" | "substitute" => parse_substitute(args),
            "" => Ok(CommandKind::Noop),
            _ => Err(ParseError::UnknownCommand(name.to_string())),
        }
    }
}

fn parse_optional_path(args: &str) -> Option<PathBuf> {
    if args.is_empty() {
        None
    } else {
        Some(PathBuf::from(args))
    }
}

fn parse_buffer_arg(args: &str) -> Result<CommandKind, ParseError> {
    if args.is_empty() {
        Err(ParseError::MissingArgument("buffer"))
    } else if let Ok(n) = args.parse::<usize>() {
        Ok(CommandKind::Buffer(BufferArg::Number(n)))
    } else {
        Ok(CommandKind::Buffer(BufferArg::Name(args.to_string())))
    }
}

fn parse_set_arg(args: &str) -> Result<CommandKind, ParseError> {
    let args = args.trim();
    if args.is_empty() {
        return Err(ParseError::MissingArgument("option"));
    }
    if args.ends_with('?') {
        let opt = args.trim_end_matches('?');
        return Ok(CommandKind::Set(SetArg::Query(opt.to_string())));
    }
    if args.starts_with("no") {
        return Ok(CommandKind::Set(SetArg::Disable(args[2..].to_string())));
    }
    if let Some(eq_pos) = args.find('=') {
        let opt = &args[..eq_pos];
        let val = &args[eq_pos + 1..];
        return Ok(CommandKind::Set(SetArg::Value(opt.to_string(), val.to_string())));
    }
    Ok(CommandKind::Set(SetArg::Enable(args.to_string())))
}

fn parse_substitute(args: &str) -> Result<CommandKind, ParseError> {
    if args.is_empty() || !args.starts_with('/') {
        return Err(ParseError::InvalidSubstitute);
    }
    let parts: Vec<&str> = args[1..].split('/').collect();
    if parts.len() < 2 {
        return Err(ParseError::InvalidSubstitute);
    }
    let pattern = parts[0].to_string();
    let replacement = parts[1].to_string();
    let mut flags = SubstituteFlags::default();
    if parts.len() > 2 {
        for c in parts[2].chars() {
            match c {
                'g' => flags.global = true,
                'i' => flags.ignore_case = true,
                'c' => flags.confirm = true,
                _ => {}
            }
        }
    }
    Ok(CommandKind::Substitute(SubstituteArgs {
        pattern,
        replacement,
        flags,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_write() {
        let mut p = CommandParser::new();
        let cmd = p.parse("w").unwrap();
        assert!(matches!(&cmd.kind, CommandKind::Write(None)));
    }

    #[test]
    fn test_parse_edit() {
        let mut p = CommandParser::new();
        let cmd = p.parse("e foo.txt").unwrap();
        if let CommandKind::Edit(path) = &cmd.kind {
            assert_eq!(path, &PathBuf::from("foo.txt"));
        } else {
            panic!("Expected Edit command");
        }
    }

    #[test]
    fn test_parse_substitute() {
        let mut p = CommandParser::new();
        let cmd = p.parse("s/foo/bar/g").unwrap();
        if let CommandKind::Substitute(args) = &cmd.kind {
            assert_eq!(args.pattern, "foo");
            assert_eq!(args.replacement, "bar");
            assert!(args.flags.global);
        } else {
            panic!("Expected Substitute command");
        }
    }

    #[test]
    fn test_parse_set_enable() {
        let mut p = CommandParser::new();
        let cmd = p.parse("set number").unwrap();
        if let CommandKind::Set(SetArg::Enable(opt)) = &cmd.kind {
            assert_eq!(opt, "number");
        } else {
            panic!("Expected Set Enable");
        }
    }

    #[test]
    fn test_parse_range() {
        let mut p = CommandParser::new();
        let cmd = p.parse("%s/a/b/").unwrap();
        assert_eq!(cmd.range, Some(Range::All));
    }
}
