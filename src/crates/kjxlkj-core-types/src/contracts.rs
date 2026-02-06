/// Technical contracts and runtime invariant checking.

/// Contract violation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractLevel { Precondition, Postcondition, Invariant }

/// A contract violation record.
#[derive(Debug, Clone)]
pub struct Violation {
    pub level: ContractLevel,
    pub module: &'static str,
    pub message: String,
}

impl Violation {
    pub fn new(level: ContractLevel, module: &'static str, msg: impl Into<String>) -> Self {
        Self { level, module, message: msg.into() }
    }
}

impl std::fmt::Display for Violation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = match self.level {
            ContractLevel::Precondition => "PRE",
            ContractLevel::Postcondition => "POST",
            ContractLevel::Invariant => "INV",
        };
        write!(f, "[{}:{}] {}", tag, self.module, self.message)
    }
}

/// Contract checker that collects violations instead of panicking.
#[derive(Debug, Default)]
pub struct ContractChecker { violations: Vec<Violation>, strict: bool }

impl ContractChecker {
    pub fn new(strict: bool) -> Self { Self { violations: Vec::new(), strict } }

    pub fn require(&mut self, cond: bool, module: &'static str, msg: impl Into<String>) {
        if !cond {
            let v = Violation::new(ContractLevel::Precondition, module, msg);
            if self.strict { panic!("{}", v); }
            self.violations.push(v);
        }
    }

    pub fn ensure(&mut self, cond: bool, module: &'static str, msg: impl Into<String>) {
        if !cond {
            let v = Violation::new(ContractLevel::Postcondition, module, msg);
            if self.strict { panic!("{}", v); }
            self.violations.push(v);
        }
    }

    pub fn invariant(&mut self, cond: bool, module: &'static str, msg: impl Into<String>) {
        if !cond {
            let v = Violation::new(ContractLevel::Invariant, module, msg);
            if self.strict { panic!("{}", v); }
            self.violations.push(v);
        }
    }

    pub fn violations(&self) -> &[Violation] { &self.violations }
    pub fn has_violations(&self) -> bool { !self.violations.is_empty() }
    pub fn clear(&mut self) { self.violations.clear(); }
    pub fn count(&self) -> usize { self.violations.len() }
}

/// Check a numeric range contract.
pub fn in_range(val: usize, lo: usize, hi: usize) -> bool { val >= lo && val <= hi }

/// Check that a string is non-empty (common precondition).
pub fn non_empty(s: &str) -> bool { !s.is_empty() }

/// Check buffer-id validity (non-zero).
pub fn valid_buffer_id(id: u64) -> bool { id > 0 }

/// Size contract: ensure value is within max.
pub fn within_limit(val: usize, max: usize) -> bool { val <= max }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precondition_violation() {
        let mut cc = ContractChecker::new(false);
        cc.require(false, "buffer", "id must not be zero");
        assert!(cc.has_violations());
        assert_eq!(cc.count(), 1);
        assert_eq!(cc.violations()[0].level, ContractLevel::Precondition);
    }

    #[test]
    fn postcondition_violation() {
        let mut cc = ContractChecker::new(false);
        cc.ensure(false, "render", "frame must not be empty");
        assert_eq!(cc.violations()[0].level, ContractLevel::Postcondition);
    }

    #[test]
    fn invariant_violation() {
        let mut cc = ContractChecker::new(false);
        cc.invariant(false, "state", "cursor within bounds");
        assert_eq!(cc.violations()[0].level, ContractLevel::Invariant);
    }

    #[test]
    fn no_violation_when_true() {
        let mut cc = ContractChecker::new(false);
        cc.require(true, "x", "ok");
        cc.ensure(true, "x", "ok");
        cc.invariant(true, "x", "ok");
        assert!(!cc.has_violations());
    }

    #[test]
    #[should_panic]
    fn strict_panics() {
        let mut cc = ContractChecker::new(true);
        cc.require(false, "test", "this should panic");
    }

    #[test]
    fn violation_display() {
        let v = Violation::new(ContractLevel::Precondition, "buf", "id > 0");
        assert_eq!(format!("{}", v), "[PRE:buf] id > 0");
    }

    #[test]
    fn helpers() {
        assert!(in_range(5, 0, 10));
        assert!(!in_range(11, 0, 10));
        assert!(non_empty("hi"));
        assert!(!non_empty(""));
        assert!(valid_buffer_id(1));
        assert!(!valid_buffer_id(0));
        assert!(within_limit(5, 10));
        assert!(!within_limit(11, 10));
    }

    #[test]
    fn clear_violations() {
        let mut cc = ContractChecker::new(false);
        cc.require(false, "a", "fail");
        cc.clear();
        assert_eq!(cc.count(), 0);
    }
}
