//! Librarian XML-like protocol parser per /docs/spec/api/librarian-xml.md.
//! Protocol name: xml_attrless. Tags MUST NOT include attributes.

use serde::{Deserialize, Serialize};

/// Parsed librarian operation per /docs/spec/api/types.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarianOperation {
    pub operation_id: String,
    pub kind: String,
    pub target_note_id: Option<String>,
    pub target_path: Option<String>,
    pub title: String,
    pub body_markdown: String,
    pub reason: String,
    pub confidence: f64,
}

/// Parse a librarian response XML into operations.
/// Per /docs/spec/api/librarian-xml.md parser rules.
pub fn parse_response(xml: &str) -> Result<Vec<LibrarianOperation>, String> {
    let xml = xml.replace("\r\n", "\n").replace('\r', "\n");
    let mut ops = Vec::new();

    // Find <operations> block
    let ops_start = xml.find("<operations>")
        .ok_or("Missing <operations> tag")?;
    let ops_end = xml.find("</operations>")
        .ok_or("Missing </operations> tag")?;
    let ops_block = &xml[ops_start..ops_end];
    let mut pos = 0;

    while let Some(op_start) = ops_block[pos..].find("<operation>") {
        let abs_start = pos + op_start;
        let op_end = ops_block[abs_start..].find("</operation>")
            .ok_or("Unclosed <operation> tag")?;
        let op_block = &ops_block[abs_start..abs_start + op_end + 12];
        let op = parse_operation(op_block)?;
        ops.push(op);
        pos = abs_start + op_end + 12;
    }

    Ok(ops)
}

fn parse_operation(block: &str) -> Result<LibrarianOperation, String> {
    Ok(LibrarianOperation {
        operation_id: extract_tag(block, "operation_id").unwrap_or_default(),
        kind: extract_tag(block, "kind").unwrap_or_default(),
        target_note_id: extract_tag(block, "target_note_id"),
        target_path: extract_tag(block, "target_path"),
        title: extract_tag(block, "title").unwrap_or_default(),
        body_markdown: extract_tag(block, "body_markdown").unwrap_or_default(),
        reason: extract_tag(block, "reason").unwrap_or_default(),
        confidence: extract_tag(block, "confidence")
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0),
    })
}

fn extract_tag(block: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = block.find(&open)? + open.len();
    let end = block[start..].find(&close)? + start;
    Some(block[start..end].to_string())
}

/// Validate operation kinds per /docs/spec/api/types.md.
pub fn validate_operations(
    ops: &[LibrarianOperation],
    max_operations: usize,
    strict_mode: bool,
) -> Result<(), String> {
    if ops.len() > max_operations {
        return Err(format!(
            "Too many operations: {} > {max_operations}", ops.len()
        ));
    }
    let allowed_strict = ["create_note", "rewrite_note"];
    for op in ops {
        if strict_mode && !allowed_strict.contains(&op.kind.as_str()) {
            return Err(format!(
                "Operation kind '{}' not allowed in strict mode", op.kind
            ));
        }
        if !(0.0..=1.0).contains(&op.confidence) {
            return Err(format!(
                "Confidence {} out of [0.0, 1.0] range", op.confidence
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_response() {
        let xml = r#"
<librarian_response>
<request_id>req_01</request_id>
<status>ok</status>
<summary>Reorganized</summary>
<operations>
<operation>
<operation_id>op_01</operation_id>
<kind>create_note</kind>
<target_path>/docs/new.md</target_path>
<title>New Note</title>
<body_markdown># Hello</body_markdown>
<reason>Organization</reason>
<confidence>0.9</confidence>
</operation>
</operations>
<warnings></warnings>
</librarian_response>"#;
        let ops = parse_response(xml).unwrap();
        assert_eq!(ops.len(), 1);
        assert_eq!(ops[0].kind, "create_note");
        assert_eq!(ops[0].confidence, 0.9);
    }

    #[test]
    fn test_validate_strict() {
        let ops = vec![LibrarianOperation {
            operation_id: "1".into(),
            kind: "retitle_note".into(),
            target_note_id: None,
            target_path: None,
            title: "t".into(),
            body_markdown: "b".into(),
            reason: "r".into(),
            confidence: 0.5,
        }];
        assert!(validate_operations(&ops, 10, true).is_err());
        assert!(validate_operations(&ops, 10, false).is_ok());
    }

    #[test]
    fn test_max_operations() {
        let ops: Vec<LibrarianOperation> = (0..5)
            .map(|i| LibrarianOperation {
                operation_id: i.to_string(),
                kind: "create_note".into(),
                target_note_id: None,
                target_path: None,
                title: "t".into(),
                body_markdown: "b".into(),
                reason: "r".into(),
                confidence: 0.5,
            })
            .collect();
        assert!(validate_operations(&ops, 3, false).is_err());
        assert!(validate_operations(&ops, 5, false).is_ok());
    }
}
