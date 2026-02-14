// xml_attrless parser per /docs/spec/api/librarian-xml.md
// Protocol: tag-only envelopes, no XML attributes.
// Parser MUST normalize line endings to \n before tokenization.
// Parser MUST reject malformed nesting with LIBRARIAN_PROTOCOL_INVALID.
// Missing required tags MUST fail as LIBRARIAN_PARSE_FAILED.
// Confidence MUST parse as decimal in [0.0, 1.0].

use crate::runner::{LibrarianOperation, OperationKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct LibrarianResponse {
    pub request_id: String,
    pub status: String,
    pub summary: String,
    pub operations: Vec<LibrarianOperation>,
    pub warnings: Vec<String>,
}

/// Parse xml_attrless librarian response per spec.
pub fn parse_response(
    raw: &str,
    max_operations: Option<usize>,
    strict_mode: bool,
) -> Result<LibrarianResponse, ParseError> {
    // Normalize line endings
    let text = raw.replace("\r\n", "\n").replace('\r', "\n");

    // Validate root tag
    let inner = extract_tag(&text, "librarian_response").ok_or(ParseError {
        code: "LIBRARIAN_PARSE_FAILED",
        message: "Missing <librarian_response> root tag".into(),
    })?;

    let request_id = extract_tag_text(inner, "request_id").ok_or(ParseError {
        code: "LIBRARIAN_PARSE_FAILED",
        message: "Missing <request_id>".into(),
    })?;

    let status = extract_tag_text(inner, "status").unwrap_or_default();
    let summary = extract_tag_text(inner, "summary").unwrap_or_default();

    // Parse warnings
    let warnings_block = extract_tag(inner, "warnings").unwrap_or("");
    let warnings: Vec<String> = extract_all_tag_text(warnings_block, "warning");

    // Parse operations
    let ops_block = extract_tag(inner, "operations").unwrap_or("");
    let op_blocks = extract_all_blocks(ops_block, "operation");

    if let Some(max) = max_operations {
        if op_blocks.len() > max {
            return Err(ParseError {
                code: "LIBRARIAN_OPERATION_REJECTED",
                message: format!(
                    "Operations count {} exceeds max {}",
                    op_blocks.len(),
                    max
                ),
            });
        }
    }

    let mut operations = Vec::new();
    for (i, block) in op_blocks.iter().enumerate() {
        let op = parse_operation(block, i, strict_mode)?;
        operations.push(op);
    }

    Ok(LibrarianResponse {
        request_id,
        status,
        summary,
        operations,
        warnings,
    })
}

fn parse_operation(
    block: &str,
    index: usize,
    strict_mode: bool,
) -> Result<LibrarianOperation, ParseError> {
    let op_id = extract_tag_text(block, "operation_id")
        .unwrap_or_else(|| format!("op_{index}"));

    let kind_str = extract_tag_text(block, "kind").ok_or(ParseError {
        code: "LIBRARIAN_PROTOCOL_INVALID",
        message: format!("Operation {index}: missing <kind>"),
    })?;

    let kind = OperationKind::from_str(&kind_str).ok_or(ParseError {
        code: "LIBRARIAN_PROTOCOL_INVALID",
        message: format!("Operation {index}: unknown kind '{kind_str}'"),
    })?;

    if strict_mode && !kind.is_strict_allowed() {
        return Err(ParseError {
            code: "LIBRARIAN_OPERATION_REJECTED",
            message: format!(
                "Operation {index}: kind '{}' not allowed in strict mode",
                kind.as_str()
            ),
        });
    }

    let confidence_str =
        extract_tag_text(block, "confidence").unwrap_or_else(|| "0.5".into());
    let confidence: f64 = confidence_str.parse().map_err(|_| ParseError {
        code: "LIBRARIAN_PROTOCOL_INVALID",
        message: format!("Operation {index}: invalid confidence '{confidence_str}'"),
    })?;
    if !(0.0..=1.0).contains(&confidence) {
        return Err(ParseError {
            code: "LIBRARIAN_PROTOCOL_INVALID",
            message: format!(
                "Operation {index}: confidence {confidence} out of [0.0, 1.0]"
            ),
        });
    }

    Ok(LibrarianOperation {
        operation_id: op_id,
        kind,
        target_note_id: extract_tag_text(block, "target_note_id"),
        target_path: extract_tag_text(block, "target_path"),
        title: extract_tag_text(block, "title").unwrap_or_default(),
        body_markdown: extract_tag_text(block, "body_markdown")
            .unwrap_or_default(),
        reason: extract_tag_text(block, "reason").unwrap_or_default(),
        confidence,
    })
}

/// Extract content between <tag> and </tag>.
fn extract_tag<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)? + open.len();
    let end = text.find(&close)?;
    if end < start {
        return None;
    }
    Some(&text[start..end])
}

/// Extract text content of a tag, trimmed.
fn extract_tag_text(text: &str, tag: &str) -> Option<String> {
    extract_tag(text, tag).map(|s| s.trim().to_string())
}

/// Extract all text occurrences of a tag.
fn extract_all_tag_text(text: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut results = Vec::new();
    let mut search_from = 0;
    while let Some(start_pos) = text[search_from..].find(&open) {
        let abs_start = search_from + start_pos + open.len();
        if let Some(end_pos) = text[abs_start..].find(&close) {
            let abs_end = abs_start + end_pos;
            results.push(text[abs_start..abs_end].trim().to_string());
            search_from = abs_end + close.len();
        } else {
            break;
        }
    }
    results
}

/// Extract all blocks between <tag>...</tag>.
fn extract_all_blocks(text: &str, tag: &str) -> Vec<String> {
    extract_all_tag_text(text, tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RESPONSE: &str = r#"
<librarian_response>
<request_id>req_01</request_id>
<status>ok</status>
<summary>Organized two notes</summary>
<operations>
<operation>
<operation_id>op_1</operation_id>
<kind>rewrite_note</kind>
<target_note_id>note_18</target_note_id>
<title>Revised Runbook</title>
<body_markdown># Runbook
Updated content here.</body_markdown>
<reason>Better structure</reason>
<confidence>0.85</confidence>
</operation>
<operation>
<operation_id>op_2</operation_id>
<kind>create_note</kind>
<target_path>new/index.md</target_path>
<title>Index</title>
<body_markdown># Index</body_markdown>
<reason>New entry point</reason>
<confidence>0.90</confidence>
</operation>
</operations>
<warnings>
<warning>Source note_19 skipped: no markdown content</warning>
</warnings>
</librarian_response>"#;

    #[test]
    fn parse_valid_response() {
        let result = parse_response(SAMPLE_RESPONSE, Some(10), false).unwrap();
        assert_eq!(result.request_id, "req_01");
        assert_eq!(result.status, "ok");
        assert_eq!(result.operations.len(), 2);
        assert_eq!(result.operations[0].kind, OperationKind::RewriteNote);
        assert_eq!(result.operations[1].kind, OperationKind::CreateNote);
        assert!((result.operations[0].confidence - 0.85).abs() < 0.001);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn reject_too_many_operations() {
        let err = parse_response(SAMPLE_RESPONSE, Some(1), false).unwrap_err();
        assert_eq!(err.code, "LIBRARIAN_OPERATION_REJECTED");
    }

    #[test]
    fn reject_strict_mode() {
        let strict_xml = r#"
<librarian_response>
<request_id>req_02</request_id>
<status>ok</status>
<summary>Test</summary>
<operations>
<operation>
<operation_id>op_1</operation_id>
<kind>retitle_note</kind>
<target_note_id>n1</target_note_id>
<title>New Title</title>
<body_markdown></body_markdown>
<reason>Rename</reason>
<confidence>0.9</confidence>
</operation>
</operations>
<warnings></warnings>
</librarian_response>"#;
        let err = parse_response(strict_xml, None, true).unwrap_err();
        assert_eq!(err.code, "LIBRARIAN_OPERATION_REJECTED");
    }

    #[test]
    fn reject_bad_confidence() {
        let xml = r#"
<librarian_response>
<request_id>req_03</request_id>
<status>ok</status>
<summary>Test</summary>
<operations>
<operation>
<operation_id>op_1</operation_id>
<kind>create_note</kind>
<title>T</title>
<body_markdown>B</body_markdown>
<reason>R</reason>
<confidence>1.5</confidence>
</operation>
</operations>
<warnings></warnings>
</librarian_response>"#;
        let err = parse_response(xml, None, false).unwrap_err();
        assert_eq!(err.code, "LIBRARIAN_PROTOCOL_INVALID");
    }

    #[test]
    fn missing_root_tag() {
        let err = parse_response("<not_valid>hi</not_valid>", None, false)
            .unwrap_err();
        assert_eq!(err.code, "LIBRARIAN_PARSE_FAILED");
    }
}
