use std::path::{Path, PathBuf};

use serde_json::json;

use super::CommandResult;

const BANNED_TERM: &str = concat!("ob", "sidian");
const DOC_LIMIT: usize = 300;
const SRC_LIMIT: usize = 200;

pub fn run_docs_validate_terms() -> CommandResult {
    match collect_files(Path::new("docs"), &|_| true) {
        Ok(files) => {
            let mut violations = 0usize;
            for path in &files {
                if let Ok(text) = std::fs::read_to_string(path) {
                    for (idx, line) in text.lines().enumerate() {
                        if line.to_ascii_lowercase().contains(BANNED_TERM) {
                            violations += 1;
                            println!(
                                "{}",
                                json!({"command":"docs.validate-terms","status":"fail","path":display(path),"line":idx+1,"term":BANNED_TERM})
                            );
                        }
                    }
                }
            }
            let status = if violations == 0 {
                CommandResult::Pass
            } else {
                CommandResult::Fail
            };
            println!(
                "{}",
                json!({"command":"docs.validate-terms","status":status.status(),"files_checked":files.len(),"violations":violations})
            );
            status
        }
        Err(error) => {
            println!(
                "{}",
                json!({"command":"docs.validate-terms","status":"fail","error":"io_error","message":error.to_string()})
            );
            CommandResult::Fail
        }
    }
}

pub fn run_quality_check_lines() -> CommandResult {
    let docs = collect_files(Path::new("docs"), &is_markdown);
    let src = collect_files(Path::new("src"), &is_source);
    match (docs, src) {
        (Ok(docs_files), Ok(src_files)) => {
            let mut violations = 0usize;
            for file in &docs_files {
                violations += emit_line_limit_violation(
                    "quality.check-lines",
                    "docs_markdown",
                    file,
                    DOC_LIMIT,
                );
            }
            for file in &src_files {
                violations += emit_line_limit_violation(
                    "quality.check-lines",
                    "source_code",
                    file,
                    SRC_LIMIT,
                );
            }
            let status = if violations == 0 {
                CommandResult::Pass
            } else {
                CommandResult::Fail
            };
            println!(
                "{}",
                json!({"command":"quality.check-lines","status":status.status(),"docs_files_checked":docs_files.len(),"source_files_checked":src_files.len(),"violations":violations})
            );
            status
        }
        (Err(error), _) | (_, Err(error)) => {
            println!(
                "{}",
                json!({"command":"quality.check-lines","status":"fail","error":"io_error","message":error.to_string()})
            );
            CommandResult::Fail
        }
    }
}

fn emit_line_limit_violation(command: &str, scope: &str, path: &Path, limit: usize) -> usize {
    let lines = std::fs::read_to_string(path)
        .map(|value| value.lines().count())
        .unwrap_or(0);
    if lines > limit {
        println!(
            "{}",
            json!({"command":command,"status":"fail","scope":scope,"path":display(path),"line_count":lines,"limit":limit})
        );
        1
    } else {
        0
    }
}

fn collect_files(
    root: &Path,
    include: &dyn Fn(&Path) -> bool,
) -> Result<Vec<PathBuf>, std::io::Error> {
    if !root.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("directory not found: {}", root.display()),
        ));
    }
    let mut output = Vec::new();
    collect_recursive(root, include, &mut output)?;
    output.sort();
    Ok(output)
}

fn collect_recursive(
    directory: &Path,
    include: &dyn Fn(&Path) -> bool,
    output: &mut Vec<PathBuf>,
) -> Result<(), std::io::Error> {
    let mut entries = std::fs::read_dir(directory)?
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let path = entry.path();
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            collect_recursive(&path, include, output)?;
        } else if entry.file_type().is_ok_and(|kind| kind.is_file()) && include(&path) {
            output.push(path);
        }
    }
    Ok(())
}

fn is_markdown(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("md")
}

fn is_source(path: &Path) -> bool {
    matches!(path.extension().and_then(|ext| ext.to_str()), Some("rs"))
}

fn display(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
