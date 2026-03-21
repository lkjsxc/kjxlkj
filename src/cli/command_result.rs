#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandResult {
    Pass,
    Fail,
}

impl CommandResult {
    pub fn from_failure_count(failure_count: usize) -> Self {
        if failure_count == 0 {
            Self::Pass
        } else {
            Self::Fail
        }
    }

    pub fn status(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
        }
    }

    pub fn is_success(self) -> bool {
        self == Self::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::CommandResult;

    #[test]
    fn result_is_pass_when_no_failures_exist() {
        assert_eq!(CommandResult::from_failure_count(0), CommandResult::Pass);
    }

    #[test]
    fn result_is_fail_when_any_failure_exists() {
        assert_eq!(CommandResult::from_failure_count(1), CommandResult::Fail);
    }

    #[test]
    fn status_values_are_stable() {
        assert_eq!(CommandResult::Pass.status(), "pass");
        assert_eq!(CommandResult::Fail.status(), "fail");
    }
}
