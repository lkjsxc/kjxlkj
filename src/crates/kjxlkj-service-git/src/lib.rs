//! Git service for kjxlkj editor.
//!
//! Provides git integration via subprocess.

use kjxlkj_services::{Service, ServiceMessage};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::process::Command;
use tokio::sync::mpsc;
use tracing::{debug, info};

/// Git status of a file.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum GitStatus {
    /// File is untracked.
    Untracked,
    /// File is modified.
    Modified,
    /// File is staged.
    Staged,
    /// File is unchanged.
    #[default]
    Unchanged,
    /// File is ignored.
    Ignored,
}

/// Branch info for statusline.
#[derive(Debug, Clone, Default)]
pub struct BranchInfo {
    /// Branch name.
    pub name: String,
    /// Commits ahead of remote.
    pub ahead: usize,
    /// Commits behind remote.
    pub behind: usize,
    /// Whether the branch is detached HEAD.
    pub detached: bool,
}

impl BranchInfo {
    /// Create a new branch info.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ahead: 0,
            behind: 0,
            detached: false,
        }
    }

    /// Set ahead/behind counts.
    pub fn with_remote(mut self, ahead: usize, behind: usize) -> Self {
        self.ahead = ahead;
        self.behind = behind;
        self
    }

    /// Mark as detached HEAD.
    pub fn detached(mut self) -> Self {
        self.detached = true;
        self
    }

    /// Format for statusline display.
    pub fn display(&self) -> String {
        if self.detached {
            format!("({})", &self.name[..7.min(self.name.len())])
        } else if self.ahead > 0 && self.behind > 0 {
            format!(" {} ↑{} ↓{}", self.name, self.ahead, self.behind)
        } else if self.ahead > 0 {
            format!(" {} ↑{}", self.name, self.ahead)
        } else if self.behind > 0 {
            format!(" {} ↓{}", self.name, self.behind)
        } else {
            format!(" {}", self.name)
        }
    }
}

/// File change indicator for explorer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileIndicator {
    /// No indicator (unchanged).
    None,
    /// Modified indicator.
    Modified,
    /// Added/untracked indicator.
    Added,
    /// Deleted indicator.
    Deleted,
    /// Renamed indicator.
    Renamed,
    /// Conflict indicator.
    Conflict,
    /// Ignored indicator.
    Ignored,
}

impl FileIndicator {
    /// Get the display character for this indicator.
    pub fn char(&self) -> Option<char> {
        match self {
            Self::None => None,
            Self::Modified => Some('M'),
            Self::Added => Some('A'),
            Self::Deleted => Some('D'),
            Self::Renamed => Some('R'),
            Self::Conflict => Some('!'),
            Self::Ignored => Some('I'),
        }
    }

    /// Convert from GitStatus.
    pub fn from_status(status: &GitStatus) -> Self {
        match status {
            GitStatus::Modified => Self::Modified,
            GitStatus::Staged => Self::Added,
            GitStatus::Untracked => Self::Added,
            GitStatus::Ignored => Self::Ignored,
            GitStatus::Unchanged => Self::None,
        }
    }
}

/// Repository stats summary.
#[derive(Debug, Clone, Default)]
pub struct RepoStats {
    /// Number of modified files.
    pub modified: usize,
    /// Number of staged files.
    pub staged: usize,
    /// Number of untracked files.
    pub untracked: usize,
    /// Number of conflicts.
    pub conflicts: usize,
}

// ============================================================================
// Hunks and Gutter Signs
// ============================================================================

/// Type of change in a hunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HunkType {
    /// Lines were added.
    Add,
    /// Lines were changed.
    Change,
    /// Lines were deleted.
    Delete,
}

/// A git hunk (contiguous change region).
#[derive(Debug, Clone)]
pub struct Hunk {
    /// Hunk type.
    pub hunk_type: HunkType,
    /// Start line (1-indexed).
    pub start_line: usize,
    /// Number of lines in working copy.
    pub lines: usize,
    /// Original start line.
    pub orig_start: usize,
    /// Original number of lines.
    pub orig_lines: usize,
    /// The diff text.
    pub diff_text: String,
}

impl Hunk {
    /// Create a new hunk.
    pub fn new(hunk_type: HunkType, start_line: usize, lines: usize) -> Self {
        Self {
            hunk_type,
            start_line,
            lines,
            orig_start: start_line,
            orig_lines: lines,
            diff_text: String::new(),
        }
    }

    /// Set original line info.
    pub fn with_original(mut self, orig_start: usize, orig_lines: usize) -> Self {
        self.orig_start = orig_start;
        self.orig_lines = orig_lines;
        self
    }

    /// Set diff text.
    pub fn with_diff(mut self, text: impl Into<String>) -> Self {
        self.diff_text = text.into();
        self
    }

    /// Check if line is within this hunk.
    pub fn contains_line(&self, line: usize) -> bool {
        line >= self.start_line && line < self.start_line + self.lines.max(1)
    }
}

/// Gutter sign for a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GutterSign {
    /// Line was added.
    Added,
    /// Line was changed.
    Changed,
    /// Line(s) were deleted at this position.
    Deleted,
    /// Top of a change region.
    ChangeStart,
    /// End of a change region.
    ChangeEnd,
}

impl GutterSign {
    /// Get display character.
    pub fn char(&self) -> char {
        match self {
            Self::Added => '+',
            Self::Changed => '~',
            Self::Deleted => '-',
            Self::ChangeStart => '┃',
            Self::ChangeEnd => '┃',
        }
    }
}

/// Buffer hunk state.
#[derive(Debug, Default, Clone)]
pub struct BufferHunks {
    /// All hunks for the buffer.
    hunks: Vec<Hunk>,
    /// Signs by line number.
    signs: HashMap<usize, GutterSign>,
}

impl BufferHunks {
    /// Create empty hunks.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a hunk.
    pub fn add_hunk(&mut self, hunk: Hunk) {
        // Update signs
        for i in 0..hunk.lines.max(1) {
            let line = hunk.start_line + i;
            let sign = match hunk.hunk_type {
                HunkType::Add => GutterSign::Added,
                HunkType::Change => GutterSign::Changed,
                HunkType::Delete if i == 0 => GutterSign::Deleted,
                _ => continue,
            };
            self.signs.insert(line, sign);
        }
        self.hunks.push(hunk);
    }

    /// Get hunks.
    pub fn hunks(&self) -> &[Hunk] {
        &self.hunks
    }

    /// Get sign for line.
    pub fn sign(&self, line: usize) -> Option<GutterSign> {
        self.signs.get(&line).copied()
    }

    /// Get hunk at line.
    pub fn hunk_at(&self, line: usize) -> Option<&Hunk> {
        self.hunks.iter().find(|h| h.contains_line(line))
    }

    /// Get next hunk after line.
    pub fn next_hunk(&self, line: usize) -> Option<&Hunk> {
        self.hunks.iter().find(|h| h.start_line > line)
    }

    /// Get previous hunk before line.
    pub fn prev_hunk(&self, line: usize) -> Option<&Hunk> {
        self.hunks.iter().rev().find(|h| h.start_line < line)
    }

    /// Clear all hunks.
    pub fn clear(&mut self) {
        self.hunks.clear();
        self.signs.clear();
    }

    /// Count hunks.
    pub fn count(&self) -> usize {
        self.hunks.len()
    }
}

// ============================================================================
// Blame
// ============================================================================

/// Blame info for a line.
#[derive(Debug, Clone)]
pub struct BlameInfo {
    /// Commit hash (short).
    pub commit: String,
    /// Author name.
    pub author: String,
    /// Relative time (e.g., "2 days ago").
    pub time: String,
    /// Commit summary.
    pub summary: String,
    /// Is this uncommitted?
    pub uncommitted: bool,
}

impl BlameInfo {
    /// Create blame info.
    pub fn new(commit: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            commit: commit.into(),
            author: author.into(),
            time: String::new(),
            summary: String::new(),
            uncommitted: false,
        }
    }

    /// Set time.
    pub fn with_time(mut self, time: impl Into<String>) -> Self {
        self.time = time.into();
        self
    }

    /// Set summary.
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    /// Mark as uncommitted.
    pub fn uncommitted() -> Self {
        Self {
            commit: "00000000".to_string(),
            author: "Not Committed Yet".to_string(),
            time: String::new(),
            summary: String::new(),
            uncommitted: true,
        }
    }

    /// Format for inline display.
    pub fn display(&self) -> String {
        if self.uncommitted {
            "Not Committed Yet".to_string()
        } else if self.time.is_empty() {
            format!("{} • {}", self.author, &self.commit[..7.min(self.commit.len())])
        } else {
            format!("{} • {} • {}", self.author, self.time, &self.summary[..50.min(self.summary.len())])
        }
    }
}

/// Buffer blame state.
#[derive(Debug, Default)]
pub struct BufferBlame {
    /// Blame info by line number.
    lines: HashMap<usize, BlameInfo>,
    /// Is blame visible?
    pub visible: bool,
}

impl BufferBlame {
    /// Create empty blame.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set blame for a line.
    pub fn set(&mut self, line: usize, info: BlameInfo) {
        self.lines.insert(line, info);
    }

    /// Get blame for a line.
    pub fn get(&self, line: usize) -> Option<&BlameInfo> {
        self.lines.get(&line)
    }

    /// Clear all blame info.
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Toggle visibility.
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}

// ============================================================================
// Merge Conflicts
// ============================================================================

/// Conflict marker type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictMarker {
    /// Start of ours (<<<<<<< HEAD).
    OursStart,
    /// Separator (=======).
    Separator,
    /// Start of theirs (>>>>>>> branch).
    TheirsStart,
    /// Base marker for 3-way (||||||| base).
    BaseMarker,
}

/// A single conflict in a file.
#[derive(Debug, Clone)]
pub struct Conflict {
    /// Start line of conflict.
    pub start_line: usize,
    /// End line of conflict.
    pub end_line: usize,
    /// Line of separator (=======).
    pub separator_line: usize,
    /// Base line for 3-way merge (if any).
    pub base_line: Option<usize>,
    /// Ours content.
    pub ours: String,
    /// Theirs content.
    pub theirs: String,
    /// Base content for 3-way.
    pub base: Option<String>,
    /// Branch name (from marker).
    pub branch: String,
}

impl Conflict {
    /// Create a new conflict.
    pub fn new(start_line: usize, separator_line: usize, end_line: usize) -> Self {
        Self {
            start_line,
            separator_line,
            end_line,
            base_line: None,
            ours: String::new(),
            theirs: String::new(),
            base: None,
            branch: String::new(),
        }
    }

    /// Line count.
    pub fn line_count(&self) -> usize {
        self.end_line - self.start_line + 1
    }

    /// Check if line is in conflict.
    pub fn contains_line(&self, line: usize) -> bool {
        line >= self.start_line && line <= self.end_line
    }
}

/// Conflict resolution choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictChoice {
    /// Keep our version.
    Ours,
    /// Keep their version.
    Theirs,
    /// Keep both (ours first).
    Both,
    /// Keep base (for 3-way).
    Base,
}

/// Buffer conflict state.
#[derive(Debug, Default)]
pub struct BufferConflicts {
    /// All conflicts in the buffer.
    conflicts: Vec<Conflict>,
}

impl BufferConflicts {
    /// Create empty conflicts.
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse conflicts from buffer content.
    pub fn parse(content: &str) -> Self {
        let mut conflicts = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            if lines[i].starts_with("<<<<<<<") {
                let start_line = i + 1;
                let mut separator_line = 0;
                let mut end_line = 0;
                let mut base_line = None;

                // Find markers
                for (j, line) in lines.iter().enumerate().skip(i + 1) {
                    if line.starts_with("|||||||") {
                        base_line = Some(j + 1);
                    } else if line.starts_with("=======") {
                        separator_line = j + 1;
                    } else if line.starts_with(">>>>>>>") {
                        end_line = j + 1;
                        break;
                    }
                }

                if separator_line > 0 && end_line > 0 {
                    let mut conflict = Conflict::new(start_line, separator_line, end_line);
                    conflict.base_line = base_line;

                    // Extract branch name
                    if let Some(branch) = lines[end_line - 1].strip_prefix(">>>>>>> ") {
                        conflict.branch = branch.to_string();
                    }

                    conflicts.push(conflict);
                    i = end_line;
                }
            }
            i += 1;
        }

        Self { conflicts }
    }

    /// Get all conflicts.
    pub fn conflicts(&self) -> &[Conflict] {
        &self.conflicts
    }

    /// Get conflict at line.
    pub fn conflict_at(&self, line: usize) -> Option<&Conflict> {
        self.conflicts.iter().find(|c| c.contains_line(line))
    }

    /// Get next conflict after line.
    pub fn next_conflict(&self, line: usize) -> Option<&Conflict> {
        self.conflicts.iter().find(|c| c.start_line > line)
    }

    /// Get previous conflict before line.
    pub fn prev_conflict(&self, line: usize) -> Option<&Conflict> {
        self.conflicts.iter().rev().find(|c| c.end_line < line)
    }

    /// Count conflicts.
    pub fn count(&self) -> usize {
        self.conflicts.len()
    }

    /// Check if there are conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }
}

// ============================================================================
// Diff Mode
// ============================================================================

/// Diff algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffAlgorithm {
    /// Myers (default, fast).
    #[default]
    Myers,
    /// Minimal (smallest diff).
    Minimal,
    /// Patience (better for code).
    Patience,
    /// Histogram (large files).
    Histogram,
}

/// Diff options.
#[derive(Debug, Clone, Default)]
pub struct DiffOptions {
    /// Algorithm to use.
    pub algorithm: DiffAlgorithm,
    /// Ignore whitespace changes.
    pub ignore_whitespace: bool,
    /// Ignore whitespace at end of line.
    pub ignore_eol_whitespace: bool,
    /// Ignore blank lines.
    pub ignore_blank_lines: bool,
    /// Context lines.
    pub context: usize,
}

impl DiffOptions {
    /// Create new options.
    pub fn new() -> Self {
        Self {
            context: 3,
            ..Default::default()
        }
    }

    /// Use patience algorithm.
    pub fn patience(mut self) -> Self {
        self.algorithm = DiffAlgorithm::Patience;
        self
    }

    /// Ignore all whitespace.
    pub fn ignore_whitespace(mut self) -> Self {
        self.ignore_whitespace = true;
        self
    }
}

/// Diff view layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiffLayout {
    /// Vertical split.
    #[default]
    Vertical,
    /// Horizontal split.
    Horizontal,
}

/// Diff view state.
#[derive(Debug, Default)]
pub struct DiffView {
    /// Left buffer path.
    pub left: Option<PathBuf>,
    /// Right buffer path.
    pub right: Option<PathBuf>,
    /// Layout.
    pub layout: DiffLayout,
    /// Options.
    pub options: DiffOptions,
    /// Scroll sync enabled.
    pub scroll_sync: bool,
    /// Is active.
    pub active: bool,
}

impl DiffView {
    /// Create new diff view.
    pub fn new() -> Self {
        Self {
            scroll_sync: true,
            ..Default::default()
        }
    }

    /// Set left and right files.
    pub fn files(mut self, left: PathBuf, right: PathBuf) -> Self {
        self.left = Some(left);
        self.right = Some(right);
        self
    }

    /// Set layout.
    pub fn with_layout(mut self, layout: DiffLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Activate.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivate.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Toggle scroll sync.
    pub fn toggle_scroll_sync(&mut self) {
        self.scroll_sync = !self.scroll_sync;
    }
}

/// Git service.
pub struct GitService {
    /// Service name.
    name: String,
    /// Repository root.
    #[allow(dead_code)]
    repo_root: Option<PathBuf>,
}

impl GitService {
    /// Create a new git service.
    pub fn new() -> Self {
        Self {
            name: "git".to_string(),
            repo_root: None,
        }
    }

    /// Find git repository root.
    pub async fn find_repo_root(path: &PathBuf) -> Option<PathBuf> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .current_dir(path)
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let root = String::from_utf8(output.stdout).ok()?;
            Some(PathBuf::from(root.trim()))
        } else {
            None
        }
    }

    /// Get current branch name.
    pub async fn current_branch(repo_root: &PathBuf) -> Option<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(repo_root)
            .output()
            .await
            .ok()?;

        if output.status.success() {
            let branch = String::from_utf8(output.stdout).ok()?;
            Some(branch.trim().to_string())
        } else {
            None
        }
    }

    /// Get file status.
    pub async fn file_status(repo_root: &PathBuf, file: &PathBuf) -> Option<GitStatus> {
        let output = Command::new("git")
            .args(["status", "--porcelain", "--"])
            .arg(file)
            .current_dir(repo_root)
            .output()
            .await
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let status = String::from_utf8(output.stdout).ok()?;
        let status = status.trim();

        if status.is_empty() {
            return Some(GitStatus::Unchanged);
        }

        let first = status.chars().next()?;
        let second = status.chars().nth(1)?;

        match (first, second) {
            ('?', '?') => Some(GitStatus::Untracked),
            ('!', '!') => Some(GitStatus::Ignored),
            (_, 'M') | (_, 'D') | (_, 'A') => Some(GitStatus::Modified),
            ('M', _) | ('A', _) | ('D', _) | ('R', _) => Some(GitStatus::Staged),
            _ => Some(GitStatus::Unchanged),
        }
    }
}

impl Default for GitService {
    fn default() -> Self {
        Self::new()
    }
}

impl Service for GitService {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(
        self: Box<Self>,
        mut rx: mpsc::Receiver<ServiceMessage>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {
            info!("Git service started");

            while let Some(msg) = rx.recv().await {
                match msg {
                    ServiceMessage::Shutdown => {
                        info!("Git service shutting down");
                        break;
                    }
                    ServiceMessage::Custom(cmd) => {
                        debug!(%cmd, "Received command");
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_service_new() {
        let service = GitService::new();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_variants() {
        assert_eq!(GitStatus::Untracked, GitStatus::Untracked);
        assert_ne!(GitStatus::Modified, GitStatus::Staged);
    }

    #[test]
    fn test_git_service_default() {
        let service = GitService::default();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_equality() {
        assert_eq!(GitStatus::Modified, GitStatus::Modified);
        assert_eq!(GitStatus::Staged, GitStatus::Staged);
        assert_eq!(GitStatus::Unchanged, GitStatus::Unchanged);
        assert_eq!(GitStatus::Ignored, GitStatus::Ignored);
    }

    #[test]
    fn test_git_status_clone() {
        let status = GitStatus::Modified;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_debug() {
        let status = GitStatus::Untracked;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Untracked"));
    }

    #[test]
    fn test_git_status_modified_debug() {
        let status = GitStatus::Modified;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Modified"));
    }

    #[test]
    fn test_git_status_staged_debug() {
        let status = GitStatus::Staged;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Staged"));
    }

    #[test]
    fn test_git_status_unchanged_debug() {
        let status = GitStatus::Unchanged;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Unchanged"));
    }

    #[test]
    fn test_git_status_ignored_debug() {
        let status = GitStatus::Ignored;
        let debug = format!("{:?}", status);
        assert!(debug.contains("Ignored"));
    }

    #[test]
    fn test_git_service_name() {
        let service = GitService::new();
        assert!(!service.name().is_empty());
    }

    #[test]
    fn test_git_status_all_variants_clone() {
        let variants = [
            GitStatus::Untracked,
            GitStatus::Modified,
            GitStatus::Staged,
            GitStatus::Unchanged,
            GitStatus::Ignored,
        ];
        for v in variants {
            let cloned = v.clone();
            assert_eq!(v, cloned);
        }
    }

    #[test]
    fn test_git_status_eq_hash() {
        // Can't hash GitStatus since it doesn't derive Hash, but we can test Eq
        assert_eq!(GitStatus::Modified, GitStatus::Modified);
        assert_ne!(GitStatus::Modified, GitStatus::Staged);
        assert_ne!(GitStatus::Untracked, GitStatus::Ignored);
        assert_ne!(GitStatus::Unchanged, GitStatus::Modified);
    }

    #[test]
    fn test_git_service_new_name() {
        let service = GitService::new();
        assert_eq!(service.name(), "git");
    }

    #[test]
    fn test_git_status_all_inequality() {
        let variants = [
            GitStatus::Untracked,
            GitStatus::Modified,
            GitStatus::Staged,
            GitStatus::Unchanged,
            GitStatus::Ignored,
        ];
        for (i, a) in variants.iter().enumerate() {
            for (j, b) in variants.iter().enumerate() {
                if i == j {
                    assert_eq!(a, b);
                } else {
                    assert_ne!(a, b);
                }
            }
        }
    }

    #[test]
    fn test_git_status_size() {
        // GitStatus is an enum and should be small
        assert!(std::mem::size_of::<GitStatus>() <= 8);
    }

    #[tokio::test]
    async fn test_find_repo_root_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        let result = GitService::find_repo_root(&path).await;
        // Should be None for non-existent path
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_current_branch_nonexistent() {
        let path = PathBuf::from("/nonexistent/path");
        let result = GitService::current_branch(&path).await;
        // Should be None for non-existent path
        assert!(result.is_none());
    }

    #[test]
    fn test_git_status_untracked_clone() {
        let status = GitStatus::Untracked;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_modified_clone() {
        let status = GitStatus::Modified;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_service_name_is_git() {
        let service = GitService::new();
        assert_eq!(service.name, "git");
    }

    #[test]
    fn test_git_service_repo_root_none_initially() {
        let service = GitService::new();
        assert!(service.repo_root.is_none());
    }

    #[test]
    fn test_git_status_ignored_clone() {
        let status = GitStatus::Ignored;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_git_status_staged_clone() {
        let status = GitStatus::Staged;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_branch_info_new() {
        let info = BranchInfo::new("main");
        assert_eq!(info.name, "main");
        assert_eq!(info.ahead, 0);
        assert_eq!(info.behind, 0);
        assert!(!info.detached);
    }

    #[test]
    fn test_branch_info_with_remote() {
        let info = BranchInfo::new("main").with_remote(2, 3);
        assert_eq!(info.ahead, 2);
        assert_eq!(info.behind, 3);
    }

    #[test]
    fn test_branch_info_detached() {
        let info = BranchInfo::new("abc123def").detached();
        assert!(info.detached);
        assert!(info.display().starts_with('('));
    }

    #[test]
    fn test_branch_info_display_simple() {
        let info = BranchInfo::new("main");
        assert_eq!(info.display(), " main");
    }

    #[test]
    fn test_branch_info_display_ahead() {
        let info = BranchInfo::new("main").with_remote(3, 0);
        assert!(info.display().contains("↑3"));
    }

    #[test]
    fn test_branch_info_display_behind() {
        let info = BranchInfo::new("main").with_remote(0, 2);
        assert!(info.display().contains("↓2"));
    }

    #[test]
    fn test_file_indicator_char() {
        assert_eq!(FileIndicator::Modified.char(), Some('M'));
        assert_eq!(FileIndicator::Added.char(), Some('A'));
        assert_eq!(FileIndicator::None.char(), None);
    }

    #[test]
    fn test_file_indicator_from_status() {
        assert_eq!(
            FileIndicator::from_status(&GitStatus::Modified),
            FileIndicator::Modified
        );
        assert_eq!(
            FileIndicator::from_status(&GitStatus::Unchanged),
            FileIndicator::None
        );
    }

    #[test]
    fn test_repo_stats_default() {
        let stats = RepoStats::default();
        assert_eq!(stats.modified, 0);
        assert_eq!(stats.staged, 0);
    }

    #[test]
    fn test_git_status_default() {
        assert_eq!(GitStatus::default(), GitStatus::Unchanged);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Hunk and Gutter Sign Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_hunk_creation() {
        let hunk = Hunk::new(HunkType::Add, 10, 5);
        assert_eq!(hunk.hunk_type, HunkType::Add);
        assert_eq!(hunk.start_line, 10);
        assert_eq!(hunk.lines, 5);
    }

    #[test]
    fn test_hunk_with_original() {
        let hunk = Hunk::new(HunkType::Change, 10, 5).with_original(8, 4);
        assert_eq!(hunk.orig_start, 8);
        assert_eq!(hunk.orig_lines, 4);
    }

    #[test]
    fn test_hunk_with_diff() {
        let hunk = Hunk::new(HunkType::Add, 10, 5).with_diff("+new line");
        assert_eq!(hunk.diff_text, "+new line");
    }

    #[test]
    fn test_hunk_contains_line() {
        let hunk = Hunk::new(HunkType::Add, 10, 5);
        assert!(!hunk.contains_line(9));
        assert!(hunk.contains_line(10));
        assert!(hunk.contains_line(14));
        assert!(!hunk.contains_line(15));
    }

    #[test]
    fn test_hunk_contains_line_delete() {
        // Delete hunk with 0 lines still covers at least 1 line (the marker)
        let hunk = Hunk::new(HunkType::Delete, 10, 0);
        assert!(hunk.contains_line(10));
        assert!(!hunk.contains_line(11));
    }

    #[test]
    fn test_gutter_sign_char() {
        assert_eq!(GutterSign::Added.char(), '+');
        assert_eq!(GutterSign::Changed.char(), '~');
        assert_eq!(GutterSign::Deleted.char(), '-');
        assert_eq!(GutterSign::ChangeStart.char(), '┃');
        assert_eq!(GutterSign::ChangeEnd.char(), '┃');
    }

    #[test]
    fn test_buffer_hunks_empty() {
        let hunks = BufferHunks::new();
        assert!(hunks.hunks().is_empty());
        assert_eq!(hunks.count(), 0);
        assert!(hunks.sign(10).is_none());
    }

    #[test]
    fn test_buffer_hunks_add_hunk() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Add, 5, 3));
        assert_eq!(hunks.count(), 1);
    }

    #[test]
    fn test_buffer_hunks_sign() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Add, 5, 3));
        hunks.add_hunk(Hunk::new(HunkType::Change, 10, 2));

        assert!(hunks.sign(4).is_none());
        assert_eq!(hunks.sign(5), Some(GutterSign::Added));
        assert_eq!(hunks.sign(6), Some(GutterSign::Added));
        assert_eq!(hunks.sign(7), Some(GutterSign::Added));
        assert!(hunks.sign(8).is_none());
        assert_eq!(hunks.sign(10), Some(GutterSign::Changed));
    }

    #[test]
    fn test_buffer_hunks_hunk_at() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Delete, 5, 1));

        assert!(hunks.hunk_at(4).is_none());
        assert!(hunks.hunk_at(5).is_some());
        assert!(hunks.hunk_at(6).is_none());
    }

    #[test]
    fn test_buffer_hunks_next_hunk() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Add, 5, 2));
        hunks.add_hunk(Hunk::new(HunkType::Change, 15, 3));

        assert!(hunks.next_hunk(1).is_some());
        assert_eq!(hunks.next_hunk(1).unwrap().start_line, 5);
        assert!(hunks.next_hunk(5).is_some());
        assert_eq!(hunks.next_hunk(5).unwrap().start_line, 15);
        assert!(hunks.next_hunk(15).is_none());
    }

    #[test]
    fn test_buffer_hunks_prev_hunk() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Add, 5, 2));
        hunks.add_hunk(Hunk::new(HunkType::Change, 15, 3));

        assert!(hunks.prev_hunk(20).is_some());
        assert_eq!(hunks.prev_hunk(20).unwrap().start_line, 15);
        assert!(hunks.prev_hunk(15).is_some());
        assert_eq!(hunks.prev_hunk(15).unwrap().start_line, 5);
        assert!(hunks.prev_hunk(5).is_none());
    }

    #[test]
    fn test_buffer_hunks_clear() {
        let mut hunks = BufferHunks::new();
        hunks.add_hunk(Hunk::new(HunkType::Add, 5, 2));
        assert_eq!(hunks.count(), 1);
        hunks.clear();
        assert_eq!(hunks.count(), 0);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Blame Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_blame_info_creation() {
        let blame = BlameInfo::new("abc123", "John Doe");
        assert_eq!(blame.commit, "abc123");
        assert_eq!(blame.author, "John Doe");
        assert!(blame.time.is_empty());
        assert!(blame.summary.is_empty());
        assert!(!blame.uncommitted);
    }

    #[test]
    fn test_blame_info_with_time() {
        let blame = BlameInfo::new("abc123", "John Doe").with_time("2 days ago");
        assert_eq!(blame.time, "2 days ago");
    }

    #[test]
    fn test_blame_info_with_summary() {
        let blame = BlameInfo::new("abc123", "John Doe").with_summary("Fix bug");
        assert_eq!(blame.summary, "Fix bug");
    }

    #[test]
    fn test_blame_info_uncommitted() {
        let blame = BlameInfo::uncommitted();
        assert!(blame.uncommitted);
        assert_eq!(blame.author, "Not Committed Yet");
    }

    #[test]
    fn test_blame_info_display() {
        let blame = BlameInfo::new("abcdef123456", "John Doe")
            .with_time("2 days ago")
            .with_summary("Fix bug in parser");
        let display = blame.display();
        assert!(display.contains("John Doe"));
        assert!(display.contains("2 days ago"));
    }

    #[test]
    fn test_blame_info_display_uncommitted() {
        let blame = BlameInfo::uncommitted();
        assert_eq!(blame.display(), "Not Committed Yet");
    }

    #[test]
    fn test_buffer_blame_empty() {
        let blame = BufferBlame::new();
        assert!(blame.get(1).is_none());
    }

    #[test]
    fn test_buffer_blame_set_and_get() {
        let mut blame = BufferBlame::new();
        let info = BlameInfo::new("abc", "Author");
        blame.set(5, info);

        assert!(blame.get(4).is_none());
        assert!(blame.get(5).is_some());
        assert_eq!(blame.get(5).unwrap().author, "Author");
    }

    #[test]
    fn test_buffer_blame_clear() {
        let mut blame = BufferBlame::new();
        blame.set(5, BlameInfo::new("abc", "A"));
        assert!(blame.get(5).is_some());
        blame.clear();
        assert!(blame.get(5).is_none());
    }

    #[test]
    fn test_buffer_blame_toggle() {
        let mut blame = BufferBlame::new();
        assert!(!blame.visible);
        blame.toggle();
        assert!(blame.visible);
        blame.toggle();
        assert!(!blame.visible);
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Merge Conflict Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_conflict_marker_variants() {
        let _ours = ConflictMarker::OursStart;
        let _sep = ConflictMarker::Separator;
        let _theirs = ConflictMarker::TheirsStart;
        let _base = ConflictMarker::BaseMarker;
    }

    #[test]
    fn test_conflict_new() {
        let conflict = Conflict::new(1, 5, 9);
        assert_eq!(conflict.start_line, 1);
        assert_eq!(conflict.separator_line, 5);
        assert_eq!(conflict.end_line, 9);
    }

    #[test]
    fn test_conflict_line_count() {
        let conflict = Conflict::new(1, 5, 9);
        assert_eq!(conflict.line_count(), 9);
    }

    #[test]
    fn test_conflict_contains_line() {
        let conflict = Conflict::new(5, 7, 10);
        assert!(!conflict.contains_line(4));
        assert!(conflict.contains_line(5));
        assert!(conflict.contains_line(7));
        assert!(conflict.contains_line(10));
        assert!(!conflict.contains_line(11));
    }

    #[test]
    fn test_buffer_conflicts_parse_simple() {
        let content = r#"normal line
<<<<<<< HEAD
our changes
=======
their changes
>>>>>>> feature-branch
more normal"#;

        let conflicts = BufferConflicts::parse(content);
        assert_eq!(conflicts.count(), 1);

        let conflict = &conflicts.conflicts()[0];
        assert_eq!(conflict.start_line, 2);
        assert_eq!(conflict.branch, "feature-branch");
    }

    #[test]
    fn test_buffer_conflicts_parse_with_base() {
        let content = r#"<<<<<<< HEAD
ours
||||||| merged common ancestors
base
=======
theirs
>>>>>>> branch"#;

        let conflicts = BufferConflicts::parse(content);
        assert_eq!(conflicts.count(), 1);

        let conflict = &conflicts.conflicts()[0];
        assert!(conflict.base_line.is_some());
    }

    #[test]
    fn test_buffer_conflicts_parse_multiple() {
        let content = r#"<<<<<<< HEAD
a1
=======
b1
>>>>>>> branch
text
<<<<<<< HEAD
a2
=======
b2
>>>>>>> branch"#;

        let conflicts = BufferConflicts::parse(content);
        assert_eq!(conflicts.count(), 2);
    }

    #[test]
    fn test_buffer_conflicts_has_conflicts() {
        let conflicts = BufferConflicts::parse("no conflicts here");
        assert!(!conflicts.has_conflicts());

        let conflicts = BufferConflicts::parse("<<<<<<< HEAD\na\n=======\nb\n>>>>>>> b");
        assert!(conflicts.has_conflicts());
    }

    #[test]
    fn test_buffer_conflicts_next_conflict() {
        let content = r#"<<<<<<< HEAD
a
=======
b
>>>>>>> branch
gap
<<<<<<< HEAD
c
=======
d
>>>>>>> branch"#;

        let conflicts = BufferConflicts::parse(content);

        let next = conflicts.next_conflict(1);
        assert!(next.is_some());

        let next = conflicts.next_conflict(6);
        assert!(next.is_some());
        assert_eq!(next.unwrap().start_line, 7);
    }

    #[test]
    fn test_buffer_conflicts_prev_conflict() {
        let content = r#"<<<<<<< HEAD
a
=======
b
>>>>>>> branch
gap
<<<<<<< HEAD
c
=======
d
>>>>>>> branch"#;

        let conflicts = BufferConflicts::parse(content);

        let prev = conflicts.prev_conflict(1);
        assert!(prev.is_none());

        let prev = conflicts.prev_conflict(100);
        assert!(prev.is_some());
    }

    #[test]
    fn test_conflict_choice_variants() {
        let _ours = ConflictChoice::Ours;
        let _theirs = ConflictChoice::Theirs;
        let _both = ConflictChoice::Both;
        let _base = ConflictChoice::Base;
    }

    // ═══════════════════════════════════════════════════════════════════════════════
    // Diff Mode Tests
    // ═══════════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_diff_algorithm_default() {
        assert_eq!(DiffAlgorithm::default(), DiffAlgorithm::Myers);
    }

    #[test]
    fn test_diff_algorithm_variants() {
        let _myers = DiffAlgorithm::Myers;
        let _minimal = DiffAlgorithm::Minimal;
        let _patience = DiffAlgorithm::Patience;
        let _histogram = DiffAlgorithm::Histogram;
    }

    #[test]
    fn test_diff_options_default() {
        let opts = DiffOptions::default();
        assert!(!opts.ignore_whitespace);
        assert_eq!(opts.algorithm, DiffAlgorithm::Myers);
    }

    #[test]
    fn test_diff_options_new() {
        let opts = DiffOptions::new();
        assert_eq!(opts.context, 3);
    }

    #[test]
    fn test_diff_options_patience() {
        let opts = DiffOptions::new().patience();
        assert_eq!(opts.algorithm, DiffAlgorithm::Patience);
    }

    #[test]
    fn test_diff_options_ignore_whitespace() {
        let opts = DiffOptions::new().ignore_whitespace();
        assert!(opts.ignore_whitespace);
    }

    #[test]
    fn test_diff_layout_default() {
        assert_eq!(DiffLayout::default(), DiffLayout::Vertical);
    }

    #[test]
    fn test_diff_view_new() {
        let view = DiffView::new();
        assert_eq!(view.layout, DiffLayout::Vertical);
        assert!(view.scroll_sync);
        assert!(!view.active);
    }

    #[test]
    fn test_diff_view_files() {
        let view = DiffView::new().files(PathBuf::from("/a"), PathBuf::from("/b"));
        assert_eq!(view.left, Some(PathBuf::from("/a")));
        assert_eq!(view.right, Some(PathBuf::from("/b")));
    }

    #[test]
    fn test_diff_view_with_layout() {
        let view = DiffView::new().with_layout(DiffLayout::Horizontal);
        assert_eq!(view.layout, DiffLayout::Horizontal);
    }

    #[test]
    fn test_diff_view_activate_deactivate() {
        let mut view = DiffView::new();
        assert!(!view.active);
        view.activate();
        assert!(view.active);
        view.deactivate();
        assert!(!view.active);
    }

    #[test]
    fn test_diff_view_toggle_scroll_sync() {
        let mut view = DiffView::new();
        assert!(view.scroll_sync);
        view.toggle_scroll_sync();
        assert!(!view.scroll_sync);
        view.toggle_scroll_sync();
        assert!(view.scroll_sync);
    }
}
