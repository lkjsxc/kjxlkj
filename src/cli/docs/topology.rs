use super::output::{fail_gate, pass_gate};
use std::fs;
use std::path::Path;

/// Validate documentation topology.
pub fn validate_topology() -> Result<(), Box<dyn std::error::Error>> {
    let docs_root = Path::new("docs");
    if !docs_root.exists() {
        return fail_gate("validate-topology", "docs directory not found");
    }

    let mut errors = Vec::new();
    validate_dir_recursive(docs_root, &mut errors)?;

    if errors.is_empty() {
        pass_gate("validate-topology");
    } else {
        fail_gate("validate-topology", &errors.join("; "))?;
    }
    Ok(())
}

fn validate_dir_recursive(dir: &Path, errors: &mut Vec<String>) -> Result<(), std::io::Error> {
    let entries: Vec<_> = fs::read_dir(dir)?
        .filter_map(Result::ok)
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .collect();

    let readme_count = entries
        .iter()
        .filter(|entry| entry.file_name() == "README.md")
        .count();
    if readme_count != 1 {
        errors.push(format!("{}: expected exactly one README.md", dir.display()));
    }

    let non_readme_children = entries
        .iter()
        .filter(|entry| entry.file_name() != "README.md")
        .count();
    if non_readme_children < 2 {
        errors.push(format!(
            "{}: needs at least two children besides README.md (has {})",
            dir.display(),
            non_readme_children
        ));
    }

    validate_child_index(dir, &entries, errors)?;

    for entry in &entries {
        if entry.path().is_dir() {
            validate_dir_recursive(&entry.path(), errors)?;
        }
    }
    Ok(())
}

fn validate_child_index(
    dir: &Path,
    entries: &[fs::DirEntry],
    errors: &mut Vec<String>,
) -> Result<(), std::io::Error> {
    let readme = dir.join("README.md");
    if !readme.exists() {
        return Ok(());
    }
    let content = fs::read_to_string(&readme)?;
    for entry in entries {
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name == "README.md" {
            continue;
        }
        if !content.contains(name.as_ref()) {
            errors.push(format!("{}: README.md must link {}", dir.display(), name));
        }
    }
    Ok(())
}
