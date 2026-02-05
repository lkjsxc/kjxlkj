//! Terminal service for kjxlkj editor.
//!
//! Provides embedded terminal functionality.

use kjxlkj_services::{Service, ServiceMessage};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::{Child, Command};
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Terminal session ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(u32);

impl TerminalId {
    /// Create a new terminal ID.
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// Get the ID value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Terminal layout type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerminalLayout {
    /// Floating terminal window.
    Float,
    /// Horizontal split (bottom).
    #[default]
    Horizontal,
    /// Vertical split (right).
    Vertical,
    /// Full-screen tab.
    Tab,
}

/// Terminal state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerminalState {
    /// Terminal is running.
    Running,
    /// Terminal is stopped/exited.
    #[default]
    Stopped,
    /// Terminal is hidden but active.
    Hidden,
}

/// Terminal info for tracking.
#[derive(Debug)]
pub struct TerminalInfo {
    /// Terminal ID.
    pub id: TerminalId,
    /// Terminal name/label.
    pub name: String,
    /// Current layout.
    pub layout: TerminalLayout,
    /// Current state.
    pub state: TerminalState,
    /// Shell command.
    pub shell: String,
    /// Size (rows, cols).
    pub size: (u16, u16),
}

impl TerminalInfo {
    /// Create new terminal info.
    pub fn new(id: TerminalId, shell: &str) -> Self {
        Self {
            id,
            name: format!("term-{}", id.value()),
            layout: TerminalLayout::default(),
            state: TerminalState::Stopped,
            shell: shell.to_string(),
            size: (24, 80),
        }
    }

    /// Set the terminal name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set the layout.
    pub fn with_layout(mut self, layout: TerminalLayout) -> Self {
        self.layout = layout;
        self
    }
}

/// Terminal manager for multiple terminals.
#[derive(Debug, Default)]
pub struct TerminalManager {
    /// Terminals by ID.
    terminals: HashMap<TerminalId, TerminalInfo>,
    /// Next terminal ID.
    next_id: u32,
    /// Active terminal.
    active: Option<TerminalId>,
    /// Default shell.
    default_shell: String,
}

impl TerminalManager {
    /// Create a new terminal manager.
    pub fn new() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        Self {
            terminals: HashMap::new(),
            next_id: 1,
            active: None,
            default_shell: shell,
        }
    }

    /// Create a new terminal.
    pub fn create(&mut self, layout: TerminalLayout) -> TerminalId {
        let id = TerminalId::new(self.next_id);
        self.next_id += 1;
        let info = TerminalInfo::new(id, &self.default_shell).with_layout(layout);
        self.terminals.insert(id, info);
        if self.active.is_none() {
            self.active = Some(id);
        }
        id
    }

    /// Create a named terminal.
    pub fn create_named(&mut self, name: &str, layout: TerminalLayout) -> TerminalId {
        let id = TerminalId::new(self.next_id);
        self.next_id += 1;
        let info = TerminalInfo::new(id, &self.default_shell)
            .with_name(name)
            .with_layout(layout);
        self.terminals.insert(id, info);
        if self.active.is_none() {
            self.active = Some(id);
        }
        id
    }

    /// Get terminal info.
    pub fn get(&self, id: TerminalId) -> Option<&TerminalInfo> {
        self.terminals.get(&id)
    }

    /// Get mutable terminal info.
    pub fn get_mut(&mut self, id: TerminalId) -> Option<&mut TerminalInfo> {
        self.terminals.get_mut(&id)
    }

    /// Remove a terminal.
    pub fn remove(&mut self, id: TerminalId) -> Option<TerminalInfo> {
        let info = self.terminals.remove(&id);
        if self.active == Some(id) {
            self.active = self.terminals.keys().next().copied();
        }
        info
    }

    /// Get active terminal.
    pub fn active(&self) -> Option<TerminalId> {
        self.active
    }

    /// Set active terminal.
    pub fn set_active(&mut self, id: TerminalId) {
        if self.terminals.contains_key(&id) {
            self.active = Some(id);
        }
    }

    /// Toggle terminal visibility.
    pub fn toggle(&mut self, id: TerminalId) {
        if let Some(info) = self.terminals.get_mut(&id) {
            info.state = match info.state {
                TerminalState::Running => TerminalState::Hidden,
                TerminalState::Hidden => TerminalState::Running,
                TerminalState::Stopped => TerminalState::Running,
            };
        }
    }

    /// List all terminals.
    pub fn list(&self) -> impl Iterator<Item = &TerminalInfo> {
        self.terminals.values()
    }

    /// Get terminal count.
    pub fn len(&self) -> usize {
        self.terminals.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.terminals.is_empty()
    }

    /// Find terminal by name.
    pub fn find_by_name(&self, name: &str) -> Option<&TerminalInfo> {
        self.terminals.values().find(|t| t.name == name)
    }
}

// ============================================================================
// Debug Adapter Protocol (DAP)
// ============================================================================

/// DAP session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DapState {
    /// Not connected.
    #[default]
    Disconnected,
    /// Initializing.
    Initializing,
    /// Stopped at breakpoint.
    Stopped,
    /// Running.
    Running,
    /// Terminated.
    Terminated,
}

/// Breakpoint type.
#[derive(Debug, Clone)]
pub enum BreakpointKind {
    /// Line breakpoint.
    Line,
    /// Conditional breakpoint with expression.
    Conditional(String),
    /// Function breakpoint.
    Function(String),
    /// Exception breakpoint.
    Exception(ExceptionBreakMode),
    /// Logpoint (logs message without stopping).
    Logpoint(String),
}

/// Exception break mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionBreakMode {
    /// Never break.
    Never,
    /// Break on unhandled.
    Unhandled,
    /// Break on user-unhandled.
    UserUnhandled,
    /// Always break.
    Always,
}

/// A breakpoint.
#[derive(Debug, Clone)]
pub struct Breakpoint {
    /// Unique ID.
    pub id: u32,
    /// File path.
    pub file: String,
    /// Line number (1-indexed).
    pub line: usize,
    /// Breakpoint kind.
    pub kind: BreakpointKind,
    /// Is verified by debug adapter.
    pub verified: bool,
    /// Is enabled.
    pub enabled: bool,
    /// Hit count.
    pub hit_count: u32,
}

impl Breakpoint {
    /// Create a line breakpoint.
    pub fn line(id: u32, file: impl Into<String>, line: usize) -> Self {
        Self {
            id,
            file: file.into(),
            line,
            kind: BreakpointKind::Line,
            verified: false,
            enabled: true,
            hit_count: 0,
        }
    }

    /// Make conditional.
    pub fn with_condition(mut self, expr: impl Into<String>) -> Self {
        self.kind = BreakpointKind::Conditional(expr.into());
        self
    }

    /// Make logpoint.
    pub fn as_logpoint(mut self, message: impl Into<String>) -> Self {
        self.kind = BreakpointKind::Logpoint(message.into());
        self
    }
}

/// Stack frame.
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame ID.
    pub id: u32,
    /// Function name.
    pub name: String,
    /// Source file.
    pub source: Option<String>,
    /// Line number.
    pub line: usize,
    /// Column.
    pub column: usize,
}

impl StackFrame {
    /// Create a new stack frame.
    pub fn new(id: u32, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            source: None,
            line: 0,
            column: 0,
        }
    }

    /// Set source location.
    pub fn at(mut self, source: impl Into<String>, line: usize, column: usize) -> Self {
        self.source = Some(source.into());
        self.line = line;
        self.column = column;
        self
    }
}

/// Variable scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableScope {
    /// Local variables.
    Local,
    /// Function arguments.
    Arguments,
    /// Global/static.
    Global,
    /// Registers.
    Registers,
}

/// A debug variable.
#[derive(Debug, Clone)]
pub struct Variable {
    /// Variable name.
    pub name: String,
    /// Value as string.
    pub value: String,
    /// Type name.
    pub type_name: Option<String>,
    /// Variables reference (for expandable).
    pub variables_reference: u32,
    /// Named children count.
    pub named_children: u32,
    /// Indexed children count.
    pub indexed_children: u32,
}

impl Variable {
    /// Create a simple variable.
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            type_name: None,
            variables_reference: 0,
            named_children: 0,
            indexed_children: 0,
        }
    }

    /// Set type.
    pub fn with_type(mut self, type_name: impl Into<String>) -> Self {
        self.type_name = Some(type_name.into());
        self
    }

    /// Mark as expandable.
    pub fn expandable(mut self, ref_id: u32) -> Self {
        self.variables_reference = ref_id;
        self
    }
}

/// DAP session.
#[derive(Debug, Default)]
pub struct DapSession {
    /// Current state.
    pub state: DapState,
    /// Active thread ID.
    pub thread_id: Option<u32>,
    /// Breakpoints.
    breakpoints: HashMap<u32, Breakpoint>,
    /// Next breakpoint ID.
    next_bp_id: u32,
    /// Call stack.
    pub stack: Vec<StackFrame>,
    /// Watch expressions.
    watches: Vec<String>,
}

impl DapSession {
    /// Create a new DAP session.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a breakpoint.
    pub fn add_breakpoint(&mut self, file: impl Into<String>, line: usize) -> u32 {
        let id = self.next_bp_id;
        self.next_bp_id += 1;
        let bp = Breakpoint::line(id, file, line);
        self.breakpoints.insert(id, bp);
        id
    }

    /// Remove a breakpoint.
    pub fn remove_breakpoint(&mut self, id: u32) -> Option<Breakpoint> {
        self.breakpoints.remove(&id)
    }

    /// Toggle a breakpoint enabled state.
    pub fn toggle_breakpoint(&mut self, id: u32) {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = !bp.enabled;
        }
    }

    /// Get breakpoints for a file.
    pub fn breakpoints_for_file(&self, file: &str) -> Vec<&Breakpoint> {
        self.breakpoints.values().filter(|bp| bp.file == file).collect()
    }

    /// Get all breakpoints.
    pub fn breakpoints(&self) -> impl Iterator<Item = &Breakpoint> {
        self.breakpoints.values()
    }

    /// Add a watch expression.
    pub fn add_watch(&mut self, expr: impl Into<String>) {
        self.watches.push(expr.into());
    }

    /// Remove a watch expression.
    pub fn remove_watch(&mut self, index: usize) -> Option<String> {
        if index < self.watches.len() {
            Some(self.watches.remove(index))
        } else {
            None
        }
    }

    /// Get watch expressions.
    pub fn watches(&self) -> &[String] {
        &self.watches
    }

    /// Clear call stack.
    pub fn clear_stack(&mut self) {
        self.stack.clear();
    }

    /// Push a stack frame.
    pub fn push_frame(&mut self, frame: StackFrame) {
        self.stack.push(frame);
    }
}

// ============================================================================
// tmux/Screen Integration
// ============================================================================

/// tmux detection and integration.
#[derive(Debug, Default)]
pub struct TmuxIntegration {
    /// Is running inside tmux.
    pub inside_tmux: bool,
    /// tmux version.
    pub version: Option<String>,
    /// True color support.
    pub true_color: bool,
    /// OSC52 clipboard support.
    pub osc52: bool,
}

impl TmuxIntegration {
    /// Detect tmux environment.
    pub fn detect() -> Self {
        let inside_tmux = std::env::var("TMUX").is_ok();
        let term = std::env::var("TERM").unwrap_or_default();

        Self {
            inside_tmux,
            version: None,
            true_color: inside_tmux && term.contains("256color"),
            osc52: inside_tmux,
        }
    }

    /// Check if escape sequences should be wrapped.
    pub fn needs_passthrough(&self) -> bool {
        self.inside_tmux
    }

    /// Wrap escape sequence for tmux passthrough.
    pub fn passthrough(&self, seq: &str) -> String {
        if self.inside_tmux {
            format!("\x1bPtmux;\x1b{}\x1b\\", seq)
        } else {
            seq.to_string()
        }
    }
}

/// Terminal session.
pub struct TerminalSession {
    /// Session ID.
    pub id: TerminalId,
    /// Shell process.
    child: Option<Child>,
    /// Output buffer.
    output: Vec<String>,
}

impl TerminalSession {
    /// Create a new terminal session.
    pub fn new(id: TerminalId) -> Self {
        Self {
            id,
            child: None,
            output: Vec::new(),
        }
    }

    /// Start the shell.
    pub async fn start(&mut self, shell: &str) -> Result<(), std::io::Error> {
        let child = Command::new(shell)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        self.child = Some(child);
        Ok(())
    }

    /// Write to the terminal.
    pub async fn write(&mut self, data: &str) -> Result<(), std::io::Error> {
        if let Some(ref mut child) = self.child {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(data.as_bytes()).await?;
                stdin.flush().await?;
            }
        }
        Ok(())
    }

    /// Kill the terminal.
    pub async fn kill(&mut self) -> Result<(), std::io::Error> {
        if let Some(ref mut child) = self.child {
            child.kill().await?;
        }
        self.child = None;
        Ok(())
    }

    /// Check if terminal is running.
    pub fn is_running(&mut self) -> bool {
        if let Some(ref mut child) = self.child {
            child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }

    /// Get output.
    pub fn output(&self) -> &[String] {
        &self.output
    }
}

/// Terminal service.
pub struct TerminalService {
    /// Service name.
    name: String,
    /// Default shell.
    shell: String,
}

impl TerminalService {
    /// Create a new terminal service.
    pub fn new() -> Self {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
        Self {
            name: "terminal".to_string(),
            shell,
        }
    }

    /// Get the default shell.
    pub fn shell(&self) -> &str {
        &self.shell
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for TerminalService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Terminal service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Terminal service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_service_new() {
        let service = TerminalService::new();
        assert_eq!(service.name(), "terminal");
    }

    #[test]
    fn test_terminal_id() {
        let id = TerminalId::new(1);
        assert_eq!(id.value(), 1);
    }

    #[test]
    fn test_terminal_session_new() {
        let session = TerminalSession::new(TerminalId::new(1));
        assert_eq!(session.id, TerminalId::new(1));
        assert!(session.output().is_empty());
    }

    #[test]
    fn test_terminal_id_equality() {
        let id1 = TerminalId::new(1);
        let id2 = TerminalId::new(1);
        let id3 = TerminalId::new(2);
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_terminal_id_clone() {
        let id = TerminalId::new(5);
        let cloned = id;
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_terminal_id_debug() {
        let id = TerminalId::new(42);
        let debug = format!("{:?}", id);
        assert!(debug.contains("42"));
    }

    #[test]
    fn test_terminal_service_default() {
        let service = TerminalService::default();
        assert_eq!(service.name(), "terminal");
    }

    #[test]
    fn test_terminal_session_not_running() {
        let mut session = TerminalSession::new(TerminalId::new(1));
        assert!(!session.is_running());
    }

    #[test]
    fn test_terminal_service_shell() {
        let service = TerminalService::new();
        // Shell should be non-empty
        assert!(!service.shell().is_empty());
    }

    #[test]
    fn test_terminal_id_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TerminalId::new(1));
        set.insert(TerminalId::new(2));
        assert_eq!(set.len(), 2);
        assert!(set.contains(&TerminalId::new(1)));
    }

    #[test]
    fn test_terminal_session_output_empty() {
        let session = TerminalSession::new(TerminalId::new(1));
        assert!(session.output().is_empty());
    }

    #[test]
    fn test_terminal_id_copy() {
        let id = TerminalId::new(5);
        let copied = id; // Copy, not move
        assert_eq!(id, copied);
    }

    #[test]
    fn test_terminal_id_zero() {
        let id = TerminalId::new(0);
        assert_eq!(id.value(), 0);
    }

    #[test]
    fn test_terminal_id_max() {
        let id = TerminalId::new(u32::MAX);
        assert_eq!(id.value(), u32::MAX);
    }

    #[test]
    fn test_terminal_session_id_value() {
        let session = TerminalSession::new(TerminalId::new(100));
        assert_eq!(session.id.value(), 100);
    }

    #[test]
    fn test_terminal_session_child_none() {
        let mut session = TerminalSession::new(TerminalId::new(1));
        assert!(!session.is_running());
    }

    #[test]
    fn test_terminal_id_hash_duplicate() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(TerminalId::new(1));
        set.insert(TerminalId::new(1));
        assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_terminal_service_name_literal() {
        let service = TerminalService::new();
        assert_eq!(service.name(), "terminal");
    }

    #[test]
    fn test_terminal_session_multiple() {
        let s1 = TerminalSession::new(TerminalId::new(1));
        let s2 = TerminalSession::new(TerminalId::new(2));
        assert_ne!(s1.id, s2.id);
    }

    #[test]
    fn test_terminal_id_sequential() {
        let id1 = TerminalId::new(1);
        let id2 = TerminalId::new(2);
        let id3 = TerminalId::new(3);
        assert_eq!(id1.value() + 1, id2.value());
        assert_eq!(id2.value() + 1, id3.value());
    }

    #[test]
    fn test_terminal_id_from_value() {
        let value = 42u32;
        let id = TerminalId::new(value);
        assert_eq!(id.value(), value);
    }

    #[tokio::test]
    async fn test_terminal_session_write_no_child() {
        let mut session = TerminalSession::new(TerminalId::new(1));
        let result = session.write("test").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_terminal_session_kill_no_child() {
        let mut session = TerminalSession::new(TerminalId::new(1));
        let result = session.kill().await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_terminal_layout_default() {
        let layout = TerminalLayout::default();
        assert_eq!(layout, TerminalLayout::Horizontal);
    }

    #[test]
    fn test_terminal_state_default() {
        let state = TerminalState::default();
        assert_eq!(state, TerminalState::Stopped);
    }

    #[test]
    fn test_terminal_info_new() {
        let info = TerminalInfo::new(TerminalId::new(1), "/bin/bash");
        assert_eq!(info.id.value(), 1);
        assert_eq!(info.name, "term-1");
        assert_eq!(info.shell, "/bin/bash");
    }

    #[test]
    fn test_terminal_info_with_name() {
        let info = TerminalInfo::new(TerminalId::new(1), "/bin/bash").with_name("my-term");
        assert_eq!(info.name, "my-term");
    }

    #[test]
    fn test_terminal_info_with_layout() {
        let info =
            TerminalInfo::new(TerminalId::new(1), "/bin/bash").with_layout(TerminalLayout::Float);
        assert_eq!(info.layout, TerminalLayout::Float);
    }

    #[test]
    fn test_terminal_manager_new() {
        let mgr = TerminalManager::new();
        assert!(mgr.is_empty());
        assert_eq!(mgr.len(), 0);
    }

    #[test]
    fn test_terminal_manager_create() {
        let mut mgr = TerminalManager::new();
        let id = mgr.create(TerminalLayout::Horizontal);
        assert_eq!(id.value(), 1);
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_terminal_manager_create_named() {
        let mut mgr = TerminalManager::new();
        let id = mgr.create_named("build", TerminalLayout::Horizontal);
        let info = mgr.get(id).unwrap();
        assert_eq!(info.name, "build");
    }

    #[test]
    fn test_terminal_manager_active() {
        let mut mgr = TerminalManager::new();
        assert!(mgr.active().is_none());
        let id = mgr.create(TerminalLayout::Horizontal);
        assert_eq!(mgr.active(), Some(id));
    }

    #[test]
    fn test_terminal_manager_remove() {
        let mut mgr = TerminalManager::new();
        let id = mgr.create(TerminalLayout::Horizontal);
        let info = mgr.remove(id);
        assert!(info.is_some());
        assert!(mgr.is_empty());
    }

    #[test]
    fn test_terminal_manager_toggle() {
        let mut mgr = TerminalManager::new();
        let id = mgr.create(TerminalLayout::Horizontal);
        mgr.toggle(id);
        assert_eq!(mgr.get(id).unwrap().state, TerminalState::Running);
        mgr.toggle(id);
        assert_eq!(mgr.get(id).unwrap().state, TerminalState::Hidden);
    }

    #[test]
    fn test_terminal_manager_find_by_name() {
        let mut mgr = TerminalManager::new();
        mgr.create_named("test-term", TerminalLayout::Float);
        let info = mgr.find_by_name("test-term");
        assert!(info.is_some());
        assert_eq!(info.unwrap().name, "test-term");
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // DAP Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_dap_state_default() {
        assert_eq!(DapState::default(), DapState::Disconnected);
    }

    #[test]
    fn test_breakpoint_line() {
        let bp = Breakpoint::line(1, "main.rs", 10);
        assert_eq!(bp.id, 1);
        assert_eq!(bp.file, "main.rs");
        assert_eq!(bp.line, 10);
        assert!(bp.enabled);
        assert!(!bp.verified);
    }

    #[test]
    fn test_breakpoint_conditional() {
        let bp = Breakpoint::line(1, "main.rs", 10).with_condition("x > 5");
        match bp.kind {
            BreakpointKind::Conditional(cond) => assert_eq!(cond, "x > 5"),
            _ => panic!("Expected conditional"),
        }
    }

    #[test]
    fn test_breakpoint_logpoint() {
        let bp = Breakpoint::line(1, "main.rs", 10).as_logpoint("value = {x}");
        match bp.kind {
            BreakpointKind::Logpoint(msg) => assert_eq!(msg, "value = {x}"),
            _ => panic!("Expected logpoint"),
        }
    }

    #[test]
    fn test_stack_frame_new() {
        let frame = StackFrame::new(1, "main");
        assert_eq!(frame.id, 1);
        assert_eq!(frame.name, "main");
        assert!(frame.source.is_none());
    }

    #[test]
    fn test_stack_frame_at() {
        let frame = StackFrame::new(1, "main").at("main.rs", 10, 5);
        assert_eq!(frame.source, Some("main.rs".to_string()));
        assert_eq!(frame.line, 10);
        assert_eq!(frame.column, 5);
    }

    #[test]
    fn test_variable_new() {
        let var = Variable::new("x", "42");
        assert_eq!(var.name, "x");
        assert_eq!(var.value, "42");
    }

    #[test]
    fn test_variable_with_type() {
        let var = Variable::new("x", "42").with_type("i32");
        assert_eq!(var.type_name, Some("i32".to_string()));
    }

    #[test]
    fn test_variable_expandable() {
        let var = Variable::new("obj", "{...}").expandable(100);
        assert_eq!(var.variables_reference, 100);
    }

    #[test]
    fn test_dap_session_new() {
        let session = DapSession::new();
        assert_eq!(session.state, DapState::Disconnected);
        assert!(session.stack.is_empty());
    }

    #[test]
    fn test_dap_session_add_breakpoint() {
        let mut session = DapSession::new();
        let id = session.add_breakpoint("main.rs", 10);
        assert_eq!(id, 0);
        let bps: Vec<_> = session.breakpoints_for_file("main.rs");
        assert_eq!(bps.len(), 1);
    }

    #[test]
    fn test_dap_session_remove_breakpoint() {
        let mut session = DapSession::new();
        let id = session.add_breakpoint("main.rs", 10);
        let bp = session.remove_breakpoint(id);
        assert!(bp.is_some());
        assert!(session.breakpoints_for_file("main.rs").is_empty());
    }

    #[test]
    fn test_dap_session_toggle_breakpoint() {
        let mut session = DapSession::new();
        let id = session.add_breakpoint("main.rs", 10);
        assert!(session.breakpoints().next().unwrap().enabled);
        session.toggle_breakpoint(id);
        assert!(!session.breakpoints().next().unwrap().enabled);
    }

    #[test]
    fn test_dap_session_watches() {
        let mut session = DapSession::new();
        session.add_watch("x + y");
        session.add_watch("self.value");
        assert_eq!(session.watches().len(), 2);
    }

    #[test]
    fn test_dap_session_remove_watch() {
        let mut session = DapSession::new();
        session.add_watch("x");
        session.add_watch("y");
        let removed = session.remove_watch(0);
        assert_eq!(removed, Some("x".to_string()));
        assert_eq!(session.watches().len(), 1);
    }

    #[test]
    fn test_dap_session_stack() {
        let mut session = DapSession::new();
        session.push_frame(StackFrame::new(0, "main"));
        session.push_frame(StackFrame::new(1, "foo"));
        assert_eq!(session.stack.len(), 2);
        session.clear_stack();
        assert!(session.stack.is_empty());
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // tmux Integration Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_tmux_integration_default() {
        let tmux = TmuxIntegration::default();
        assert!(!tmux.inside_tmux);
    }

    #[test]
    fn test_tmux_passthrough_not_in_tmux() {
        let tmux = TmuxIntegration::default();
        let result = tmux.passthrough("\x1b[31m");
        assert_eq!(result, "\x1b[31m");
    }

    #[test]
    fn test_tmux_passthrough_in_tmux() {
        let tmux = TmuxIntegration {
            inside_tmux: true,
            version: None,
            true_color: false,
            osc52: true,
        };
        let result = tmux.passthrough("[31m");
        assert!(result.starts_with("\x1bPtmux;"));
        assert!(result.ends_with("\x1b\\"));
    }

    #[test]
    fn test_tmux_needs_passthrough() {
        let not_tmux = TmuxIntegration::default();
        assert!(!not_tmux.needs_passthrough());

        let in_tmux = TmuxIntegration {
            inside_tmux: true,
            ..Default::default()
        };
        assert!(in_tmux.needs_passthrough());
    }
}
