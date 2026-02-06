//! Supervised service lifecycle management.
//!
//! Provides restart policies, health checks, and lifecycle tracking
//! for long-running services (FS, LSP, terminal, git).

/// Health status of a supervised service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Failed,
    Stopped,
}

/// Service restart decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartDecision {
    Restart,
    GiveUp,
    Backoff(u64),
}

/// Configuration for service supervision.
#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    pub max_restarts: u32,
    pub backoff_base_ms: u64,
    pub backoff_max_ms: u64,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self { max_restarts: 5, backoff_base_ms: 100, backoff_max_ms: 30_000 }
    }
}

/// Tracks the lifecycle of a single supervised service.
#[derive(Debug, Clone)]
pub struct ServiceState {
    pub name: String,
    pub status: HealthStatus,
    pub restart_count: u32,
    pub last_error: Option<String>,
    pub total_uptime_ms: u64,
    config: SupervisorConfig,
}

impl ServiceState {
    pub fn new(name: &str, config: SupervisorConfig) -> Self {
        Self {
            name: name.to_string(),
            status: HealthStatus::Stopped,
            restart_count: 0,
            last_error: None,
            total_uptime_ms: 0,
            config,
        }
    }

    /// Start the service.
    pub fn start(&mut self) {
        self.status = HealthStatus::Healthy;
    }

    /// Record a failure and decide whether to restart.
    pub fn record_failure(&mut self, error: &str) -> RestartDecision {
        self.status = HealthStatus::Failed;
        self.last_error = Some(error.to_string());
        self.restart_count += 1;
        if self.restart_count > self.config.max_restarts {
            return RestartDecision::GiveUp;
        }
        let backoff = compute_backoff(
            self.restart_count,
            self.config.backoff_base_ms,
            self.config.backoff_max_ms,
        );
        RestartDecision::Backoff(backoff)
    }

    /// Restart the service (after backoff).
    pub fn restart(&mut self) {
        self.status = HealthStatus::Healthy;
    }

    /// Record degraded health.
    pub fn degrade(&mut self, reason: &str) {
        self.status = HealthStatus::Degraded;
        self.last_error = Some(reason.to_string());
    }

    /// Stop the service gracefully.
    pub fn stop(&mut self) {
        self.status = HealthStatus::Stopped;
    }

    /// Check if the service can be restarted.
    pub fn can_restart(&self) -> bool {
        self.restart_count < self.config.max_restarts
    }

    /// Accumulate uptime.
    pub fn add_uptime(&mut self, ms: u64) {
        self.total_uptime_ms += ms;
    }
}

/// Compute exponential backoff with a cap.
pub fn compute_backoff(attempt: u32, base_ms: u64, max_ms: u64) -> u64 {
    let delay = base_ms.saturating_mul(1u64 << attempt.min(20));
    delay.min(max_ms)
}

/// Format a summary of all supervised services.
pub fn format_service_summary(services: &[ServiceState]) -> String {
    let mut out = String::from("Service Summary:\n");
    for s in services {
        out.push_str(&format!("  {} — {:?} (restarts: {})\n", s.name, s.status, s.restart_count));
        if let Some(ref err) = s.last_error { out.push_str(&format!("    error: {}\n", err)); }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_service(name: &str) -> ServiceState {
        ServiceState::new(name, SupervisorConfig::default())
    }

    #[test]
    fn start_and_stop() {
        let mut s = default_service("fs");
        s.start(); assert_eq!(s.status, HealthStatus::Healthy);
        s.stop(); assert_eq!(s.status, HealthStatus::Stopped);
    }

    #[test]
    fn failure_triggers_backoff() {
        let mut s = default_service("lsp");
        s.start();
        let d = s.record_failure("connection lost");
        assert!(matches!(d, RestartDecision::Backoff(_))); assert_eq!(s.restart_count, 1);
    }

    #[test]
    fn exceed_max_restarts() {
        let mut s = ServiceState::new("terminal", SupervisorConfig { max_restarts: 2, ..Default::default() });
        s.start(); s.record_failure("e1"); s.restart(); s.record_failure("e2"); s.restart();
        assert_eq!(s.record_failure("e3"), RestartDecision::GiveUp);
    }

    #[test]
    fn degrade_status() {
        let mut s = default_service("git"); s.start(); s.degrade("slow");
        assert_eq!(s.status, HealthStatus::Degraded);
    }

    #[test]
    fn backoff_exponential() {
        assert_eq!(compute_backoff(1, 100, 30_000), 200);
        assert_eq!(compute_backoff(2, 100, 30_000), 400);
        assert_eq!(compute_backoff(3, 100, 30_000), 800);
        assert_eq!(compute_backoff(10, 100, 30_000), 30_000); // capped
    }

    #[test]
    fn uptime_tracking() {
        let mut s = default_service("fs"); s.start(); s.add_uptime(5000); s.add_uptime(3000);
        assert_eq!(s.total_uptime_ms, 8000);
    }

    #[test]
    fn service_summary() {
        let mut svcs = vec![default_service("fs"), default_service("lsp")];
        svcs[0].start(); svcs[1].start(); svcs[1].record_failure("timeout");
        let summary = format_service_summary(&svcs);
        assert!(summary.contains("fs") && summary.contains("timeout"));
    }
}
