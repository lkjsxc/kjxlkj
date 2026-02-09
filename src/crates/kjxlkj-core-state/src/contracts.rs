//! Contract checking and invariant enforcement.
//!
//! Implements the required contracts from docs/spec/technical/contracts.md.
//! Contract violations are logged and optionally panic in debug builds.

/// Contract violation record.
#[derive(Debug, Clone)]
pub struct ContractViolation {
    /// Which contract was violated.
    pub contract: ContractKind,
    /// Human-readable description.
    pub message: String,
    /// Source location (file:line).
    pub location: String,
}

/// Contract categories from the spec.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractKind {
    /// Determinism: edit serialization, versioning, stale results.
    Determinism,
    /// Service: typed messages, supervision, timeouts, isolation.
    Service,
    /// Snapshot: read-only, completeness, atomicity.
    Snapshot,
    /// Buffer: UTF-8, line index, change notification.
    Buffer,
    /// Observability: queue depth, latency, errors, memory.
    Observability,
    /// Persistence: atomic writes, backup, encoding.
    Persistence,
}

/// Contract checker that tracks violations.
#[derive(Debug, Clone, Default)]
pub struct ContractChecker {
    /// Recorded violations.
    violations: Vec<ContractViolation>,
    /// Whether to panic on violation (debug mode).
    strict_mode: bool,
}

impl ContractChecker {
    pub fn new() -> Self {
        Self {
            violations: Vec::new(),
            strict_mode: cfg!(debug_assertions),
        }
    }

    /// Assert a contract condition. Records violation if false.
    pub fn assert(
        &mut self,
        condition: bool,
        contract: ContractKind,
        message: &str,
        location: &str,
    ) {
        if !condition {
            self.record_violation(contract, message, location);
        }
    }

    /// Record a contract violation.
    pub fn record_violation(&mut self, contract: ContractKind, message: &str, location: &str) {
        let violation = ContractViolation {
            contract,
            message: message.to_string(),
            location: location.to_string(),
        };

        self.violations.push(violation.clone());

        if self.strict_mode {
            panic!(
                "Contract violation [{:?}]: {} at {}",
                violation.contract, violation.message, violation.location
            );
        }
    }

    /// Get all violations.
    pub fn violations(&self) -> &[ContractViolation] {
        &self.violations
    }

    /// Check if any violations occurred.
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Clear all violations.
    pub fn clear(&mut self) {
        self.violations.clear();
    }

    /// Set strict mode (panic on violation).
    pub fn set_strict(&mut self, strict: bool) {
        self.strict_mode = strict;
    }

    /// Number of violations.
    pub fn count(&self) -> usize {
        self.violations.len()
    }
}

/// Assert that buffer content is valid UTF-8.
pub fn assert_buffer_utf8(content: &str) -> bool {
    // Rust strings are always valid UTF-8, but we verify rope content
    content.is_ascii() || std::str::from_utf8(content.as_bytes()).is_ok()
}

/// Assert monotonic buffer version.
pub fn assert_monotonic_version(old_version: u64, new_version: u64) -> bool {
    new_version > old_version
}

/// Assert channel capacity bounds.
pub fn assert_channel_bounded(capacity: usize, max_capacity: usize) -> bool {
    capacity <= max_capacity
}

/// Assert stale result detection — result version must match buffer version.
pub fn assert_not_stale(result_version: u64, buffer_version: u64) -> bool {
    result_version >= buffer_version
}

/// Macro for asserting contracts with location tracking.
#[macro_export]
macro_rules! contract_assert {
    ($checker:expr, $cond:expr, $kind:expr, $msg:expr) => {
        $checker.assert($cond, $kind, $msg, concat!(file!(), ":", line!()));
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_no_violation() {
        let mut checker = ContractChecker::new();
        checker.set_strict(false);
        checker.assert(true, ContractKind::Buffer, "ok", "test");
        assert!(!checker.has_violations());
    }

    #[test]
    fn test_contract_violation_recorded() {
        let mut checker = ContractChecker::new();
        checker.set_strict(false);
        checker.assert(false, ContractKind::Buffer, "bad", "test:1");
        assert!(checker.has_violations());
        assert_eq!(checker.count(), 1);
        assert_eq!(checker.violations()[0].message, "bad");
    }

    #[test]
    fn test_buffer_utf8() {
        assert!(assert_buffer_utf8("hello world"));
        assert!(assert_buffer_utf8("日本語"));
    }

    #[test]
    fn test_monotonic_version() {
        assert!(assert_monotonic_version(1, 2));
        assert!(!assert_monotonic_version(2, 2));
        assert!(!assert_monotonic_version(3, 2));
    }

    #[test]
    fn test_channel_bounded() {
        assert!(assert_channel_bounded(100, 256));
        assert!(!assert_channel_bounded(300, 256));
    }

    #[test]
    fn test_stale_result() {
        assert!(assert_not_stale(5, 5));
        assert!(assert_not_stale(6, 5));
        assert!(!assert_not_stale(4, 5));
    }

    #[test]
    fn test_clear_violations() {
        let mut checker = ContractChecker::new();
        checker.set_strict(false);
        checker.assert(false, ContractKind::Determinism, "fail", "loc");
        assert_eq!(checker.count(), 1);
        checker.clear();
        assert_eq!(checker.count(), 0);
    }
}
