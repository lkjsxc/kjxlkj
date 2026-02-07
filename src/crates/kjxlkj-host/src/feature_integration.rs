//! Integration scenario definitions for full-flow testing.

use serde::{Deserialize, Serialize};

/// A single step in an integration scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScenarioStep {
    OpenFile(String),
    TypeText(String),
    ExecuteCommand(String),
    SendKey(String),
    WaitMs(u64),
    Assert(String),
}

/// A complete integration scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationScenario {
    pub name: String,
    pub steps: Vec<ScenarioStep>,
    pub expected_state: String,
}

/// Validate an integration scenario for structural correctness.
pub fn validate_scenario(scenario: &IntegrationScenario) -> Result<(), String> {
    if scenario.name.is_empty() {
        return Err("scenario name is empty".into());
    }
    if scenario.steps.is_empty() {
        return Err("scenario has no steps".into());
    }
    for (i, step) in scenario.steps.iter().enumerate() {
        match step {
            ScenarioStep::OpenFile(p) if p.is_empty() => {
                return Err(format!("step {i}: empty OpenFile path"));
            }
            ScenarioStep::ExecuteCommand(c) if c.is_empty() => {
                return Err(format!("step {i}: empty ExecuteCommand"));
            }
            _ => {}
        }
    }
    Ok(())
}

/// Scenario: open a file, edit it, save it.
pub fn open_edit_save_scenario() -> IntegrationScenario {
    IntegrationScenario {
        name: "open-edit-save".into(),
        steps: vec![
            ScenarioStep::OpenFile("test.txt".into()),
            ScenarioStep::SendKey("i".into()),
            ScenarioStep::TypeText("hello world".into()),
            ScenarioStep::SendKey("Escape".into()),
            ScenarioStep::ExecuteCommand(":w".into()),
            ScenarioStep::Assert("buffer saved".into()),
        ],
        expected_state: "normal mode, buffer not modified".into(),
    }
}

/// Scenario: undo and redo preserve content.
pub fn undo_redo_scenario() -> IntegrationScenario {
    IntegrationScenario {
        name: "undo-redo".into(),
        steps: vec![
            ScenarioStep::SendKey("i".into()),
            ScenarioStep::TypeText("abc".into()),
            ScenarioStep::SendKey("Escape".into()),
            ScenarioStep::SendKey("u".into()),
            ScenarioStep::Assert("content undone".into()),
            ScenarioStep::ExecuteCommand("redo".into()),
            ScenarioStep::Assert("content redone".into()),
        ],
        expected_state: "normal mode, content = abc".into(),
    }
}

/// Scenario: open multiple buffers and switch between them.
pub fn multi_buffer_scenario() -> IntegrationScenario {
    IntegrationScenario {
        name: "multi-buffer".into(),
        steps: vec![
            ScenarioStep::OpenFile("a.txt".into()),
            ScenarioStep::TypeText("buffer a".into()),
            ScenarioStep::ExecuteCommand(":e b.txt".into()),
            ScenarioStep::TypeText("buffer b".into()),
            ScenarioStep::ExecuteCommand(":bp".into()),
            ScenarioStep::Assert("active buffer is a.txt".into()),
        ],
        expected_state: "two buffers, a.txt active".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_edit_save_valid() {
        assert!(validate_scenario(&open_edit_save_scenario()).is_ok());
    }

    #[test]
    fn undo_redo_valid() {
        assert!(validate_scenario(&undo_redo_scenario()).is_ok());
    }

    #[test]
    fn multi_buffer_valid() {
        assert!(validate_scenario(&multi_buffer_scenario()).is_ok());
    }

    #[test]
    fn empty_name_invalid() {
        let s = IntegrationScenario {
            name: String::new(),
            steps: vec![ScenarioStep::SendKey("i".into())],
            expected_state: "ok".into(),
        };
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn no_steps_invalid() {
        let s = IntegrationScenario {
            name: "x".into(),
            steps: vec![],
            expected_state: "ok".into(),
        };
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn empty_open_path_invalid() {
        let s = IntegrationScenario {
            name: "x".into(),
            steps: vec![ScenarioStep::OpenFile(String::new())],
            expected_state: "ok".into(),
        };
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn scenario_count() {
        let scenarios = [
            open_edit_save_scenario(),
            undo_redo_scenario(),
            multi_buffer_scenario(),
        ];
        assert_eq!(scenarios.len(), 3);
    }
}
