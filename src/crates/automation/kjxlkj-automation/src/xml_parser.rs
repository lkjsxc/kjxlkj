use tracing::{debug, warn};

/// Parsed agent instruction per docs/spec/api/librarian-xml.md.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    StateAdd { state: String },
    StateDelete { state: String },
    RamAdd { key: String, value: String },
    RamDelete { key: String },
    RecordAdd { keywords: String, value: String },
    RecordIssue { key: String, value: String, metadata: String },
    RecordUpdate { key: String, value: String },
    RecordSearch { query: Option<String>, ids: Option<String> },
}

/// Parse agent XML output into instructions.
/// Follows attribute-free XML protocol.
pub fn parse_instructions(input: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions = Vec::new();
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(instructions);
    }

    // Simple XML parser for attribute-free tags
    let mut pos = 0;
    let bytes = trimmed.as_bytes();

    while pos < bytes.len() {
        // Skip whitespace
        while pos < bytes.len() && bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        if pos >= bytes.len() {
            break;
        }

        if bytes[pos] != b'<' {
            pos += 1;
            continue;
        }

        // Find tag name
        let tag_start = pos + 1;
        let tag_end = match trimmed[tag_start..].find('>') {
            Some(i) => tag_start + i,
            None => return Err(ParseError::MalformedXml("unclosed tag".into())),
        };
        let tag_name = &trimmed[tag_start..tag_end];

        // Skip closing tags
        if tag_name.starts_with('/') {
            pos = tag_end + 1;
            continue;
        }

        let close_tag = format!("</{tag_name}>");
        let content_start = tag_end + 1;
        let content_end = match trimmed[content_start..].find(&close_tag) {
            Some(i) => content_start + i,
            None => {
                warn!("unclosed tag: {tag_name}");
                pos = tag_end + 1;
                continue;
            }
        };

        let content = &trimmed[content_start..content_end];
        pos = content_end + close_tag.len();

        match tag_name {
            "state_add" => {
                let state = extract_child(content, "state")?;
                instructions.push(Instruction::StateAdd { state });
            }
            "state_delete" => {
                let state = extract_child(content, "state")?;
                instructions.push(Instruction::StateDelete { state });
            }
            "ram_add" => {
                let key = extract_child(content, "key")?;
                let value = extract_child(content, "value")?;
                instructions.push(Instruction::RamAdd { key, value });
            }
            "ram_delete" => {
                let key = extract_child(content, "key")?;
                instructions.push(Instruction::RamDelete { key });
            }
            "record_add" => {
                let keywords = extract_child(content, "keywords")?;
                let value = extract_child(content, "value")?;
                instructions.push(Instruction::RecordAdd { keywords, value });
            }
            "record_issue" => {
                let key = extract_child(content, "key")?;
                let value = extract_child(content, "value")?;
                let metadata = extract_child(content, "metadata")?;
                instructions.push(Instruction::RecordIssue { key, value, metadata });
            }
            "record_update" => {
                let key = extract_child(content, "key")?;
                let value = extract_child(content, "value")?;
                instructions.push(Instruction::RecordUpdate { key, value });
            }
            "record_search" => {
                let query = extract_child_opt(content, "query");
                let ids = extract_child_opt(content, "ids");
                instructions.push(Instruction::RecordSearch { query, ids });
            }
            _ => {
                debug!("ignoring unknown tag: {tag_name}");
            }
        }
    }

    Ok(instructions)
}

/// Extract a required child element value.
fn extract_child(content: &str, tag: &str) -> Result<String, ParseError> {
    extract_child_opt(content, tag)
        .ok_or_else(|| ParseError::MissingChild(tag.to_string()))
}

/// Extract an optional child element value.
fn extract_child_opt(content: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = content.find(&open)?;
    let value_start = start + open.len();
    let end = content[value_start..].find(&close)?;
    Some(content[value_start..value_start + end].trim().to_string())
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("malformed XML: {0}")]
    MalformedXml(String),
    #[error("missing required child element: {0}")]
    MissingChild(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_state_add() {
        let input = "<state_add><state>planning</state></state_add>";
        let result = parse_instructions(input).unwrap();
        assert_eq!(
            result,
            vec![Instruction::StateAdd {
                state: "planning".into()
            }]
        );
    }

    #[test]
    fn test_parse_ram_add() {
        let input = "<ram_add><key>think_log</key><value>hello</value></ram_add>";
        let result = parse_instructions(input).unwrap();
        assert_eq!(
            result,
            vec![Instruction::RamAdd {
                key: "think_log".into(),
                value: "hello".into(),
            }]
        );
    }

    #[test]
    fn test_parse_multiple() {
        let input = r#"
<state_add><state>executing</state></state_add>
<ram_add><key>step</key><value>1</value></ram_add>
<record_search><query>test notes</query></record_search>
"#;
        let result = parse_instructions(input).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_parse_empty() {
        let result = parse_instructions("").unwrap();
        assert!(result.is_empty());
    }
}
