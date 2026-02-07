//! Crate dependency topology validation.

use serde::{Deserialize, Serialize};

/// Role of a crate in the architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CrateRole {
    Core,
    Service,
    UI,
    IO,
    Host,
}

/// A dependency edge between crates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrateDep {
    pub from: String,
    pub to: String,
    pub role: CrateRole,
}

/// Check if a dependency direction is allowed.
/// Rules: no Service -> Host, no Core -> Host.
pub fn check_dep_direction(from_role: CrateRole, to_role: CrateRole) -> bool {
    match (from_role, to_role) {
        (CrateRole::Service, CrateRole::Host) => false,
        (CrateRole::Core, CrateRole::Host) => false,
        _ => true,
    }
}

/// Expected dependency topology for all crates.
pub fn expected_topology() -> Vec<CrateDep> {
    vec![
        dep("kjxlkj-core-edit", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-core-mode", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-core-state", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-core-text", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-core-ui", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-core-undo", "kjxlkj-core-types", CrateRole::Core),
        dep("kjxlkj-input", "kjxlkj-core-types", CrateRole::IO),
        dep("kjxlkj-render", "kjxlkj-core-types", CrateRole::UI),
        dep("kjxlkj-services", "kjxlkj-core-types", CrateRole::Service),
        dep("kjxlkj-host", "kjxlkj-core-types", CrateRole::Host),
        dep("kjxlkj-host", "kjxlkj-services", CrateRole::Host),
    ]
}

fn dep(from: &str, to: &str, role: CrateRole) -> CrateDep {
    CrateDep { from: from.to_string(), to: to.to_string(), role }
}

/// Validate a set of dependencies against the direction rules.
/// Returns a list of violation descriptions.
pub fn validate_topology(deps: &[CrateDep]) -> Vec<String> {
    let mut violations = Vec::new();
    for d in deps {
        let from_role = d.role;
        // Infer target role from name (heuristic).
        let to_role = if d.to.contains("host") {
            CrateRole::Host
        } else if d.to.contains("service") {
            CrateRole::Service
        } else if d.to.contains("core") {
            CrateRole::Core
        } else if d.to.contains("render") {
            CrateRole::UI
        } else if d.to.contains("input") {
            CrateRole::IO
        } else {
            CrateRole::Core
        };
        if !check_dep_direction(from_role, to_role) {
            violations.push(format!(
                "invalid dep: {} ({:?}) -> {} ({:?})",
                d.from, from_role, d.to, to_role
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_directions() {
        assert!(check_dep_direction(CrateRole::Service, CrateRole::Core));
        assert!(check_dep_direction(CrateRole::Host, CrateRole::Service));
        assert!(check_dep_direction(CrateRole::Host, CrateRole::Core));
    }

    #[test]
    fn invalid_directions() {
        assert!(!check_dep_direction(CrateRole::Service, CrateRole::Host));
        assert!(!check_dep_direction(CrateRole::Core, CrateRole::Host));
    }

    #[test]
    fn expected_topology_is_valid() {
        let topo = expected_topology();
        let violations = validate_topology(&topo);
        assert!(violations.is_empty(), "violations: {:?}", violations);
    }
}
