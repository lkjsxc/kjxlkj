//! Runtime ordering, lifecycle rules, and failure-recovery semantics.

use crate::{ServiceId, ServiceStatus};

/// Runtime phase — tracks editor runtime lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimePhase {
    Init, CoreStarting, ServicesStarting, Running, ShuttingDown, Terminated,
}

/// Ordering rules: which phase transitions are valid.
pub fn is_valid_phase_transition(from: RuntimePhase, to: RuntimePhase) -> bool {
    use RuntimePhase::*;
    matches!(
        (from, to),
        (Init, CoreStarting) | (CoreStarting, ServicesStarting) |
        (ServicesStarting, Running) | (Running, ShuttingDown) |
        (ShuttingDown, Terminated) |
        // Emergency shutdown from any running state
        (CoreStarting, Terminated) | (ServicesStarting, ShuttingDown) |
        (Running, Terminated)
    )
}

/// Failure recovery policy for a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartPolicy { Never, Limited(u32), Always }

/// Service lifecycle record: tracks restarts and failure history.
#[derive(Debug, Clone)]
pub struct ServiceLifecycle {
    pub id: ServiceId,
    pub policy: RestartPolicy,
    pub restart_count: u32,
    pub status: ServiceStatus,
    pub last_error: Option<String>,
}

impl ServiceLifecycle {
    pub fn new(id: ServiceId, policy: RestartPolicy) -> Self {
        Self { id, policy, restart_count: 0, status: ServiceStatus::Stopped, last_error: None }
    }

    /// Whether the service should be restarted after failure.
    pub fn should_restart(&self) -> bool {
        match self.policy {
            RestartPolicy::Never => false,
            RestartPolicy::Limited(max) => self.restart_count < max,
            RestartPolicy::Always => true,
        }
    }

    /// Record a failure and attempt restart if policy allows.
    pub fn record_failure(&mut self, error: String) -> bool {
        self.status = ServiceStatus::Failed;
        self.last_error = Some(error);
        if self.should_restart() {
            self.restart_count += 1;
            self.status = ServiceStatus::Starting;
            true
        } else { false }
    }

    /// Reset lifecycle on successful start.
    pub fn mark_running(&mut self) {
        self.status = ServiceStatus::Running;
    }

    /// Backoff delay in milliseconds (exponential with cap).
    pub fn backoff_ms(&self) -> u64 {
        let base = 100u64;
        let max = 30_000u64;
        base.saturating_mul(2u64.saturating_pow(self.restart_count)).min(max)
    }
}

/// Message bus capacity check — ordered delivery guarantee.
#[derive(Debug, Clone)]
pub struct BusCapacity {
    pub capacity: usize,
    pub pending: usize,
}

impl BusCapacity {
    pub fn new(capacity: usize) -> Self { Self { capacity, pending: 0 } }
    pub fn has_room(&self) -> bool { self.pending < self.capacity }
    pub fn send(&mut self) -> bool {
        if self.has_room() { self.pending += 1; true } else { false }
    }
    pub fn receive(&mut self) { self.pending = self.pending.saturating_sub(1); }
    pub fn utilization(&self) -> f64 {
        if self.capacity == 0 { 1.0 } else { self.pending as f64 / self.capacity as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_phase_transitions() {
        use RuntimePhase::*;
        assert!(is_valid_phase_transition(Init, CoreStarting));
        assert!(is_valid_phase_transition(CoreStarting, ServicesStarting));
        assert!(is_valid_phase_transition(ServicesStarting, Running));
        assert!(is_valid_phase_transition(Running, ShuttingDown));
        assert!(is_valid_phase_transition(ShuttingDown, Terminated));
    }

    #[test]
    fn invalid_phase_transitions() {
        use RuntimePhase::*;
        assert!(!is_valid_phase_transition(Init, Running));
        assert!(!is_valid_phase_transition(Terminated, Init));
        assert!(!is_valid_phase_transition(Running, CoreStarting));
        assert!(!is_valid_phase_transition(ShuttingDown, Running));
    }

    #[test]
    fn emergency_shutdown() {
        use RuntimePhase::*;
        assert!(is_valid_phase_transition(Running, Terminated));
        assert!(is_valid_phase_transition(ServicesStarting, ShuttingDown));
    }

    #[test]
    fn restart_policy_never() {
        let mut lc = ServiceLifecycle::new(ServiceId("test".into()), RestartPolicy::Never);
        assert!(!lc.should_restart());
        assert!(!lc.record_failure("boom".into()));
        assert_eq!(lc.status, ServiceStatus::Failed);
    }

    #[test]
    fn restart_policy_limited() {
        let mut lc = ServiceLifecycle::new(ServiceId("svc".into()), RestartPolicy::Limited(2));
        assert!(lc.record_failure("err1".into()));
        assert_eq!(lc.restart_count, 1);
        assert!(lc.record_failure("err2".into()));
        assert_eq!(lc.restart_count, 2);
        assert!(!lc.record_failure("err3".into()));
        assert_eq!(lc.status, ServiceStatus::Failed);
    }

    #[test]
    fn restart_policy_always() {
        let mut lc = ServiceLifecycle::new(ServiceId("x".into()), RestartPolicy::Always);
        for i in 0..10 { assert!(lc.record_failure(format!("err{i}"))); }
        assert_eq!(lc.restart_count, 10);
    }

    #[test]
    fn backoff_exponential_with_cap() {
        let mut lc = ServiceLifecycle::new(ServiceId("b".into()), RestartPolicy::Always);
        assert_eq!(lc.backoff_ms(), 100);
        lc.restart_count = 1; assert_eq!(lc.backoff_ms(), 200);
        lc.restart_count = 5; assert_eq!(lc.backoff_ms(), 3200);
        lc.restart_count = 20; assert_eq!(lc.backoff_ms(), 30_000); // capped
    }

    #[test]
    fn bus_capacity_send_receive() {
        let mut bus = BusCapacity::new(3);
        assert!(bus.has_room());
        assert!(bus.send());
        assert!(bus.send());
        assert!(bus.send());
        assert!(!bus.send()); // full
        bus.receive();
        assert!(bus.send()); // room again
    }

    #[test]
    fn bus_utilization() {
        let mut bus = BusCapacity::new(100);
        assert!((bus.utilization() - 0.0).abs() < f64::EPSILON);
        for _ in 0..50 { bus.send(); }
        assert!((bus.utilization() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn mark_running_resets_status() {
        let mut lc = ServiceLifecycle::new(ServiceId("s".into()), RestartPolicy::Limited(3));
        lc.record_failure("err".into());
        lc.mark_running();
        assert_eq!(lc.status, ServiceStatus::Running);
    }
}
