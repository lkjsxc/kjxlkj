/// Crate topology alignment — validates module boundaries match spec.

/// Crate role classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrateRole { Core, Service, UI, IO, Host }

/// Dependency direction rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepDirection { Allowed, Forbidden }

/// A crate boundary record.
#[derive(Debug, Clone)]
pub struct CrateBoundary { pub name: String, pub role: CrateRole, pub deps: Vec<String> }

impl CrateBoundary {
    pub fn new(name: impl Into<String>, role: CrateRole) -> Self {
        Self { name: name.into(), role, deps: Vec::new() }
    }

    pub fn with_dep(mut self, dep: impl Into<String>) -> Self { self.deps.push(dep.into()); self }
}

/// Validate dependency direction: services should not depend on host.
pub fn check_dep_direction(from: CrateRole, to: CrateRole) -> DepDirection {
    match (from, to) {
        (CrateRole::Service, CrateRole::Host) => DepDirection::Forbidden,
        (CrateRole::Core, CrateRole::Host) => DepDirection::Forbidden,
        (CrateRole::Core, CrateRole::IO) => DepDirection::Forbidden,
        (CrateRole::UI, CrateRole::Host) => DepDirection::Forbidden,
        _ => DepDirection::Allowed,
    }
}

/// Build expected topology for validation.
pub fn expected_topology() -> Vec<CrateBoundary> {
    vec![
        CrateBoundary::new("kjxlkj-core-types", CrateRole::Core),
        CrateBoundary::new("kjxlkj-core-text", CrateRole::Core).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-core-edit", CrateRole::Core).with_dep("kjxlkj-core-types").with_dep("kjxlkj-core-text"),
        CrateBoundary::new("kjxlkj-core-mode", CrateRole::Core).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-core-state", CrateRole::Core).with_dep("kjxlkj-core-types").with_dep("kjxlkj-core-text"),
        CrateBoundary::new("kjxlkj-core-undo", CrateRole::Core).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-core-ui", CrateRole::UI).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-input", CrateRole::IO).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-render", CrateRole::IO).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-services", CrateRole::Service).with_dep("kjxlkj-core-types"),
        CrateBoundary::new("kjxlkj-host", CrateRole::Host).with_dep("kjxlkj-core-types"),
    ]
}

/// Validate all deps in a topology.
pub fn validate_topology(topo: &[CrateBoundary]) -> Vec<String> {
    let mut issues = Vec::new();
    for c in topo {
        for dep in &c.deps {
            if let Some(target) = topo.iter().find(|t| t.name == *dep) {
                if check_dep_direction(c.role, target.role) == DepDirection::Forbidden {
                    issues.push(format!("{} -> {} is forbidden", c.name, dep));
                }
            }
        }
    }
    issues
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allowed_deps() { assert_eq!(check_dep_direction(CrateRole::Host, CrateRole::Core), DepDirection::Allowed); }

    #[test]
    fn forbidden_deps() { assert_eq!(check_dep_direction(CrateRole::Service, CrateRole::Host), DepDirection::Forbidden); }

    #[test]
    fn expected_topology_valid() {
        let topo = expected_topology();
        assert!(topo.len() >= 10);
        let issues = validate_topology(&topo);
        assert!(issues.is_empty(), "unexpected issues: {:?}", issues);
    }

    #[test]
    fn detect_forbidden() {
        let topo = vec![
            CrateBoundary::new("svc", CrateRole::Service).with_dep("host"),
            CrateBoundary::new("host", CrateRole::Host),
        ];
        let issues = validate_topology(&topo);
        assert_eq!(issues.len(), 1);
    }

    #[test]
    fn crate_roles() {
        for role in [CrateRole::Core, CrateRole::Service, CrateRole::UI, CrateRole::IO, CrateRole::Host] {
            let c = CrateBoundary::new("x", role);
            assert_eq!(c.role, role);
        }
    }

    #[test]
    fn boundary_builder() {
        let b = CrateBoundary::new("a", CrateRole::Core).with_dep("b").with_dep("c");
        assert_eq!(b.deps.len(), 2);
    }
}
