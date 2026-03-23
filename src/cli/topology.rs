use std::path::Path;

use serde_json::json;

use super::CommandResult;

pub fn run_docs_validate_topology() -> CommandResult {
    let mut directories_checked = 0usize;
    let mut violations = Vec::<String>::new();
    if let Err(error) = walk(Path::new("docs"), &mut directories_checked, &mut violations) {
        println!(
            "{}",
            json!({"command":"docs.validate-topology","status":"fail","error":"io_error","message":error.to_string()})
        );
        return CommandResult::Fail;
    }
    for path in &violations {
        println!(
            "{}",
            json!({"command":"docs.validate-topology","status":"fail","path":path,"rule":"directory_contract"})
        );
    }
    let status = if violations.is_empty() {
        CommandResult::Pass
    } else {
        CommandResult::Fail
    };
    println!(
        "{}",
        json!({"command":"docs.validate-topology","status":status.status(),"directories_checked":directories_checked,"violations":violations.len()})
    );
    status
}

fn walk(
    root: &Path,
    directories_checked: &mut usize,
    violations: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    if !root.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("directory not found: {}", root.display()),
        ));
    }
    *directories_checked += 1;
    let mut children = std::fs::read_dir(root)?
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .collect::<Vec<_>>();
    children.sort_by_key(|entry| entry.path());
    let readme_count = children
        .iter()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter(|entry| entry.file_name() == "README.md")
        .count();
    if readme_count != 1 || children.len() < 2 {
        violations.push(root.to_string_lossy().replace('\\', "/"));
    }
    for entry in children {
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            walk(&entry.path(), directories_checked, violations)?;
        }
    }
    Ok(())
}
