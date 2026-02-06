//! Plugin prevention — compile-time and test-time checks.
//!
//! Ensures the codebase does not introduce plugin/extension loading.
//! All features are internal modules or services, never dynamically loaded.

/// List of forbidden patterns that indicate plugin loading.
const FORBIDDEN_PATTERNS: &[&str] = &[
    "dlopen",
    "libloading",
    "dynamic_lib",
    "plugin_load",
    "load_plugin",
    "register_plugin",
    "PluginManager",
    "ExtensionHost",
];

/// Result of a plugin prevention audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditResult {
    pub passed: bool,
    pub violations: Vec<AuditViolation>,
}

/// A single audit violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditViolation {
    pub file: String,
    pub line: usize,
    pub pattern: String,
    pub context: String,
}

/// Check a source file's content for forbidden plugin-loading patterns.
pub fn audit_source(file_path: &str, content: &str) -> Vec<AuditViolation> {
    let mut violations = Vec::new();
    for (i, line) in content.lines().enumerate() {
        for pattern in FORBIDDEN_PATTERNS {
            if line.contains(pattern) {
                violations.push(AuditViolation {
                    file: file_path.to_string(),
                    line: i + 1,
                    pattern: pattern.to_string(),
                    context: line.trim().to_string(),
                });
            }
        }
    }
    violations
}

/// Run audit across multiple files.
pub fn audit_files(files: &[(&str, &str)]) -> AuditResult {
    let mut all_violations = Vec::new();
    for (path, content) in files {
        all_violations.extend(audit_source(path, content));
    }
    AuditResult {
        passed: all_violations.is_empty(),
        violations: all_violations,
    }
}

/// Verify that a dependency list does not include known plugin crates.
pub fn check_dependencies(deps: &[&str]) -> Vec<String> {
    let forbidden_crates = ["libloading", "dlopen", "abi_stable", "plugin-api"];
    deps.iter()
        .filter(|d| forbidden_crates.iter().any(|f| d.contains(f)))
        .map(|d| d.to_string())
        .collect()
}

/// Verify that the architecture description matches the no-plugin rule.
pub fn verify_architecture_rule(description: &str) -> bool {
    let required = ["internal", "module", "service"];
    let forbidden = ["plugin", "extension", "dynamic"];
    required.iter().any(|r| description.to_lowercase().contains(r))
        && !forbidden.iter().any(|f| description.to_lowercase().contains(f))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_source_passes() {
        let v = audit_source("foo.rs", "fn main() { println!(\"hello\"); }");
        assert!(v.is_empty());
    }

    #[test]
    fn detect_dlopen() {
        let code = "use libc::dlopen;\nlet handle = dlopen(path);";
        let v = audit_source("bad.rs", code);
        assert!(v.len() >= 2);
        assert!(v.iter().all(|x| x.pattern == "dlopen"));
    }

    #[test]
    fn detect_plugin_manager() {
        let v = audit_source("x.rs", "struct PluginManager {}");
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].pattern, "PluginManager");
    }

    #[test]
    fn audit_multiple_files() {
        let files = vec![
            ("a.rs", "fn safe() {}"),
            ("b.rs", "fn also_safe() {}"),
        ];
        let result = audit_files(&files);
        assert!(result.passed);
    }

    #[test]
    fn audit_catches_violation() {
        let files = vec![("c.rs", "use libloading::Library;")];
        let result = audit_files(&files);
        assert!(!result.passed);
        assert_eq!(result.violations.len(), 1);
    }

    #[test]
    fn clean_dependencies() {
        let deps = vec!["tokio", "serde", "ropey"];
        assert!(check_dependencies(&deps).is_empty());
    }

    #[test]
    fn forbidden_dependency() {
        let deps = vec!["tokio", "libloading", "serde"];
        let bad = check_dependencies(&deps);
        assert_eq!(bad, vec!["libloading"]);
    }

    #[test]
    fn architecture_rule_pass() {
        assert!(verify_architecture_rule("All features are internal modules and services."));
    }

    #[test]
    fn architecture_rule_fail() {
        assert!(!verify_architecture_rule("Extensions are loaded via a plugin API."));
    }
}
