//! Index/finder domain types.

use std::path::PathBuf;

/// Configuration for the finder.
#[derive(Debug, Clone)]
pub struct FinderConfig {
    /// Maximum results to return.
    pub max_results: usize,
    /// Whether to respect .gitignore.
    pub respect_gitignore: bool,
    /// Whether to include hidden files.
    pub include_hidden: bool,
}

impl Default for FinderConfig {
    fn default() -> Self {
        Self {
            max_results: 200,
            respect_gitignore: true,
            include_hidden: false,
        }
    }
}

/// A query to the finder.
#[derive(Debug, Clone)]
pub enum FinderQuery {
    /// Find files by name pattern.
    Files { pattern: String },
    /// List open buffers matching pattern.
    Buffers { pattern: String },
    /// Search for symbols in document.
    DocumentSymbols { pattern: String },
    /// Search for symbols across workspace.
    WorkspaceSymbols { pattern: String },
    /// Command palette search.
    Commands { pattern: String },
}

/// A single item in the finder results.
#[derive(Debug, Clone)]
pub struct FinderItem {
    /// Display label.
    pub label: String,
    /// Optional description (e.g. file path for symbols).
    pub description: Option<String>,
    /// Associated file path if applicable.
    pub path: Option<PathBuf>,
    /// Line number if applicable (1-based).
    pub line: Option<usize>,
    /// Match score for sorting.
    pub score: MatchScore,
}

/// Score for fuzzy match ranking.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MatchScore(pub f64);

impl MatchScore {
    pub fn zero() -> Self {
        Self(0.0)
    }
}

/// Result set from a finder query.
#[derive(Debug, Clone)]
pub struct FinderResult {
    pub items: Vec<FinderItem>,
    pub total_candidates: usize,
    pub truncated: bool,
}

/// A live grep search query.
#[derive(Debug, Clone)]
pub struct SearchQuery {
    /// The search pattern (literal or regex).
    pub pattern: String,
    /// Whether pattern is a regex.
    pub is_regex: bool,
    /// Case sensitivity.
    pub case_sensitive: bool,
    /// Directory to search in (None = workspace root).
    pub directory: Option<PathBuf>,
    /// File glob filter.
    pub file_glob: Option<String>,
}

/// A single search match from live grep.
#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub text: String,
    pub match_start: usize,
    pub match_end: usize,
}
