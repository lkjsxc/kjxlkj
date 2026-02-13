use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchOp {
    Retain { retain: usize },
    Insert { insert: String },
    Delete { delete: usize },
}

#[derive(Debug, Error)]
pub enum PatchError {
    #[error("invalid patch operation")]
    Invalid,
}

pub fn apply_patch(base: &str, operations: &[PatchOp]) -> Result<String, PatchError> {
    let source_chars: Vec<char> = base.chars().collect();
    let mut cursor = 0usize;
    let mut output = String::new();

    for operation in operations {
        match operation {
            PatchOp::Retain { retain } => {
                let end = cursor.saturating_add(*retain);
                if end > source_chars.len() {
                    return Err(PatchError::Invalid);
                }
                output.extend(source_chars[cursor..end].iter());
                cursor = end;
            }
            PatchOp::Insert { insert } => {
                output.push_str(insert);
            }
            PatchOp::Delete { delete } => {
                let end = cursor.saturating_add(*delete);
                if end > source_chars.len() {
                    return Err(PatchError::Invalid);
                }
                cursor = end;
            }
        }
    }

    output.extend(source_chars[cursor..].iter());
    Ok(output)
}

pub fn extract_backlinks(markdown: &str) -> Vec<String> {
    let mut links = HashSet::new();
    let bytes = markdown.as_bytes();
    let mut index = 0usize;

    while index + 3 < bytes.len() {
        if &bytes[index..index + 2] == b"[[" {
            let mut end = index + 2;
            while end + 1 < bytes.len() {
                if &bytes[end..end + 2] == b"]]" {
                    break;
                }
                end += 1;
            }

            if end + 1 < bytes.len() {
                let token = markdown[index + 2..end].trim();
                if !token.is_empty() {
                    links.insert(token.to_owned());
                }
                index = end + 2;
                continue;
            }
        }
        index += 1;
    }

    let mut values: Vec<String> = links.into_iter().collect();
    values.sort();
    values
}
