//! DAP (Debug Adapter Protocol) types for debugging integration.

/// A breakpoint in a source file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Breakpoint {
    pub id: u32,
    pub kind: BreakpointKind,
    pub verified: bool,
    pub source: String,
    pub line: usize,
    pub condition: Option<String>,
    pub hit_count: u32,
}

/// Breakpoint kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakpointKind { Line, Conditional, Logpoint, Function, Data }

impl Breakpoint {
    pub fn line(id: u32, source: impl Into<String>, line: usize) -> Self {
        Self { id, kind: BreakpointKind::Line, verified: false, source: source.into(),
               line, condition: None, hit_count: 0 }
    }
    pub fn conditional(id: u32, source: impl Into<String>, line: usize, cond: impl Into<String>) -> Self {
        Self { id, kind: BreakpointKind::Conditional, verified: false, source: source.into(),
               line, condition: Some(cond.into()), hit_count: 0 }
    }
}

/// A single stack frame in the call stack.
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub id: u32,
    pub name: String,
    pub source: Option<String>,
    pub line: usize,
    pub col: usize,
}

/// A variable in the debug scope.
#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub value: String,
    pub var_type: Option<String>,
    pub children_ref: u32,
}

impl Variable {
    pub fn has_children(&self) -> bool { self.children_ref > 0 }
}

/// Debug adapter connection state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DapConnectionState { Disconnected, Connecting, Connected, Terminated }

/// Debug execution state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DapRunState { Stopped, Running, Paused, Stepping }

/// Complete DAP session state.
#[derive(Debug, Clone)]
pub struct DapState {
    pub connection: DapConnectionState,
    pub run_state: DapRunState,
    pub breakpoints: Vec<Breakpoint>,
    pub stack_frames: Vec<StackFrame>,
    pub variables: Vec<Variable>,
    pub adapter_name: Option<String>,
}

impl DapState {
    pub fn new() -> Self {
        Self { connection: DapConnectionState::Disconnected, run_state: DapRunState::Stopped,
               breakpoints: Vec::new(), stack_frames: Vec::new(), variables: Vec::new(),
               adapter_name: None }
    }
    pub fn is_active(&self) -> bool { self.connection == DapConnectionState::Connected }

    pub fn add_breakpoint(&mut self, bp: Breakpoint) { self.breakpoints.push(bp); }

    pub fn remove_breakpoint(&mut self, id: u32) -> bool {
        let len = self.breakpoints.len();
        self.breakpoints.retain(|b| b.id != id);
        self.breakpoints.len() < len
    }

    pub fn toggle_breakpoint(&mut self, source: &str, line: usize, next_id: u32) -> bool {
        if let Some(idx) = self.breakpoints.iter().position(|b| b.source == source && b.line == line) {
            self.breakpoints.remove(idx); false
        } else {
            self.breakpoints.push(Breakpoint::line(next_id, source, line)); true
        }
    }

    pub fn breakpoints_in_file(&self, source: &str) -> Vec<&Breakpoint> {
        self.breakpoints.iter().filter(|b| b.source == source).collect()
    }

    pub fn current_frame(&self) -> Option<&StackFrame> { self.stack_frames.first() }
}

impl Default for DapState {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_is_disconnected() {
        let d = DapState::new();
        assert_eq!(d.connection, DapConnectionState::Disconnected);
        assert!(!d.is_active());
    }

    #[test]
    fn add_and_remove_breakpoint() {
        let mut d = DapState::new();
        d.add_breakpoint(Breakpoint::line(1, "main.rs", 10));
        assert_eq!(d.breakpoints.len(), 1);
        assert!(d.remove_breakpoint(1));
        assert!(d.breakpoints.is_empty());
    }

    #[test]
    fn toggle_breakpoint() {
        let mut d = DapState::new();
        let added = d.toggle_breakpoint("main.rs", 5, 1);
        assert!(added);
        assert_eq!(d.breakpoints.len(), 1);
        let removed = d.toggle_breakpoint("main.rs", 5, 2);
        assert!(!removed);
        assert!(d.breakpoints.is_empty());
    }

    #[test]
    fn conditional_breakpoint() {
        let bp = Breakpoint::conditional(1, "lib.rs", 20, "x > 5");
        assert_eq!(bp.kind, BreakpointKind::Conditional);
        assert_eq!(bp.condition.as_deref(), Some("x > 5"));
    }

    #[test]
    fn breakpoints_in_file() {
        let mut d = DapState::new();
        d.add_breakpoint(Breakpoint::line(1, "a.rs", 1));
        d.add_breakpoint(Breakpoint::line(2, "b.rs", 2));
        d.add_breakpoint(Breakpoint::line(3, "a.rs", 10));
        assert_eq!(d.breakpoints_in_file("a.rs").len(), 2);
    }

    #[test]
    fn variable_children() {
        let v = Variable { name: "obj".into(), value: "{}".into(), var_type: Some("Object".into()), children_ref: 5 };
        assert!(v.has_children());
        let s = Variable { name: "x".into(), value: "42".into(), var_type: None, children_ref: 0 };
        assert!(!s.has_children());
    }

    #[test]
    fn stack_frame_current() {
        let mut d = DapState::new();
        assert!(d.current_frame().is_none());
        d.stack_frames.push(StackFrame { id: 0, name: "main".into(), source: Some("main.rs".into()), line: 1, col: 0 });
        assert_eq!(d.current_frame().unwrap().name, "main");
    }

    #[test]
    fn run_states() {
        let states = [DapRunState::Stopped, DapRunState::Running, DapRunState::Paused, DapRunState::Stepping];
        assert_eq!(states.len(), 4);
    }
}
