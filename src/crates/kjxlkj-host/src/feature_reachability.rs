//! Feature reachability analysis: verify every feature has an entry point.

use serde::{Deserialize, Serialize};

/// How a feature is reachable by the user.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntryKind {
    Keybinding,
    ExCommand,
    LeaderChord,
    MouseAction,
}

/// A declared feature with its entry points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSpec {
    pub name: String,
    pub entry_points: Vec<(EntryKind, String)>,
    pub tested: bool,
}

/// Report summarising reachability of all features.
#[derive(Debug, Clone)]
pub struct ReachabilityReport {
    pub features: Vec<FeatureSpec>,
    pub reachable_count: usize,
    pub unreachable_count: usize,
}

/// Check whether a feature has at least one keybinding entry.
pub fn has_keybinding_entry(spec: &FeatureSpec) -> bool {
    spec.entry_points.iter().any(|(k, _)| *k == EntryKind::Keybinding)
}

/// Check whether a feature has at least one ex-command entry.
pub fn has_command_entry(spec: &FeatureSpec) -> bool {
    spec.entry_points.iter().any(|(k, _)| *k == EntryKind::ExCommand)
}

/// Analyse reachability: features with at least one entry point are reachable.
pub fn check_reachability(features: Vec<FeatureSpec>) -> ReachabilityReport {
    let reachable_count = features.iter()
        .filter(|f| !f.entry_points.is_empty())
        .count();
    let unreachable_count = features.len() - reachable_count;
    ReachabilityReport { features, reachable_count, unreachable_count }
}

/// Define the core features of the editor (15+ entries).
pub fn define_core_features() -> Vec<FeatureSpec> {
    vec![
        feat("cursor-movement", &[(EntryKind::Keybinding, "h/j/k/l")], true),
        feat("insert-mode", &[(EntryKind::Keybinding, "i")], true),
        feat("visual-mode", &[(EntryKind::Keybinding, "v")], true),
        feat("command-mode", &[(EntryKind::Keybinding, ":")], true),
        feat("undo", &[(EntryKind::Keybinding, "u")], true),
        feat("redo", &[(EntryKind::Keybinding, "Ctrl-r")], true),
        feat("write", &[(EntryKind::ExCommand, ":w")], true),
        feat("quit", &[(EntryKind::ExCommand, ":q")], true),
        feat("search", &[(EntryKind::Keybinding, "/")], true),
        feat("replace", &[(EntryKind::ExCommand, ":s")], true),
        feat("yank", &[(EntryKind::Keybinding, "y")], true),
        feat("paste", &[(EntryKind::Keybinding, "p")], true),
        feat("delete", &[(EntryKind::Keybinding, "d")], true),
        feat("file-explorer", &[(EntryKind::LeaderChord, "<leader>e")], true),
        feat("terminal-pane", &[(EntryKind::LeaderChord, "<leader>t")], true),
        feat("goto-definition", &[(EntryKind::Keybinding, "gd")], false),
    ]
}

fn feat(name: &str, entries: &[(EntryKind, &str)], tested: bool) -> FeatureSpec {
    FeatureSpec {
        name: name.into(),
        entry_points: entries.iter()
            .map(|(k, s)| (k.clone(), s.to_string()))
            .collect(),
        tested,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_features_count() {
        assert!(define_core_features().len() >= 15);
    }

    #[test]
    fn all_reachable() {
        let report = check_reachability(define_core_features());
        assert_eq!(report.unreachable_count, 0);
    }

    #[test]
    fn has_keybinding() {
        let f = define_core_features();
        assert!(has_keybinding_entry(&f[0]));
    }

    #[test]
    fn has_command() {
        let f = define_core_features();
        let write = f.iter().find(|x| x.name == "write").unwrap();
        assert!(has_command_entry(write));
    }

    #[test]
    fn unreachable_feature() {
        let spec = FeatureSpec {
            name: "orphan".into(),
            entry_points: vec![],
            tested: false,
        };
        let report = check_reachability(vec![spec]);
        assert_eq!(report.unreachable_count, 1);
        assert_eq!(report.reachable_count, 0);
    }

    #[test]
    fn entry_kind_eq() {
        assert_eq!(EntryKind::Keybinding, EntryKind::Keybinding);
        assert_ne!(EntryKind::Keybinding, EntryKind::ExCommand);
    }
}
