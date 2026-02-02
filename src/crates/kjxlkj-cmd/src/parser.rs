//! Ex command parser.

use crate::{Command, CommandKind, Range};
use crate::command::{BufferArg, SetArg, SubstituteArgs, SubstituteFlags};
use std::path::PathBuf;

pub use crate::parser_types::ParseError;

/// Parser for Ex commands.
#[derive(Debug, Default)]
pub struct CommandParser {
    input: String,
    pos: usize,
}

impl CommandParser {
    /// Creates a new parser.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses a command string.
    pub fn parse(&mut self, input: &str) -> Result<Command, ParseError> {
        self.input = input.to_string();
        self.pos = 0;
        self.skip_whitespace();

        // Parse optional range
        let range = self.parse_range()?;

        // Parse command
        let (name, force) = self.parse_command_name();
        self.skip_whitespace();
        let args = self.remaining();

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

    fn parse_range(&mut self) -> Result<Option<Range>, ParseError> {
        let start = self.pos;
        
        if self.peek() == Some('%') {
            self.advance();
            return Ok(Some(Range::All));
        }
        
        if self.peek() == Some('.') {
            self.advance();
            if self.peek() == Some(',') {
                self.advance();
                if self.peek() == Some('$') {
                    self.advance();
                    return Ok(Some(Range::All));
                }
            }
            return Ok(Some(Range::Current));
        }

        if self.peek() == Some('$') {
            self.advance();
            return Ok(Some(Range::Last));
        }

        // Try to parse line number
        if let Some(n) = self.parse_number() {
            if self.peek() == Some(',') {
                self.advance();
                if let Some(m) = self.parse_number() {
                    return Ok(Some(Range::FromTo(n.saturating_sub(1), m.saturating_sub(1))));
                }
            }
            return Ok(Some(Range::Line(n.saturating_sub(1))));
        }

        self.pos = start;
        Ok(None)
    }

    fn parse_command_name(&mut self) -> (String, bool) {
        let mut name = String::new();
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '!' {
                name.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let force = name.ends_with('!');
        if force {
            name.pop();
        }
        (name, force)
    }

    fn resolve_command(&self, name: &str, args: &str) -> Result<CommandKind, ParseError> {
        match name {
            "w" | "write" => {
                if args.is_empty() {
                    Ok(CommandKind::Write(None))
                } else {
                    Ok(CommandKind::Write(Some(PathBuf::from(args))))
                }
            }
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
            "b" | "buffer" => {
                if args.is_empty() {
                    Err(ParseError::MissingArgument("buffer"))
                } else if let Ok(n) = args.parse::<usize>() {
                    Ok(CommandKind::Buffer(BufferArg::Number(n)))
                } else {
                    Ok(CommandKind::Buffer(BufferArg::Name(args.to_string())))
                }
            }
            "bn" | "bnext" => Ok(CommandKind::BufferNext),
            "bp" | "bprev" => Ok(CommandKind::BufferPrev),
            "bd" | "bdelete" => Ok(CommandKind::BufferDelete),
            "bw" | "bwipe" => Ok(CommandKind::BufferWipe),
            "sp" | "split" => {
                if args.is_empty() {
                    Ok(CommandKind::Split(None))
                } else {
                    Ok(CommandKind::Split(Some(PathBuf::from(args))))
                }
            }
            "vsp" | "vsplit" => {
                if args.is_empty() {
                    Ok(CommandKind::VSplit(None))
                } else {
                    Ok(CommandKind::VSplit(Some(PathBuf::from(args))))
                }
            }
            "clo" | "close" => Ok(CommandKind::Close),
            "on" | "only" => Ok(CommandKind::Only),
            "set" => self.parse_set_arg(args),
            "d" | "delete" => Ok(CommandKind::Delete),
            "y" | "yank" => Ok(CommandKind::Yank),
            "pu" | "put" => Ok(CommandKind::Put),
            "s" | "substitute" => self.parse_substitute(args),
            "" => Ok(CommandKind::Noop),
            _ => Err(ParseError::UnknownCommand(name.to_string())),
        }
    }

    fn parse_set_arg(&self, args: &str) -> Result<CommandKind, ParseError> {
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

    fn parse_substitute(&self, args: &str) -> Result<CommandKind, ParseError> {
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
        Ok(CommandKind::Substitute(SubstituteArgs { pattern, replacement, flags }))
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_whitespace()) {
            self.advance();
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos += c.len_utf8();
        }
    }

    fn parse_number(&mut self) -> Option<usize> {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.pos > start {
            self.input[start..self.pos].parse().ok()
        } else {
            None
        }
    }

    fn remaining(&self) -> String {
        self.input[self.pos..].trim().to_string()
    }
}
