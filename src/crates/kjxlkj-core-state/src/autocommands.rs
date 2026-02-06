//! Autocommand system: BufRead, BufWrite, FileType events.

use std::collections::HashMap;

/// Autocommand event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoEvent {
    BufNewFile,
    BufRead,
    BufReadPost,
    BufWrite,
    BufWritePre,
    BufWritePost,
    BufEnter,
    BufLeave,
    FileType,
    InsertEnter,
    InsertLeave,
    VimEnter,
    VimLeave,
}

/// A single autocommand.
#[derive(Debug, Clone)]
pub struct AutoCmd {
    /// Event that triggers this autocommand.
    pub event: AutoEvent,
    /// File pattern (glob-style, e.g., "*.rs").
    pub pattern: String,
    /// Command to execute.
    pub command: String,
    /// Optional group name.
    pub group: Option<String>,
}

/// Storage for autocommands.
#[derive(Debug, Clone, Default)]
pub struct AutoCmdTable {
    cmds: Vec<AutoCmd>,
    groups: HashMap<String, Vec<usize>>,
}

impl AutoCmdTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an autocommand.
    pub fn add(
        &mut self,
        event: AutoEvent,
        pattern: &str,
        command: &str,
        group: Option<&str>,
    ) {
        let idx = self.cmds.len();
        self.cmds.push(AutoCmd {
            event,
            pattern: pattern.to_string(),
            command: command.to_string(),
            group: group.map(|s| s.to_string()),
        });
        if let Some(g) = group {
            self.groups
                .entry(g.to_string())
                .or_default()
                .push(idx);
        }
    }

    /// Get commands matching an event and filename.
    pub fn matching(
        &self,
        event: AutoEvent,
        filename: &str,
    ) -> Vec<String> {
        self.cmds
            .iter()
            .filter(|c| {
                c.event == event
                    && pattern_matches(&c.pattern, filename)
            })
            .map(|c| c.command.clone())
            .collect()
    }

    /// Remove all autocommands in a group.
    pub fn clear_group(&mut self, group: &str) {
        let indices: Vec<usize> = self
            .groups
            .remove(group)
            .unwrap_or_default();
        // Mark commands for removal by clearing them
        for &idx in indices.iter().rev() {
            if idx < self.cmds.len() {
                self.cmds.remove(idx);
            }
        }
        // Rebuild group indices
        self.rebuild_groups();
    }

    /// Remove all autocommands.
    pub fn clear_all(&mut self) {
        self.cmds.clear();
        self.groups.clear();
    }

    /// List all autocommands as strings.
    pub fn display(&self) -> String {
        if self.cmds.is_empty() {
            return "No autocommands".into();
        }
        self.cmds
            .iter()
            .map(|c| {
                let group = c
                    .group
                    .as_deref()
                    .unwrap_or("");
                format!(
                    "{}{:?}\t{}\t{}",
                    if group.is_empty() {
                        String::new()
                    } else {
                        format!("[{}] ", group)
                    },
                    c.event,
                    c.pattern,
                    c.command,
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn rebuild_groups(&mut self) {
        self.groups.clear();
        for (i, cmd) in self.cmds.iter().enumerate() {
            if let Some(ref g) = cmd.group {
                self.groups
                    .entry(g.clone())
                    .or_default()
                    .push(i);
            }
        }
    }
}

/// Simple glob-style pattern matching.
fn pattern_matches(pattern: &str, filename: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.starts_with("*.") {
        let ext = &pattern[2..];
        return filename.ends_with(&format!(".{}", ext));
    }
    filename == pattern
}

/// Parse an autocommand event name.
pub fn parse_event(name: &str) -> Option<AutoEvent> {
    match name {
        "BufNewFile" => Some(AutoEvent::BufNewFile),
        "BufRead" | "BufReadPost" => {
            Some(AutoEvent::BufRead)
        }
        "BufWrite" | "BufWritePre" => {
            Some(AutoEvent::BufWrite)
        }
        "BufWritePost" => Some(AutoEvent::BufWritePost),
        "BufEnter" => Some(AutoEvent::BufEnter),
        "BufLeave" => Some(AutoEvent::BufLeave),
        "FileType" => Some(AutoEvent::FileType),
        "InsertEnter" => Some(AutoEvent::InsertEnter),
        "InsertLeave" => Some(AutoEvent::InsertLeave),
        "VimEnter" => Some(AutoEvent::VimEnter),
        "VimLeave" => Some(AutoEvent::VimLeave),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_match() {
        let mut t = AutoCmdTable::new();
        t.add(
            AutoEvent::BufRead,
            "*.rs",
            "set tabstop=4",
            None,
        );
        let cmds = t.matching(AutoEvent::BufRead, "main.rs");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0], "set tabstop=4");
    }

    #[test]
    fn no_match_wrong_ext() {
        let mut t = AutoCmdTable::new();
        t.add(
            AutoEvent::BufRead,
            "*.rs",
            "set tabstop=4",
            None,
        );
        let cmds = t.matching(AutoEvent::BufRead, "main.py");
        assert!(cmds.is_empty());
    }

    #[test]
    fn wildcard_matches_all() {
        let mut t = AutoCmdTable::new();
        t.add(
            AutoEvent::BufRead,
            "*",
            "echo loaded",
            None,
        );
        let cmds =
            t.matching(AutoEvent::BufRead, "anything.txt");
        assert_eq!(cmds.len(), 1);
    }

    #[test]
    fn group_clear() {
        let mut t = AutoCmdTable::new();
        t.add(
            AutoEvent::BufRead,
            "*",
            "cmd1",
            Some("mygroup"),
        );
        t.add(
            AutoEvent::BufRead,
            "*",
            "cmd2",
            None,
        );
        t.clear_group("mygroup");
        let cmds = t.matching(AutoEvent::BufRead, "a.txt");
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0], "cmd2");
    }

    #[test]
    fn parse_events() {
        assert_eq!(
            parse_event("BufRead"),
            Some(AutoEvent::BufRead)
        );
        assert_eq!(
            parse_event("FileType"),
            Some(AutoEvent::FileType)
        );
        assert_eq!(parse_event("Invalid"), None);
    }

    #[test]
    fn display() {
        let mut t = AutoCmdTable::new();
        t.add(
            AutoEvent::FileType,
            "*.py",
            "set tabstop=4",
            None,
        );
        let s = t.display();
        assert!(s.contains("*.py"));
        assert!(s.contains("tabstop"));
    }
}
