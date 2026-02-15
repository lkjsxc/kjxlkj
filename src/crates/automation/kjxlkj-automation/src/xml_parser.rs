//! Attribute-less XML parser per /docs/spec/api/librarian-xml.md.
//! Types are in xml_types.rs; this module has parsing logic only.

pub use crate::xml_types::*;

/// Parse xml_attrless response text into structured response.
/// Per spec: normalize line endings, reject attributes, validate tags.
pub fn parse_response(
    text: &str,
    max_operations: usize,
    strict_mode: bool,
) -> Result<LibrarianResponse, ParseError> {
    let text = text.replace("\r\n", "\n").replace('\r', "\n");

    let root_content = extract_tag(&text, "librarian_response")
        .ok_or_else(|| ParseError::ProtocolInvalid(
            "missing <librarian_response> root".into(),
        ))?;

    if has_attributes(&text, "librarian_response") {
        return Err(ParseError::ProtocolInvalid(
            "attributes not allowed on tags".into(),
        ));
    }

    let request_id = extract_tag_required(root_content, "request_id")?;
    let status = extract_tag_required(root_content, "status")?;
    let summary = extract_tag_required(root_content, "summary")?;

    let ops_block = extract_tag(root_content, "operations").unwrap_or("");
    let operations = parse_operations(ops_block, strict_mode)?;

    if operations.len() > max_operations {
        return Err(ParseError::TooManyOperations {
            max: max_operations,
            found: operations.len(),
        });
    }

    let warnings_block = extract_tag(root_content, "warnings").unwrap_or("");
    let warnings = extract_all_tags(warnings_block, "warning");

    Ok(LibrarianResponse {
        request_id,
        status,
        summary,
        operations,
        warnings,
    })
}

fn parse_operations(
    block: &str,
    strict_mode: bool,
) -> Result<Vec<ParsedOperation>, ParseError> {
    let mut ops = Vec::new();
    let mut search = block;
    while let Some(start) = search.find("<operation>") {
        let end = search[start..]
            .find("</operation>")
            .ok_or_else(|| ParseError::ProtocolInvalid("unclosed <operation>".into()))?;
        let inner = &search[start + 11..start + end];
        ops.push(parse_single_operation(inner, strict_mode)?);
        search = &search[start + end + 12..];
    }
    Ok(ops)
}

fn parse_single_operation(
    inner: &str,
    strict_mode: bool,
) -> Result<ParsedOperation, ParseError> {
    let operation_id = extract_tag_required(inner, "operation_id")?;
    let kind = extract_tag_required(inner, "kind")?;

    let allowed = if strict_mode { STRICT_KINDS } else { VALID_KINDS };
    if !allowed.contains(&kind.as_str()) {
        return Err(ParseError::ParseFailed(format!(
            "unknown or disallowed kind: {kind}"
        )));
    }

    let target_note_id = extract_tag(inner, "target_note_id").map(String::from);
    let target_path = extract_tag(inner, "target_path").map(String::from);
    let title = extract_tag_required(inner, "title")?;
    let body_markdown = extract_tag_required(inner, "body_markdown")?;
    let reason = extract_tag_required(inner, "reason")?;
    let confidence_str = extract_tag_required(inner, "confidence")?;
    let confidence: f32 = confidence_str
        .trim()
        .parse()
        .map_err(|_| ParseError::InvalidConfidence(confidence_str.clone()))?;
    if !(0.0..=1.0).contains(&confidence) {
        return Err(ParseError::InvalidConfidence(confidence_str));
    }

    Ok(ParsedOperation {
        operation_id,
        kind,
        target_note_id,
        target_path,
        title,
        body_markdown,
        reason,
        confidence,
    })
}

fn extract_tag<'a>(text: &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let start = text.find(&open)?;
    let after_open = start + open.len();
    let end = text[after_open..].find(&close)?;
    Some(&text[after_open..after_open + end])
}

fn extract_tag_required(text: &str, tag: &str) -> Result<String, ParseError> {
    extract_tag(text, tag)
        .map(|s| s.trim().to_string())
        .ok_or_else(|| ParseError::MissingTag(tag.into()))
}

fn extract_all_tags(text: &str, tag: &str) -> Vec<String> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let mut results = Vec::new();
    let mut search = text;
    while let Some(start) = search.find(&open) {
        let after = start + open.len();
        if let Some(end) = search[after..].find(&close) {
            results.push(search[after..after + end].trim().to_string());
            search = &search[after + end + close.len()..];
        } else {
            break;
        }
    }
    results
}

fn has_attributes(text: &str, tag: &str) -> bool {
    let pattern = format!("<{tag}");
    if let Some(pos) = text.find(&pattern) {
        let after = pos + pattern.len();
        if let Some(ch) = text[after..].chars().next() {
            return ch != '>' && ch != '/';
        }
    }
    false
}
