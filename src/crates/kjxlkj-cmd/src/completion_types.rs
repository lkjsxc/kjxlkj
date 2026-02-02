//! Command-line completion types.
//!
//! Types for command completion candidates and sources.

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
