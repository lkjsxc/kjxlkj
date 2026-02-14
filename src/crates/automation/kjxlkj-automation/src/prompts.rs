// JSON prompt-pack loading per /docs/spec/technical/librarian-prompts/README.md
// Stage definitions loaded only through manifest.json.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manifest per /docs/spec/technical/librarian-prompts/manifest.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptManifest {
    pub pack_version: String,
    pub protocol: String,
    pub stages: HashMap<String, String>,
    pub hash_policy: HashPolicy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashPolicy {
    pub algorithm: String,
    pub include_files: Vec<String>,
}

/// Stage prompt definition per individual stage JSON files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagePrompt {
    pub stage: String,
    pub model_profile: String,
    pub temperature: f64,
    pub max_tokens: u32,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub input_bindings: Vec<String>,
    pub output_contract: OutputContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputContract {
    pub format: String,
    #[serde(default)]
    pub required_keys: Vec<String>,
    #[serde(default)]
    pub required_root_tag: Option<String>,
    #[serde(default)]
    pub retry_limit: Option<u32>,
}

/// Loaded prompt pack with all stage definitions.
#[derive(Debug, Clone)]
pub struct PromptPack {
    pub manifest: PromptManifest,
    pub stages: HashMap<String, StagePrompt>,
}

/// Load embedded prompt pack from compile-time strings.
/// In production, these would be loaded from filesystem.
pub fn load_embedded_pack() -> PromptPack {
    let manifest: PromptManifest =
        serde_json::from_str(MANIFEST_JSON).expect("valid manifest");
    let mut stages = HashMap::new();
    for (stage_name, _filename) in &manifest.stages {
        let json = match stage_name.as_str() {
            "ingest" => STAGE_INGEST_JSON,
            "plan" => STAGE_PLAN_JSON,
            "propose" => STAGE_PROPOSE_JSON,
            "validate_repair" => STAGE_VALIDATE_REPAIR_JSON,
            _ => continue,
        };
        let prompt: StagePrompt =
            serde_json::from_str(json).expect("valid stage prompt");
        stages.insert(stage_name.clone(), prompt);
    }
    PromptPack { manifest, stages }
}

/// Render a user prompt template by substituting {{binding}} placeholders.
pub fn render_template(
    template: &str,
    bindings: &HashMap<String, String>,
) -> String {
    let mut result = template.to_string();
    for (key, value) in bindings {
        let placeholder = format!("{{{{{key}}}}}");
        result = result.replace(&placeholder, value);
    }
    result
}

// Embedded prompt files from docs/spec/technical/librarian-prompts/
const MANIFEST_JSON: &str = include_str!(
    "../../../../../docs/spec/technical/librarian-prompts/manifest.json"
);
const STAGE_INGEST_JSON: &str = include_str!(
    "../../../../../docs/spec/technical/librarian-prompts/stage-ingest.json"
);
const STAGE_PLAN_JSON: &str = include_str!(
    "../../../../../docs/spec/technical/librarian-prompts/stage-plan.json"
);
const STAGE_PROPOSE_JSON: &str = include_str!(
    "../../../../../docs/spec/technical/librarian-prompts/stage-propose.json"
);
const STAGE_VALIDATE_REPAIR_JSON: &str = include_str!(
    "../../../../../docs/spec/technical/librarian-prompts/stage-validate-repair.json"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_pack() {
        let pack = load_embedded_pack();
        assert_eq!(pack.manifest.protocol, "xml_attrless");
        assert_eq!(pack.stages.len(), 4);
        assert!(pack.stages.contains_key("ingest"));
        assert!(pack.stages.contains_key("plan"));
        assert!(pack.stages.contains_key("propose"));
        assert!(pack.stages.contains_key("validate_repair"));
    }

    #[test]
    fn render_bindings() {
        let tmpl = "Using {{source_bundle}} and {{taxonomy}}";
        let mut bindings = HashMap::new();
        bindings.insert("source_bundle".into(), "SOURCES".into());
        bindings.insert("taxonomy".into(), "TOPICS".into());
        let rendered = render_template(tmpl, &bindings);
        assert_eq!(rendered, "Using SOURCES and TOPICS");
    }

    #[test]
    fn stage_ingest_bindings() {
        let pack = load_embedded_pack();
        let ingest = &pack.stages["ingest"];
        assert!(ingest.input_bindings.contains(&"source_bundle".into()));
        assert!(ingest.input_bindings.contains(&"scope".into()));
    }

    #[test]
    fn validate_repair_retry_limit() {
        let pack = load_embedded_pack();
        let vr = &pack.stages["validate_repair"];
        assert_eq!(vr.output_contract.retry_limit, Some(2));
    }
}
