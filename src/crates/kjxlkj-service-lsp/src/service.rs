//! LSP service.

use crate::LspClient;
use std::collections::HashMap;

/// LSP service managing multiple clients.
pub struct LspService {
    /// Clients by language.
    clients: HashMap<String, LspClient>,
}

impl LspService {
    /// Creates a new LSP service.
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Gets a client by language.
    pub fn get(&self, language: &str) -> Option<&LspClient> {
        self.clients.get(language)
    }

    /// Gets a mutable client by language.
    pub fn get_mut(&mut self, language: &str) -> Option<&mut LspClient> {
        self.clients.get_mut(language)
    }

    /// Adds a client.
    pub fn add(&mut self, language: impl Into<String>, client: LspClient) {
        self.clients.insert(language.into(), client);
    }

    /// Runs the service.
    pub async fn run(self) {
        // Service loop
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}
