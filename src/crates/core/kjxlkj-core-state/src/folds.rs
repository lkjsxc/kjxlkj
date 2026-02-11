//! Fold state management for code folding.
//!
//! See /docs/spec/features/syntax/folding.md. Supports indent-based
//! fold computation. Tree-sitter/expression folds deferred.

use std::collections::HashSet;

/// Per-buffer fold state.
#[derive(Debug, Clone)]
pub struct FoldState {
    fold_starts: Vec<FoldRegion>,
    closed: HashSet<usize>,
    pub fold_level: usize,
    pub fold_level_max: usize,
}

/// A fold region (start/end inclusive, 0-indexed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoldRegion { pub start: usize, pub end: usize, pub level: usize }

impl FoldState {
    pub fn new() -> Self {
        Self { fold_starts: Vec::new(), closed: HashSet::new(), fold_level: 99, fold_level_max: 99 }
    }

    /// Compute indent-based folds from buffer lines.
    pub fn compute_indent_folds(&mut self, lines: &[&str]) {
        self.fold_starts.clear();
        self.closed.clear();
        if lines.is_empty() { return; }
        let levels: Vec<usize> = lines.iter().map(|l| {
            if l.trim().is_empty() { usize::MAX } else { indent_level(l) }
        }).collect();
        let resolved: Vec<usize> = levels.iter().enumerate().map(|(i, &lv)| {
            if lv == usize::MAX {
                let prev = (0..i).rev().find_map(|j| if levels[j] != usize::MAX { Some(levels[j]) } else { None }).unwrap_or(0);
                let next = ((i+1)..levels.len()).find_map(|j| if levels[j] != usize::MAX { Some(levels[j]) } else { None }).unwrap_or(0);
                prev.min(next)
            } else { lv }
        }).collect();
        let n = resolved.len();
        for i in 0..n {
            let cur = resolved[i];
            if i + 1 < n && resolved[i + 1] > cur {
                if let Some(end) = fold_end(&resolved, i, cur) {
                    self.fold_starts.push(FoldRegion { start: i, end, level: cur + 1 });
                }
            }
        }
    }

    /// Open fold at cursor line. Returns true if a fold was opened.
    pub fn open(&mut self, line: usize) -> bool { self.closed.remove(&line) }

    /// Close fold at cursor line. Returns true if fold exists.
    pub fn close(&mut self, line: usize) -> bool {
        if self.fold_starts.iter().any(|r| r.start == line) {
            self.closed.insert(line);
            true
        } else { false }
    }

    /// Toggle fold at cursor line.
    pub fn toggle(&mut self, line: usize) -> bool {
        if self.closed.contains(&line) { self.open(line) }
        else { self.close(line) }
    }

    /// Open all folds.
    pub fn open_all(&mut self) { self.closed.clear(); self.fold_level = self.fold_level_max; }

    /// Close all folds.
    pub fn close_all(&mut self) {
        self.fold_level = 0;
        for r in &self.fold_starts { self.closed.insert(r.start); }
    }

    /// Reduce fold level by 1 (open one more level).
    pub fn reduce(&mut self) {
        if self.fold_level < self.fold_level_max { self.fold_level += 1; }
        let starts = self.fold_starts.clone();
        self.closed.retain(|&line| starts.iter().any(|r| r.start == line && r.level > self.fold_level));
    }

    /// Increase fold level by 1 (close one more level).
    pub fn more(&mut self) {
        if self.fold_level > 0 { self.fold_level -= 1; }
        for r in &self.fold_starts { if r.level > self.fold_level { self.closed.insert(r.start); } }
    }

    /// Find the next closed fold start after `line`.
    pub fn next_closed(&self, line: usize) -> Option<usize> {
        self.fold_starts.iter().filter(|r| r.start > line && self.closed.contains(&r.start)).map(|r| r.start).min()
    }

    /// Find the previous closed fold start before `line`.
    pub fn prev_closed(&self, line: usize) -> Option<usize> {
        self.fold_starts.iter().filter(|r| r.start < line && self.closed.contains(&r.start)).map(|r| r.start).max()
    }

    /// Check if a line is inside a closed fold (hidden).
    pub fn is_hidden(&self, line: usize) -> bool {
        self.fold_starts.iter().any(|r| self.closed.contains(&r.start) && line > r.start && line <= r.end)
    }
    pub fn regions(&self) -> &[FoldRegion] { &self.fold_starts }
    pub fn closed_set(&self) -> &HashSet<usize> { &self.closed }
}

fn indent_level(line: &str) -> usize {
    let spaces: usize = line.chars().take_while(|c| c.is_whitespace()).count();
    spaces / 4 // 4-space indentation
}

fn fold_end(levels: &[usize], start: usize, base: usize) -> Option<usize> {
    let mut end = start;
    for i in (start + 1)..levels.len() {
        if levels[i] > base { end = i; } else { break; }
    }
    if end > start { Some(end) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indent_level_basic() {
        assert_eq!(indent_level("hello"), 0);
        assert_eq!(indent_level("    hello"), 1);
        assert_eq!(indent_level("        hello"), 2);
    }

    #[test]
    fn compute_folds_simple() {
        let lines = vec!["fn main() {", "    let x = 1;", "    let y = 2;", "}"];
        let mut fs = FoldState::new();
        fs.compute_indent_folds(&lines);
        assert!(!fs.regions().is_empty());
        assert_eq!(fs.regions()[0].start, 0);
    }

    #[test]
    fn open_close_toggle() {
        let lines = vec!["fn main() {", "    body", "}"];
        let mut fs = FoldState::new();
        fs.compute_indent_folds(&lines);
        assert!(!fs.closed.contains(&0));
        fs.close(0);
        assert!(fs.closed.contains(&0));
        fs.toggle(0);
        assert!(!fs.closed.contains(&0));
    }

    #[test]
    fn close_all_open_all() {
        let lines = vec!["fn a() {", "    x", "}", "fn b() {", "    y", "}"];
        let mut fs = FoldState::new();
        fs.compute_indent_folds(&lines);
        fs.close_all();
        assert!(!fs.closed.is_empty());
        fs.open_all();
        assert!(fs.closed.is_empty());
    }

    #[test]
    fn is_hidden_in_closed_fold() {
        let lines = vec!["fn main() {", "    body", "}"];
        let mut fs = FoldState::new();
        fs.compute_indent_folds(&lines);
        fs.close(0);
        assert!(fs.is_hidden(1));
        assert!(!fs.is_hidden(0));
    }

    #[test]
    fn next_prev_closed() {
        let lines = vec!["a {", "    b", "}", "c {", "    d", "}"];
        let mut fs = FoldState::new();
        fs.compute_indent_folds(&lines);
        fs.close_all();
        assert_eq!(fs.next_closed(0), Some(3));
        assert_eq!(fs.prev_closed(5), Some(3));
    }
}
