// Patch operations per /docs/spec/api/types.md
use serde::{Deserialize, Serialize};

/// Patch operation enum per PatchOp spec
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatchOp {
    Retain(usize),
    Insert(String),
    Delete(usize),
}

/// Apply patch operations to a base document.
/// Returns the resulting document or error.
pub fn apply_patch(base: &str, ops: &[PatchOp]) -> Result<String, String> {
    let chars: Vec<char> = base.chars().collect();
    let mut cursor = 0usize;
    let mut result = String::new();

    for op in ops {
        match op {
            PatchOp::Retain(n) => {
                let end = cursor + n;
                if end > chars.len() {
                    return Err(format!(
                        "retain({n}) at {cursor} exceeds length {}",
                        chars.len()
                    ));
                }
                for c in &chars[cursor..end] {
                    result.push(*c);
                }
                cursor = end;
            }
            PatchOp::Insert(text) => {
                result.push_str(text);
            }
            PatchOp::Delete(n) => {
                let end = cursor + n;
                if end > chars.len() {
                    return Err(format!(
                        "delete({n}) at {cursor} exceeds length {}",
                        chars.len()
                    ));
                }
                cursor = end;
            }
        }
    }

    // Implicit retain of remaining characters
    for c in &chars[cursor..] {
        result.push(*c);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_at_beginning() {
        let ops = vec![PatchOp::Insert("Hello ".into())];
        assert_eq!(apply_patch("world", &ops).unwrap(), "Hello world");
    }

    #[test]
    fn test_delete_and_insert() {
        let ops = vec![
            PatchOp::Delete(5),
            PatchOp::Insert("Hi".into()),
        ];
        assert_eq!(apply_patch("Hello world", &ops).unwrap(), "Hi world");
    }

    #[test]
    fn test_retain_and_insert() {
        let ops = vec![
            PatchOp::Retain(5),
            PatchOp::Insert(" beautiful".into()),
        ];
        assert_eq!(
            apply_patch("Hello world", &ops).unwrap(),
            "Hello beautiful world"
        );
    }

    #[test]
    fn test_overflow_delete_rejected() {
        let ops = vec![PatchOp::Delete(100)];
        assert!(apply_patch("Hi", &ops).is_err());
    }
}
