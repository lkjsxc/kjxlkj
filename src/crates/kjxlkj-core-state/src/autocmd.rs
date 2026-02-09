//! Autocommand infrastructure: event-driven hooks.

use crate::EditorState;

/// Autocommand event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoEvent {
    BufNew,
    BufRead,
    BufWrite,
    BufWritePost,
    BufEnter,
    BufLeave,
    BufDelete,
    FileType,
    WinEnter,
    WinLeave,
    WinNew,
    WinClosed,
    InsertEnter,
    InsertLeave,
    TextChanged,
    CursorMoved,
    VimEnter,
    VimLeave,
}

/// A single autocommand registration.
#[derive(Debug, Clone)]
pub struct AutoCmd {
    /// Event that triggers this command.
    pub event: AutoEvent,
    /// File pattern (glob, e.g. "*.rs"). Empty = all.
    pub pattern: String,
    /// Command to execute.
    pub command: String,
    /// Group name (for clearing).
    pub group: String,
    /// Fire only once then auto-remove.
    pub once: bool,
    /// Allow nested event triggering.
    pub nested: bool,
}

/// Autocommand registry.
#[derive(Debug, Default)]
pub struct AutoCmdRegistry {
    /// Registered autocommands.
    commands: Vec<AutoCmd>,
}

impl AutoCmdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an autocommand.
    pub fn add(&mut self, cmd: AutoCmd) {
        self.commands.push(cmd);
    }

    /// Clear all commands in a group.
    pub fn clear_group(&mut self, group: &str) {
        self.commands
            .retain(|c| c.group != group);
    }

    /// Clear all autocommands.
    pub fn clear_all(&mut self) {
        self.commands.clear();
    }

    /// Get commands matching an event and optional
    /// file path.
    pub fn matching(
        &self,
        event: AutoEvent,
        path: &str,
    ) -> Vec<&AutoCmd> {
        self.commands
            .iter()
            .filter(|c| {
                c.event == event
                    && (c.pattern.is_empty()
                        || glob_match(
                            &c.pattern, path,
                        ))
            })
            .collect()
    }

    /// Remove once-only commands that have fired.
    pub fn remove_once_fired(
        &mut self,
        event: AutoEvent,
        path: &str,
    ) {
        self.commands.retain(|c| {
            !(c.once
                && c.event == event
                && (c.pattern.is_empty()
                    || glob_match(&c.pattern, path)))
        });
    }
}

/// Simple glob matching for autocommand patterns.
fn glob_match(pattern: &str, path: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    // *.ext matching
    if let Some(ext) = pattern.strip_prefix("*.") {
        return path.ends_with(&format!(".{}", ext));
    }
    // Exact match
    let name = std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    pattern == name || pattern == path
}

impl EditorState {
    /// Fire autocommand event, executing matching
    /// commands.
    pub(crate) fn fire_autocmd(
        &mut self,
        event: AutoEvent,
    ) {
        let path = self
            .active_buffer()
            .and_then(|b| {
                b.path.as_ref().map(|p| {
                    p.to_string_lossy().to_string()
                })
            })
            .unwrap_or_default();
        let cmds: Vec<String> = self
            .autocmds
            .matching(event, &path)
            .iter()
            .map(|c| c.command.clone())
            .collect();
        self.autocmds.remove_once_fired(event, &path);
        for cmd in cmds {
            if let Some(action) =
                crate::dispatch_command(&cmd)
            {
                self.dispatch(action);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn glob_match_star_ext() {
        assert!(glob_match("*.rs", "main.rs"));
        assert!(!glob_match("*.rs", "main.py"));
    }

    #[test]
    fn registry_add_and_match() {
        let mut reg = AutoCmdRegistry::new();
        reg.add(AutoCmd {
            event: AutoEvent::BufRead,
            pattern: "*.rs".to_string(),
            command: "set filetype=rust".to_string(),
            group: "ft".to_string(),
            once: false,
            nested: false,
        });
        let m = reg.matching(
            AutoEvent::BufRead,
            "main.rs",
        );
        assert_eq!(m.len(), 1);
        let m2 = reg.matching(
            AutoEvent::BufRead,
            "main.py",
        );
        assert_eq!(m2.len(), 0);
    }

    #[test]
    fn clear_group() {
        let mut reg = AutoCmdRegistry::new();
        reg.add(AutoCmd {
            event: AutoEvent::BufWrite,
            pattern: "".to_string(),
            command: "noh".to_string(),
            group: "g1".to_string(),
            once: false,
            nested: false,
        });
        assert_eq!(reg.commands.len(), 1);
        reg.clear_group("g1");
        assert_eq!(reg.commands.len(), 0);
    }
}
