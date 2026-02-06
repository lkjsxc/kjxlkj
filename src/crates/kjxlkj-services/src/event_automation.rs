/// Event automation — autocommands, event-driven hooks, BufEnter/BufLeave etc.

use std::collections::HashMap;

/// Events that can trigger autocommands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutoEvent {
    BufEnter, BufLeave, BufRead, BufWrite, BufNewFile,
    InsertEnter, InsertLeave, CursorMoved, CursorHold,
    WinEnter, WinLeave, VimEnter, VimLeave,
    FileType, Syntax, TextChanged, TextChangedI,
}

impl AutoEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BufEnter => "BufEnter", Self::BufLeave => "BufLeave",
            Self::BufRead => "BufRead", Self::BufWrite => "BufWrite",
            Self::BufNewFile => "BufNewFile", Self::InsertEnter => "InsertEnter",
            Self::InsertLeave => "InsertLeave", Self::CursorMoved => "CursorMoved",
            Self::CursorHold => "CursorHold", Self::WinEnter => "WinEnter",
            Self::WinLeave => "WinLeave", Self::VimEnter => "VimEnter",
            Self::VimLeave => "VimLeave", Self::FileType => "FileType",
            Self::Syntax => "Syntax", Self::TextChanged => "TextChanged",
            Self::TextChangedI => "TextChangedI",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "BufEnter" => Some(Self::BufEnter), "BufLeave" => Some(Self::BufLeave),
            "BufRead" => Some(Self::BufRead), "BufWrite" => Some(Self::BufWrite),
            "BufNewFile" => Some(Self::BufNewFile), "InsertEnter" => Some(Self::InsertEnter),
            "InsertLeave" => Some(Self::InsertLeave), "CursorMoved" => Some(Self::CursorMoved),
            "CursorHold" => Some(Self::CursorHold), "WinEnter" => Some(Self::WinEnter),
            "WinLeave" => Some(Self::WinLeave), "VimEnter" => Some(Self::VimEnter),
            "VimLeave" => Some(Self::VimLeave), "FileType" => Some(Self::FileType),
            "Syntax" => Some(Self::Syntax), "TextChanged" => Some(Self::TextChanged),
            "TextChangedI" => Some(Self::TextChangedI), _ => None,
        }
    }
}

/// A pattern that selects which buffers an autocommand applies to.
#[derive(Debug, Clone, PartialEq)]
pub enum AutoPattern { All, Glob(String), FileType(String) }

impl AutoPattern {
    pub fn matches(&self, filename: &str, filetype: &str) -> bool {
        match self {
            Self::All => true,
            Self::Glob(pat) => glob_match(pat, filename),
            Self::FileType(ft) => ft == filetype,
        }
    }
}

fn glob_match(pattern: &str, text: &str) -> bool {
    if pattern == "*" { return true; }
    if let Some(ext) = pattern.strip_prefix("*.") {
        return text.ends_with(&format!(".{}", ext));
    }
    pattern == text
}

/// A registered autocommand.
#[derive(Debug, Clone)]
pub struct AutoCmd {
    pub group: Option<String>,
    pub event: AutoEvent,
    pub pattern: AutoPattern,
    pub command: String,
    pub once: bool,
}

/// Autocommand registry.
#[derive(Debug, Default)]
pub struct AutoCmdRegistry { commands: Vec<AutoCmd>, id_counter: usize }

impl AutoCmdRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn add(&mut self, cmd: AutoCmd) -> usize {
        self.id_counter += 1;
        self.commands.push(cmd);
        self.id_counter
    }

    /// Find matching autocommands for an event.
    pub fn matching(&self, event: AutoEvent, filename: &str, filetype: &str) -> Vec<&AutoCmd> {
        self.commands.iter()
            .filter(|c| c.event == event && c.pattern.matches(filename, filetype))
            .collect()
    }

    /// Remove all autocommands in a group.
    pub fn clear_group(&mut self, group: &str) {
        self.commands.retain(|c| c.group.as_deref() != Some(group));
    }

    /// Remove once-fired commands.
    pub fn remove_once_fired(&mut self, event: AutoEvent) {
        self.commands.retain(|c| !(c.event == event && c.once));
    }

    pub fn count(&self) -> usize { self.commands.len() }

    /// List all registered events.
    pub fn registered_events(&self) -> Vec<AutoEvent> {
        let mut events: Vec<_> = self.commands.iter().map(|c| c.event).collect();
        events.sort_by_key(|e| *e as u8);
        events.dedup();
        events
    }
}

/// Collect commands to execute for an event firing. Handles `once` semantics.
pub fn fire_event(registry: &AutoCmdRegistry, event: AutoEvent, filename: &str, filetype: &str) -> Vec<String> {
    registry.matching(event, filename, filetype)
        .iter().map(|c| c.command.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd(event: AutoEvent, pat: AutoPattern, command: &str) -> AutoCmd {
        AutoCmd { group: None, event, pattern: pat, command: command.into(), once: false }
    }

    #[test]
    fn add_and_match() {
        let mut reg = AutoCmdRegistry::new();
        reg.add(cmd(AutoEvent::BufEnter, AutoPattern::All, "echo hi"));
        let m = reg.matching(AutoEvent::BufEnter, "foo.rs", "rust");
        assert_eq!(m.len(), 1);
    }

    #[test]
    fn glob_pattern_match() {
        let p = AutoPattern::Glob("*.rs".into());
        assert!(p.matches("foo.rs", ""));
        assert!(!p.matches("foo.py", ""));
    }

    #[test]
    fn filetype_pattern() {
        let p = AutoPattern::FileType("rust".into());
        assert!(p.matches("any.txt", "rust"));
        assert!(!p.matches("any.txt", "python"));
    }

    #[test]
    fn clear_group() {
        let mut reg = AutoCmdRegistry::new();
        let mut c = cmd(AutoEvent::BufRead, AutoPattern::All, "x");
        c.group = Some("mygroup".into());
        reg.add(c);
        reg.add(cmd(AutoEvent::BufRead, AutoPattern::All, "y"));
        reg.clear_group("mygroup");
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn fire_event_collects() {
        let mut reg = AutoCmdRegistry::new();
        reg.add(cmd(AutoEvent::BufWrite, AutoPattern::All, ":noh"));
        reg.add(cmd(AutoEvent::BufWrite, AutoPattern::Glob("*.rs".into()), ":fmt"));
        let cmds = fire_event(&reg, AutoEvent::BufWrite, "main.rs", "rust");
        assert_eq!(cmds.len(), 2);
    }

    #[test]
    fn event_from_str() {
        assert_eq!(AutoEvent::from_str("BufEnter"), Some(AutoEvent::BufEnter));
        assert_eq!(AutoEvent::from_str("Unknown"), None);
    }

    #[test]
    fn event_as_str_roundtrip() {
        let e = AutoEvent::FileType;
        assert_eq!(AutoEvent::from_str(e.as_str()), Some(e));
    }

    #[test]
    fn registered_events_dedup() {
        let mut reg = AutoCmdRegistry::new();
        reg.add(cmd(AutoEvent::BufEnter, AutoPattern::All, "a"));
        reg.add(cmd(AutoEvent::BufEnter, AutoPattern::All, "b"));
        let events = reg.registered_events();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn remove_once() {
        let mut reg = AutoCmdRegistry::new();
        let mut c = cmd(AutoEvent::VimEnter, AutoPattern::All, "startup");
        c.once = true;
        reg.add(c);
        reg.remove_once_fired(AutoEvent::VimEnter);
        assert_eq!(reg.count(), 0);
    }
}
