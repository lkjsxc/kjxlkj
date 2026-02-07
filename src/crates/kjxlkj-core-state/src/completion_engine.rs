//! Command-line completion engine.

/// State of an active completion session.
#[derive(Debug, Clone)]
pub struct CompletionState {
    pub candidates: Vec<String>,
    pub index: usize,
    pub active: bool,
}

impl CompletionState {
    pub fn new() -> Self {
        Self {
            candidates: Vec::new(),
            index: 0,
            active: false,
        }
    }

    /// Cycle to the next candidate.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&String> {
        if self.candidates.is_empty() {
            return None;
        }
        self.index = (self.index + 1) % self.candidates.len();
        self.candidates.get(self.index)
    }

    /// Reset completion state.
    pub fn reset(&mut self) {
        self.candidates.clear();
        self.index = 0;
        self.active = false;
    }
}

impl Default for CompletionState {
    fn default() -> Self {
        Self::new()
    }
}

/// Source of completion candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionSource {
    Command,
    Path,
    Option,
    Buffer,
    None,
}

/// Detect what kind of completion is appropriate for the cmdline.
pub fn detect_source(cmdline: &str) -> CompletionSource {
    let trimmed = cmdline.trim();
    if trimmed.is_empty() {
        return CompletionSource::Command;
    }
    if trimmed.starts_with("e ")
        || trimmed.starts_with("edit ")
        || trimmed.starts_with("w ")
        || trimmed.starts_with("r ")
        || trimmed.starts_with("source ")
    {
        return CompletionSource::Path;
    }
    if trimmed.starts_with("set ") {
        return CompletionSource::Option;
    }
    if trimmed.starts_with("b ") || trimmed.starts_with("buffer ") {
        return CompletionSource::Buffer;
    }
    if !trimmed.contains(' ') {
        return CompletionSource::Command;
    }
    CompletionSource::None
}

pub(crate) const COMMANDS: &[&str] = &[
    "quit",
    "q",
    "qa",
    "write",
    "w",
    "wa",
    "wq",
    "x",
    "exit",
    "edit",
    "e",
    "enew",
    "new",
    "vnew",
    "split",
    "sp",
    "vsplit",
    "vsp",
    "only",
    "set",
    "ls",
    "bn",
    "bp",
    "bd",
    "saveas",
    "source",
    "marks",
    "reg",
    "jumps",
    "changes",
    "noh",
    "sort",
    "terminal",
    "explorer",
    "find",
    "livegrep",
    "undotree",
    "syntax",
    "highlight",
    "map",
    "unmap",
    "mapclear",
    "autocmd",
    "cd",
    "pwd",
    "ft",
];

// Re-export completion helpers from dedicated module.
pub use crate::completion_engine_ext::{
    common_prefix, complete_commands, complete_options, complete_paths,
};
