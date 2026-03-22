use std::io;
use std::path::Path;

use serde_json::json;

use super::compose::{verify_compose, ProcessCommandRunner, COMPOSE_VERIFY_STEP_COUNT};
use super::line_limits::scan::{scan_line_limits, scan_text_for_terms};
use super::topology::{scan_docs_topology, TopologyRule};
use super::CommandResult;

const FORBIDDEN_TERM: &str = concat!("ob", "sidian");

pub fn run_docs_validate_terms() -> CommandResult {
    let root = Path::new("docs");
    match scan_text_for_terms(root, &[FORBIDDEN_TERM]) {
        Ok(report) => {
            for violation in &report.violations {
                emit(json!({
                    "command": "docs.validate-terms",
                    "status": "fail",
                    "path": path_display(&violation.path),
                    "term": violation.term,
                    "line": violation.line,
                }));
            }
            let result = report.result();
            emit(json!({
                "command": "docs.validate-terms",
                "status": result.status(),
                "files_checked": report.files_checked,
                "violations": report.violations.len(),
            }));
            result
        }
        Err(error) => {
            emit_io_failure("docs.validate-terms", &error);
            CommandResult::Fail
        }
    }
}

pub fn run_docs_validate_topology() -> CommandResult {
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

pub fn run_quality_check_lines() -> CommandResult {
    match scan_line_limits(Path::new("docs"), Path::new("src")) {
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
                "source_files_checked": report.source_files_checked,
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

pub fn run_compose_verify() -> CommandResult {
    let cli_program = resolve_cli_program();
    let report = verify_compose(&ProcessCommandRunner, &cli_program);
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

fn resolve_cli_program() -> String {
    std::env::current_exe()
        .ok()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "kjxlkj".to_owned())
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
