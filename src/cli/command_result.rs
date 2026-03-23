#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandResult {
    Pass,
    Fail,
}

impl CommandResult {
    pub fn is_success(self) -> bool {
        self == Self::Pass
    }

    pub fn status(self) -> &'static str {
        if self.is_success() {
            "pass"
        } else {
            "fail"
        }
    }
}
