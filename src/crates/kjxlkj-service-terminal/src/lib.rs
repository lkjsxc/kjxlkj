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
}
