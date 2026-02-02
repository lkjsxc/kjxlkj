//! Command registry for completion sources.

use std::collections::HashMap;

use crate::completion_types::CompletionSource;

/// Command registry for completions.
#[derive(Debug, Default)]
pub struct CommandRegistry {
    commands: HashMap<String, CompletionSource>,
}

impl CommandRegistry {
    /// Creates a new registry with built-in commands.
    pub fn new() -> Self {
        let mut reg = Self::default();
        reg.register_builtins();
        reg
    }

    /// Registers built-in commands.
    fn register_builtins(&mut self) {
        let file_cmds = ["e", "edit", "w", "write", "saveas", "r", "read"];
        let buffer_cmds = ["b", "buffer", "bd", "bdelete", "bn", "bp"];
        let option_cmds = ["set", "setlocal", "setglobal"];
        let help_cmds = ["h", "help"];
        let dir_cmds = ["cd", "lcd", "tcd", "pwd"];

        for cmd in file_cmds {
            self.commands.insert(cmd.to_string(), CompletionSource::File);
        }
        for cmd in buffer_cmds {
            self.commands.insert(cmd.to_string(), CompletionSource::Buffer);
        }
        for cmd in option_cmds {
            self.commands.insert(cmd.to_string(), CompletionSource::Option);
        }
        for cmd in help_cmds {
            self.commands.insert(cmd.to_string(), CompletionSource::Help);
        }
        for cmd in dir_cmds {
            self.commands.insert(cmd.to_string(), CompletionSource::Directory);
        }
    }

    /// Gets the completion source for a command.
    pub fn get_source(&self, command: &str) -> Option<CompletionSource> {
        self.commands.get(command).copied()
    }

    /// Registers a custom command.
    pub fn register(&mut self, command: &str, source: CompletionSource) {
        self.commands.insert(command.to_string(), source);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_builtin() {
        let reg = CommandRegistry::new();
        assert_eq!(reg.get_source("edit"), Some(CompletionSource::File));
        assert_eq!(reg.get_source("buffer"), Some(CompletionSource::Buffer));
        assert_eq!(reg.get_source("set"), Some(CompletionSource::Option));
    }

    #[test]
    fn test_registry_custom() {
        let mut reg = CommandRegistry::new();
        reg.register("mycommand", CompletionSource::Custom);
        assert_eq!(reg.get_source("mycommand"), Some(CompletionSource::Custom));
    }
}
