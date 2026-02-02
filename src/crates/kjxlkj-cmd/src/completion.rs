//! Command-line completion for ex commands.

use std::collections::HashMap;

/// A completion candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// The completion text.
    pub text: String,
    /// Description of the completion.
    pub description: Option<String>,
}

impl Candidate {
    /// Creates a new candidate.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            description: None,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
}

/// Command completion source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionSource {
    /// Built-in commands.
    Command,
    /// File paths.
    File,
    /// Directory paths.
    Directory,
    /// Buffer names.
    Buffer,
    /// Colorschemes.
    Colorscheme,
    /// Settings/options.
    Option,
    /// Help topics.
    Help,
    /// Mappings.
    Mapping,
    /// Custom user function.
    Custom,
}

/// Command completion state.
#[derive(Debug, Default)]
pub struct CommandCompletion {
    /// Current candidates.
    candidates: Vec<Candidate>,
    /// Selected index.
    selected: Option<usize>,
    /// Original text before completion.
    original: String,
    /// Completion source type.
    source: Option<CompletionSource>,
}

impl CommandCompletion {
    /// Creates a new command completion.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the candidates.
    pub fn set_candidates(&mut self, candidates: Vec<Candidate>, original: &str) {
        self.candidates = candidates;
        self.original = original.to_string();
        self.selected = if self.candidates.is_empty() {
            None
        } else {
            Some(0)
        };
    }

    /// Returns the candidates.
    pub fn candidates(&self) -> &[Candidate] {
        &self.candidates
    }

    /// Returns the selected candidate.
    pub fn selected(&self) -> Option<&Candidate> {
        self.selected.and_then(|i| self.candidates.get(i))
    }

    /// Returns the selected index.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Selects the next candidate.
    pub fn next(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % self.candidates.len(),
            None => 0,
        });
    }

    /// Selects the previous candidate.
    pub fn prev(&mut self) {
        if self.candidates.is_empty() {
            return;
        }
        self.selected = Some(match self.selected {
            Some(0) => self.candidates.len() - 1,
            Some(i) => i - 1,
            None => self.candidates.len() - 1,
        });
    }

    /// Confirms the selection.
    pub fn confirm(&self) -> Option<String> {
        self.selected().map(|c| c.text.clone())
    }

    /// Cancels completion, returning original text.
    pub fn cancel(&self) -> &str {
        &self.original
    }

    /// Clears the completion state.
    pub fn clear(&mut self) {
        self.candidates.clear();
        self.selected = None;
        self.original.clear();
        self.source = None;
    }

    /// Returns whether completion is active.
    pub fn is_active(&self) -> bool {
        !self.candidates.is_empty()
    }
}

/// Command registry for completions.
#[derive(Debug, Default)]
pub struct CommandRegistry {
    /// Commands and their completion sources.
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
    fn test_candidate_new() {
        let c = Candidate::new("test");
        assert_eq!(c.text, "test");
        assert!(c.description.is_none());
    }

    #[test]
    fn test_candidate_with_description() {
        let c = Candidate::new("test").with_description("A test");
        assert_eq!(c.description, Some("A test".to_string()));
    }

    #[test]
    fn test_completion_set_candidates() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(
            vec![Candidate::new("edit"), Candidate::new("exit")],
            "e",
        );
        assert_eq!(cc.candidates().len(), 2);
        assert_eq!(cc.selected_index(), Some(0));
    }

    #[test]
    fn test_completion_next() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(
            vec![Candidate::new("a"), Candidate::new("b"), Candidate::new("c")],
            "",
        );
        assert_eq!(cc.selected().unwrap().text, "a");
        cc.next();
        assert_eq!(cc.selected().unwrap().text, "b");
        cc.next();
        assert_eq!(cc.selected().unwrap().text, "c");
        cc.next();
        assert_eq!(cc.selected().unwrap().text, "a"); // Wrap around
    }

    #[test]
    fn test_completion_prev() {
        let mut cc = CommandCompletion::new();
        cc.set_candidates(
            vec![Candidate::new("a"), Candidate::new("b"), Candidate::new("c")],
            "",
        );
        cc.prev();
        assert_eq!(cc.selected().unwrap().text, "c"); // Wrap around
    }

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
