//! Merge conflict detection, diff viewer types, and file status indicators.

use std::path::PathBuf;

/// File status indicator for explorer/gutter integration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileIndicator {
    Modified, Added, Deleted, Renamed, Untracked, Conflicted, Ignored,
}

impl FileIndicator {
    /// Short character representation for gutter/explorer.
    pub fn symbol(self) -> char {
        match self {
            Self::Modified => 'M', Self::Added => 'A', Self::Deleted => 'D',
            Self::Renamed => 'R', Self::Untracked => '?', Self::Conflicted => 'C',
            Self::Ignored => '!',
        }
    }
}

/// A merge conflict region in a buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    pub ours_start: usize,
    pub ours_end: usize,
    pub theirs_start: usize,
    pub theirs_end: usize,
    pub base_start: Option<usize>,
    pub base_end: Option<usize>,
}

/// A conflict marker type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictMarker { Ours, Base, Theirs, Separator }

/// A resolution choice for a conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictChoice { Ours, Theirs, Both, Base, None }

/// All conflicts in a buffer.
#[derive(Debug, Clone, Default)]
pub struct BufferConflicts { pub conflicts: Vec<Conflict> }

impl BufferConflicts {
    pub fn new() -> Self { Self { conflicts: Vec::new() } }
    pub fn len(&self) -> usize { self.conflicts.len() }
    pub fn is_empty(&self) -> bool { self.conflicts.is_empty() }

    /// Detect conflicts from buffer lines by scanning for markers.
    pub fn detect(lines: &[&str]) -> Self {
        let mut conflicts = Vec::new();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].starts_with("<<<<<<<") {
                let ours_start = i;
                let mut sep = None;
                let mut theirs_end = None;
                let mut j = i + 1;
                while j < lines.len() {
                    if lines[j].starts_with("=======") { sep = Some(j); }
                    if lines[j].starts_with(">>>>>>>") { theirs_end = Some(j); break; }
                    j += 1;
                }
                if let (Some(s), Some(e)) = (sep, theirs_end) {
                    conflicts.push(Conflict {
                        ours_start, ours_end: s, theirs_start: s + 1, theirs_end: e,
                        base_start: None, base_end: None,
                    });
                    i = e + 1; continue;
                }
            }
            i += 1;
        }
        Self { conflicts }
    }
}

/// Diff algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffAlgorithm { Myers, Patience, Histogram }

/// Diff display options.
#[derive(Debug, Clone)]
pub struct DiffOptions {
    pub algorithm: DiffAlgorithm,
    pub context_lines: usize,
    pub ignore_whitespace: bool,
}

impl Default for DiffOptions {
    fn default() -> Self {
        Self { algorithm: DiffAlgorithm::Myers, context_lines: 3, ignore_whitespace: false }
    }
}

/// Diff view layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLayout { Unified, SideBySide, Inline }

/// A diff line in the viewer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    Context(String),
    Added(String),
    Removed(String),
    Header(String),
}

/// A diff view between two versions.
#[derive(Debug, Clone)]
pub struct DiffView {
    pub file_path: PathBuf,
    pub layout: DiffLayout,
    pub lines: Vec<DiffLine>,
    pub options: DiffOptions,
}

impl DiffView {
    pub fn new(path: PathBuf) -> Self {
        Self { file_path: path, layout: DiffLayout::Unified, lines: Vec::new(),
            options: DiffOptions::default() }
    }
    pub fn count_added(&self) -> usize { self.lines.iter().filter(|l| matches!(l, DiffLine::Added(_))).count() }
    pub fn count_removed(&self) -> usize { self.lines.iter().filter(|l| matches!(l, DiffLine::Removed(_))).count() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn file_indicator_symbols() {
        assert_eq!(FileIndicator::Modified.symbol(), 'M');
        assert_eq!(FileIndicator::Untracked.symbol(), '?');
        assert_eq!(FileIndicator::Conflicted.symbol(), 'C');
    }
    #[test]
    fn detect_conflict_markers() {
        let lines = vec![
            "normal line", "<<<<<<< HEAD", "our version", "=======",
            "their version", ">>>>>>> branch", "after conflict",
        ];
        let bc = BufferConflicts::detect(&lines);
        assert_eq!(bc.len(), 1);
        let c = &bc.conflicts[0];
        assert_eq!(c.ours_start, 1);
        assert_eq!(c.ours_end, 3);
        assert_eq!(c.theirs_start, 4);
        assert_eq!(c.theirs_end, 5);
    }
    #[test]
    fn no_conflicts_in_clean_buffer() {
        let lines = vec!["hello", "world"];
        let bc = BufferConflicts::detect(&lines);
        assert!(bc.is_empty());
    }
    #[test]
    fn diff_view_counts() {
        let mut dv = DiffView::new(PathBuf::from("test.rs"));
        dv.lines = vec![
            DiffLine::Context("ctx".into()), DiffLine::Added("new".into()),
            DiffLine::Removed("old".into()), DiffLine::Added("new2".into()),
        ];
        assert_eq!(dv.count_added(), 2);
        assert_eq!(dv.count_removed(), 1);
    }
    #[test]
    fn diff_options_default() {
        let opts = DiffOptions::default();
        assert_eq!(opts.algorithm, DiffAlgorithm::Myers);
        assert_eq!(opts.context_lines, 3);
    }
    #[test]
    fn conflict_choice_variants() {
        assert_ne!(ConflictChoice::Ours, ConflictChoice::Theirs);
        assert_eq!(ConflictMarker::Ours, ConflictMarker::Ours);
    }
}
