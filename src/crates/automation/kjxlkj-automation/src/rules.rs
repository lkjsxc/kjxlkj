// Automation rule validation per /docs/spec/domain/automation.md
use kjxlkj_domain::types::LibrarianProviderKind;

/// Validate an automation rule's action JSON.
/// Librarian rules MUST validate provider mode.
pub fn validate_rule_action(action: &serde_json::Value) -> Result<(), String> {
    let kind = action.get("kind").and_then(|v| v.as_str());
    if kind == Some("librarian_structure") {
        // Validate provider
        let provider = action.get("provider").ok_or("missing provider config")?;
        let pk = provider.get("provider_kind").and_then(|v| v.as_str())
            .ok_or("missing provider_kind")?;
        match pk {
            "openrouter" | "lmstudio" => {}
            other => return Err(format!("unknown provider: {other}")),
        }
        // Validate protocol
        let protocol = action.get("protocol").and_then(|v| v.as_str());
        if protocol != Some("xml_attrless") {
            return Err("protocol must be xml_attrless".into());
        }
        // Validate plan exists
        action.get("plan").ok_or("missing structuring plan")?;
    }
    Ok(())
}

/// Parse provider kind from string.
pub fn parse_provider_kind(s: &str) -> Option<LibrarianProviderKind> {
    match s {
        "openrouter" => Some(LibrarianProviderKind::Openrouter),
        "lmstudio" => Some(LibrarianProviderKind::Lmstudio),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_librarian_rule() {
        let action = serde_json::json!({
            "kind": "librarian_structure",
            "provider": { "provider_kind": "openrouter", "model": "test" },
            "protocol": "xml_attrless",
            "plan": { "goal": "restructure" }
        });
        assert!(validate_rule_action(&action).is_ok());
    }

    #[test]
    fn test_unknown_provider_rejected() {
        let action = serde_json::json!({
            "kind": "librarian_structure",
            "provider": { "provider_kind": "unknown" },
            "protocol": "xml_attrless",
            "plan": {}
        });
        assert!(validate_rule_action(&action).is_err());
    }
}
