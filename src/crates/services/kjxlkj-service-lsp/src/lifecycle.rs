//! LSP server lifecycle and capability tracking.
//! See /docs/spec/features/lsp/lsp.md.

/// Server lifecycle phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerPhase {
    Starting,
    Initializing,
    Running,
    ShuttingDown,
    Stopped,
    Failed,
}

/// Capabilities declared by the server during initialization.
#[derive(Debug, Clone, Default)]
pub struct ServerCapabilities {
    pub completion: bool,
    pub hover: bool,
    pub definition: bool,
    pub references: bool,
    pub rename: bool,
    pub code_action: bool,
    pub formatting: bool,
    pub range_formatting: bool,
    pub signature_help: bool,
    pub code_lens: bool,
    pub inlay_hints: bool,
    pub document_symbols: bool,
    pub workspace_symbols: bool,
    pub declaration: bool,
    pub type_definition: bool,
    pub implementation: bool,
    pub diagnostics: bool,
}

/// Configuration for a single language server.
#[derive(Debug, Clone)]
pub struct LspServerConfig {
    pub language: &'static str,
    pub command: Vec<String>,
    pub root_markers: Vec<String>,
    pub filetypes: Vec<String>,
}

/// State of a single running LSP server instance.
#[derive(Debug)]
pub struct LspServerState {
    pub config: LspServerConfig,
    pub phase: ServerPhase,
    pub capabilities: ServerCapabilities,
    pub crash_count: u32,
    pub root_uri: Option<String>,
    pub progress_message: Option<String>,
}

impl LspServerState {
    pub fn new(config: LspServerConfig) -> Self {
        Self {
            config, phase: ServerPhase::Starting,
            capabilities: ServerCapabilities::default(), crash_count: 0,
            root_uri: None, progress_message: None,
        }
    }
    pub fn transition(&mut self, to: ServerPhase) { self.phase = to; }
    pub fn record_crash(&mut self) -> bool {
        self.crash_count += 1;
        self.phase = ServerPhase::Failed;
        self.crash_count < 3
    }
    pub fn can_restart(&self) -> bool { self.crash_count < 3 }
    pub fn reset_for_restart(&mut self) {
        self.phase = ServerPhase::Starting;
        self.capabilities = ServerCapabilities::default();
        self.progress_message = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn cfg() -> LspServerConfig {
        LspServerConfig {
            language: "rust", command: vec!["rust-analyzer".into()],
            root_markers: vec!["Cargo.toml".into()], filetypes: vec!["rust".into()],
        }
    }
    #[test]
    fn new_server_starts_in_starting() {
        let s = LspServerState::new(cfg());
        assert_eq!(s.phase, ServerPhase::Starting);
        assert_eq!(s.crash_count, 0);
    }
    #[test]
    fn transition_phases() {
        let mut s = LspServerState::new(cfg());
        s.transition(ServerPhase::Initializing);
        assert_eq!(s.phase, ServerPhase::Initializing);
        s.transition(ServerPhase::Running);
        assert_eq!(s.phase, ServerPhase::Running);
    }
    #[test]
    fn crash_increments_and_limits() {
        let mut s = LspServerState::new(cfg());
        assert!(s.record_crash()); // 1/3
        assert!(s.record_crash()); // 2/3
        assert!(!s.record_crash()); // 3/3 → false
        assert!(!s.can_restart());
    }
    #[test]
    fn reset_for_restart_clears_state() {
        let mut s = LspServerState::new(cfg());
        s.transition(ServerPhase::Running);
        s.capabilities.completion = true;
        s.progress_message = Some("indexing".into());
        s.record_crash();
        s.reset_for_restart();
        assert_eq!(s.phase, ServerPhase::Starting);
        assert!(!s.capabilities.completion);
        assert!(s.progress_message.is_none());
        assert_eq!(s.crash_count, 1); // crash_count NOT reset
    }
    #[test]
    fn capabilities_default_all_false() {
        let c = ServerCapabilities::default();
        assert!(!c.completion);
        assert!(!c.hover);
        assert!(!c.diagnostics);
    }
}
