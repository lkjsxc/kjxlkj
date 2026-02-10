//! LSP client implementation.

use crate::response::{LspResponse, WorkspaceEdit};
use crate::types::LspRequest;
use std::collections::HashMap;
use std::path::PathBuf;

/// State of the language server connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerState {
    Stopped,
    Initializing,
    Running,
    ShuttingDown,
}

/// Configuration for a language server.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub cmd: Vec<String>,
    pub filetypes: Vec<String>,
    pub root_markers: Vec<String>,
}

/// Advertised server capabilities (subset).
#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
    pub completion: bool,
    pub hover: bool,
    pub definition: bool,
    pub references: bool,
    pub rename: bool,
    pub formatting: bool,
    pub diagnostics: bool,
    pub code_actions: bool,
    pub signature_help: bool,
    pub inlay_hints: bool,
}

/// State for a single server instance.
struct ServerEntry {
    config: ServerConfig,
    state: ServerState,
    root: Option<PathBuf>,
    capabilities: ServerCapabilities,
}

/// LSP service managing language server processes.
pub struct LspService {
    servers: HashMap<String, ServerEntry>,
    next_request_id: u64,
    crash_counts: HashMap<String, u32>,
    max_crashes: u32,
}

impl LspService {
    pub fn new() -> Self {
        Self {
            servers: HashMap::new(),
            next_request_id: 1,
            crash_counts: HashMap::new(),
            max_crashes: 3,
        }
    }

    pub fn register_server(&mut self, lang: &str, config: ServerConfig) {
        let entry = ServerEntry {
            config,
            state: ServerState::Stopped,
            root: None,
            capabilities: ServerCapabilities::default(),
        };
        self.servers.insert(lang.to_string(), entry);
    }

    pub fn server_config(&self, lang: &str) -> Option<&ServerConfig> {
        self.servers.get(lang).map(|e| &e.config)
    }

    pub fn server_state(&self, lang: &str) -> ServerState {
        self.servers
            .get(lang)
            .map(|e| e.state)
            .unwrap_or(ServerState::Stopped)
    }

    pub fn capabilities(&self, lang: &str) -> Option<&ServerCapabilities> {
        self.servers.get(lang).map(|e| &e.capabilities)
    }

    /// Process an LSP request and return a response.
    pub fn handle_request(&mut self, request: &LspRequest) -> Option<LspResponse> {
        match request {
            LspRequest::Initialize { root } => {
                tracing::info!(?root, "LSP initialize");
                for entry in self.servers.values_mut() {
                    if entry.state == ServerState::Stopped {
                        entry.state = ServerState::Initializing;
                        entry.root = Some(root.clone());
                    }
                }
                Some(LspResponse::Initialized)
            }
            LspRequest::Completion { file, position } => {
                tracing::debug!(?file, ?position, "completion");
                Some(LspResponse::Completions(Vec::new()))
            }
            LspRequest::Hover { file, position } => {
                tracing::debug!(?file, ?position, "hover");
                Some(LspResponse::Hover(None))
            }
            LspRequest::Definition { file, position } => {
                tracing::debug!(?file, ?position, "definition");
                Some(LspResponse::Locations(Vec::new()))
            }
            LspRequest::References { file, position } => {
                tracing::debug!(?file, ?position, "references");
                Some(LspResponse::Locations(Vec::new()))
            }
            LspRequest::Rename {
                file,
                position,
                new_name,
            } => {
                tracing::debug!(?file, ?position, new_name, "rename");
                Some(LspResponse::WorkspaceEdit(WorkspaceEdit::default()))
            }
            LspRequest::Format { file } => {
                tracing::debug!(?file, "format");
                Some(LspResponse::TextEdits(Vec::new()))
            }
            LspRequest::Shutdown => {
                for e in self.servers.values_mut() {
                    e.state = ServerState::ShuttingDown;
                }
                Some(LspResponse::ShutdownAck)
            }
        }
    }

    /// Mark server as ready after initialization.
    pub fn mark_ready(&mut self, lang: &str) {
        if let Some(entry) = self.servers.get_mut(lang) {
            if entry.state == ServerState::Initializing {
                entry.state = ServerState::Running;
                entry.capabilities = ServerCapabilities {
                    completion: true,
                    hover: true,
                    definition: true,
                    references: true,
                    rename: true,
                    formatting: true,
                    diagnostics: true,
                    code_actions: true,
                    signature_help: true,
                    inlay_hints: false,
                };
            }
        }
    }

    /// Record crash; returns true if server should be restarted.
    pub fn record_crash(&mut self, lang: &str) -> bool {
        let count = self.crash_counts.entry(lang.to_string()).or_insert(0);
        *count += 1;
        if *count > self.max_crashes {
            tracing::warn!(lang, count, "LSP server disabled after crash limit");
            false
        } else {
            tracing::info!(lang, count, "LSP server crashed, will restart");
            if let Some(e) = self.servers.get_mut(lang) {
                e.state = ServerState::Stopped;
            }
            true
        }
    }

    /// Allocate the next JSON-RPC request id.
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_request_id;
        self.next_request_id += 1;
        id
    }
}

impl Default for LspService {
    fn default() -> Self {
        Self::new()
    }
}
