//! Plugin prevention audit: ensure no dynamic-loading code leaks in.

/// Forbidden patterns that indicate plugin/dynamic-loading usage.
const FORBIDDEN_PATTERNS: &[&str] = &[
    "dlopen",
    "libloading",
    "PluginManager",
    "dynamic_lib",
    "load_plugin",
    "plugin_registry",
    "dlsym",
    "LoadLibrary",
    "PluginHost",
];

/// Forbidden dependency crate names.
const FORBIDDEN_CRATES: &[&str] = &[
    "libloading",
    "dlopen",
    "dlopen2",
    "sharedlib",
    "abi_stable",
    "plugins",
];

/// Audit a single source string for forbidden patterns.
///
/// Returns a list of violations found.
pub fn audit_source(source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    for pat in FORBIDDEN_PATTERNS {
        if source.contains(pat) {
            violations.push(format!("forbidden pattern found: `{pat}`"));
        }
    }
    violations
}

/// Audit multiple files. Returns (filename, violations) for files with hits.
pub fn audit_files(files: &[(String, String)]) -> Vec<(String, Vec<String>)> {
    files
        .iter()
        .filter_map(|(name, content)| {
            let v = audit_source(content);
            if v.is_empty() { None } else { Some((name.clone(), v)) }
        })
        .collect()
}

/// Check dependency names against the forbidden crate list.
pub fn check_dependencies(deps: &[String]) -> Vec<String> {
    deps.iter()
        .filter(|d| FORBIDDEN_CRATES.contains(&d.as_str()))
        .map(|d| format!("forbidden dependency: `{d}`"))
        .collect()
}

/// Verify a high-level architecture rule by description.
///
/// Currently checks that the description does not mention plugins
/// or dynamic loading in an affirmative context. Returns `true` if
/// the rule is consistent with a plugin-free architecture.
pub fn verify_architecture_rule(description: &str) -> bool {
    let lower = description.to_lowercase();
    let plugin_mentions = ["allow plugin", "enable plugin", "support plugin"];
    !plugin_mentions.iter().any(|p| lower.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_source() {
        assert!(audit_source("fn main() {}").is_empty());
    }

    #[test]
    fn detects_dlopen() {
        let v = audit_source("let lib = dlopen(\"foo.so\");");
        assert_eq!(v.len(), 1);
        assert!(v[0].contains("dlopen"));
    }

    #[test]
    fn detects_libloading() {
        let v = audit_source("use libloading::Library;");
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn detects_plugin_manager() {
        let v = audit_source("struct PluginManager;");
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn audit_files_mixed() {
        let files = vec![
            ("clean.rs".into(), "fn foo() {}".into()),
            ("bad.rs".into(), "use libloading;".into()),
        ];
        let result = audit_files(&files);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "bad.rs");
    }

    #[test]
    fn check_deps_clean() {
        let deps = vec!["serde".into(), "tokio".into()];
        assert!(check_dependencies(&deps).is_empty());
    }

    #[test]
    fn check_deps_forbidden() {
        let deps = vec!["serde".into(), "libloading".into()];
        let v = check_dependencies(&deps);
        assert_eq!(v.len(), 1);
    }

    #[test]
    fn architecture_ok() {
        assert!(verify_architecture_rule("No dynamic loading"));
    }

    #[test]
    fn architecture_violated() {
        assert!(!verify_architecture_rule("We allow plugin loading"));
    }

    #[test]
    fn multiple_violations() {
        let src = "dlopen then libloading then PluginManager";
        let v = audit_source(src);
        assert_eq!(v.len(), 3);
    }
}
