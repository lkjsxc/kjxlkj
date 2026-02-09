//! DAP (Debug Adapter Protocol) integration.
//!
//! Handles breakpoints, step/continue, variables, and
//! debug session lifecycle.

use std::collections::HashMap;
use std::path::PathBuf;

/// A breakpoint set in source.
#[derive(Debug, Clone)]
pub struct Breakpoint {
    /// File path.
    pub file: PathBuf,
    /// 1-indexed line number.
    pub line: usize,
    /// Optional condition expression.
    pub condition: Option<String>,
    /// Whether this breakpoint is enabled.
    pub enabled: bool,
    /// Unique ID assigned by the debug adapter.
    pub id: Option<u64>,
}

/// Debug session state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugStatus {
    /// Not debugging.
    Inactive,
    /// Connected, waiting for launch/attach.
    Connected,
    /// Running the debuggee.
    Running,
    /// Stopped at breakpoint or step.
    Stopped,
    /// Debug session terminated.
    Terminated,
}

/// A stack frame from the debug adapter.
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame ID.
    pub id: u64,
    /// Function or scope name.
    pub name: String,
    /// Source file path.
    pub source: Option<PathBuf>,
    /// Line number.
    pub line: usize,
    /// Column number.
    pub column: usize,
}

/// Variable from a scope.
#[derive(Debug, Clone)]
pub struct Variable {
    /// Variable name.
    pub name: String,
    /// Display value.
    pub value: String,
    /// Type name if available.
    pub var_type: Option<String>,
    /// Reference for expanding children.
    pub children_ref: u64,
}

/// Top-level DAP state.
#[derive(Debug, Clone)]
pub struct DapState {
    /// Current debug status.
    pub status: DebugStatus,
    /// All breakpoints keyed by file path.
    pub breakpoints: HashMap<PathBuf, Vec<Breakpoint>>,
    /// Current stack frames when stopped.
    pub stack_frames: Vec<StackFrame>,
    /// Variables in current scope.
    pub variables: Vec<Variable>,
    /// Next breakpoint ID.
    next_bp_id: u64,
}

impl DapState {
    pub fn new() -> Self {
        Self {
            status: DebugStatus::Inactive,
            breakpoints: HashMap::new(),
            stack_frames: Vec::new(),
            variables: Vec::new(),
            next_bp_id: 1,
        }
    }

    /// Toggle a breakpoint at file:line.
    pub fn toggle_breakpoint(&mut self, file: PathBuf, line: usize) {
        let bps = self.breakpoints.entry(file.clone()).or_default();
        if let Some(idx) = bps.iter().position(|b| b.line == line) {
            bps.remove(idx);
        } else {
            bps.push(Breakpoint {
                file,
                line,
                condition: None,
                enabled: true,
                id: Some(self.next_bp_id),
            });
            self.next_bp_id += 1;
        }
    }

    /// Get all breakpoints for a file.
    pub fn file_breakpoints(&self, file: &PathBuf) -> &[Breakpoint] {
        self.breakpoints
            .get(file)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Step over (next line).
    pub fn step_over(&mut self) {
        if self.status == DebugStatus::Stopped {
            self.status = DebugStatus::Running;
        }
    }

    /// Step into.
    pub fn step_into(&mut self) {
        if self.status == DebugStatus::Stopped {
            self.status = DebugStatus::Running;
        }
    }

    /// Continue execution.
    pub fn continue_exec(&mut self) {
        if self.status == DebugStatus::Stopped {
            self.status = DebugStatus::Running;
        }
    }

    /// Terminate debug session.
    pub fn terminate(&mut self) {
        self.status = DebugStatus::Terminated;
        self.stack_frames.clear();
        self.variables.clear();
    }
}

impl Default for DapState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_breakpoint() {
        let mut dap = DapState::new();
        let file = PathBuf::from("main.rs");
        dap.toggle_breakpoint(file.clone(), 10);
        assert_eq!(dap.file_breakpoints(&file).len(), 1);
        dap.toggle_breakpoint(file.clone(), 10);
        assert_eq!(dap.file_breakpoints(&file).len(), 0);
    }

    #[test]
    fn debug_lifecycle() {
        let mut dap = DapState::new();
        assert_eq!(dap.status, DebugStatus::Inactive);
        dap.status = DebugStatus::Stopped;
        dap.step_over();
        assert_eq!(dap.status, DebugStatus::Running);
        dap.terminate();
        assert_eq!(dap.status, DebugStatus::Terminated);
    }

    #[test]
    fn breakpoint_with_condition() {
        let mut dap = DapState::new();
        let file = PathBuf::from("lib.rs");
        dap.toggle_breakpoint(file.clone(), 5);
        let bps = dap.file_breakpoints(&file);
        assert!(bps[0].enabled);
        assert!(bps[0].condition.is_none());
    }
}
