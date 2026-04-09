use super::output::{fail_gate, pass_gate};
use std::fs;
use std::path::Path;

struct TermRule {
    name: &'static str,
    words: &'static [&'static str],
    literals: &'static [&'static str],
}

const RULES: &[TermRule] = &[
    TermRule {
        name: "old shared term `record`",
        words: &["record", "records"],
        literals: &[],
    },
    TermRule {
        name: "old shared term `revision`",
        words: &["revision", "revisions"],
        literals: &[],
    },
    TermRule {
        name: "old attachment response field `current_note`",
        words: &[],
        literals: &["current_note"],
    },
];

/// Validate canonical terms usage.
pub fn validate_terms() -> Result<(), Box<dyn std::error::Error>> {
    let docs_root = Path::new("docs");
    if !docs_root.exists() {
        return fail_gate("validate-terms", "docs directory not found");
    }

    let mut errors = Vec::new();
    collect_term_errors(docs_root, &mut errors)?;
    if errors.is_empty() {
        pass_gate("validate-terms");
    } else {
        fail_gate("validate-terms", &errors.join("; "))?;
    }
    Ok(())
}

fn collect_term_errors(dir: &Path, errors: &mut Vec<String>) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_term_errors(&path, errors)?;
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let content = fs::read_to_string(&path)?;
        for (index, line) in content.lines().enumerate() {
            if allowed_reference_line(&path, line) {
                continue;
            }
            for rule in RULES {
                if matches_rule(line, rule) {
                    errors.push(format!("{}:{} -> {}", path.display(), index + 1, rule.name));
                }
            }
        }
    }
    Ok(())
}

fn allowed_reference_line(path: &Path, line: &str) -> bool {
    path == Path::new("docs/vision/llm-optimization.md")
        && line.contains("Avoid older shared terms")
}

fn matches_rule(line: &str, rule: &TermRule) -> bool {
    rule.words.iter().any(|word| contains_word(line, word))
        || rule.literals.iter().any(|literal| line.contains(literal))
}

fn contains_word(line: &str, word: &str) -> bool {
    let line = line.to_ascii_lowercase();
    let word = word.to_ascii_lowercase();
    let mut start = 0usize;
    while let Some(offset) = line[start..].find(&word) {
        let index = start + offset;
        let end = index + word.len();
        if is_boundary(line.as_bytes(), index, end) {
            return true;
        }
        start = end;
    }
    false
}

fn is_boundary(bytes: &[u8], start: usize, end: usize) -> bool {
    let before = start.checked_sub(1).and_then(|index| bytes.get(index));
    let after = bytes.get(end);
    before.is_none_or(|byte| !is_word_byte(*byte)) && after.is_none_or(|byte| !is_word_byte(*byte))
}

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}
