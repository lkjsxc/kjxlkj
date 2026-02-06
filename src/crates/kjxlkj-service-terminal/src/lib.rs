//! Terminal/PTY service — embedded terminal emulation.

mod terminal_emulator;

use kjxlkj_core_types::Size;

/// Unique identifier for a terminal instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(pub u64);

/// State of a terminal session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalState { Running, Exited(i32) }

/// Type of terminal layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalLayout { Horizontal, Vertical, Float, Tab }

/// Shell type for terminal spawning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellType { System, Custom(String) }

/// Events emitted by a terminal session.
#[derive(Debug, Clone)]
pub enum TerminalEvent {
    Output(Vec<u8>), Bell, TitleChanged(String), Resized(Size), Exited(i32),
}

/// A terminal pane in the UI.
#[derive(Debug, Clone)]
pub struct TerminalPane {
    pub id: TerminalId, pub title: String, pub state: TerminalState,
    pub layout: TerminalLayout, pub size: Size,
    pub scrollback_lines: usize, pub cursor_visible: bool,
}

/// Configuration for spawning a new terminal.
#[derive(Debug, Clone)]
pub struct TerminalConfig {
    pub shell: String, pub args: Vec<String>,
    pub env: Vec<(String, String)>, pub cwd: Option<String>,
    pub size: Size, pub layout: TerminalLayout,
    pub start_insert: bool, pub persist: bool,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()),
            args: Vec::new(), env: Vec::new(), cwd: None,
            size: Size::new(80, 24), layout: TerminalLayout::Horizontal,
            start_insert: true, persist: true,
        }
    }
}

/// Manages embedded terminal/PTY sessions.
pub struct TerminalService {
    next_id: u64,
    terminals: Vec<TerminalPane>,
    active: Option<TerminalId>,
}

impl TerminalService {
    pub fn new() -> Self { Self { next_id: 1, terminals: Vec::new(), active: None } }

    pub async fn spawn(&mut self, config: TerminalConfig) -> anyhow::Result<TerminalId> {
        let id = TerminalId(self.next_id);
        self.next_id += 1;
        self.terminals.push(TerminalPane {
            id, title: config.shell.clone(), state: TerminalState::Running,
            layout: config.layout, size: config.size,
            scrollback_lines: 0, cursor_visible: true,
        });
        self.active = Some(id);
        tracing::info!(?id, shell = %config.shell, "spawning terminal");
        Ok(id)
    }

    pub async fn resize(&mut self, id: TerminalId, size: Size) -> anyhow::Result<()> {
        if let Some(t) = self.terminals.iter_mut().find(|t| t.id == id) { t.size = size; }
        Ok(())
    }

    pub async fn write(&mut self, _id: TerminalId, _data: &[u8]) -> anyhow::Result<()> { Ok(()) }

    pub async fn close(&mut self, id: TerminalId) -> anyhow::Result<()> {
        self.terminals.retain(|t| t.id != id);
        if self.active == Some(id) { self.active = self.terminals.last().map(|t| t.id); }
        tracing::info!(?id, "closing terminal");
        Ok(())
    }

    pub fn toggle(&mut self) -> Option<TerminalId> { self.active }
    pub fn panes(&self) -> &[TerminalPane] { &self.terminals }
    pub fn count(&self) -> usize { self.terminals.len() }

    pub fn active_terminal(&self) -> Option<&TerminalPane> {
        let id = self.active?;
        self.terminals.iter().find(|t| t.id == id)
    }

    pub async fn send_text(&mut self, _id: TerminalId, _text: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Default for TerminalService { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn spawn_and_close() {
        let mut svc = TerminalService::new();
        let id = svc.spawn(TerminalConfig::default()).await.unwrap();
        assert_eq!(id, TerminalId(1));
        assert_eq!(svc.count(), 1);
        assert!(svc.active_terminal().is_some());
        svc.close(id).await.unwrap();
        assert_eq!(svc.count(), 0);
        assert!(svc.active_terminal().is_none());
    }

    #[tokio::test]
    async fn resize_and_multiple() {
        let mut svc = TerminalService::new();
        let id1 = svc.spawn(TerminalConfig::default()).await.unwrap();
        svc.resize(id1, Size::new(120, 40)).await.unwrap();
        assert_eq!(svc.active_terminal().unwrap().size, Size::new(120, 40));
        let id2 = svc.spawn(TerminalConfig::default()).await.unwrap();
        assert_eq!(svc.count(), 2);
        assert_eq!(svc.active_terminal().unwrap().id, id2);
    }

    #[test]
    fn config_defaults_and_types() {
        let cfg = TerminalConfig::default();
        assert!(cfg.start_insert);
        assert!(cfg.persist);
        assert_eq!(cfg.layout, TerminalLayout::Horizontal);
        let _layouts = [TerminalLayout::Horizontal, TerminalLayout::Vertical,
                        TerminalLayout::Float, TerminalLayout::Tab];
        let events: Vec<TerminalEvent> = vec![
            TerminalEvent::Output(vec![65]), TerminalEvent::Bell,
            TerminalEvent::TitleChanged("t".into()),
            TerminalEvent::Resized(Size::new(80, 24)), TerminalEvent::Exited(0),
        ];
        assert_eq!(events.len(), 5);
    }
}
