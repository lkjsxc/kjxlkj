//! Service supervisor: health tracking, restart decisions, backoff.

use serde::{Deserialize, Serialize};

/// Health status of a service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Failed,
    Stopped,
}

/// Restart decision from the supervisor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestartDecision {
    Restart,
    GiveUp,
    Backoff(u64),
}

/// Supervisor configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupervisorConfig {
    pub max_restarts: u32,
    pub backoff_base_ms: u64,
    pub backoff_max_ms: u64,
}

impl Default for SupervisorConfig {
    fn default() -> Self {
        Self {
            max_restarts: 5,
            backoff_base_ms: 100,
            backoff_max_ms: 30_000,
        }
    }
}

/// State of a managed service.
#[derive(Debug, Clone)]
pub struct ServiceState {
    pub name: String,
    pub status: HealthStatus,
    pub restart_count: u32,
    pub last_error: Option<String>,
}

/// Compute exponential backoff delay.
pub fn compute_backoff(attempt: u32, base: u64, max: u64) -> u64 {
    let delay = base.saturating_mul(1u64 << attempt.min(20));
    delay.min(max)
}

/// Decide whether to restart a failed service.
pub fn decide_restart(state: &ServiceState, config: &SupervisorConfig) -> RestartDecision {
    if state.restart_count >= config.max_restarts {
        return RestartDecision::GiveUp;
    }
    if state.restart_count == 0 {
        return RestartDecision::Restart;
    }
    let delay = compute_backoff(state.restart_count, config.backoff_base_ms, config.backoff_max_ms);
    RestartDecision::Backoff(delay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_exponential() {
        assert_eq!(compute_backoff(0, 100, 30_000), 100);
        assert_eq!(compute_backoff(1, 100, 30_000), 200);
        assert_eq!(compute_backoff(4, 100, 30_000), 1600);
    }

    #[test]
    fn backoff_capped() {
        assert_eq!(compute_backoff(20, 100, 5000), 5000);
    }

    #[test]
    fn decide_restart_first_attempt() {
        let state = ServiceState {
            name: "lsp".into(),
            status: HealthStatus::Failed,
            restart_count: 0,
            last_error: Some("crash".into()),
        };
        assert_eq!(decide_restart(&state, &SupervisorConfig::default()), RestartDecision::Restart);
    }

    #[test]
    fn decide_give_up() {
        let state = ServiceState {
            name: "lsp".into(),
            status: HealthStatus::Failed,
            restart_count: 5,
            last_error: None,
        };
        assert_eq!(decide_restart(&state, &SupervisorConfig::default()), RestartDecision::GiveUp);
    }

    #[test]
    fn decide_backoff() {
        let state = ServiceState {
            name: "lsp".into(),
            status: HealthStatus::Failed,
            restart_count: 2,
            last_error: None,
        };
        let decision = decide_restart(&state, &SupervisorConfig::default());
        assert!(matches!(decision, RestartDecision::Backoff(_)));
    }
}
