//! UX keyboard-only coverage tracking.

use serde::{Deserialize, Serialize};

/// A single coverage entry for a key binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageEntry {
    pub key: String,
    pub action: String,
    pub mode: String,
    pub tested: bool,
    pub documented: bool,
}

/// Summary of coverage across entries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CoverageSummary {
    pub total: usize,
    pub tested: usize,
    pub documented: usize,
    pub gaps: Vec<String>,
}

/// Compute a summary from entries.
pub fn compute_summary(entries: &[CoverageEntry]) -> CoverageSummary {
    let tested = entries.iter().filter(|e| e.tested).count();
    let documented = entries.iter().filter(|e| e.documented).count();
    let gaps: Vec<String> = entries.iter()
        .filter(|e| !e.tested || !e.documented)
        .map(|e| e.key.clone())
        .collect();
    CoverageSummary { total: entries.len(), tested, documented, gaps }
}

/// Find entries that have not been tested.
pub fn find_untested(entries: &[CoverageEntry]) -> Vec<&CoverageEntry> {
    entries.iter().filter(|e| !e.tested).collect()
}

/// Find entries that have not been documented.
pub fn find_undocumented(entries: &[CoverageEntry]) -> Vec<&CoverageEntry> {
    entries.iter().filter(|e| !e.documented).collect()
}

/// Check that every entry is keyboard-accessible (has a non-empty key).
pub fn keyboard_only_check(entries: &[CoverageEntry]) -> bool {
    entries.iter().all(|e| !e.key.is_empty())
}

fn entry(key: &str, action: &str, mode: &str, tested: bool, doc: bool) -> CoverageEntry {
    CoverageEntry {
        key: key.into(), action: action.into(), mode: mode.into(), tested, documented: doc,
    }
}

/// Build normal-mode coverage list (30+ keys).
pub fn build_normal_coverage() -> Vec<CoverageEntry> {
    vec![
        entry("h","move_left","normal",true,true),
        entry("j","move_down","normal",true,true),
        entry("k","move_up","normal",true,true),
        entry("l","move_right","normal",true,true),
        entry("w","word_fwd","normal",true,true),
        entry("b","word_back","normal",true,true),
        entry("e","word_end","normal",true,true),
        entry("0","line_start","normal",true,true),
        entry("$","line_end","normal",true,true),
        entry("gg","goto_top","normal",true,true),
        entry("G","goto_bottom","normal",true,true),
        entry("d","delete","normal",true,true),
        entry("c","change","normal",true,true),
        entry("y","yank","normal",true,true),
        entry("p","paste","normal",true,true),
        entry("u","undo","normal",true,true),
        entry("i","insert","normal",true,true),
        entry("a","append","normal",true,true),
        entry("o","open_below","normal",true,true),
        entry("v","visual","normal",true,true),
        entry("/","search","normal",true,true),
        entry("n","next_match","normal",true,true),
        entry(":","command","normal",true,true),
        entry("x","del_char","normal",true,true),
        entry("r","replace_char","normal",true,true),
        entry("J","join","normal",true,true),
        entry(".","repeat","normal",true,true),
        entry("f","find_char","normal",true,true),
        entry("t","till_char","normal",true,true),
        entry("*","star_search","normal",true,true),
        entry("m","set_mark","normal",true,false),
        entry("q","macro","normal",true,false),
    ]
}

/// Build insert-mode coverage list.
pub fn build_insert_coverage() -> Vec<CoverageEntry> {
    vec![
        entry("<Esc>","exit_insert","insert",true,true),
        entry("<C-c>","exit_insert","insert",true,true),
        entry("<C-w>","delete_word","insert",true,true),
        entry("<C-u>","delete_line","insert",true,true),
        entry("<C-h>","backspace","insert",true,true),
        entry("<C-n>","complete_next","insert",true,false),
        entry("<C-p>","complete_prev","insert",true,false),
        entry("<C-o>","normal_once","insert",true,true),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_coverage_30_plus() {
        assert!(build_normal_coverage().len() >= 30);
    }

    #[test]
    fn summary_computed() {
        let entries = build_normal_coverage();
        let s = compute_summary(&entries);
        assert_eq!(s.total, entries.len());
        assert!(s.tested > 0);
    }

    #[test]
    fn find_gaps() {
        let entries = build_normal_coverage();
        let undoc = find_undocumented(&entries);
        assert!(!undoc.is_empty());
    }

    #[test]
    fn keyboard_only() {
        assert!(keyboard_only_check(&build_normal_coverage()));
    }

    #[test]
    fn insert_coverage() {
        let entries = build_insert_coverage();
        assert!(entries.len() >= 5);
    }

    #[test]
    fn untested_empty_for_normal() {
        let entries = build_normal_coverage();
        assert!(find_untested(&entries).is_empty());
    }
}
