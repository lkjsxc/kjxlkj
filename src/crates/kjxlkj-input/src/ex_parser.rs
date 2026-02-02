//! Parser for Ex (command-line) commands.

use crate::ex_command::{BufferCommand, ExCommand, TabCommand, WindowCommand};
use crate::ex_helpers::{
    parse_bdelete, parse_buffer, parse_edit, parse_global, parse_grep, parse_path, parse_set,
    parse_substitute, parse_wq, parse_write,
};
use std::path::PathBuf;

/// Parser for Ex commands.
#[derive(Debug, Default)]
pub struct ExParser;

impl ExParser {
    /// Creates a new parser.
    pub fn new() -> Self {
        Self
    }

    /// Parses an Ex command string.
    pub fn parse(&self, line: &str) -> ExCommand {
        let line = line.trim();
        if line.is_empty() {
            return ExCommand::Unknown {
                line: String::new(),
            };
        }

        // Handle range prefix (we'll skip it for now)
        let (range, cmd) = self.split_range(line);

        // Parse the command
        self.parse_command(cmd, range)
    }

    fn split_range<'a>(&self, line: &'a str) -> (Option<String>, &'a str) {
        // Simple range detection (%, ., $, numbers, etc.)
        // For now, just look for common patterns
        let line = line.trim();
        if let Some(rest) = line.strip_prefix('%') {
            (Some("%".to_string()), rest.trim())
        } else if line.starts_with('.') || line.starts_with('$') {
            // More complex range parsing would go here
            (None, line)
        } else {
            (None, line)
        }
    }

    fn parse_command(&self, line: &str, range: Option<String>) -> ExCommand {
        // Extract command name and arguments
        let (cmd, args) = self.split_cmd_args(line);
        let cmd = cmd.to_lowercase();

        match cmd.as_str() {
            // Write commands
            "w" | "write" => parse_write(args, false),
            "w!" | "write!" => parse_write(args, true),
            "wa" | "wall" => ExCommand::Write {
                path: None,
                force: false,
                all: true,
            },
            "wa!" | "wall!" => ExCommand::Write {
                path: None,
                force: true,
                all: true,
            },

            // Quit commands
            "q" | "quit" => ExCommand::Quit {
                force: false,
                all: false,
            },
            "q!" | "quit!" => ExCommand::Quit {
                force: true,
                all: false,
            },
            "qa" | "qall" => ExCommand::Quit {
                force: false,
                all: true,
            },
            "qa!" | "qall!" => ExCommand::Quit {
                force: true,
                all: true,
            },

            // Write and quit
            "wq" => parse_wq(args, false),
            "wq!" => parse_wq(args, true),
            "x" | "xit" => parse_wq(args, false),
            "x!" | "xit!" => parse_wq(args, true),
            "wqa" | "wqall" => ExCommand::WriteQuit {
                path: None,
                force: false,
                all: true,
            },
            "xa" | "xall" => ExCommand::WriteQuit {
                path: None,
                force: false,
                all: true,
            },

            // Edit
            "e" | "edit" => parse_edit(args, false),
            "e!" | "edit!" => parse_edit(args, true),
            "enew" => ExCommand::NewBuffer,

            // Buffer commands
            "ls" | "buffers" => ExCommand::Buffer(BufferCommand::List { all: false }),
            "ls!" | "buffers!" => ExCommand::Buffer(BufferCommand::List { all: true }),
            "b" | "buffer" => parse_buffer(args),
            "bd" | "bdelete" => parse_bdelete(args, false),
            "bd!" | "bdelete!" => parse_bdelete(args, true),
            "bn" | "bnext" => ExCommand::Buffer(BufferCommand::Next),
            "bp" | "bprev" | "bprevious" => ExCommand::Buffer(BufferCommand::Previous),
            "bf" | "bfirst" => ExCommand::Buffer(BufferCommand::First),
            "bl" | "blast" => ExCommand::Buffer(BufferCommand::Last),

            // Window commands
            "sp" | "split" => ExCommand::Window(WindowCommand::SplitHorizontal {
                path: parse_path(args),
            }),
            "vs" | "vsplit" => ExCommand::Window(WindowCommand::SplitVertical {
                path: parse_path(args),
            }),
            "close" | "clo" => ExCommand::Window(WindowCommand::Close { force: false }),
            "close!" | "clo!" => ExCommand::Window(WindowCommand::Close { force: true }),
            "only" | "on" => ExCommand::Window(WindowCommand::Only),
            "new" => ExCommand::Window(WindowCommand::New),
            "vnew" => ExCommand::Window(WindowCommand::VerticalNew),

            // Tab commands
            "tabnew" | "tabe" | "tabedit" => ExCommand::Tab(TabCommand::New {
                path: parse_path(args),
            }),
            "tabclose" | "tabc" => ExCommand::Tab(TabCommand::Close { force: false }),
            "tabclose!" | "tabc!" => ExCommand::Tab(TabCommand::Close { force: true }),
            "tabn" | "tabnext" => ExCommand::Tab(TabCommand::Next),
            "tabp" | "tabprev" | "tabprevious" => ExCommand::Tab(TabCommand::Previous),
            "tabo" | "tabonly" => ExCommand::Tab(TabCommand::Only),

            // Set
            "set" | "se" => parse_set(args),

            // Substitute
            "s" | "substitute" => parse_substitute(args, range),

            // Global
            "g" | "global" => parse_global(args, false),
            "v" | "vglobal" => parse_global(args, true),

            // Normal
            "norm" | "normal" => ExCommand::Normal {
                keys: args.to_string(),
                range,
            },

            // Help and info
            "h" | "help" => ExCommand::Help {
                topic: if args.is_empty() {
                    None
                } else {
                    Some(args.to_string())
                },
            },
            "version" | "ver" => ExCommand::Version,
            "marks" => ExCommand::Marks {
                filter: if args.is_empty() {
                    None
                } else {
                    Some(args.to_string())
                },
            },
            "reg" | "registers" => ExCommand::Registers {
                filter: if args.is_empty() {
                    None
                } else {
                    Some(args.to_string())
                },
            },
            "jumps" | "ju" => ExCommand::Jumps,
            "changes" => ExCommand::Changes,
            "history" | "his" => ExCommand::History {
                kind: if args.is_empty() {
                    None
                } else {
                    Some(args.to_string())
                },
            },

            // Source
            "so" | "source" => ExCommand::Source {
                path: PathBuf::from(args),
            },

            // Colorscheme
            "colo" | "colorscheme" => ExCommand::Colorscheme {
                name: args.to_string(),
            },

            // Make and grep
            "make" | "mak" => ExCommand::Make {
                args: args.to_string(),
            },
            "grep" | "gr" => parse_grep(args),

            // Shell
            "!" => ExCommand::Shell {
                command: args.to_string(),
            },

            // Read
            "r" | "read" => ExCommand::Read {
                source: args.to_string(),
            },

            // Lua
            "lua" => ExCommand::Lua {
                code: args.to_string(),
            },

            // Unknown
            _ => ExCommand::Unknown {
                line: line.to_string(),
            },
        }
    }

    fn split_cmd_args<'a>(&self, line: &'a str) -> (&'a str, &'a str) {
        if let Some(idx) = line.find(' ') {
            (&line[..idx], line[idx..].trim())
        } else {
            (line, "")
        }
    }
}
