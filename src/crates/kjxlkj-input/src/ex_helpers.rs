//! Helper functions for Ex command parsing.

use crate::ex_command::{BufferCommand, BufferTarget, ExCommand};
use std::path::PathBuf;

/// Parses a write command.
pub fn parse_write(args: &str, force: bool) -> ExCommand {
    ExCommand::Write {
        path: parse_path(args),
        force,
        all: false,
    }
}

/// Parses a write-quit command.
pub fn parse_wq(args: &str, force: bool) -> ExCommand {
    ExCommand::WriteQuit {
        path: parse_path(args),
        force,
        all: false,
    }
}

/// Parses an edit command.
pub fn parse_edit(args: &str, force: bool) -> ExCommand {
    if args.is_empty() {
        ExCommand::NewBuffer
    } else {
        ExCommand::Edit {
            path: PathBuf::from(args),
            force,
        }
    }
}

/// Parses a path argument.
pub fn parse_path(args: &str) -> Option<PathBuf> {
    if args.is_empty() {
        None
    } else {
        Some(PathBuf::from(args))
    }
}

/// Parses a buffer command.
pub fn parse_buffer(args: &str) -> ExCommand {
    let target = if args.is_empty() {
        BufferTarget::Current
    } else if let Ok(n) = args.parse::<usize>() {
        BufferTarget::Number(n)
    } else {
        BufferTarget::Name(args.to_string())
    };
    ExCommand::Buffer(BufferCommand::Go { target })
}

/// Parses a buffer delete command.
pub fn parse_bdelete(args: &str, force: bool) -> ExCommand {
    let target = if args.is_empty() {
        BufferTarget::Current
    } else if let Ok(n) = args.parse::<usize>() {
        BufferTarget::Number(n)
    } else {
        BufferTarget::Name(args.to_string())
    };
    ExCommand::Buffer(BufferCommand::Delete { target, force })
}

/// Parses a set command.
pub fn parse_set(args: &str) -> ExCommand {
    if let Some((opt, val)) = args.split_once('=') {
        ExCommand::Set {
            option: opt.trim().to_string(),
            value: Some(val.trim().to_string()),
        }
    } else {
        ExCommand::Set {
            option: args.to_string(),
            value: None,
        }
    }
}

/// Parses a substitute command.
pub fn parse_substitute(args: &str, range: Option<String>) -> ExCommand {
    if let Some(rest) = args.strip_prefix('/') {
        let parts: Vec<&str> = rest.splitn(3, '/').collect();
        if parts.len() >= 2 {
            return ExCommand::Substitute {
                pattern: parts[0].to_string(),
                replacement: parts[1].to_string(),
                flags: parts.get(2).unwrap_or(&"").to_string(),
                range,
            };
        }
    }
    ExCommand::Unknown {
        line: format!("s{}", args),
    }
}

/// Parses a global command.
pub fn parse_global(args: &str, inverse: bool) -> ExCommand {
    if let Some(rest) = args.strip_prefix('/') {
        if let Some((pattern, cmd)) = rest.split_once('/') {
            return ExCommand::Global {
                pattern: pattern.to_string(),
                command: cmd.to_string(),
                inverse,
            };
        }
    }
    ExCommand::Unknown {
        line: format!("g{}", args),
    }
}

/// Parses a grep command.
pub fn parse_grep(args: &str) -> ExCommand {
    let parts: Vec<&str> = args.splitn(2, ' ').collect();
    ExCommand::Grep {
        pattern: parts.first().unwrap_or(&"").to_string(),
        path: parts.get(1).map(|s| s.to_string()),
    }
}
