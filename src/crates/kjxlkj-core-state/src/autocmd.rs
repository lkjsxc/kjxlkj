//! Autocommand events.
//!
//! Event-driven automation for editor actions.

use std::collections::HashMap;

pub use crate::autocmd_types::{AutoCmd, AutoEvent};

/// Autocommand manager.
#[derive(Debug, Default)]
pub struct AutoCmdManager {
    /// Commands by event.
    commands: HashMap<AutoEvent, Vec<AutoCmd>>,
    /// Defined groups.
    groups: Vec<String>,
}

impl AutoCmdManager {
    /// Creates a new manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Defines a group.
    pub fn define_group(&mut self, name: &str) {
        if !self.groups.contains(&name.to_string()) {
            self.groups.push(name.to_string());
        }
    }

    /// Adds an autocommand.
    pub fn add(&mut self, cmd: AutoCmd) {
        self.commands.entry(cmd.event).or_default().push(cmd);
    }

    /// Removes all commands for a group.
    pub fn clear_group(&mut self, group: &str) {
        for cmds in self.commands.values_mut() {
            cmds.retain(|c| c.group.as_deref() != Some(group));
        }
    }

    /// Gets commands for an event.
    pub fn get(&self, event: AutoEvent, filename: &str) -> Vec<&AutoCmd> {
        self.commands
            .get(&event)
            .map(|cmds| cmds.iter().filter(|c| c.matches(filename)).collect())
            .unwrap_or_default()
    }

    /// Returns command count.
    pub fn len(&self) -> usize {
        self.commands.values().map(|v| v.len()).sum()
    }

    /// Returns whether empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_event_name() {
        assert_eq!(AutoEvent::BufEnter.name(), "BufEnter");
    }

    #[test]
    fn test_autocmd_new() {
        let cmd = AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "setlocal ft=rust");
        assert_eq!(cmd.pattern, "*.rs");
    }

    #[test]
    fn test_autocmd_matches() {
        let cmd = AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "echo");
        assert!(cmd.matches("main.rs"));
        assert!(!cmd.matches("main.py"));
    }

    #[test]
    fn test_autocmd_matches_star() {
        let cmd = AutoCmd::new(AutoEvent::BufEnter, "*", "echo");
        assert!(cmd.matches("anything.txt"));
    }

    #[test]
    fn test_autocmd_manager_add() {
        let mut mgr = AutoCmdManager::new();
        mgr.add(AutoCmd::new(AutoEvent::BufEnter, "*", "echo"));
        assert_eq!(mgr.len(), 1);
    }

    #[test]
    fn test_autocmd_manager_get() {
        let mut mgr = AutoCmdManager::new();
        mgr.add(AutoCmd::new(AutoEvent::BufReadPost, "*.rs", "echo"));
        let cmds = mgr.get(AutoEvent::BufReadPost, "main.rs");
        assert_eq!(cmds.len(), 1);
    }
}
