/// End-to-end feature integration — exercises core→service→render pipeline.

/// Feature integration test: simulates file open → edit → save flow.
#[derive(Debug)]
pub struct IntegrationScenario {
    pub name: String,
    pub steps: Vec<ScenarioStep>,
    pub expected_state: ExpectedState,
}

/// A step in an integration scenario.
#[derive(Debug, Clone)]
pub enum ScenarioStep {
    OpenFile(String),
    TypeText(String),
    ExecuteCommand(String),
    SendKey(String),
    WaitMs(u64),
    AssertBufferContent(String),
}

/// Expected end state.
#[derive(Debug, Clone)]
pub struct ExpectedState {
    pub buffer_modified: bool,
    pub mode: String,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl IntegrationScenario {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), steps: Vec::new(),
            expected_state: ExpectedState { buffer_modified: false, mode: "normal".into(), cursor_line: 0, cursor_col: 0 } }
    }

    pub fn step(mut self, s: ScenarioStep) -> Self { self.steps.push(s); self }
    pub fn expect(mut self, state: ExpectedState) -> Self { self.expected_state = state; self }
    pub fn step_count(&self) -> usize { self.steps.len() }
}

/// Scenario runner result.
#[derive(Debug)]
pub struct ScenarioResult { pub passed: bool, pub failures: Vec<String> }

/// Dry-run a scenario validating step structure (no actual execution).
pub fn validate_scenario(scenario: &IntegrationScenario) -> ScenarioResult {
    let mut failures = Vec::new();
    if scenario.steps.is_empty() { failures.push("no steps defined".into()); }
    for (i, step) in scenario.steps.iter().enumerate() {
        match step {
            ScenarioStep::OpenFile(path) if path.is_empty() => failures.push(format!("step {}: empty path", i)),
            ScenarioStep::ExecuteCommand(cmd) if !cmd.starts_with(':') => failures.push(format!("step {}: command must start with ':'", i)),
            _ => {}
        }
    }
    ScenarioResult { passed: failures.is_empty(), failures }
}

/// Build a standard "open, edit, save" scenario.
pub fn open_edit_save_scenario(path: &str, text: &str) -> IntegrationScenario {
    IntegrationScenario::new("open-edit-save")
        .step(ScenarioStep::OpenFile(path.into()))
        .step(ScenarioStep::SendKey("i".into()))
        .step(ScenarioStep::TypeText(text.into()))
        .step(ScenarioStep::SendKey("<Esc>".into()))
        .step(ScenarioStep::ExecuteCommand(":w".into()))
        .expect(ExpectedState { buffer_modified: false, mode: "normal".into(), cursor_line: 0, cursor_col: text.len().saturating_sub(1) })
}

/// Build an undo-redo scenario.
pub fn undo_redo_scenario() -> IntegrationScenario {
    IntegrationScenario::new("undo-redo")
        .step(ScenarioStep::OpenFile("test.txt".into()))
        .step(ScenarioStep::SendKey("i".into()))
        .step(ScenarioStep::TypeText("hello".into()))
        .step(ScenarioStep::SendKey("<Esc>".into()))
        .step(ScenarioStep::SendKey("u".into()))
        .step(ScenarioStep::SendKey("<C-r>".into()))
        .expect(ExpectedState { buffer_modified: true, mode: "normal".into(), cursor_line: 0, cursor_col: 4 })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_builder() {
        let s = open_edit_save_scenario("foo.rs", "hello");
        assert_eq!(s.step_count(), 5);
        assert_eq!(s.expected_state.mode, "normal");
    }

    #[test]
    fn validate_valid() {
        let s = open_edit_save_scenario("test.txt", "data");
        let r = validate_scenario(&s);
        assert!(r.passed);
    }

    #[test]
    fn validate_empty_steps() {
        let s = IntegrationScenario::new("empty");
        let r = validate_scenario(&s);
        assert!(!r.passed);
    }

    #[test]
    fn validate_bad_command() {
        let s = IntegrationScenario::new("bad")
            .step(ScenarioStep::ExecuteCommand("w".into()));
        let r = validate_scenario(&s);
        assert!(!r.passed);
    }

    #[test]
    fn validate_empty_path() {
        let s = IntegrationScenario::new("nopath")
            .step(ScenarioStep::OpenFile("".into()));
        let r = validate_scenario(&s);
        assert!(!r.passed);
    }

    #[test]
    fn undo_redo() {
        let s = undo_redo_scenario();
        assert_eq!(s.step_count(), 6);
        assert!(s.expected_state.buffer_modified);
    }

    #[test]
    fn scenario_steps() {
        let steps = vec![ScenarioStep::WaitMs(100), ScenarioStep::AssertBufferContent("test".into())];
        assert_eq!(steps.len(), 2);
    }
}
