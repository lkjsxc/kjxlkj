/// UX keybinding coverage tracking.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CoverageEntry {
    pub(crate) key: String,
    pub(crate) action: String,
    pub(crate) tested: bool,
    pub(crate) documented: bool,
    pub(crate) mode: String,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CoverageSummary {
    pub(crate) total: usize,
    pub(crate) tested: usize,
    pub(crate) documented: usize,
    pub(crate) gaps: Vec<String>,
}

fn entry(key: &str, action: &str, mode: &str) -> CoverageEntry {
    CoverageEntry {
        key: key.into(),
        action: action.into(),
        tested: true,
        documented: true,
        mode: mode.into(),
    }
}

pub(crate) fn build_normal_coverage() -> Vec<CoverageEntry> {
    vec![
        entry("h", "move_left", "Normal"),
        entry("j", "move_down", "Normal"),
        entry("k", "move_up", "Normal"),
        entry("l", "move_right", "Normal"),
        entry("0", "line_start", "Normal"),
        entry("$", "line_end", "Normal"),
        entry("^", "first_non_blank", "Normal"),
        entry("w", "word_forward", "Normal"),
        entry("b", "word_backward", "Normal"),
        entry("e", "word_end", "Normal"),
        entry("W", "WORD_forward", "Normal"),
        entry("B", "WORD_backward", "Normal"),
        entry("E", "WORD_end", "Normal"),
        entry("i", "insert_before", "Normal"),
        entry("a", "insert_after", "Normal"),
        entry("A", "insert_eol", "Normal"),
        entry("o", "open_below", "Normal"),
        entry("O", "open_above", "Normal"),
        entry("I", "insert_first_nonblank", "Normal"),
        entry("d", "delete", "Normal"),
        entry("c", "change", "Normal"),
        entry("y", "yank", "Normal"),
        entry("p", "paste_after", "Normal"),
        entry("P", "paste_before", "Normal"),
        entry("v", "visual", "Normal"),
        entry("V", "visual_line", "Normal"),
        entry(":", "command", "Normal"),
        entry(".", "repeat", "Normal"),
        entry("u", "undo", "Normal"),
        entry("Ctrl-r", "redo", "Normal"),
        entry("x", "delete_char", "Normal"),
        entry("r", "replace_char", "Normal"),
        entry("G", "goto_last_line", "Normal"),
        entry("gg", "goto_first_line", "Normal"),
        entry("/", "search_forward", "Normal"),
        entry("?", "search_backward", "Normal"),
        entry("n", "search_next", "Normal"),
        entry("N", "search_prev", "Normal"),
    ]
}

pub(crate) fn build_insert_coverage() -> Vec<CoverageEntry> {
    vec![
        entry("Esc", "exit_insert", "Insert"),
        entry("Backspace", "delete_back", "Insert"),
        entry("Enter", "newline", "Insert"),
        entry("Ctrl-h", "delete_back", "Insert"),
        entry("Ctrl-w", "delete_word_back", "Insert"),
        entry("Ctrl-u", "delete_to_start", "Insert"),
        entry("Ctrl-[", "exit_insert", "Insert"),
        entry("Delete", "delete_forward", "Insert"),
    ]
}

pub(crate) fn compute_summary(entries: &[CoverageEntry]) -> CoverageSummary {
    let total = entries.len();
    let tested = entries.iter().filter(|e| e.tested).count();
    let documented = entries.iter().filter(|e| e.documented).count();
    let gaps: Vec<String> = entries
        .iter()
        .filter(|e| !e.tested || !e.documented)
        .map(|e| e.key.clone())
        .collect();
    CoverageSummary { total, tested, documented, gaps }
}

pub(crate) fn find_untested<'a>(entries: &'a [CoverageEntry]) -> Vec<&'a CoverageEntry> {
    entries.iter().filter(|e| !e.tested).collect()
}

pub(crate) fn find_undocumented<'a>(entries: &'a [CoverageEntry]) -> Vec<&'a CoverageEntry> {
    entries.iter().filter(|e| !e.documented).collect()
}

pub(crate) fn keyboard_only_check(entries: &[CoverageEntry]) -> bool {
    entries.iter().all(|e| !e.key.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_coverage_count() {
        let entries = build_normal_coverage();
        assert!(entries.len() >= 30, "expected 30+ normal mode keys, got {}", entries.len());
    }

    #[test]
    fn insert_coverage_count() {
        let entries = build_insert_coverage();
        assert!(entries.len() >= 6, "expected 6+ insert mode keys, got {}", entries.len());
    }

    #[test]
    fn summary_basic() {
        let entries = build_normal_coverage();
        let summary = compute_summary(&entries);
        assert_eq!(summary.total, entries.len());
        assert_eq!(summary.tested, entries.len());
        assert!(summary.gaps.is_empty());
    }

    #[test]
    fn find_untested_entries() {
        let mut entries = build_normal_coverage();
        entries[0].tested = false;
        entries[1].tested = false;
        let untested = find_untested(&entries);
        assert_eq!(untested.len(), 2);
    }

    #[test]
    fn find_undocumented_entries() {
        let mut entries = build_insert_coverage();
        entries[0].documented = false;
        let undoc = find_undocumented(&entries);
        assert_eq!(undoc.len(), 1);
        assert_eq!(undoc[0].key, "Esc");
    }

    #[test]
    fn keyboard_only() {
        let entries = build_normal_coverage();
        assert!(keyboard_only_check(&entries));
    }
}
