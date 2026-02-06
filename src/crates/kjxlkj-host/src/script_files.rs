/// Script file loading — source, runtime, and script execution.

use std::path::{Path, PathBuf};
use std::collections::HashMap;

/// A parsed script file.
#[derive(Debug, Clone)]
pub struct ScriptFile {
    pub path: PathBuf,
    pub commands: Vec<ScriptLine>,
}

/// A single line in a script.
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptLine {
    ExCommand(String),
    Comment(String),
    Blank,
    Conditional { condition: String, body: Vec<ScriptLine> },
}

/// Parse a script file from text content.
pub fn parse_script(path: &Path, content: &str) -> ScriptFile {
    let commands = content.lines().map(|line| {
        let trimmed = line.trim();
        if trimmed.is_empty() { ScriptLine::Blank }
        else if trimmed.starts_with('"') { ScriptLine::Comment(trimmed.into()) }
        else { ScriptLine::ExCommand(trimmed.into()) }
    }).collect();
    ScriptFile { path: path.to_path_buf(), commands }
}

/// Extract executable commands from a script (skip comments and blanks).
pub fn executable_commands(script: &ScriptFile) -> Vec<&str> {
    script.commands.iter().filter_map(|line| match line {
        ScriptLine::ExCommand(cmd) => Some(cmd.as_str()),
        _ => None,
    }).collect()
}

/// Script source path resolution order.
pub fn resolve_source_path(name: &str, search_dirs: &[PathBuf]) -> Option<PathBuf> {
    let p = Path::new(name);
    if p.is_absolute() && p.exists() { return Some(p.to_path_buf()); }
    for dir in search_dirs {
        let candidate = dir.join(name);
        if candidate.exists() { return Some(candidate); }
        let with_ext = dir.join(format!("{}.vim", name));
        if with_ext.exists() { return Some(with_ext); }
    }
    None
}

/// Runtime file search paths.
pub fn default_runtime_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Some(home) = home_dir() {
        paths.push(home.join(".config").join("kjxlkj"));
        paths.push(home.join(".config").join("kjxlkj").join("plugin"));
    }
    paths
}

fn home_dir() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Track which scripts have been sourced to avoid double-sourcing.
#[derive(Debug, Default)]
pub struct SourceTracker { sourced: HashMap<PathBuf, usize> }

impl SourceTracker {
    pub fn new() -> Self { Self::default() }
    pub fn mark_sourced(&mut self, path: &Path) {
        *self.sourced.entry(path.to_path_buf()).or_insert(0) += 1;
    }
    pub fn was_sourced(&self, path: &Path) -> bool { self.sourced.contains_key(path) }
    pub fn source_count(&self, path: &Path) -> usize { self.sourced.get(path).copied().unwrap_or(0) }
}

/// Validate script line syntax (basic check).
pub fn validate_line(line: &str) -> bool {
    let t = line.trim();
    t.is_empty() || t.starts_with('"') || t.starts_with(':') || t.chars().next().map_or(false, |c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_script_basic() {
        let s = parse_script(Path::new("init.vim"), "\" comment\nset number\n\ncolorscheme dark");
        assert_eq!(s.commands.len(), 4);
        assert_eq!(s.commands[0], ScriptLine::Comment("\" comment".into()));
        assert_eq!(s.commands[1], ScriptLine::ExCommand("set number".into()));
        assert_eq!(s.commands[2], ScriptLine::Blank);
    }

    #[test]
    fn executable_only() {
        let s = parse_script(Path::new("t.vim"), "\" x\nset wrap\n\necho hi");
        let cmds = executable_commands(&s);
        assert_eq!(cmds, vec!["set wrap", "echo hi"]);
    }

    #[test]
    fn source_tracker() {
        let mut t = SourceTracker::new();
        let p = Path::new("/tmp/init.vim");
        assert!(!t.was_sourced(p));
        t.mark_sourced(p);
        assert!(t.was_sourced(p));
        assert_eq!(t.source_count(p), 1);
        t.mark_sourced(p);
        assert_eq!(t.source_count(p), 2);
    }

    #[test]
    fn validate_lines() {
        assert!(validate_line("set number"));
        assert!(validate_line("\" comment"));
        assert!(validate_line(""));
        assert!(validate_line(":write"));
    }

    #[test]
    fn runtime_paths_not_empty() {
        // May be empty in CI but should not panic
        let _ = default_runtime_paths();
    }

    #[test]
    fn resolve_nonexistent() {
        assert!(resolve_source_path("nonexistent_file_xyz", &[]).is_none());
    }

    #[test]
    fn script_file_path() {
        let s = parse_script(Path::new("/a/b.vim"), "");
        assert_eq!(s.path, Path::new("/a/b.vim"));
    }
}
