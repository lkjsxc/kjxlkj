//! Index service — file indexing, symbol search, workspace scanning, fuzzy finder.

mod highlight_groups;

use std::path::PathBuf;

/// An indexed file entry.
#[derive(Debug, Clone)]
pub struct IndexEntry { pub path: PathBuf, pub size: u64, pub modified: std::time::SystemTime }

/// Result of a search query.
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub path: PathBuf, pub line: usize, pub col: usize, pub text: String,
}

/// Source for the fuzzy finder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinderSource { Files, Buffers, Grep, Symbols, Commands, RecentFiles }

/// A finder item (generic for fuzzy matching).
#[derive(Debug, Clone)]
pub struct FinderItem {
    pub label: String, pub path: Option<PathBuf>,
    pub line: Option<usize>, pub score: f64,
}

/// Finder query state.
#[derive(Debug, Clone)]
pub struct FinderQuery {
    pub text: String, pub source: FinderSource,
    pub results: Vec<FinderItem>, pub selected: usize, pub visible: bool,
}

impl FinderQuery {
    pub fn new(source: FinderSource) -> Self {
        Self { text: String::new(), source, results: Vec::new(), selected: 0, visible: true }
    }

    pub fn select_prev(&mut self) { self.selected = self.selected.saturating_sub(1); }

    pub fn select_next(&mut self) {
        if self.selected + 1 < self.results.len() { self.selected += 1; }
    }

    pub fn current_item(&self) -> Option<&FinderItem> { self.results.get(self.selected) }
}

/// Simple fuzzy match score (higher = better).
pub fn fuzzy_score(query: &str, target: &str) -> Option<f64> {
    if query.is_empty() { return Some(0.0); }
    let query_lower = query.to_lowercase();
    let target_lower = target.to_lowercase();
    let mut qi = 0;
    let qchars: Vec<char> = query_lower.chars().collect();
    let mut score = 0.0;
    let mut prev_match = false;
    for (i, tc) in target_lower.chars().enumerate() {
        if qi < qchars.len() && tc == qchars[qi] {
            score += if prev_match { 2.0 } else { 1.0 };
            if i == 0 || target.as_bytes().get(i.wrapping_sub(1)).map_or(false, |&b| b == b'/') {
                score += 3.0; // bonus for start-of-word/path
            }
            qi += 1;
            prev_match = true;
        } else { prev_match = false; }
    }
    if qi == qchars.len() { Some(score) } else { None }
}

/// File indexing and search service.
pub struct IndexService {
    root: Option<PathBuf>,
    entries: Vec<IndexEntry>,
}

impl IndexService {
    pub fn new() -> Self { Self { root: None, entries: Vec::new() } }

    pub fn set_root(&mut self, root: PathBuf) { self.root = Some(root); }

    pub async fn scan(&mut self) -> anyhow::Result<()> {
        let root = self.root.as_ref().ok_or_else(|| anyhow::anyhow!("no workspace root"))?;
        tracing::info!(root = %root.display(), "scanning workspace");
        self.entries.clear();
        Ok(())
    }

    pub async fn find_files(&self, pattern: &str) -> anyhow::Result<Vec<FinderItem>> {
        let mut results = Vec::new();
        for entry in &self.entries {
            let name = entry.path.to_string_lossy();
            if let Some(score) = fuzzy_score(pattern, &name) {
                results.push(FinderItem {
                    label: name.to_string(), path: Some(entry.path.clone()),
                    line: None, score,
                });
            }
        }
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(results)
    }

    pub async fn grep(&self, _pattern: &str) -> anyhow::Result<Vec<SearchResult>> {
        Ok(Vec::new())
    }

    pub fn entry_count(&self) -> usize { self.entries.len() }
}

impl Default for IndexService { fn default() -> Self { Self::new() } }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_score_basic() {
        assert!(fuzzy_score("main", "main.rs").is_some());
        assert!(fuzzy_score("mr", "main.rs").is_some());
        assert!(fuzzy_score("xyz", "main.rs").is_none());
        assert!(fuzzy_score("", "anything").is_some());
    }

    #[test]
    fn fuzzy_score_ordering() {
        let s1 = fuzzy_score("main", "main.rs").unwrap();
        let s2 = fuzzy_score("main", "my_domain.rs").unwrap();
        assert!(s1 > s2, "exact prefix should score higher");
    }

    #[test]
    fn finder_query_navigation() {
        let mut q = FinderQuery::new(FinderSource::Files);
        q.results = vec![
            FinderItem { label: "a".into(), path: None, line: None, score: 1.0 },
            FinderItem { label: "b".into(), path: None, line: None, score: 0.5 },
        ];
        assert_eq!(q.selected, 0);
        q.select_next();
        assert_eq!(q.selected, 1);
        q.select_next(); // should not overflow
        assert_eq!(q.selected, 1);
        q.select_prev();
        assert_eq!(q.selected, 0);
        assert_eq!(q.current_item().unwrap().label, "a");
    }

    #[test]
    fn finder_sources() {
        let sources = [FinderSource::Files, FinderSource::Buffers, FinderSource::Grep,
                       FinderSource::Symbols, FinderSource::Commands, FinderSource::RecentFiles];
        assert_eq!(sources.len(), 6);
    }

    #[tokio::test]
    async fn index_service_no_root() {
        let mut svc = IndexService::new();
        assert!(svc.scan().await.is_err());
    }
}
