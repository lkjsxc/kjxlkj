//! User-defined ex commands (`:command`, `:delcommand`, `:comclear`).
//!
//! Supports `-nargs`, `-range`, `-bang`, `-complete` flags.
//! Command names must start with an uppercase letter.

use std::collections::HashMap;

/// Nargs constraint for user commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Nargs {
    /// No arguments (default).
    #[default]
    Zero,
    /// Exactly one argument.
    One,
    /// Any number of arguments.
    Any,
    /// Zero or one argument.
    Optional,
    /// One or more arguments.
    OneOrMore,
}

/// Range mode for user commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RangeMode {
    /// No range accepted.
    #[default]
    None,
    /// Range accepted, default is current line.
    CurrentLine,
    /// Range accepted, default is whole file.
    WholeFile,
    /// Count accepted with default value.
    Count(usize),
}

/// Completion type for user command arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionType {
    File,
    Dir,
    Buffer,
    Command,
    Option,
    Color,
    Help,
    Custom(String),
}

/// A single user-defined command.
#[derive(Debug, Clone)]
pub struct UserCommand {
    /// Command name (must start with uppercase).
    pub name: String,
    /// Replacement text (command body).
    pub replacement: String,
    /// Number of arguments.
    pub nargs: Nargs,
    /// Range mode.
    pub range: RangeMode,
    /// Whether `:command!` was used (allows overwrite).
    pub bang: bool,
    /// Completion type, if specified.
    pub complete: Option<CompletionType>,
}

/// Registry of user-defined commands.
#[derive(Debug, Clone, Default)]
pub struct UserCommandRegistry {
    commands: HashMap<String, UserCommand>,
}

impl UserCommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Define a user command. Returns Err if name exists and overwrite is false.
    pub fn define(&mut self, cmd: UserCommand, overwrite: bool) -> Result<(), String> {
        if !cmd.name.starts_with(|c: char| c.is_ascii_uppercase()) {
            return Err(format!(
                "E183: User command must begin with uppercase: {}",
                cmd.name
            ));
        }
        if !overwrite && self.commands.contains_key(&cmd.name) {
            return Err(format!("E174: Command already exists: {}", cmd.name));
        }
        self.commands.insert(cmd.name.clone(), cmd);
        Ok(())
    }

    /// Remove a user command by name.
    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.commands.remove(name).is_none() {
            Err(format!("E184: No such user command: {name}"))
        } else {
            Ok(())
        }
    }

    /// Remove all user commands.
    pub fn clear(&mut self) {
        self.commands.clear();
    }

    /// Look up a user command by name.
    pub fn get(&self, name: &str) -> Option<&UserCommand> {
        self.commands.get(name)
    }

    /// List all user commands.
    pub fn list(&self) -> Vec<&UserCommand> {
        let mut cmds: Vec<_> = self.commands.values().collect();
        cmds.sort_by(|a, b| a.name.cmp(&b.name));
        cmds
    }

    /// Expand a user command invocation with arguments.
    pub fn expand(&self, name: &str, args: &str, bang: bool) -> Option<String> {
        let cmd = self.commands.get(name)?;
        let mut result = cmd.replacement.clone();
        result = result.replace("<args>", args);
        result = result.replace("<q-args>", &format!("\"{}\"", args.replace('"', "\\\"")));
        result = result.replace("<bang>", if bang { "!" } else { "" });
        Some(result)
    }

    /// Check if a command name exists.
    pub fn contains(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Number of user commands.
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Whether registry is empty.
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
}
