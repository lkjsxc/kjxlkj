//! Snippets engine stub.
//!
//! Provides a minimal snippet registry and expand trigger.
//! Actual snippet expansion with tab-stops is not yet implemented.

use std::collections::HashMap;

/// A snippet definition.
#[derive(Debug, Clone)]
pub struct Snippet {
    /// Trigger prefix text.
    pub trigger: String,
    /// Expansion body (may contain $1, $2 tab-stop placeholders).
    pub body: String,
    /// Optional description.
    pub description: String,
}

/// Registry of available snippets.
#[derive(Debug, Default)]
pub struct SnippetRegistry {
    snippets: HashMap<String, Snippet>,
}

impl SnippetRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a snippet.
    pub fn add(&mut self, trigger: &str, body: &str, desc: &str) {
        self.snippets.insert(
            trigger.to_string(),
            Snippet {
                trigger: trigger.to_string(),
                body: body.to_string(),
                description: desc.to_string(),
            },
        );
    }

    /// Look up a snippet by trigger.
    pub fn get(&self, trigger: &str) -> Option<&Snippet> {
        self.snippets.get(trigger)
    }

    /// Try to expand a trigger: returns the expansion body or None.
    pub fn expand(&self, trigger: &str) -> Option<String> {
        self.snippets.get(trigger).map(|s| {
            // Strip tab-stop markers for now ($1, $2, $0).
            let mut body = s.body.clone();
            for i in 0..=9 {
                body = body.replace(&format!("${i}"), "");
                body = body.replace(&format!("${{{i}}}"), "");
            }
            body
        })
    }

    /// List all registered snippets.
    pub fn list(&self) -> Vec<&Snippet> {
        self.snippets.values().collect()
    }

    /// Remove a snippet by trigger.
    pub fn remove(&mut self, trigger: &str) -> bool {
        self.snippets.remove(trigger).is_some()
    }

    /// Clear all snippets.
    pub fn clear(&mut self) {
        self.snippets.clear();
    }
}
