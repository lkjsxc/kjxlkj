/// Feature behavior reachability via keybindings/commands.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FeatureSpec {
    pub(crate) name: String,
    pub(crate) entry_points: Vec<String>,
    pub(crate) tested: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum EntryKind {
    Keybinding(String),
    ExCommand(String),
    LeaderChord(String),
    MouseAction(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ReachabilityReport {
    pub(crate) features: Vec<FeatureSpec>,
    pub(crate) reachable: usize,
    pub(crate) unreachable: usize,
}

fn feature(name: &str, entries: &[&str]) -> FeatureSpec {
    FeatureSpec {
        name: name.into(),
        entry_points: entries.iter().map(|s| s.to_string()).collect(),
        tested: true,
    }
}

pub(crate) fn define_core_features() -> Vec<FeatureSpec> {
    vec![
        feature("open_file", &[":e", ":edit", "<Leader>e"]),
        feature("save_file", &[":w", ":write", "ZZ"]),
        feature("quit", &[":q", ":quit", "ZQ"]),
        feature("save_quit", &[":wq", ":x", "ZZ"]),
        feature("search", &["/", "?", ":s"]),
        feature("replace", &[":s///", ":%s///"]),
        feature("undo", &["u", ":undo"]),
        feature("redo", &["Ctrl-r", ":redo"]),
        feature("split_horizontal", &[":sp", ":split", "Ctrl-w s"]),
        feature("split_vertical", &[":vs", ":vsplit", "Ctrl-w v"]),
        feature("tab_new", &[":tabnew", ":tabe"]),
        feature("tab_switch", &["gt", "gT", ":tabn"]),
        feature("terminal", &[":terminal", ":term"]),
        feature("explorer", &[":Explore", "<Leader>f"]),
        feature("git_status", &[":Git", "<Leader>g"]),
        feature("lsp_goto", &["gd", ":LspDef"]),
        feature("session_save", &[":mksession"]),
    ]
}

pub(crate) fn check_reachability(features: &[FeatureSpec]) -> ReachabilityReport {
    let reachable = features.iter().filter(|f| !f.entry_points.is_empty()).count();
    let unreachable = features.len() - reachable;
    ReachabilityReport {
        features: features.to_vec(),
        reachable,
        unreachable,
    }
}

pub(crate) fn format_reachability(report: &ReachabilityReport) -> String {
    let mut out = String::from("Reachability Report\n===================\n");
    for f in &report.features {
        let status = if f.entry_points.is_empty() { "UNREACHABLE" } else { "OK" };
        out.push_str(&format!("[{}] {} (entries: {})\n", status, f.name, f.entry_points.len()));
    }
    out.push_str(&format!(
        "\nTotal: {} | Reachable: {} | Unreachable: {}\n",
        report.features.len(),
        report.reachable,
        report.unreachable
    ));
    out
}

pub(crate) fn has_keybinding_entry(spec: &FeatureSpec) -> bool {
    spec.entry_points.iter().any(|e| {
        !e.starts_with(':') && !e.starts_with("<Leader>")
    })
}

pub(crate) fn has_command_entry(spec: &FeatureSpec) -> bool {
    spec.entry_points.iter().any(|e| e.starts_with(':'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_features_defined() {
        let features = define_core_features();
        assert!(features.len() >= 15, "expected 15+ features, got {}", features.len());
    }

    #[test]
    fn reachability_all_covered() {
        let features = define_core_features();
        let report = check_reachability(&features);
        assert_eq!(report.unreachable, 0);
        assert_eq!(report.reachable, features.len());
    }

    #[test]
    fn format_report() {
        let features = define_core_features();
        let report = check_reachability(&features);
        let text = format_reachability(&report);
        assert!(text.contains("Reachability Report"));
        assert!(text.contains("OK"));
    }

    #[test]
    fn has_keybinding() {
        let spec = feature("undo", &["u", ":undo"]);
        assert!(has_keybinding_entry(&spec));
    }

    #[test]
    fn has_command() {
        let spec = feature("quit", &[":q", ":quit", "ZQ"]);
        assert!(has_command_entry(&spec));
    }

    #[test]
    fn mixed_entries() {
        let spec = feature("search", &["/", ":s"]);
        assert!(has_keybinding_entry(&spec));
        assert!(has_command_entry(&spec));
    }

    #[test]
    fn empty_features() {
        let report = check_reachability(&[]);
        assert_eq!(report.reachable, 0);
        assert_eq!(report.unreachable, 0);
        let text = format_reachability(&report);
        assert!(text.contains("Total: 0"));
    }
}
