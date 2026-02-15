//! Patch operations per /docs/spec/api/types.md.

use serde::{Deserialize, Serialize};

/// A single patch operation per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchOp {
    Retain { retain: usize },
    Insert { insert: String },
    Delete { delete: usize },
}

/// Apply patch operations to a base document.
/// Returns the resulting string or an error message.
pub fn apply_patch(base: &str, ops: &[PatchOp]) -> Result<String, String> {
    let chars: Vec<char> = base.chars().collect();
    let mut pos = 0usize;
    let mut result = String::new();
    for op in ops {
        match op {
            PatchOp::Retain { retain } => {
                let end = pos + retain;
                if end > chars.len() {
                    return Err(format!(
                        "retain({retain}) at pos {pos} exceeds doc length {}",
                        chars.len()
                    ));
                }
                for c in &chars[pos..end] {
                    result.push(*c);
                }
                pos = end;
            }
            PatchOp::Insert { insert } => {
                result.push_str(insert);
            }
            PatchOp::Delete { delete } => {
                let end = pos + delete;
                if end > chars.len() {
                    return Err(format!(
                        "delete({delete}) at pos {pos} exceeds doc length {}",
                        chars.len()
                    ));
                }
                pos = end;
            }
        }
    }
    // Append remaining characters
    for c in &chars[pos..] {
        result.push(*c);
    }
    Ok(result)
}

/// Parse wiki-links from markdown for backlink extraction.
pub fn extract_wiki_links(markdown: &str) -> Vec<String> {
    let mut links = Vec::new();
    let chars: Vec<char> = markdown.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' && i + 1 < chars.len() && chars[i + 1] == '[' {
            i += 2;
            let mut target = String::new();
            while i < chars.len() {
                if chars[i] == ']' && i + 1 < chars.len() && chars[i + 1] == ']' {
                    i += 2;
                    break;
                }
                target.push(chars[i]);
                i += 1;
            }
            if !target.is_empty() {
                links.push(target);
            }
        } else {
            i += 1;
        }
    }
    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_patch_insert() {
        let base = "hello";
        let ops = vec![
            PatchOp::Retain { retain: 5 },
            PatchOp::Insert {
                insert: " world".to_string(),
            },
        ];
        assert_eq!(apply_patch(base, &ops).unwrap(), "hello world");
    }

    #[test]
    fn test_apply_patch_delete() {
        let base = "hello world";
        let ops = vec![
            PatchOp::Retain { retain: 5 },
            PatchOp::Delete { delete: 6 },
        ];
        assert_eq!(apply_patch(base, &ops).unwrap(), "hello");
    }

    #[test]
    fn test_apply_patch_replace() {
        let base = "hello";
        let ops = vec![
            PatchOp::Delete { delete: 5 },
            PatchOp::Insert {
                insert: "world".to_string(),
            },
        ];
        assert_eq!(apply_patch(base, &ops).unwrap(), "world");
    }

    #[test]
    fn test_patch_overflow() {
        let base = "hi";
        let ops = vec![PatchOp::Retain { retain: 10 }];
        assert!(apply_patch(base, &ops).is_err());
    }

    #[test]
    fn test_extract_wiki_links() {
        let md = "See [[Note A]] and [[Note B]] for details.";
        let links = extract_wiki_links(md);
        assert_eq!(links, vec!["Note A", "Note B"]);
    }

    #[test]
    fn test_extract_wiki_links_empty() {
        let md = "No links here.";
        let links = extract_wiki_links(md);
        assert!(links.is_empty());
    }
}
