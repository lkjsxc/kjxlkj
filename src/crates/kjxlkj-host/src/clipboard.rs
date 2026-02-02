//! Clipboard operations.

/// Clipboard abstraction.
#[derive(Debug, Default)]
pub struct Clipboard {
    /// Internal clipboard store (fallback).
    internal: String,
}

impl Clipboard {
    /// Creates a new clipboard instance.
    pub fn new() -> Self {
        Self {
            internal: String::new(),
        }
    }

    /// Gets clipboard content.
    pub fn get(&self) -> String {
        // TODO: Integrate with system clipboard
        self.internal.clone()
    }

    /// Sets clipboard content.
    pub fn set(&mut self, content: String) {
        // TODO: Integrate with system clipboard
        self.internal = content;
    }

    /// Returns true if clipboard has content.
    pub fn has_content(&self) -> bool {
        !self.internal.is_empty()
    }
}
