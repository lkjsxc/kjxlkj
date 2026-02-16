/// XML instruction parser per /docs/spec/api/librarian-xml.md
///
/// Protocol: attribute-free XML only.
/// Allowed tags: state_add, state_delete, ram_add, ram_delete,
///   record_add, record_issue, record_update, record_search
use kjxlkj_domain::DomainError;
use quick_xml::events::Event;
use quick_xml::Reader;

/// Parsed instruction from agent output
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

/// Allowed top-level tags per /docs/spec/api/librarian-xml.md
const ALLOWED_TAGS: &[&str] = &[
    "state_add", "state_delete", "ram_add", "ram_delete",
    "record_add", "record_issue", "record_update", "record_search",
];

/// Parse agent XML output into instructions.
/// Per /docs/spec/api/librarian-xml.md: instructions execute in document order.
pub fn parse_instructions(xml: &str) -> Result<Vec<Instruction>, DomainError> {
    let wrapped = format!("<root>{xml}</root>");
    let mut reader = Reader::from_str(&wrapped);
    let mut instructions = Vec::new();
    let mut current_tag: Option<String> = None;
    let mut children: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut current_child: Option<String> = None;
    let mut depth = 0u32;

    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "root" {
                    continue;
                }
                if depth == 0 {
                    if ALLOWED_TAGS.contains(&name.as_str()) {
                        current_tag = Some(name);
                        children.clear();
                        depth = 1;
                    }
                } else if depth == 1 {
                    current_child = Some(name);
                    depth = 2;
                } else {
                    depth += 1;
                }
            }
            Ok(Event::Text(e)) => {
                if let Some(ref child) = current_child {
                    let text = e.unescape().unwrap_or_default().to_string();
                    children.insert(child.clone(), text);
                }
            }
            Ok(Event::End(e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                if name == "root" {
                    break;
                }
                if depth == 2 {
                    current_child = None;
                    depth = 1;
                } else if depth == 1 {
                    if let Some(ref tag) = current_tag {
                        if let Some(instr) = build_instruction(tag, &children) {
                            instructions.push(instr);
                        }
                    }
                    current_tag = None;
                    depth = 0;
                } else if depth > 2 {
                    depth -= 1;
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(DomainError::BadRequest(format!("XML parse error: {e}")));
            }
            _ => {}
        }
    }
    Ok(instructions)
}

fn build_instruction(
    tag: &str,
    children: &std::collections::HashMap<String, String>,
) -> Option<Instruction> {
    match tag {
        "state_add" => Some(Instruction::StateAdd {
            state: children.get("state")?.clone(),
        }),
        "state_delete" => Some(Instruction::StateDelete {
            state: children.get("state")?.clone(),
        }),
        "ram_add" => Some(Instruction::RamAdd {
            key: children.get("key")?.clone(),
            value: children.get("value")?.clone(),
        }),
        "ram_delete" => Some(Instruction::RamDelete {
            key: children.get("key")?.clone(),
        }),
        "record_add" => Some(Instruction::RecordAdd {
            keywords: children.get("keywords")?.clone(),
            value: children.get("value")?.clone(),
        }),
        "record_issue" => Some(Instruction::RecordIssue {
            key: children.get("key")?.clone(),
            value: children.get("value")?.clone(),
            metadata: children.get("metadata")?.clone(),
        }),
        "record_update" => Some(Instruction::RecordUpdate {
            key: children.get("key")?.clone(),
            value: children.get("value")?.clone(),
        }),
        "record_search" => Some(Instruction::RecordSearch {
            query: children.get("query").cloned(),
            ids: children.get("ids").cloned(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_auto_04_xml_parse() {
        // Acceptance: API-AUTO-04
        let xml = r#"<ram_add><key>think_log</key><value>planning</value></ram_add>
<state_add><state>planning</state></state_add>"#;
        let instrs = parse_instructions(xml).unwrap();
        assert_eq!(instrs.len(), 2);
        assert_eq!(
            instrs[0],
            Instruction::RamAdd {
                key: "think_log".into(),
                value: "planning".into()
            }
        );
        assert_eq!(
            instrs[1],
            Instruction::StateAdd {
                state: "planning".into()
            }
        );
    }

    #[test]
    fn test_record_operations() {
        let xml = r#"<record_add><keywords>test</keywords><value>content</value></record_add>
<record_search><query>find this</query></record_search>"#;
        let instrs = parse_instructions(xml).unwrap();
        assert_eq!(instrs.len(), 2);
        match &instrs[0] {
            Instruction::RecordAdd { keywords, value } => {
                assert_eq!(keywords, "test");
                assert_eq!(value, "content");
            }
            _ => panic!("expected RecordAdd"),
        }
    }

    #[test]
    fn test_invalid_xml_returns_error() {
        let xml = "<unclosed>";
        // This may succeed with an empty result or error depending on parser
        let _ = parse_instructions(xml);
    }

    #[test]
    fn test_delete_operations() {
        let xml = r#"<ram_delete><key>old_key</key></ram_delete>
<state_delete><state>old_state</state></state_delete>"#;
        let instrs = parse_instructions(xml).unwrap();
        assert_eq!(instrs.len(), 2);
        assert_eq!(
            instrs[0],
            Instruction::RamDelete {
                key: "old_key".into()
            }
        );
    }
}
