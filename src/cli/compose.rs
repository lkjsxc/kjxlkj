use std::io;
use std::process::Command;

use super::CommandResult;

pub const COMPOSE_VERIFY_STEP_COUNT: usize = 4;
const DETAIL_LIMIT: usize = 200;

struct ComposeStep<'a> {
    name: &'static str,
    program: &'a str,
    args: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExecution {
    pub success: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<CommandExecution>;
}

pub struct ProcessCommandRunner;

impl CommandRunner for ProcessCommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> io::Result<CommandExecution> {
        let output = Command::new(program).args(args).output()?;
        Ok(CommandExecution {
            success: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeStepResult {
    pub step: &'static str,
    pub result: CommandResult,
    pub exit_code: Option<i32>,
    pub detail: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposeVerifyReport {
    pub steps: Vec<ComposeStepResult>,
    pub failed_step: Option<&'static str>,
}

impl ComposeVerifyReport {
    pub fn result(&self) -> CommandResult {
        if self.failed_step.is_some() {
            CommandResult::Fail
        } else {
            CommandResult::Pass
        }
    }

    pub fn steps_passed(&self) -> usize {
        self.steps
            .iter()
            .filter(|step| step.result == CommandResult::Pass)
            .count()
    }
}

pub fn verify_compose<R: CommandRunner>(runner: &R, cli_program: &str) -> ComposeVerifyReport {
    let mut steps = Vec::new();

    for step in compose_steps(cli_program) {
        match runner.run(step.program, step.args) {
            Ok(execution) if execution.success => {
                steps.push(ComposeStepResult {
                    step: step.name,
                    result: CommandResult::Pass,
                    exit_code: execution.exit_code,
                    detail: None,
                });
            }
            Ok(execution) => {
                steps.push(ComposeStepResult {
                    step: step.name,
                    result: CommandResult::Fail,
                    exit_code: execution.exit_code,
                    detail: compact_detail(&execution.stderr)
                        .or_else(|| compact_detail(&execution.stdout)),
                });

                return ComposeVerifyReport {
                    steps,
                    failed_step: Some(step.name),
                };
            }
            Err(error) => {
                steps.push(ComposeStepResult {
                    step: step.name,
                    result: CommandResult::Fail,
                    exit_code: None,
                    detail: compact_detail(&error.to_string()),
                });

                return ComposeVerifyReport {
                    steps,
                    failed_step: Some(step.name),
                };
            }
        }
    }

    ComposeVerifyReport {
        steps,
        failed_step: None,
    }
}

fn compose_steps(cli_program: &str) -> [ComposeStep<'_>; COMPOSE_VERIFY_STEP_COUNT] {
    [
        ComposeStep {
            name: "docs-validate-topology",
            program: cli_program,
            args: &["docs", "validate-topology"],
        },
        ComposeStep {
            name: "docs-validate-terms",
            program: cli_program,
            args: &["docs", "validate-terms"],
        },
        ComposeStep {
            name: "quality-check-lines",
            program: cli_program,
            args: &["quality", "check-lines"],
        },
        ComposeStep {
            name: "verify-profile-run",
            program: "docker",
            args: &["compose", "--profile", "verify", "run", "--rm", "verify"],
        },
    ]
}

fn compact_detail(detail: &str) -> Option<String> {
    let trimmed = detail.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut compact = String::new();
    let mut truncated = false;

    for (index, character) in trimmed.chars().enumerate() {
        if index >= DETAIL_LIMIT {
            truncated = true;
            break;
        }

        compact.push(character);
    }

    if truncated {
        compact.push_str("...");
    }

    Some(compact)
}
