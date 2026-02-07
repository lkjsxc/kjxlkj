//! kjxlkj-services: Supervisor and cross-service types.

pub mod benchmark_suite;
pub mod crate_topology;
pub mod notification_dispatch;
pub mod profiling_workflow;
pub mod supervisor;

// Re-export all service crates.
pub use kjxlkj_service_fs;
pub use kjxlkj_service_git;
pub use kjxlkj_service_index;
pub use kjxlkj_service_lsp;
pub use kjxlkj_service_terminal;

pub use benchmark_suite::{BenchmarkConfig, BenchmarkKind, BenchmarkResult};
pub use crate_topology::{CrateDep, CrateRole};
pub use notification_dispatch::{Dispatcher, Notification, NotifySource, Severity};
pub use profiling_workflow::{ProfileConfig, ProfileResult, ProfileTarget};
pub use supervisor::{HealthStatus, RestartDecision, ServiceState, SupervisorConfig};
