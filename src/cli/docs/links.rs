use super::output::{fail_gate, pass_gate};
use std::fs;
use std::path::{Path, PathBuf};

/// Validate relative markdown links.
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
