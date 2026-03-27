//! Quality gate commands

use serde::Serialize;
use std::fs;
use std::path::Path;

const DOCS_LINE_LIMIT: usize = 300;
const SRC_LINE_LIMIT: usize = 200;

#[derive(Serialize)]
struct LineCheckResult {
    command: &'static str,
    status: &'static str,
    docs_max: usize,
    src_max: usize,
    visual_max: usize,
    violations: Vec<String>,
}

/// Check file line limits
pub fn check_lines() -> Result<(), Box<dyn std::error::Error>> {
    let mut violations = Vec::new();
    let mut docs_max = 0usize;
    let mut src_max = 0usize;
    let mut visual_max = 0usize;

    check_dir_lines(
        Path::new("docs"),
        DOCS_LINE_LIMIT,
        &mut violations,
        &mut docs_max,
    )?;
    check_dir_lines(
        Path::new("src"),
        SRC_LINE_LIMIT,
        &mut violations,
        &mut src_max,
    )?;
    check_dir_lines(
        Path::new("visual"),
        SRC_LINE_LIMIT,
        &mut violations,
        &mut visual_max,
    )?;

    let result = LineCheckResult {
        command: "check-lines",
        status: if violations.is_empty() {
            "pass"
        } else {
            "fail"
        },
        docs_max,
        src_max,
        visual_max,
        violations: violations.clone(),
    };
    println!("{}", serde_json::to_string(&result)?);

    if !violations.is_empty() {
        std::process::exit(1);
    }
    Ok(())
}

fn check_dir_lines(
    dir: &Path,
    limit: usize,
    violations: &mut Vec<String>,
    max_lines: &mut usize,
) -> Result<(), std::io::Error> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            check_dir_lines(&path, limit, violations, max_lines)?;
        } else {
            let ext = path.extension().and_then(|e| e.to_str());
            let should_check = matches!(
                ext,
                Some("md") | Some("rs") | Some("js") | Some("mjs") | Some("css")
            );
            if should_check {
                let content = fs::read_to_string(&path)?;
                let lines = content.lines().count();
                *max_lines = (*max_lines).max(lines);
                if lines > limit {
                    violations.push(format!(
                        "{}: {} lines (limit {})",
                        path.display(),
                        lines,
                        limit
                    ));
                }
            }
        }
    }
    Ok(())
}
