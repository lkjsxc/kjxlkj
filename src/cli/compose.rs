use std::process::Command;

use serde_json::json;

use super::CommandResult;

pub fn run_compose_verify() -> CommandResult {
    let steps = [
        (
            "docs-validate-topology",
            current_program(),
            vec!["docs", "validate-topology"],
        ),
        (
            "docs-validate-terms",
            current_program(),
            vec!["docs", "validate-terms"],
        ),
        (
            "quality-check-lines",
            current_program(),
            vec!["quality", "check-lines"],
        ),
        (
            "verify-profile-run",
            "docker".to_owned(),
            vec![
                "compose",
                "--profile",
                "verify",
                "run",
                "--rm",
                "--build",
                "verify",
            ],
        ),
    ];
    let mut passed = 0usize;
    for (name, program, args) in steps {
        let output = Command::new(&program).args(&args).output();
        let success = output.as_ref().is_ok_and(|value| value.status.success());
        let status = if success {
            CommandResult::Pass
        } else {
            CommandResult::Fail
        };
        if status.is_success() {
            passed += 1;
            println!(
                "{}",
                json!({"command":"compose.verify","step":name,"status":"pass"})
            );
            continue;
        }
        let detail = output
            .as_ref()
            .ok()
            .and_then(|value| {
                let stderr = String::from_utf8_lossy(&value.stderr).trim().to_owned();
                if stderr.is_empty() {
                    let stdout = String::from_utf8_lossy(&value.stdout).trim().to_owned();
                    if stdout.is_empty() {
                        None
                    } else {
                        Some(stdout)
                    }
                } else {
                    Some(stderr)
                }
            })
            .unwrap_or_else(|| "command failed".to_owned());
        println!(
            "{}",
            json!({"command":"compose.verify","step":name,"status":"fail","detail":detail})
        );
        println!(
            "{}",
            json!({"command":"compose.verify","status":"fail","steps_passed":passed,"steps_total":4,"failed_step":name})
        );
        return CommandResult::Fail;
    }
    println!(
        "{}",
        json!({"command":"compose.verify","status":"pass","steps_passed":4,"steps_total":4})
    );
    CommandResult::Pass
}

fn current_program() -> String {
    std::env::current_exe()
        .ok()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_else(|| "kjxlkj".to_owned())
}
