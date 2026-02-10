//! Index service implementation.

use crate::types::{
    FinderConfig, FinderItem, FinderQuery, FinderResult, MatchScore, SearchMatch, SearchQuery,
};
use std::path::PathBuf;

/// Index service for file finding, grep, and symbol search.
pub struct IndexService {
    /// Root directory for the workspace.
    root: Option<PathBuf>,
    /// Finder configuration.
    config: FinderConfig,
    /// Cached file list for the workspace.
    file_cache: Vec<PathBuf>,
}

impl IndexService {
    /// Create a new index service.
    pub fn new() -> Self {
        Self {
            root: None,
            config: FinderConfig::default(),
            file_cache: Vec::new(),
        }
    }

    /// Set the workspace root.
    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// Get workspace root.
    pub fn root(&self) -> Option<&PathBuf> {
        self.root.as_ref()
    }

    /// Update configuration.
    pub fn set_config(&mut self, config: FinderConfig) {
        self.config = config;
    }

    /// Get current configuration.
    pub fn config(&self) -> &FinderConfig {
        &self.config
    }

    /// Set the cached file list (from filesystem scan).
    pub fn set_file_cache(&mut self, files: Vec<PathBuf>) {
        self.file_cache = files;
    }

    /// Execute a finder query using the fuzzy matching algorithm.
    pub fn find(&self, query: &FinderQuery) -> FinderResult {
        match query {
            FinderQuery::Files { pattern } => self.find_files(pattern),
            FinderQuery::Buffers { pattern: _ } => {
                // Buffer search is handled by the core, return empty
                FinderResult {
                    items: Vec::new(),
                    total_candidates: 0,
                    truncated: false,
                }
            }
            FinderQuery::DocumentSymbols { pattern: _ } => {
                // Symbol search delegated to LSP, return empty
                FinderResult {
                    items: Vec::new(),
                    total_candidates: 0,
                    truncated: false,
                }
            }
            FinderQuery::WorkspaceSymbols { pattern: _ } => FinderResult {
                items: Vec::new(),
                total_candidates: 0,
                truncated: false,
            },
            FinderQuery::Commands { pattern: _ } => FinderResult {
                items: Vec::new(),
                total_candidates: 0,
                truncated: false,
            },
        }
    }

    /// Find files matching a fuzzy pattern.
    fn find_files(&self, pattern: &str) -> FinderResult {
        let total = self.file_cache.len();
        let mut items: Vec<FinderItem> = self
            .file_cache
            .iter()
            .filter_map(|path| {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let score = fuzzy_score(pattern, name);
                if score.0 > 0.0 {
                    Some(FinderItem {
                        label: name.to_string(),
                        description: Some(path.display().to_string()),
                        path: Some(path.clone()),
                        line: None,
                        score,
                    })
                } else {
                    None
                }
            })
            .collect();

        // Sort by score descending.
        items.sort_by(|a, b| {
            b.score
                .0
                .partial_cmp(&a.score.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let truncated = items.len() > self.config.max_results;
        items.truncate(self.config.max_results);

        FinderResult {
            items,
            total_candidates: total,
            truncated,
        }
    }

    /// Execute a grep search across workspace files.
    pub fn grep(&self, query: &SearchQuery) -> Vec<SearchMatch> {
        // In production: spawn ripgrep or walk files.
        // Here we provide the interface scaffold.
        tracing::debug!(pattern = %query.pattern, "live grep");
        Vec::new()
    }
}

impl Default for IndexService {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple fuzzy matching score.
///
/// Scores consecutive matches higher and word boundary matches higher.
/// Returns 0.0 for no match.
pub fn fuzzy_score(pattern: &str, candidate: &str) -> MatchScore {
    if pattern.is_empty() {
        return MatchScore(1.0);
    }

    let pattern_lower = pattern.to_lowercase();
    let candidate_lower = candidate.to_lowercase();
    let pattern_chars: Vec<char> = pattern_lower.chars().collect();
    let candidate_chars: Vec<char> = candidate_lower.chars().collect();

    let mut pi = 0;
    let mut score = 0.0;
    let mut consecutive = 0;
    let mut last_match = None;

    for (ci, &cc) in candidate_chars.iter().enumerate() {
        if pi < pattern_chars.len() && cc == pattern_chars[pi] {
            score += 1.0;
            if let Some(last) = last_match {
                if ci == last + 1 {
                    consecutive += 1;
                    score += consecutive as f64 * 0.5;
                } else {
                    consecutive = 0;
                }
            }
            // Bonus for word boundary match.
            if ci == 0
                || candidate_chars
                    .get(ci.wrapping_sub(1))
                    .map(|c| *c == '_' || *c == '-' || *c == '/')
                    .unwrap_or(false)
            {
                score += 0.5;
            }
            last_match = Some(ci);
            pi += 1;
        }
    }

    if pi < pattern_chars.len() {
        MatchScore(0.0)
    } else {
        MatchScore(score)
    }
}
