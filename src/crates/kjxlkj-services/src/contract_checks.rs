//! API boundary contracts — assertions and invariants at core↔service↔render boundaries.

/// Contract violation severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity { Warning, Error, Fatal }

/// A contract check result.
#[derive(Debug, Clone)]
pub struct ContractResult {
    pub name: String,
    pub passed: bool,
    pub severity: Severity,
    pub message: Option<String>,
}

impl ContractResult {
    pub fn pass(name: &str) -> Self {
        Self { name: name.into(), passed: true, severity: Severity::Warning, message: None }
    }
    pub fn fail(name: &str, sev: Severity, msg: &str) -> Self {
        Self { name: name.into(), passed: false, severity: sev, message: Some(msg.into()) }
    }
}

/// A collection of contract checks.
#[derive(Debug, Clone, Default)]
pub struct ContractChecker { results: Vec<ContractResult> }

impl ContractChecker {
    pub fn new() -> Self { Self::default() }
    pub fn add(&mut self, r: ContractResult) { self.results.push(r); }
    pub fn all_passed(&self) -> bool { self.results.iter().all(|r| r.passed) }
    pub fn failures(&self) -> Vec<&ContractResult> { self.results.iter().filter(|r| !r.passed).collect() }
    pub fn summary(&self) -> String {
        let p = self.results.iter().filter(|r| r.passed).count();
        let f = self.results.len() - p;
        format!("{} passed, {} failed", p, f)
    }

    /// Check: snapshot generation is viewport-bounded (does not clone entire buffer).
    pub fn check_viewport_bounded(&mut self, buf_lines: usize, snapshot_lines: usize, viewport_h: usize) {
        let max = viewport_h + 10; // allow small overshoot for context
        if snapshot_lines > max && buf_lines > max {
            self.add(ContractResult::fail("viewport_bounded", Severity::Error,
                &format!("snapshot has {} lines but viewport is {}", snapshot_lines, viewport_h)));
        } else {
            self.add(ContractResult::pass("viewport_bounded"));
        }
    }

    /// Check: input ordering is preserved (sequence numbers must be monotonic).
    pub fn check_input_ordering(&mut self, sequence_nums: &[u64]) {
        let monotonic = sequence_nums.windows(2).all(|w| w[0] < w[1]);
        if monotonic || sequence_nums.len() <= 1 {
            self.add(ContractResult::pass("input_ordering"));
        } else {
            self.add(ContractResult::fail("input_ordering", Severity::Fatal,
                "input sequence numbers are not monotonically increasing"));
        }
    }

    /// Check: message bus utilization is within bounds.
    pub fn check_bus_utilization(&mut self, current: usize, capacity: usize) {
        let util = if capacity > 0 { current * 100 / capacity } else { 0 };
        if util > 90 {
            self.add(ContractResult::fail("bus_utilization", Severity::Warning,
                &format!("bus at {}% capacity ({}/{})", util, current, capacity)));
        } else {
            self.add(ContractResult::pass("bus_utilization"));
        }
    }

    /// Check: all buffers have valid IDs (non-zero).
    pub fn check_buffer_ids(&mut self, ids: &[u64]) {
        if ids.iter().any(|id| *id == 0) {
            self.add(ContractResult::fail("buffer_ids", Severity::Error, "buffer ID 0 is reserved"));
        } else {
            self.add(ContractResult::pass("buffer_ids"));
        }
    }

    /// Check: no plugin dynamic loading exists.
    pub fn check_no_plugin_loading(&mut self, has_dlopen: bool) {
        if has_dlopen {
            self.add(ContractResult::fail("no_plugins", Severity::Fatal,
                "dynamic plugin loading detected; all extensions must be built-in"));
        } else {
            self.add(ContractResult::pass("no_plugins"));
        }
    }

    /// Check: service restart count within policy limits.
    pub fn check_restart_limit(&mut self, restarts: u32, max: u32) {
        if restarts > max {
            self.add(ContractResult::fail("restart_limit", Severity::Error,
                &format!("service restarted {} times, limit is {}", restarts, max)));
        } else {
            self.add(ContractResult::pass("restart_limit"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_pass() {
        let mut c = ContractChecker::new();
        c.check_viewport_bounded(1000, 24, 24);
        c.check_input_ordering(&[1, 2, 3, 4]);
        c.check_bus_utilization(10, 100);
        c.check_buffer_ids(&[1, 2, 3]);
        c.check_no_plugin_loading(false);
        c.check_restart_limit(2, 5);
        assert!(c.all_passed());
        assert_eq!(c.summary(), "6 passed, 0 failed");
    }

    #[test]
    fn viewport_violation() {
        let mut c = ContractChecker::new();
        c.check_viewport_bounded(10000, 500, 24);
        assert!(!c.all_passed());
    }

    #[test]
    fn input_ordering_violation() {
        let mut c = ContractChecker::new();
        c.check_input_ordering(&[1, 3, 2, 4]);
        assert!(!c.all_passed());
        assert_eq!(c.failures()[0].severity, Severity::Fatal);
    }

    #[test]
    fn bus_high_util() {
        let mut c = ContractChecker::new();
        c.check_bus_utilization(95, 100);
        assert!(!c.all_passed());
        assert_eq!(c.failures()[0].severity, Severity::Warning);
    }

    #[test]
    fn zero_buffer_id() {
        let mut c = ContractChecker::new();
        c.check_buffer_ids(&[0, 1, 2]);
        assert!(!c.all_passed());
    }

    #[test]
    fn plugin_loading_detected() {
        let mut c = ContractChecker::new();
        c.check_no_plugin_loading(true);
        assert!(!c.all_passed());
        assert_eq!(c.failures()[0].severity, Severity::Fatal);
    }

    #[test]
    fn restart_limit() {
        let mut c = ContractChecker::new();
        c.check_restart_limit(10, 5);
        assert!(!c.all_passed());
        c.check_restart_limit(3, 5);
        // Second check passes; 1 fail + 1 pass
        assert_eq!(c.failures().len(), 1);
    }

    #[test]
    fn empty_checks_pass() {
        let c = ContractChecker::new();
        assert!(c.all_passed());
        assert_eq!(c.summary(), "0 passed, 0 failed");
    }
}
