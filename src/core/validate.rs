use crate::error::AppError;

use super::RecordInput;

pub fn validate_id(id: &str) -> Result<(), AppError> {
    let valid = !id.is_empty()
        && id.len() >= 3
        && id.len() <= 48
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
        && !id.ends_with('-')
        && !id.contains("--");
    if valid {
        Ok(())
    } else {
        Err(AppError::InvalidRequest(
            "id must be lowercase kebab-case and 3..48 chars".to_owned(),
        ))
    }
}

pub fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut seen = std::collections::BTreeSet::<String>::new();
    let mut output = Vec::new();
    for tag in tags {
        let normalized = tag.trim().to_ascii_lowercase();
        if normalized.is_empty() {
            continue;
        }
        if seen.insert(normalized.clone()) {
            output.push(normalized);
        }
    }
    output
}

pub fn validate_input(input: &RecordInput) -> Result<(), AppError> {
    if input.title.trim().is_empty() {
        return Err(AppError::InvalidRequest(
            "title must not be empty".to_owned(),
        ));
    }
    Ok(())
}
