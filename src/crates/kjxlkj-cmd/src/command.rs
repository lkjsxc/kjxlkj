//! Ex command definitions.

use crate::Range;
use std::path::PathBuf;

/// A parsed Ex command.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    /// Optional range.
    pub range: Option<Range>,
    /// Command kind.
    pub kind: CommandKind,
    /// Force flag (e.g., :q!).
    pub force: bool,
}

impl Command {
    /// Creates a new command.
    pub fn new(kind: CommandKind) -> Self {
        Self {
            range: None,
            kind,
            force: false,
        }
    }

    /// Creates a command with range.
    pub fn with_range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }

    /// Sets the force flag.
    pub fn with_force(mut self) -> Self {
        self.force = true;
        self
    }
}

/// Command kinds.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandKind {
    // File operations
    /// Write buffer (:w).
    Write(Option<PathBuf>),
    /// Edit file (:e).
    Edit(PathBuf),
    /// Write all (:wa).
    WriteAll,

    // Quit operations
    /// Quit (:q).
    Quit,
    /// Quit all (:qa).
    QuitAll,
    /// Write and quit (:wq, :x).
    WriteQuit,

    // Buffer operations
    /// List buffers (:ls, :buffers).
    ListBuffers,
    /// Switch to buffer (:b).
    Buffer(BufferArg),
    /// Next buffer (:bn).
    BufferNext,
    /// Previous buffer (:bp).
    BufferPrev,
    /// Delete buffer (:bd).
    BufferDelete,
    /// Wipe buffer (:bw).
    BufferWipe,

    // Window operations
    /// Horizontal split (:sp).
    Split(Option<PathBuf>),
    /// Vertical split (:vsp).
    VSplit(Option<PathBuf>),
    /// Close window (:close).
    Close,
    /// Keep only current window (:only).
    Only,

    // Options
    /// Set option (:set).
    Set(SetArg),

    // External commands
    /// Execute external command (:!).
    External(String),
    /// Filter through external command (:[range]!).
    Filter(String),
    /// Read external command output (:r !).
    ReadCommand(String),

    // Line operations
    /// Delete lines (:d).
    Delete,
    /// Yank lines (:y).
    Yank,
    /// Put lines (:put).
    Put,

    // Substitute
    /// Substitute (:s).
    Substitute(SubstituteArgs),

    // Global
    /// Global command (:g).
    Global(GlobalArgs),

    // Other
    /// No-op.
    Noop,
}

/// Buffer argument.
#[derive(Debug, Clone, PartialEq)]
pub enum BufferArg {
    /// Buffer number.
    Number(usize),
    /// Buffer name.
    Name(String),
}

/// Set command argument.
#[derive(Debug, Clone, PartialEq)]
pub enum SetArg {
    /// Set boolean true.
    Enable(String),
    /// Set boolean false.
    Disable(String),
    /// Set value.
    Value(String, String),
    /// Query value.
    Query(String),
}

/// Substitute arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct SubstituteArgs {
    /// Search pattern.
    pub pattern: String,
    /// Replacement string.
    pub replacement: String,
    /// Flags.
    pub flags: SubstituteFlags,
}

/// Substitute flags.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SubstituteFlags {
    /// Global flag (g).
    pub global: bool,
    /// Case insensitive (i).
    pub ignore_case: bool,
    /// Confirm each (c).
    pub confirm: bool,
}

/// Global command arguments.
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalArgs {
    /// Search pattern.
    pub pattern: String,
    /// Command to execute.
    pub command: String,
    /// Invert match (:v).
    pub invert: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() {
        let cmd = Command::new(CommandKind::Quit);
        assert_eq!(cmd.kind, CommandKind::Quit);
        assert!(!cmd.force);
        assert!(cmd.range.is_none());
    }

    #[test]
    fn test_command_with_force() {
        let cmd = Command::new(CommandKind::Quit).with_force();
        assert!(cmd.force);
    }

    #[test]
    fn test_command_with_range() {
        let cmd = Command::new(CommandKind::Delete)
            .with_range(Range::FromTo(1, 10));
        assert!(cmd.range.is_some());
    }
}
