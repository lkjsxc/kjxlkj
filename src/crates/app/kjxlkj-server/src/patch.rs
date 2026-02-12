use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PatchOp {
    Retain { retain: usize },
    Insert { insert: String },
    Delete { delete: usize },
}

pub fn apply_patch(input: &str, ops: &[PatchOp]) -> Result<String, AppError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0usize;
    let mut out = String::with_capacity(input.len());
    for op in ops {
        match op {
            PatchOp::Retain { retain } => {
                if index + retain > chars.len() {
                    return Err(AppError::BadRequest(
                        "retain past input boundary".to_string(),
                    ));
                }
                for c in &chars[index..index + retain] {
                    out.push(*c);
                }
                index += retain;
            }
            PatchOp::Insert { insert } => out.push_str(insert),
            PatchOp::Delete { delete } => {
                if index + delete > chars.len() {
                    return Err(AppError::BadRequest(
                        "delete past input boundary".to_string(),
                    ));
                }
                index += delete;
            }
        }
    }
    for c in &chars[index..] {
        out.push(*c);
    }
    Ok(out)
}

pub fn parse_wikilinks(markdown: &str) -> Vec<String> {
    let re = Regex::new(r"\[\[([^\]]+)\]\]").expect("regex");
    let mut links = Vec::new();
    for cap in re.captures_iter(markdown) {
        if let Some(m) = cap.get(1) {
            let title = m.as_str().trim();
            if !title.is_empty() {
                links.push(title.to_string());
            }
        }
    }
    links.sort();
    links.dedup();
    links
}

pub fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = tags
        .iter()
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

#[cfg(test)]
mod tests {
    use super::{apply_patch, normalize_tags, parse_wikilinks, PatchOp};

    #[test]
    fn patch_ops_apply_in_order() {
        let ops = vec![
            PatchOp::Retain { retain: 1 },
            PatchOp::Delete { delete: 1 },
            PatchOp::Insert {
                insert: "ello".to_string(),
            },
        ];
        let out = apply_patch("hX", &ops).expect("patch should apply");
        assert_eq!(out, "hello");
    }

    #[test]
    fn patch_rejects_delete_out_of_bounds() {
        let err = apply_patch("abc", &[PatchOp::Delete { delete: 4 }]).expect_err("must fail");
        assert!(err.to_string().contains("delete past input boundary"));
    }

    #[test]
    fn wikilinks_are_deduped_and_trimmed() {
        let got = parse_wikilinks("[[ Alpha ]] and [[Beta]] and [[Alpha]]");
        assert_eq!(got, vec!["Alpha".to_string(), "Beta".to_string()]);
    }

    #[test]
    fn tags_are_normalized() {
        let got = normalize_tags(&[
            "  Topic  ".to_string(),
            "topic".to_string(),
            "".to_string(),
            "Records".to_string(),
        ]);
        assert_eq!(got, vec!["records".to_string(), "topic".to_string()]);
    }
}
