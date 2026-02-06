//! Mapping expansion — resolve key mappings with recursion guard and mode scoping.

use std::collections::HashSet;

/// A mapping entry for expansion.
#[derive(Debug, Clone)]
pub struct MappingEntry {
    pub lhs: String,
    pub rhs: String,
    pub mode: String,
    pub recursive: bool,
}

/// Result of expanding a key sequence through mappings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpansionResult {
    Expanded(String),
    NoMapping,
    RecursionLimit(String),
}

/// Maximum recursion depth for recursive mappings.
const MAX_DEPTH: usize = 100;

/// Expand a key sequence through a set of mappings.
pub fn expand_mapping(input: &str, mappings: &[MappingEntry], mode: &str) -> ExpansionResult {
    let mode_maps: Vec<_> = mappings.iter()
        .filter(|m| m.mode == mode || m.mode == "all")
        .collect();
    expand_recursive(input, &mode_maps, 0, &mut HashSet::new())
}

fn expand_recursive(input: &str, mappings: &[&MappingEntry], depth: usize, seen: &mut HashSet<String>) -> ExpansionResult {
    if depth >= MAX_DEPTH { return ExpansionResult::RecursionLimit(input.into()); }
    // Find longest-prefix match
    let mut best: Option<&MappingEntry> = None;
    for m in mappings {
        if input.starts_with(&m.lhs) {
            if best.map_or(true, |b| m.lhs.len() > b.lhs.len()) { best = Some(m); }
        }
    }
    let entry = match best {
        Some(e) => e,
        None => return if depth == 0 { ExpansionResult::NoMapping } else { ExpansionResult::Expanded(input.into()) },
    };
    let remainder = &input[entry.lhs.len()..];
    let expanded_rhs = format!("{}{}", entry.rhs, remainder);
    if !entry.recursive { return ExpansionResult::Expanded(expanded_rhs); }
    if seen.contains(&entry.lhs) { return ExpansionResult::RecursionLimit(expanded_rhs); }
    seen.insert(entry.lhs.clone());
    expand_recursive(&expanded_rhs, mappings, depth + 1, seen)
}

/// Check if a key sequence has a pending prefix match (for timeout behavior).
pub fn has_prefix_match(input: &str, mappings: &[MappingEntry], mode: &str) -> bool {
    mappings.iter()
        .filter(|m| m.mode == mode || m.mode == "all")
        .any(|m| m.lhs.starts_with(input) && m.lhs.len() > input.len())
}

/// Get all mappings for a specific mode, sorted by lhs.
pub fn list_mappings<'a>(mappings: &'a [MappingEntry], mode: &str) -> Vec<&'a MappingEntry> {
    let mut result: Vec<_> = mappings.iter()
        .filter(|m| m.mode == mode || m.mode == "all")
        .collect();
    result.sort_by_key(|m| &m.lhs);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    fn me(lhs: &str, rhs: &str, mode: &str, recursive: bool) -> MappingEntry {
        MappingEntry { lhs: lhs.into(), rhs: rhs.into(), mode: mode.into(), recursive }
    }

    #[test]
    fn noremap_expansion() {
        let maps = vec![me("jj", "<Esc>", "insert", false)];
        let r = expand_mapping("jj", &maps, "insert");
        assert_eq!(r, ExpansionResult::Expanded("<Esc>".into()));
    }

    #[test]
    fn recursive_expansion() {
        let maps = vec![me("a", "b", "normal", true), me("b", "c", "normal", true)];
        let r = expand_mapping("a", &maps, "normal");
        assert_eq!(r, ExpansionResult::Expanded("c".into()));
    }

    #[test]
    fn recursion_guard() {
        let maps = vec![me("a", "a", "normal", true)];
        let r = expand_mapping("a", &maps, "normal");
        assert!(matches!(r, ExpansionResult::RecursionLimit(_)));
    }

    #[test]
    fn no_mapping() {
        let r = expand_mapping("x", &[], "normal");
        assert_eq!(r, ExpansionResult::NoMapping);
    }

    #[test]
    fn mode_filtering() {
        let maps = vec![me("jj", "<Esc>", "insert", false)];
        let r = expand_mapping("jj", &maps, "normal"); // wrong mode
        assert_eq!(r, ExpansionResult::NoMapping);
    }

    #[test]
    fn longest_prefix_match() {
        let maps = vec![me("j", "x", "normal", false), me("jj", "y", "normal", false)];
        let r = expand_mapping("jj", &maps, "normal");
        assert_eq!(r, ExpansionResult::Expanded("y".into()));
    }

    #[test]
    fn remainder_preserved() {
        let maps = vec![me("jj", "<Esc>", "insert", false)];
        let r = expand_mapping("jjk", &maps, "insert");
        assert_eq!(r, ExpansionResult::Expanded("<Esc>k".into()));
    }

    #[test]
    fn prefix_match_detection() {
        let maps = vec![me("jj", "<Esc>", "insert", false)];
        assert!(has_prefix_match("j", &maps, "insert"));
        assert!(!has_prefix_match("jj", &maps, "insert"));
    }

    #[test]
    fn list_sorted() {
        let maps = vec![me("z", "1", "normal", false), me("a", "2", "normal", false)];
        let listed = list_mappings(&maps, "normal");
        assert_eq!(listed[0].lhs, "a");
        assert_eq!(listed[1].lhs, "z");
    }
}
