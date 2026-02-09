//! Live grep: async search across project files using
//! ripgrep-style matching with result streaming.

use std::path::PathBuf;

/// A live grep match result.
#[derive(Debug, Clone)]
pub struct GrepMatch {
    /// File path.
    pub file: PathBuf,
    /// 1-indexed line number.
    pub line: usize,
    /// 0-indexed column of match start.
    pub col: usize,
    /// The matching line text.
    pub text: String,
    /// Matched substring.
    pub matched: String,
}

/// Live grep state.
#[derive(Debug, Clone, Default)]
pub struct LiveGrepState {
    /// Current search query.
    pub query: String,
    /// Whether search is in progress.
    pub searching: bool,
    /// Results accumulated so far.
    pub results: Vec<GrepMatch>,
    /// Selected result index.
    pub selected: usize,
    /// Maximum results to collect.
    pub max_results: usize,
    /// Search paths (directories to include).
    pub search_paths: Vec<PathBuf>,
    /// File type filter (e.g., "rs", "py").
    pub file_types: Vec<String>,
    /// Whether to use regex.
    pub use_regex: bool,
    /// Case sensitivity.
    pub case_sensitive: bool,
}

impl LiveGrepState {
    pub fn new() -> Self {
        Self {
            max_results: 1000,
            case_sensitive: false,
            ..Default::default()
        }
    }

    /// Start a new search.
    pub fn start_search(&mut self, query: &str) {
        self.query = query.to_string();
        self.searching = true;
        self.results.clear();
        self.selected = 0;
    }

    /// Add a match result.
    pub fn add_result(&mut self, m: GrepMatch) {
        if self.results.len() < self.max_results {
            self.results.push(m);
        }
    }

    /// Mark search as complete.
    pub fn finish_search(&mut self) {
        self.searching = false;
    }

    /// Select next result.
    pub fn select_next(&mut self) {
        if !self.results.is_empty() {
            self.selected =
                (self.selected + 1) % self.results.len();
        }
    }

    /// Select previous result.
    pub fn select_prev(&mut self) {
        if !self.results.is_empty() {
            self.selected = if self.selected == 0 {
                self.results.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Get currently selected match.
    pub fn current_match(&self) -> Option<&GrepMatch> {
        self.results.get(self.selected)
    }

    /// Build a grep command line for external process.
    pub fn build_command(&self) -> Vec<String> {
        let mut args = vec!["rg".to_string()];
        if !self.case_sensitive {
            args.push("-i".to_string());
        }
        args.push("--line-number".to_string());
        args.push("--column".to_string());
        if !self.use_regex {
            args.push("--fixed-strings".to_string());
        }
        for ft in &self.file_types {
            args.push(format!("--type={}", ft));
        }
        args.push(self.query.clone());
        for p in &self.search_paths {
            args.push(p.to_string_lossy().to_string());
        }
        args
    }

    /// Parse a ripgrep output line into a GrepMatch.
    pub fn parse_rg_line(line: &str) -> Option<GrepMatch> {
        // Format: file:line:col:text
        let mut parts = line.splitn(4, ':');
        let file = parts.next()?;
        let line_num: usize = parts.next()?.parse().ok()?;
        let col: usize = parts.next()?.parse().ok()?;
        let text = parts.next().unwrap_or("").to_string();
        Some(GrepMatch {
            file: PathBuf::from(file),
            line: line_num,
            col: col.saturating_sub(1),
            text: text.clone(),
            matched: String::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_add_results() {
        let mut state = LiveGrepState::new();
        state.start_search("todo");
        assert!(state.searching);
        state.add_result(GrepMatch {
            file: PathBuf::from("src/main.rs"),
            line: 10,
            col: 5,
            text: "// TODO: fix".into(),
            matched: "TODO".into(),
        });
        assert_eq!(state.results.len(), 1);
        state.finish_search();
        assert!(!state.searching);
    }

    #[test]
    fn select_navigation() {
        let mut state = LiveGrepState::new();
        for i in 0..3 {
            state.add_result(GrepMatch {
                file: PathBuf::from(format!("f{}.rs", i)),
                line: i + 1,
                col: 0,
                text: String::new(),
                matched: String::new(),
            });
        }
        assert_eq!(state.selected, 0);
        state.select_next();
        assert_eq!(state.selected, 1);
        state.select_prev();
        assert_eq!(state.selected, 0);
        state.select_prev();
        assert_eq!(state.selected, 2);
    }

    #[test]
    fn parse_rg_output() {
        let m = LiveGrepState::parse_rg_line(
            "src/lib.rs:42:5:let x = 10;",
        )
        .unwrap();
        assert_eq!(m.file, PathBuf::from("src/lib.rs"));
        assert_eq!(m.line, 42);
        assert_eq!(m.col, 4);
    }
}
