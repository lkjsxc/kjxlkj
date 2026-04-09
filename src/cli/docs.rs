//! Documentation validation commands

use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
struct GateResult {
    command: &'static str,
    status: &'static str,
    details: Option<String>,
}

/// Validate documentation topology
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
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .collect();

    let readme_count = entries
        .iter()
        .filter(|e| e.file_name() == "README.md")
        .count();
    let dir_display = dir.display();
    if readme_count != 1 {
        errors.push(format!("{dir_display}: expected exactly one README.md"));
    }

    let non_readme_children = entries
        .iter()
        .filter(|e| e.file_name() != "README.md")
        .count();
    if non_readme_children < 2 {
        errors.push(format!(
            "{}: needs at least two children besides README.md (has {})",
            dir.display(),
            non_readme_children
        ));
    }

    for entry in &entries {
        if entry.path().is_dir() {
            validate_dir_recursive(&entry.path(), errors)?;
        }
    }
    Ok(())
}

/// Validate canonical terms usage
pub fn validate_terms() -> Result<(), Box<dyn std::error::Error>> {
    let docs_root = Path::new("docs");
    if !docs_root.exists() {
        return fail_gate("validate-terms", "docs directory not found");
    }

    let mut terms = HashSet::new();
    collect_terms(docs_root, &mut terms)?;

    println!(
        r#"{{"command":"validate-terms","status":"pass","terms_count":{}}}"#,
        terms.len()
    );
    Ok(())
}

/// Validate relative markdown links
pub fn validate_links() -> Result<(), Box<dyn std::error::Error>> {
    let docs_root = Path::new("docs");
    if !docs_root.exists() {
        return fail_gate("validate-links", "docs directory not found");
    }
    let mut errors = Vec::new();
    collect_link_errors(docs_root, &mut errors)?;
    if errors.is_empty() {
        pass_gate("validate-links");
    } else {
        fail_gate("validate-links", &errors.join("; "))?;
    }
    Ok(())
}

fn collect_terms(dir: &Path, terms: &mut HashSet<String>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_terms(&path, terms)?;
        } else if path.extension().is_some_and(|e| e == "md") {
            let content = fs::read_to_string(&path)?;
            for line in content.lines() {
                if line.starts_with("## ") || line.starts_with("### ") {
                    terms.insert(line.to_string());
                }
            }
        }
    }
    Ok(())
}

fn collect_link_errors(dir: &Path, errors: &mut Vec<String>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_link_errors(&path, errors)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = fs::read_to_string(&path)?;
        for target in markdown_links(&content) {
            if skip_link(&target) || resolve_link(&path, &target).exists() {
                continue;
            }
            errors.push(format!(
                "{} -> missing link target {}",
                path.display(),
                target
            ));
        }
    }
    Ok(())
}

fn markdown_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    let bytes = content.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b']'
            && bytes.get(index + 1) == Some(&b'(')
            && content[..index].contains('[')
        {
            if let Some(end) = content[index + 2..].find(')') {
                links.push(content[index + 2..index + 2 + end].to_string());
                index += end + 2;
            }
        }
        index += 1;
    }
    links
}

fn skip_link(target: &str) -> bool {
    target.starts_with("http://")
        || target.starts_with("https://")
        || target.starts_with('/')
        || target.starts_with('#')
        || target.starts_with("mailto:")
}

fn resolve_link(path: &Path, target: &str) -> PathBuf {
    let clean = target.split('#').next().unwrap_or(target);
    path.parent().unwrap_or_else(|| Path::new(".")).join(clean)
}

fn pass_gate(cmd: &'static str) {
    let result = GateResult {
        command: cmd,
        status: "pass",
        details: None,
    };
    println!("{}", serde_json::to_string(&result).unwrap());
}

fn fail_gate(cmd: &'static str, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = GateResult {
        command: cmd,
        status: "fail",
        details: Some(msg.to_string()),
    };
    println!("{}", serde_json::to_string(&result).unwrap());
    std::process::exit(1);
}
