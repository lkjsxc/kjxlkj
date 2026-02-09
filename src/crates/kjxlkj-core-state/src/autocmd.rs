//! Autocommand infrastructure.

use crate::EditorState;

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

#[derive(Debug, Clone)]
pub struct AutoCmd {
    pub event: AutoEvent,
    pub pattern: String,
    pub command: String,
    pub group: String,
    pub once: bool,
    pub nested: bool,
}

#[derive(Debug, Default)]
pub struct AutoCmdRegistry {
    commands: Vec<AutoCmd>,
}

impl AutoCmdRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, cmd: AutoCmd) {
        self.commands.push(cmd);
    }

    pub fn clear_group(&mut self, group: &str) {
        self.commands.retain(|c| c.group != group);
    }

    pub fn clear_all(&mut self) {
        self.commands.clear();
    }

    pub fn matching(&self, event: AutoEvent, path: &str) -> Vec<&AutoCmd> {
        self.commands
            .iter()
            .filter(|c| c.event == event && (c.pattern.is_empty() || glob_match(&c.pattern, path)))
            .collect()
    }

    pub fn remove_once_fired(&mut self, event: AutoEvent, path: &str) {
        self.commands.retain(|c| {
            !(c.once && c.event == event && (c.pattern.is_empty() || glob_match(&c.pattern, path)))
        });
    }
}

fn glob_match(pattern: &str, path: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if let Some(ext) = pattern.strip_prefix("*.") {
        return path.ends_with(&format!(".{}", ext));
    }
    let name = std::path::Path::new(path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    pattern == name || pattern == path
}

impl EditorState {
    pub(crate) fn fire_autocmd(&mut self, event: AutoEvent) {
        let path = self
            .active_buffer()
            .and_then(|b| b.path.as_ref().map(|p| p.to_string_lossy().to_string()))
            .unwrap_or_default();
        let cmds: Vec<String> = self
            .autocmds
            .matching(event, &path)
            .iter()
            .map(|c| c.command.clone())
            .collect();
        self.autocmds.remove_once_fired(event, &path);
        for cmd in cmds {
            if let Some(action) = crate::dispatch_command(&cmd) {
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
        let m = reg.matching(AutoEvent::BufRead, "main.rs");
        assert_eq!(m.len(), 1);
        let m2 = reg.matching(AutoEvent::BufRead, "main.py");
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
