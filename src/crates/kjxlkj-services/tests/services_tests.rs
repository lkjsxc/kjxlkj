use kjxlkj_services::{
    Dispatcher, HealthStatus, NotifySource, RestartDecision, ServiceState, Severity,
    SupervisorConfig,
};
use kjxlkj_services::crate_topology::{check_dep_direction, expected_topology, validate_topology};
use kjxlkj_services::supervisor::{compute_backoff, decide_restart};
use kjxlkj_services::crate_topology::CrateRole;

// --- Supervisor config ---

#[test]
fn supervisor_config_defaults() {
    let c = SupervisorConfig::default();
    assert_eq!(c.max_restarts, 5);
    assert_eq!(c.backoff_base_ms, 100);
    assert_eq!(c.backoff_max_ms, 30_000);
}

#[test]
fn health_status_eq() {
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
    assert_ne!(HealthStatus::Healthy, HealthStatus::Failed);
}

#[test]
fn restart_first_attempt() {
    let state = ServiceState {
        name: "lsp".into(),
        status: HealthStatus::Failed,
        restart_count: 0,
        last_error: None,
    };
    assert_eq!(decide_restart(&state, &SupervisorConfig::default()), RestartDecision::Restart);
}

#[test]
fn restart_give_up_at_max() {
    let state = ServiceState {
        name: "git".into(),
        status: HealthStatus::Failed,
        restart_count: 5,
        last_error: Some("crash".into()),
    };
    assert_eq!(decide_restart(&state, &SupervisorConfig::default()), RestartDecision::GiveUp);
}

#[test]
fn restart_backoff_second_attempt() {
    let state = ServiceState {
        name: "idx".into(),
        status: HealthStatus::Failed,
        restart_count: 1,
        last_error: None,
    };
    let d = decide_restart(&state, &SupervisorConfig::default());
    assert!(matches!(d, RestartDecision::Backoff(200)));
}

#[test]
fn backoff_exponential_growth() {
    assert_eq!(compute_backoff(0, 100, 30_000), 100);
    assert_eq!(compute_backoff(1, 100, 30_000), 200);
    assert_eq!(compute_backoff(2, 100, 30_000), 400);
    assert_eq!(compute_backoff(3, 100, 30_000), 800);
}

#[test]
fn backoff_capped_at_max() {
    assert_eq!(compute_backoff(20, 100, 5000), 5000);
}

#[test]
fn service_state_last_error() {
    let s = ServiceState {
        name: "term".into(),
        status: HealthStatus::Degraded,
        restart_count: 2,
        last_error: Some("timeout".into()),
    };
    assert_eq!(s.last_error.as_deref(), Some("timeout"));
}

// --- Crate topology ---

#[test]
fn topology_core_to_core_allowed() {
    assert!(check_dep_direction(CrateRole::Core, CrateRole::Core));
}

#[test]
fn topology_service_to_host_forbidden() {
    assert!(!check_dep_direction(CrateRole::Service, CrateRole::Host));
}

#[test]
fn topology_core_to_host_forbidden() {
    assert!(!check_dep_direction(CrateRole::Core, CrateRole::Host));
}

#[test]
fn expected_topology_valid() {
    let violations = validate_topology(&expected_topology());
    assert!(violations.is_empty(), "{:?}", violations);
}

// --- Notification dispatch ---

#[test]
fn dispatcher_add_and_count() {
    let mut d = Dispatcher::new(5);
    d.add(Severity::Info, NotifySource::Editor, "hello", 100, None);
    d.add(Severity::Warning, NotifySource::Lsp, "warn", 200, None);
    assert_eq!(d.notifications.len(), 2);
}

#[test]
fn dispatcher_dismiss_by_id() {
    let mut d = Dispatcher::new(5);
    let id = d.add(Severity::Error, NotifySource::System, "err", 300, None);
    d.dismiss(id);
    assert!(d.notifications.is_empty());
}

#[test]
fn dispatcher_gc_expired() {
    let mut d = Dispatcher::new(5);
    d.add(Severity::Info, NotifySource::Git, "tmp", 1000, Some(500));
    d.gc(1600);
    assert!(d.notifications.is_empty());
}

#[test]
fn dispatcher_gc_keeps_unexpired() {
    let mut d = Dispatcher::new(5);
    d.add(Severity::Info, NotifySource::Plugin, "tmp", 1000, Some(500));
    d.gc(1400);
    assert_eq!(d.notifications.len(), 1);
}

#[test]
fn severity_ordering() {
    assert!(Severity::Debug < Severity::Info);
    assert!(Severity::Info < Severity::Warning);
    assert!(Severity::Warning < Severity::Error);
}
