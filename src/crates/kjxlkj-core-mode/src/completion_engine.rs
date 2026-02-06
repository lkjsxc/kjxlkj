/// Command-line completion engine — tab/path/command/option completion.
use std::path::Path;

/// Source of completion candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionSource { Command, Path, Option, Buffer, Help, ColorScheme, Custom }

/// A single completion candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionItem {
    pub text: String,
    pub source: CompletionSource,
    pub info: Option<String>,
}

/// State of completion cycling.
#[derive(Debug, Clone)]
pub struct CompletionState {
    pub prefix: String,
    pub candidates: Vec<CompletionItem>,
    pub index: Option<usize>,
    pub source: CompletionSource,
}

impl CompletionState {
    pub fn new(prefix: &str, source: CompletionSource) -> Self {
        Self { prefix: prefix.to_string(), candidates: Vec::new(), index: None, source }
    }
    pub fn add(&mut self, text: &str, info: Option<&str>) {
        self.candidates.push(CompletionItem {
            text: text.to_string(), source: self.source, info: info.map(|s| s.to_string()),
        });
    }
    pub fn next(&mut self) -> Option<&CompletionItem> {
        if self.candidates.is_empty() { return None; }
        let idx = match self.index { Some(i) => (i + 1) % self.candidates.len(), None => 0 };
        self.index = Some(idx);
        self.candidates.get(idx)
    }
    pub fn prev(&mut self) -> Option<&CompletionItem> {
        if self.candidates.is_empty() { return None; }
        let idx = match self.index {
            Some(0) | None => self.candidates.len() - 1,
            Some(i) => i - 1,
        };
        self.index = Some(idx);
        self.candidates.get(idx)
    }
    pub fn current(&self) -> Option<&CompletionItem> {
        self.index.and_then(|i| self.candidates.get(i))
    }
    pub fn reset(&mut self) { self.candidates.clear(); self.index = None; }
    pub fn is_active(&self) -> bool { !self.candidates.is_empty() }
}

/// Detect what kind of completion to offer based on the command line content.
pub fn detect_source(cmdline: &str) -> CompletionSource {
    let parts: Vec<&str> = cmdline.trim().splitn(2, ' ').collect();
    if parts.len() < 2 { return CompletionSource::Command; }
    let cmd = parts[0];
    match cmd {
        "e" | "edit" | "w" | "write" | "saveas" | "sp" | "split" | "vs" | "vsplit" | "tabe" => CompletionSource::Path,
        "set" | "setlocal" | "setglobal" => CompletionSource::Option,
        "b" | "buffer" | "bd" | "bdelete" => CompletionSource::Buffer,
        "h" | "help" => CompletionSource::Help,
        "colorscheme" => CompletionSource::ColorScheme,
        _ => CompletionSource::Custom,
    }
}

/// Generate command completions from a prefix.
pub fn complete_commands(prefix: &str, commands: &[&str]) -> Vec<String> {
    commands.iter().filter(|c| c.starts_with(prefix)).map(|c| c.to_string()).collect()
}

/// Generate path completions from a partial path.
pub fn complete_paths(partial: &str) -> Vec<String> {
    let path = Path::new(partial);
    let (dir, prefix) = if partial.ends_with('/') || partial.ends_with(std::path::MAIN_SEPARATOR) {
        (path.to_path_buf(), "")
    } else {
        let dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let prefix_str = path.file_name().map(|n| n.to_str().unwrap_or("")).unwrap_or("");
        (dir, prefix_str)
    };
    let Ok(entries) = std::fs::read_dir(&dir) else { return Vec::new() };
    let mut results = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with(prefix) {
            let full = dir.join(&name).to_string_lossy().into_owned();
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                results.push(format!("{}/", full));
            } else {
                results.push(full);
            }
        }
    }
    results.sort();
    results
}

/// Find the longest common prefix among completions.
pub fn common_prefix(items: &[String]) -> String {
    if items.is_empty() { return String::new(); }
    let first = &items[0];
    let mut len = first.len();
    for item in &items[1..] {
        len = first.chars().zip(item.chars()).take_while(|(a, b)| a == b).count().min(len);
    }
    first[..first.char_indices().nth(len).map(|(i, _)| i).unwrap_or(first.len())].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_command() { assert_eq!(detect_source("w"), CompletionSource::Command); }
    #[test]
    fn detect_path() { assert_eq!(detect_source("e src/"), CompletionSource::Path); }
    #[test]
    fn detect_option() { assert_eq!(detect_source("set wrap"), CompletionSource::Option); }
    #[test]
    fn detect_buffer() { assert_eq!(detect_source("b main"), CompletionSource::Buffer); }

    #[test]
    fn cycling() {
        let mut s = CompletionState::new("w", CompletionSource::Command);
        s.add("write", None); s.add("wall", None); s.add("wq", None);
        assert_eq!(s.next().unwrap().text, "write");
        assert_eq!(s.next().unwrap().text, "wall");
        assert_eq!(s.next().unwrap().text, "wq");
        assert_eq!(s.next().unwrap().text, "write");
    }

    #[test]
    fn prev_cycling() {
        let mut s = CompletionState::new("", CompletionSource::Command);
        s.add("a", None); s.add("b", None);
        assert_eq!(s.prev().unwrap().text, "b");
        assert_eq!(s.prev().unwrap().text, "a");
    }

    #[test]
    fn complete_cmds() {
        let cmds = vec!["write", "wall", "wq", "quit", "qa"];
        let results = complete_commands("w", &cmds);
        assert_eq!(results, vec!["write", "wall", "wq"]);
    }

    #[test]
    fn common_prefix_simple() {
        let items = vec!["write".into(), "wall".into(), "wq".into()];
        assert_eq!(common_prefix(&items), "w");
    }

    #[test]
    fn common_prefix_full() {
        let items = vec!["hello".into(), "hello".into()];
        assert_eq!(common_prefix(&items), "hello");
    }

    #[test]
    fn empty_state() {
        let s = CompletionState::new("", CompletionSource::Command);
        assert!(!s.is_active()); assert!(s.current().is_none());
    }
}
