//! Terminal/PTY service — embedded terminal emulation.

use kjxlkj_core_types::Size;

/// Unique identifier for a terminal instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(pub u64);

/// State of a terminal session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState {
    /// The terminal is running.
    Running,
    /// The terminal process has exited.
    Exited(i32),
}

/// Configuration for spawning a new terminal.
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub shell: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub cwd: Option<String>,
    pub size: Size,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
            args: Vec::new(),
            env: Vec::new(),
            cwd: None,
            size: Size::new(80, 24),
        }
    }
}

/// Manages embedded terminal/PTY sessions.
pub struct TerminalService {
    next_id: u64,
}

impl TerminalService {
    pub fn new() -> Self {
        Self { next_id: 1 }
    }

    /// Spawn a new terminal session with the given configuration.
    pub async fn spawn(&mut self, config: TerminalConfig) -> anyhow::Result<TerminalId> {
        let id = TerminalId(self.next_id);
        self.next_id += 1;
        tracing::info!(?id, shell = %config.shell, "spawning terminal");
        Ok(id)
    }

    /// Resize a terminal session.
    pub async fn resize(&mut self, _id: TerminalId, _size: Size) -> anyhow::Result<()> {
        Ok(())
    }

    /// Write input data to a terminal.
    pub async fn write(&mut self, _id: TerminalId, _data: &[u8]) -> anyhow::Result<()> {
        Ok(())
    }

    /// Close a terminal session.
    pub async fn close(&mut self, id: TerminalId) -> anyhow::Result<()> {
        tracing::info!(?id, "closing terminal");
        Ok(())
    }
}

impl Default for TerminalService {
    fn default() -> Self {
        Self::new()
    }
}
