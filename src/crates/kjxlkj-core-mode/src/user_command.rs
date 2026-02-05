//! User-defined commands for kjxlkj editor.
//!
//! Implements user command registration and execution as specified in
//! `/docs/spec/scripting/user-commands.md`.
//!
//! This module provides:
//! - Command registration with custom behavior
//! - Argument parsing and validation
//! - Range and count support
//! - Command completion hints

use std::collections::HashMap;

/// Attributes for a user-defined command.
#[derive(Debug, Clone, Default)]
pub struct CommandAttributes {
    /// Minimum number of required arguments.
    pub nargs_min: usize,
    /// Maximum number of arguments (None = unlimited).
    pub nargs_max: Option<usize>,
    /// Whether the command accepts a range.
    pub range: bool,
    /// Default range value.
    pub range_default: Option<RangeDefault>,
    /// Whether the command accepts a count.
    pub count: bool,
    /// Whether the command accepts a bang (!).
    pub bang: bool,
    /// Whether the command is buffer-local.
    pub buffer_local: bool,
    /// Completion type for arguments.
    pub complete: Option<CompletionType>,
    /// Short description.
    pub description: Option<String>,
}

impl CommandAttributes {
    /// Create attributes for a command with no arguments.
    pub fn no_args() -> Self {
        Self {
            nargs_min: 0,
            nargs_max: Some(0),
            ..Default::default()
        }
    }

    /// Create attributes for a command with exactly n arguments.
    pub fn nargs(n: usize) -> Self {
        Self {
            nargs_min: n,
            nargs_max: Some(n),
            ..Default::default()
        }
    }

    /// Create attributes for a command with optional arguments.
    pub fn optional_args() -> Self {
        Self {
            nargs_min: 0,
            nargs_max: None,
            ..Default::default()
        }
    }

    /// Set the command to accept a range.
    pub fn with_range(mut self) -> Self {
        self.range = true;
        self
    }

    /// Set the command to accept a count.
    pub fn with_count(mut self) -> Self {
        self.count = true;
        self
    }

    /// Set the command to accept a bang.
    pub fn with_bang(mut self) -> Self {
        self.bang = true;
        self
    }

    /// Set completion type.
    pub fn with_completion(mut self, complete: CompletionType) -> Self {
        self.complete = Some(complete);
        self
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Default range value for commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RangeDefault {
    /// Current line (default for most).
    CurrentLine,
    /// Whole file (%).
    WholeFile,
    /// No default range.
    None,
}

/// Completion type for command arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompletionType {
    /// File path completion.
    File,
    /// Directory completion.
    Dir,
    /// Buffer name completion.
    Buffer,
    /// Command completion.
    Command,
    /// Custom completion function.
    Custom(String),
}

/// Parsed command invocation.
#[derive(Debug, Clone)]
pub struct CommandCall {
    /// The command name.
    pub name: String,
    /// The range if provided.
    pub range: Option<(usize, usize)>,
    /// The count if provided.
    pub count: Option<usize>,
    /// Whether bang was used.
    pub bang: bool,
    /// The arguments.
    pub args: Vec<String>,
    /// The raw argument string.
    pub args_raw: String,
}

impl CommandCall {
    /// Create a new command call.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            range: None,
            count: None,
            bang: false,
            args: Vec::new(),
            args_raw: String::new(),
        }
    }

    /// Set the range.
    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.range = Some((start, end));
        self
    }

    /// Set the count.
    pub fn with_count(mut self, count: usize) -> Self {
        self.count = Some(count);
        self
    }

    /// Set bang.
    pub fn with_bang(mut self) -> Self {
        self.bang = true;
        self
    }

    /// Add an argument.
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        let arg = arg.into();
        if !self.args_raw.is_empty() {
            self.args_raw.push(' ');
        }
        self.args_raw.push_str(&arg);
        self.args.push(arg);
        self
    }

    /// Get the first argument.
    pub fn first_arg(&self) -> Option<&str> {
        self.args.first().map(|s| s.as_str())
    }

    /// Get the number of arguments.
    pub fn arg_count(&self) -> usize {
        self.args.len()
    }
}

/// Result of command execution.
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// Command executed successfully.
    Ok,
    /// Command produced output.
    Output(String),
    /// Command failed with error.
    Error(String),
    /// Command not found.
    NotFound,
    /// Invalid arguments.
    InvalidArgs(String),
}

impl CommandResult {
    /// Check if the result is ok.
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok | Self::Output(_))
    }

    /// Check if the result is an error.
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }
}

/// A user-defined command.
#[derive(Clone)]
pub struct UserCommand {
    /// The command name.
    pub name: String,
    /// Command attributes.
    pub attrs: CommandAttributes,
    /// The command implementation (as a string of commands to execute).
    pub body: String,
}

impl UserCommand {
    /// Create a new user command.
    pub fn new(name: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attrs: CommandAttributes::default(),
            body: body.into(),
        }
    }

    /// Set command attributes.
    pub fn with_attrs(mut self, attrs: CommandAttributes) -> Self {
        self.attrs = attrs;
        self
    }

    /// Validate a command call against this command's attributes.
    pub fn validate(&self, call: &CommandCall) -> Result<(), String> {
        // Check argument count
        let argc = call.arg_count();
        if argc < self.attrs.nargs_min {
            return Err(format!(
                "E471: Argument required: {} requires at least {} argument(s)",
                self.name, self.attrs.nargs_min
            ));
        }
        if let Some(max) = self.attrs.nargs_max {
            if argc > max {
                return Err(format!(
                    "E488: Trailing characters: {} takes at most {} argument(s)",
                    self.name, max
                ));
            }
        }

        // Check range
        if call.range.is_some() && !self.attrs.range {
            return Err(format!("E481: No range allowed for {}", self.name));
        }

        // Check bang
        if call.bang && !self.attrs.bang {
            return Err(format!("E477: No ! allowed for {}", self.name));
        }

        Ok(())
    }
}

impl std::fmt::Debug for UserCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UserCommand")
            .field("name", &self.name)
            .field("attrs", &self.attrs)
            .field("body", &self.body)
            .finish()
    }
}

/// Registry for user-defined commands.
#[derive(Debug, Default)]
pub struct CommandRegistry {
    /// Global commands.
    global: HashMap<String, UserCommand>,
    /// Buffer-local commands (buffer_id -> commands).
    buffer_local: HashMap<u64, HashMap<String, UserCommand>>,
}

impl CommandRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a global command.
    pub fn register(&mut self, cmd: UserCommand) {
        self.global.insert(cmd.name.clone(), cmd);
    }

    /// Register a buffer-local command.
    pub fn register_buffer_local(&mut self, buffer_id: u64, cmd: UserCommand) {
        let buffer_cmds = self.buffer_local.entry(buffer_id).or_default();
        buffer_cmds.insert(cmd.name.clone(), cmd);
    }

    /// Unregister a global command.
    pub fn unregister(&mut self, name: &str) -> Option<UserCommand> {
        self.global.remove(name)
    }

    /// Get a command by name (buffer-local takes precedence).
    pub fn get(&self, name: &str, buffer_id: Option<u64>) -> Option<&UserCommand> {
        // Check buffer-local first
        if let Some(bid) = buffer_id {
            if let Some(buffer_cmds) = self.buffer_local.get(&bid) {
                if let Some(cmd) = buffer_cmds.get(name) {
                    return Some(cmd);
                }
            }
        }
        // Fall back to global
        self.global.get(name)
    }

    /// List all global commands.
    pub fn list_global(&self) -> Vec<&UserCommand> {
        self.global.values().collect()
    }

    /// List buffer-local commands for a buffer.
    pub fn list_buffer_local(&self, buffer_id: u64) -> Vec<&UserCommand> {
        self.buffer_local
            .get(&buffer_id)
            .map(|m| m.values().collect())
            .unwrap_or_default()
    }

    /// Clear buffer-local commands for a buffer.
    pub fn clear_buffer(&mut self, buffer_id: u64) {
        self.buffer_local.remove(&buffer_id);
    }

    /// Get the total number of commands.
    pub fn len(&self) -> usize {
        let buffer_count: usize = self.buffer_local.values().map(|m| m.len()).sum();
        self.global.len() + buffer_count
    }

    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.global.is_empty() && self.buffer_local.values().all(|m| m.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_attributes_default() {
        let attrs = CommandAttributes::default();
        assert_eq!(attrs.nargs_min, 0);
        assert_eq!(attrs.nargs_max, None);
        assert!(!attrs.range);
        assert!(!attrs.bang);
    }

    #[test]
    fn test_command_attributes_no_args() {
        let attrs = CommandAttributes::no_args();
        assert_eq!(attrs.nargs_min, 0);
        assert_eq!(attrs.nargs_max, Some(0));
    }

    #[test]
    fn test_command_attributes_nargs() {
        let attrs = CommandAttributes::nargs(2);
        assert_eq!(attrs.nargs_min, 2);
        assert_eq!(attrs.nargs_max, Some(2));
    }

    #[test]
    fn test_command_attributes_chain() {
        let attrs = CommandAttributes::optional_args()
            .with_range()
            .with_bang()
            .with_count()
            .with_completion(CompletionType::File)
            .with_description("test command");

        assert!(attrs.range);
        assert!(attrs.bang);
        assert!(attrs.count);
        assert_eq!(attrs.complete, Some(CompletionType::File));
        assert_eq!(attrs.description, Some("test command".to_string()));
    }

    #[test]
    fn test_command_call_new() {
        let call = CommandCall::new("MyCmd");
        assert_eq!(call.name, "MyCmd");
        assert!(call.range.is_none());
        assert!(call.count.is_none());
        assert!(!call.bang);
        assert!(call.args.is_empty());
    }

    #[test]
    fn test_command_call_with_args() {
        let call = CommandCall::new("Cmd")
            .with_arg("arg1")
            .with_arg("arg2")
            .with_range(1, 10)
            .with_count(5)
            .with_bang();

        assert_eq!(call.args, vec!["arg1", "arg2"]);
        assert_eq!(call.args_raw, "arg1 arg2");
        assert_eq!(call.range, Some((1, 10)));
        assert_eq!(call.count, Some(5));
        assert!(call.bang);
        assert_eq!(call.first_arg(), Some("arg1"));
        assert_eq!(call.arg_count(), 2);
    }

    #[test]
    fn test_user_command_new() {
        let cmd = UserCommand::new("Hello", "echo 'Hello, World!'");
        assert_eq!(cmd.name, "Hello");
        assert_eq!(cmd.body, "echo 'Hello, World!'");
    }

    #[test]
    fn test_user_command_validate_ok() {
        let cmd = UserCommand::new("Test", "echo test")
            .with_attrs(CommandAttributes::nargs(1).with_bang());

        let call = CommandCall::new("Test").with_arg("foo").with_bang();
        assert!(cmd.validate(&call).is_ok());
    }

    #[test]
    fn test_user_command_validate_too_few_args() {
        let cmd = UserCommand::new("Test", "echo")
            .with_attrs(CommandAttributes::nargs(2));

        let call = CommandCall::new("Test").with_arg("foo");
        let result = cmd.validate(&call);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E471"));
    }

    #[test]
    fn test_user_command_validate_too_many_args() {
        let cmd = UserCommand::new("Test", "echo")
            .with_attrs(CommandAttributes::no_args());

        let call = CommandCall::new("Test").with_arg("foo");
        let result = cmd.validate(&call);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E488"));
    }

    #[test]
    fn test_user_command_validate_range_not_allowed() {
        let cmd = UserCommand::new("Test", "echo")
            .with_attrs(CommandAttributes::no_args());

        let call = CommandCall::new("Test").with_range(1, 10);
        let result = cmd.validate(&call);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E481"));
    }

    #[test]
    fn test_user_command_validate_bang_not_allowed() {
        let cmd = UserCommand::new("Test", "echo")
            .with_attrs(CommandAttributes::no_args());

        let call = CommandCall::new("Test").with_bang();
        let result = cmd.validate(&call);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("E477"));
    }

    #[test]
    fn test_registry_register_get() {
        let mut registry = CommandRegistry::new();
        let cmd = UserCommand::new("Hello", "echo hello");
        registry.register(cmd);

        let retrieved = registry.get("Hello", None);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "Hello");
    }

    #[test]
    fn test_registry_buffer_local() {
        let mut registry = CommandRegistry::new();
        let global = UserCommand::new("Cmd", "global");
        let local = UserCommand::new("Cmd", "local");

        registry.register(global);
        registry.register_buffer_local(1, local);

        // Without buffer_id, get global
        let cmd = registry.get("Cmd", None);
        assert_eq!(cmd.unwrap().body, "global");

        // With buffer_id, get local (takes precedence)
        let cmd = registry.get("Cmd", Some(1));
        assert_eq!(cmd.unwrap().body, "local");

        // Different buffer_id, get global
        let cmd = registry.get("Cmd", Some(2));
        assert_eq!(cmd.unwrap().body, "global");
    }

    #[test]
    fn test_registry_unregister() {
        let mut registry = CommandRegistry::new();
        registry.register(UserCommand::new("Test", "echo"));

        let removed = registry.unregister("Test");
        assert!(removed.is_some());
        assert!(registry.get("Test", None).is_none());
    }

    #[test]
    fn test_registry_list() {
        let mut registry = CommandRegistry::new();
        registry.register(UserCommand::new("A", "a"));
        registry.register(UserCommand::new("B", "b"));

        let list = registry.list_global();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_registry_len() {
        let mut registry = CommandRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);

        registry.register(UserCommand::new("A", "a"));
        registry.register_buffer_local(1, UserCommand::new("B", "b"));
        assert_eq!(registry.len(), 2);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_registry_clear_buffer() {
        let mut registry = CommandRegistry::new();
        registry.register_buffer_local(1, UserCommand::new("A", "a"));
        registry.register_buffer_local(1, UserCommand::new("B", "b"));

        assert_eq!(registry.list_buffer_local(1).len(), 2);
        registry.clear_buffer(1);
        assert_eq!(registry.list_buffer_local(1).len(), 0);
    }

    #[test]
    fn test_command_result_ok() {
        assert!(CommandResult::Ok.is_ok());
        assert!(CommandResult::Output("test".to_string()).is_ok());
        assert!(!CommandResult::Error("err".to_string()).is_ok());
    }

    #[test]
    fn test_command_result_err() {
        assert!(!CommandResult::Ok.is_err());
        assert!(CommandResult::NotFound.is_err());
        assert!(CommandResult::InvalidArgs("bad".to_string()).is_err());
    }

    #[test]
    fn test_range_default() {
        assert_eq!(RangeDefault::CurrentLine, RangeDefault::CurrentLine);
        assert_ne!(RangeDefault::WholeFile, RangeDefault::CurrentLine);
    }

    #[test]
    fn test_completion_type() {
        assert_eq!(CompletionType::File, CompletionType::File);
        assert_eq!(
            CompletionType::Custom("fn".to_string()),
            CompletionType::Custom("fn".to_string())
        );
    }

    #[test]
    fn test_user_command_debug() {
        let cmd = UserCommand::new("Debug", "test");
        let debug = format!("{:?}", cmd);
        assert!(debug.contains("Debug"));
    }
}
