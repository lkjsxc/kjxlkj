//! Contract checking utilities for runtime invariant verification.

use std::fmt;

/// The severity/kind of a contract violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractLevel {
    Precondition,
    Postcondition,
    Invariant,
}

impl fmt::Display for ContractLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Precondition => write!(f, "PRECONDITION"),
            Self::Postcondition => write!(f, "POSTCONDITION"),
            Self::Invariant => write!(f, "INVARIANT"),
        }
    }
}

/// A single contract violation record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub level: ContractLevel,
    pub module: String,
    pub message: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.level, self.module, self.message)
    }
}

/// Runtime contract checker that accumulates violations.
#[derive(Debug, Clone)]
pub struct ContractChecker {
    pub strict: bool,
    pub violations: Vec<Violation>,
}

impl ContractChecker {
    pub fn new(strict: bool) -> Self {
        Self {
            strict,
            violations: Vec::new(),
        }
    }

    fn record(&mut self, level: ContractLevel, module: &str, message: &str) {
        let v = Violation {
            level,
            module: module.to_string(),
            message: message.to_string(),
        };
        if self.strict {
            panic!("Contract violation: {v}");
        }
        tracing::warn!("{v}");
        self.violations.push(v);
    }

    /// Check a precondition; records a violation if `cond` is false.
    pub fn require(&mut self, cond: bool, module: &str, msg: &str) {
        if !cond {
            self.record(ContractLevel::Precondition, module, msg);
        }
    }

    /// Check a postcondition.
    pub fn ensure(&mut self, cond: bool, module: &str, msg: &str) {
        if !cond {
            self.record(ContractLevel::Postcondition, module, msg);
        }
    }

    /// Check an invariant.
    pub fn invariant(&mut self, cond: bool, module: &str, msg: &str) {
        if !cond {
            self.record(ContractLevel::Invariant, module, msg);
        }
    }

    /// Assert `val` is in `[0, limit)`.
    pub fn in_range(&mut self, val: usize, limit: usize, module: &str) {
        self.require(
            val < limit,
            module,
            &format!("{val} not in range [0, {limit})"),
        );
    }

    /// Assert a slice is non-empty.
    pub fn non_empty<T>(&mut self, slice: &[T], module: &str) {
        self.require(!slice.is_empty(), module, "expected non-empty slice");
    }

    /// Assert a buffer id is non-zero (valid).
    pub fn valid_buffer_id(&mut self, id: u64, module: &str) {
        self.require(id > 0, module, "buffer id must be > 0");
    }

    /// Assert a value does not exceed a limit.
    pub fn within_limit(&mut self, val: usize, limit: usize, module: &str) {
        self.require(
            val <= limit,
            module,
            &format!("{val} exceeds limit {limit}"),
        );
    }

    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    pub fn summary(&self) -> String {
        if self.violations.is_empty() {
            "No contract violations".to_string()
        } else {
            format!("{} violation(s) recorded", self.violations.len())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_violations_on_success() {
        let mut c = ContractChecker::new(false);
        c.require(true, "test", "ok");
        assert!(!c.has_violations());
    }

    #[test]
    fn records_violation() {
        let mut c = ContractChecker::new(false);
        c.require(false, "test", "fail");
        assert!(c.has_violations());
        assert_eq!(c.violations[0].level, ContractLevel::Precondition);
    }

    #[test]
    fn in_range_check() {
        let mut c = ContractChecker::new(false);
        c.in_range(5, 3, "test");
        assert!(c.has_violations());
    }
}
