// Automation run orchestration per /docs/spec/domain/automation.md
// Placeholder for run lifecycle management

/// Run lifecycle states matching RunStatus enum
pub enum RunPhase {
    Initialize,
    Execute,
    Finalize,
}
