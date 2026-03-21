use std::io;
use std::path::Path;

use serde_json::json;

use crate::error::AppError;

use super::compose::{verify_compose, ProcessCommandRunner, COMPOSE_VERIFY_STEP_COUNT};
use super::line_limits::scan_line_limits;
use super::topology::{scan_docs_topology, TopologyRule};
use super::CommandResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliCommand {
    DocsValidateTopology,
    QualityCheckLines,
    ComposeVerify,
}

pub async fn run_cli(args: &[String]) -> Result<CommandResult, AppError> {
    match parse_command(args)? {
        CliCommand::DocsValidateTopology => Ok(run_docs_validate_topology()),
        CliCommand::QualityCheckLines => Ok(run_quality_check_lines()),
        CliCommand::ComposeVerify => Ok(run_compose_verify()),
    }
}

fn parse_command(args: &[String]) -> Result<CliCommand, AppError> {
    let as_str = args.iter().map(String::as_str).collect::<Vec<_>>();
    match as_str.as_slice() {
        ["docs", "validate-topology"] => Ok(CliCommand::DocsValidateTopology),
        ["quality", "check-lines"] => Ok(CliCommand::QualityCheckLines),
        ["compose", "verify"] => Ok(CliCommand::ComposeVerify),
        _ => Err(AppError::unsupported_command(if args.is_empty() {
            String::new()
        } else {
            args.join(" ")
        })),
    }
}

fn run_docs_validate_topology() -> CommandResult {
    match scan_docs_topology(Path::new("docs")) {
        Ok(report) => {
            for violation in &report.violations {
                emit(json!({
                    "command": "docs.validate-topology",
                    "status": "fail",
                    "path": path_display(&violation.path),
                    "rule": topology_rule_name(violation.rule),
                    "expected": violation.expected,
                    "actual": violation.actual,
                }));
            }

            let result = report.result();
            emit(json!({
                "command": "docs.validate-topology",
                "status": result.status(),
                "directories_checked": report.directories_checked,
                "violations": report.violations.len(),
            }));
            result
        }
        Err(error) => {
            emit_io_failure("docs.validate-topology", &error);
            CommandResult::Fail
        }
    }
}

fn run_quality_check_lines() -> CommandResult {
    match scan_line_limits(Path::new("docs"), Path::new("src/tests")) {
        Ok(report) => {
            for violation in &report.violations {
                emit(json!({
                    "command": "quality.check-lines",
                    "status": "fail",
                    "scope": violation.scope.as_str(),
                    "path": path_display(&violation.path),
                    "line_count": violation.line_count,
                    "limit": violation.limit,
                }));
            }

            let result = report.result();
            emit(json!({
                "command": "quality.check-lines",
                "status": result.status(),
                "docs_files_checked": report.docs_files_checked,
                "test_source_files_checked": report.test_source_files_checked,
                "violations": report.violations.len(),
            }));
            result
        }
        Err(error) => {
            emit_io_failure("quality.check-lines", &error);
            CommandResult::Fail
        }
    }
}

fn run_compose_verify() -> CommandResult {
    let report = verify_compose(&ProcessCommandRunner);

    for step in &report.steps {
        let mut event = json!({
            "command": "compose.verify",
            "step": step.step,
            "status": step.result.status(),
        });

        if let Some(exit_code) = step.exit_code {
            event["exit_code"] = json!(exit_code);
        }
        if let Some(detail) = &step.detail {
            event["detail"] = json!(detail);
        }

        emit(event);
    }

    let result = report.result();
    let mut summary = json!({
        "command": "compose.verify",
        "status": result.status(),
        "steps_passed": report.steps_passed(),
        "steps_total": COMPOSE_VERIFY_STEP_COUNT,
    });

    if let Some(failed_step) = report.failed_step {
        summary["failed_step"] = json!(failed_step);
    }

    emit(summary);
    result
}

fn emit_io_failure(command: &str, error: &io::Error) {
    emit(json!({
        "command": command,
        "status": "fail",
        "error": "io_error",
        "message": error.to_string(),
    }));
}

fn emit(value: serde_json::Value) {
    println!("{value}");
}

fn topology_rule_name(rule: TopologyRule) -> &'static str {
    rule.as_str()
}

fn path_display(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
