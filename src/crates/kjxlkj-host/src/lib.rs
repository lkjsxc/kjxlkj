//! kjxlkj-host: Terminal host integration and lifecycle management.
//!
//! This crate connects the core editor, input decoder, renderer, and
//! services into a runnable terminal application.

pub mod feature_integration;
pub mod feature_reachability;
pub mod file_flows;
pub mod host;
pub mod host_args;
pub mod plugin_audit;
pub mod pty_harness;
pub mod pty_regressions;
pub mod session_full;
pub mod terminal_setup;

pub use feature_integration::{
    multi_buffer_scenario, open_edit_save_scenario,
    undo_redo_scenario as integration_undo_redo_scenario,
    validate_scenario as validate_integration_scenario, IntegrationScenario, ScenarioStep,
};
pub use feature_reachability::{
    check_reachability, define_core_features, has_command_entry, has_keybinding_entry, EntryKind,
    FeatureSpec, ReachabilityReport,
};
pub use file_flows::{
    build_edit_flow, build_wq_flow, detect_encoding, detect_line_ending, resolve_path,
    validate_write_target, FileOp, FileResult, OpenOptions, WriteOptions,
};
pub use host::Host;
pub use host_args::{parse_args, HostArgs};
pub use plugin_audit::{audit_files, audit_source, check_dependencies, verify_architecture_rule};
pub use pty_harness::{
    estimate_duration, validate_scenario, PtyAction, PtyConfig, PtyExpectation, PtyScenario,
};
pub use pty_regressions::all_regression_scenarios;
pub use session_full::{
    parse_session_buffers, serialize_session, SessionBuffer, SessionData, SessionMark,
    SessionWindow,
};
pub use terminal_setup::{restore_terminal, setup_terminal, TerminalGuard};
