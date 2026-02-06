/// Full buffer features — buffer variables, local options, autocommands.

use std::collections::HashMap;

/// Buffer-local variable store.
#[derive(Debug, Default, Clone)]
pub struct BufferVariables { vars: HashMap<String, String> }

impl BufferVariables {
    pub fn new() -> Self { Self::default() }
    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) { self.vars.insert(key.into(), val.into()); }
    pub fn get(&self, key: &str) -> Option<&str> { self.vars.get(key).map(|s| s.as_str()) }
    pub fn remove(&mut self, key: &str) -> bool { self.vars.remove(key).is_some() }
    pub fn count(&self) -> usize { self.vars.len() }
    pub fn keys(&self) -> Vec<&str> { self.vars.keys().map(|k| k.as_str()).collect() }
}

/// Buffer-local option overrides.
#[derive(Debug, Clone)]
pub struct BufferLocalOptions {
    pub tabstop: Option<u16>,
    pub shift_width: Option<u16>,
    pub expand_tab: Option<bool>,
    pub text_width: Option<u16>,
    pub file_format: Option<FileFormat>,
    pub spell_lang: Option<String>,
}

impl Default for BufferLocalOptions {
    fn default() -> Self {
        Self { tabstop: None, shift_width: None, expand_tab: None,
            text_width: None, file_format: None, spell_lang: None }
    }
}

/// File format (line endings).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat { Unix, Dos, Mac }

impl FileFormat {
    pub fn line_ending(&self) -> &'static str {
        match self { FileFormat::Unix => "\n", FileFormat::Dos => "\r\n", FileFormat::Mac => "\r" }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s { "unix" => Some(Self::Unix), "dos" => Some(Self::Dos), "mac" => Some(Self::Mac), _ => None }
    }
}

/// Buffer auto-command event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufEvent { BufEnter, BufLeave, BufRead, BufWrite, BufNew, BufDelete, BufWinEnter, BufWinLeave, BufModified }

/// Auto-command handler entry.
#[derive(Debug, Clone)]
pub struct AutoCmd { pub event: BufEvent, pub pattern: String, pub command: String }

/// Auto-command registry.
#[derive(Debug, Default)]
pub struct AutoCmdRegistry { cmds: Vec<AutoCmd> }

impl AutoCmdRegistry {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, event: BufEvent, pattern: impl Into<String>, cmd: impl Into<String>) {
        self.cmds.push(AutoCmd { event, pattern: pattern.into(), command: cmd.into() });
    }
    pub fn for_event(&self, event: BufEvent) -> Vec<&AutoCmd> {
        self.cmds.iter().filter(|c| c.event == event).collect()
    }
    pub fn remove_pattern(&mut self, pattern: &str) { self.cmds.retain(|c| c.pattern != pattern); }
    pub fn count(&self) -> usize { self.cmds.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_variables() {
        let mut v = BufferVariables::new();
        v.set("filetype", "rust"); assert_eq!(v.get("filetype"), Some("rust"));
        v.remove("filetype"); assert_eq!(v.count(), 0);
    }

    #[test]
    fn file_format_endings() {
        assert_eq!(FileFormat::Unix.line_ending(), "\n");
        assert_eq!(FileFormat::Dos.line_ending(), "\r\n");
        assert_eq!(FileFormat::Mac.line_ending(), "\r");
    }

    #[test]
    fn file_format_parse() {
        assert_eq!(FileFormat::from_str("unix"), Some(FileFormat::Unix));
        assert_eq!(FileFormat::from_str("invalid"), None);
    }

    #[test]
    fn autocmd_registry() {
        let mut r = AutoCmdRegistry::new();
        r.add(BufEvent::BufEnter, "*.rs", "setlocal tabstop=4");
        r.add(BufEvent::BufWrite, "*", "echo 'saved'");
        assert_eq!(r.for_event(BufEvent::BufEnter).len(), 1);
        assert_eq!(r.count(), 2);
    }

    #[test]
    fn autocmd_remove_pattern() {
        let mut r = AutoCmdRegistry::new();
        r.add(BufEvent::BufRead, "*.py", "set ft=python");
        r.remove_pattern("*.py"); assert_eq!(r.count(), 0);
    }

    #[test]
    fn local_options_default() {
        let o = BufferLocalOptions::default();
        assert!(o.tabstop.is_none()); assert!(o.expand_tab.is_none());
    }

    #[test]
    fn variable_keys() {
        let mut v = BufferVariables::new();
        v.set("a", "1"); v.set("b", "2");
        assert_eq!(v.keys().len(), 2);
    }
}
