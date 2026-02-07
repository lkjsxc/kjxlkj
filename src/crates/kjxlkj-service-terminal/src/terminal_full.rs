//! Terminal pane management, tmux integration types.

use crate::terminal_grid::TerminalGrid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A terminal pane with its grid and scrollback.
pub struct TerminalPane {
    pub id: u32,
    pub title: String,
    pub grid: TerminalGrid,
    pub scrollback: Vec<Vec<crate::terminal_grid::Cell>>,
    pub scroll_region: Option<(u16, u16)>,
}

impl TerminalPane {
    pub fn new(id: u32, title: &str, width: u16, height: u16) -> Self {
        Self {
            id,
            title: title.to_string(),
            grid: TerminalGrid::new(width, height),
            scrollback: Vec::new(),
            scroll_region: None,
        }
    }
}

/// Manages multiple terminal panes.
#[derive(Default)]
pub struct PaneManager {
    panes: HashMap<u32, TerminalPane>,
    active: Option<u32>,
    next_id: u32,
}

impl PaneManager {
    pub fn new() -> Self {
        Self {
            panes: HashMap::new(),
            active: None,
            next_id: 1,
        }
    }

    pub fn create_pane(&mut self, title: &str, width: u16, height: u16) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        let pane = TerminalPane::new(id, title, width, height);
        self.panes.insert(id, pane);
        if self.active.is_none() {
            self.active = Some(id);
        }
        id
    }

    pub fn close_pane(&mut self, id: u32) {
        self.panes.remove(&id);
        if self.active == Some(id) {
            self.active = self.panes.keys().next().copied();
        }
    }

    pub fn get(&self, id: u32) -> Option<&TerminalPane> {
        self.panes.get(&id)
    }

    pub fn set_active(&mut self, id: u32) {
        if self.panes.contains_key(&id) {
            self.active = Some(id);
        }
    }

    pub fn list(&self) -> Vec<u32> {
        let mut ids: Vec<u32> = self.panes.keys().copied().collect();
        ids.sort();
        ids
    }
}

/// Tmux session state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmuxState {
    pub session_name: String,
    pub windows: Vec<String>,
    pub attached: bool,
}

/// Tmux actions that can be dispatched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TmuxAction {
    NewWindow,
    CloseWindow,
    SplitH,
    SplitV,
    SelectPane(u32),
    ResizePane(u16, u16),
    SendKeys(String),
    DetachClient,
}

/// Map a key name to tmux key syntax.
pub fn map_tmux_key(key: &str) -> String {
    match key {
        "Enter" | "enter" => "Enter".to_string(),
        "Escape" | "escape" | "esc" => "Escape".to_string(),
        "Tab" | "tab" => "Tab".to_string(),
        "Backspace" | "backspace" => "BSpace".to_string(),
        "Up" | "up" => "Up".to_string(),
        "Down" | "down" => "Down".to_string(),
        "Left" | "left" => "Left".to_string(),
        "Right" | "right" => "Right".to_string(),
        other => other.to_string(),
    }
}

/// Compute scrollback capacity (capped at 10,000 lines).
pub fn scrollback_capacity(height: u16) -> usize {
    let base = (height as usize) * 10;
    base.min(10_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pane_lifecycle() {
        let mut pm = PaneManager::new();
        let id1 = pm.create_pane("shell", 80, 24);
        let id2 = pm.create_pane("build", 80, 24);
        assert_eq!(pm.list().len(), 2);
        pm.close_pane(id1);
        assert_eq!(pm.list(), vec![id2]);
    }

    #[test]
    fn tmux_key_mapping() {
        assert_eq!(map_tmux_key("Enter"), "Enter");
        assert_eq!(map_tmux_key("backspace"), "BSpace");
        assert_eq!(map_tmux_key("x"), "x");
    }

    #[test]
    fn scrollback_cap() {
        assert_eq!(scrollback_capacity(24), 240);
        assert_eq!(scrollback_capacity(2000), 10_000);
    }
}
