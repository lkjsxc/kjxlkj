use std::path::Path;

/// Load and validate config JSON from file.
pub fn load_config(path: &Path) -> Result<serde_json::Value, String> {
    let data = std::fs::read_to_string(path)
        .map_err(|e| format!("failed to read {path:?}: {e}"))?;

    let config: serde_json::Value = serde_json::from_str(&data)
        .map_err(|e| format!("invalid config JSON: {e}"))?;

    // Validate required sections
    let required = ["logging", "server", "database", "agent", "search"];
    for section in required {
        if config.get(section).is_none() {
            return Err(format!("missing required config section: {section}"));
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_data() {
        let path = Path::new("../../../data/config.json");
        if path.exists() {
            let config = load_config(path).unwrap();
            assert!(config.get("logging").is_some());
            assert!(config.get("server").is_some());
            assert!(config.get("database").is_some());
        }
    }
}
