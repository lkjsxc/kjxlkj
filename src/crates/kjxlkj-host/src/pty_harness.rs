//! PTY-based end-to-end test harness (structural definitions).

use serde::{Deserialize, Serialize};

/// Configuration for a PTY test session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyConfig {
    pub term_type: String,
    pub width: u16,
    pub height: u16,
    pub timeout_ms: u64,
}

impl Default for PtyConfig {
    fn default() -> Self {
        Self {
            term_type: "xterm-256color".into(),
            width: 80,
            height: 24,
            timeout_ms: 5000,
        }
    }
}

/// Actions a PTY test can perform.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PtyAction {
    TypeText(String),
    SendKey(String),
    WaitMs(u64),
    WriteFile(String, String),
    Quit,
}

/// Expected outcomes after a PTY scenario runs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PtyExpectation {
    FileContains(String, String),
    FileExists(String),
    ExitCode(i32),
}

/// A complete PTY test scenario.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtyScenario {
    pub name: String,
    pub description: String,
    pub config: PtyConfig,
    pub actions: Vec<PtyAction>,
    pub expectations: Vec<PtyExpectation>,
}

/// Validate a PTY scenario for structural correctness.
pub fn validate_scenario(scenario: &PtyScenario) -> Result<(), String> {
    if scenario.name.is_empty() {
        return Err("scenario name is empty".into());
    }
    if scenario.actions.is_empty() {
        return Err("scenario has no actions".into());
    }
    if scenario.expectations.is_empty() {
        return Err("scenario has no expectations".into());
    }
    for (i, action) in scenario.actions.iter().enumerate() {
        match action {
            PtyAction::TypeText(t) if t.is_empty() => {
                return Err(format!("action {i}: empty TypeText"));
            }
            PtyAction::SendKey(k) if k.is_empty() => {
                return Err(format!("action {i}: empty SendKey"));
            }
            _ => {}
        }
    }
    Ok(())
}

/// Estimate the total duration of a scenario in milliseconds.
pub fn estimate_duration(scenario: &PtyScenario) -> u64 {
    let base_per_action: u64 = 50;
    let mut total: u64 = 0;
    for action in &scenario.actions {
        match action {
            PtyAction::WaitMs(ms) => total += ms,
            PtyAction::TypeText(t) => total += t.len() as u64 * 10,
            _ => total += base_per_action,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_scenario() -> PtyScenario {
        PtyScenario {
            name: "test".into(),
            description: "desc".into(),
            config: PtyConfig::default(),
            actions: vec![PtyAction::TypeText("hello".into())],
            expectations: vec![PtyExpectation::ExitCode(0)],
        }
    }

    #[test]
    fn validate_ok() {
        assert!(validate_scenario(&minimal_scenario()).is_ok());
    }

    #[test]
    fn validate_empty_name() {
        let mut s = minimal_scenario();
        s.name = String::new();
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn validate_no_actions() {
        let mut s = minimal_scenario();
        s.actions.clear();
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn validate_no_expectations() {
        let mut s = minimal_scenario();
        s.expectations.clear();
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn validate_empty_text() {
        let mut s = minimal_scenario();
        s.actions = vec![PtyAction::TypeText(String::new())];
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn estimate_simple() {
        let s = minimal_scenario();
        assert!(estimate_duration(&s) > 0);
    }

    #[test]
    fn estimate_with_wait() {
        let mut s = minimal_scenario();
        s.actions.push(PtyAction::WaitMs(200));
        assert!(estimate_duration(&s) >= 200);
    }

    #[test]
    fn pty_config_default() {
        let c = PtyConfig::default();
        assert_eq!(c.width, 80);
        assert_eq!(c.height, 24);
    }
}
