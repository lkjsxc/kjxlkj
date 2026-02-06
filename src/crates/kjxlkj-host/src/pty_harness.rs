/// PTY E2E test harness framework.

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PtyConfig {
    pub(crate) term: String,
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) timeout_ms: u64,
}

impl PtyConfig {
    pub(crate) fn default() -> Self {
        Self {
            term: "xterm-256color".to_string(),
            width: 80,
            height: 24,
            timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PtyAction {
    TypeText(String),
    SendKey(String),
    WaitMs(u64),
    WriteFile(String),
    Quit,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PtyExpectation {
    FileContains { path: String, expected: String },
    FileExists(String),
    ExitCode(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PtyScenario {
    pub(crate) name: String,
    pub(crate) actions: Vec<PtyAction>,
    pub(crate) expectations: Vec<PtyExpectation>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct PtyResult {
    pub(crate) passed: bool,
    pub(crate) scenario_name: String,
    pub(crate) details: String,
}

impl PtyScenario {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            actions: Vec::new(),
            expectations: Vec::new(),
        }
    }

    pub(crate) fn add_action(&mut self, action: PtyAction) -> &mut Self {
        self.actions.push(action);
        self
    }

    pub(crate) fn add_expectation(&mut self, exp: PtyExpectation) -> &mut Self {
        self.expectations.push(exp);
        self
    }
}

pub(crate) fn validate_scenario(scenario: &PtyScenario) -> Result<(), String> {
    if scenario.name.is_empty() {
        return Err("Scenario name must not be empty".to_string());
    }
    if scenario.actions.is_empty() {
        return Err("Scenario must have at least one action".to_string());
    }
    Ok(())
}

pub(crate) fn format_scenario(scenario: &PtyScenario) -> String {
    let mut out = format!("Scenario: {}\n", scenario.name);
    for (i, action) in scenario.actions.iter().enumerate() {
        out.push_str(&format!("  Step {}: {:?}\n", i + 1, action));
    }
    for exp in &scenario.expectations {
        out.push_str(&format!("  Expect: {:?}\n", exp));
    }
    out
}

pub(crate) fn estimate_duration(scenario: &PtyScenario, config: &PtyConfig) -> u64 {
    let mut total: u64 = 0;
    for action in &scenario.actions {
        total += match action {
            PtyAction::TypeText(t) => t.len() as u64 * 10,
            PtyAction::SendKey(_) => 50,
            PtyAction::WaitMs(ms) => *ms,
            PtyAction::WriteFile(_) => 100,
            PtyAction::Quit => 50,
        };
    }
    total.min(config.timeout_ms)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults() {
        let cfg = PtyConfig::default();
        assert_eq!(cfg.term, "xterm-256color");
        assert_eq!(cfg.width, 80);
        assert_eq!(cfg.height, 24);
        assert_eq!(cfg.timeout_ms, 5000);
    }

    #[test]
    fn new_scenario() {
        let s = PtyScenario::new("test");
        assert_eq!(s.name, "test");
        assert!(s.actions.is_empty());
        assert!(s.expectations.is_empty());
    }

    #[test]
    fn add_actions() {
        let mut s = PtyScenario::new("demo");
        s.add_action(PtyAction::TypeText("hello".into()));
        s.add_action(PtyAction::Quit);
        assert_eq!(s.actions.len(), 2);
    }

    #[test]
    fn validate_empty_name() {
        let s = PtyScenario { name: "".into(), actions: vec![PtyAction::Quit], expectations: vec![] };
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn validate_no_actions() {
        let s = PtyScenario::new("empty");
        assert!(validate_scenario(&s).is_err());
    }

    #[test]
    fn format_output() {
        let mut s = PtyScenario::new("fmt");
        s.add_action(PtyAction::Quit);
        let out = format_scenario(&s);
        assert!(out.contains("fmt"));
        assert!(out.contains("Quit"));
    }

    #[test]
    fn estimate_duration_test() {
        let cfg = PtyConfig::default();
        let mut s = PtyScenario::new("dur");
        s.add_action(PtyAction::WaitMs(100));
        s.add_action(PtyAction::TypeText("abc".into()));
        assert_eq!(estimate_duration(&s, &cfg), 130);
    }
}
