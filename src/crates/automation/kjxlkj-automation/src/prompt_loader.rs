//! JSON prompt-pack loader per /docs/spec/technical/librarian-prompts/README.md.
//! Loads stage definitions from manifest.json and validates schema keys.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

/// Manifest per /docs/spec/technical/librarian-prompts/manifest.json.
#[derive(Debug, Deserialize)]
pub struct PromptManifest {
    pub pack_version: String,
    pub protocol: String,
    pub stages: HashMap<String, String>,
    pub hash_policy: HashPolicy,
}

#[derive(Debug, Deserialize)]
pub struct HashPolicy {
    pub algorithm: String,
    pub include_files: Vec<String>,
}

/// Stage prompt definition per librarian-prompts README.md.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StagePrompt {
    pub stage: String,
    pub model_profile: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: String,
    pub user_prompt_template: String,
    pub input_bindings: Vec<String>,
    pub output_contract: serde_json::Value,
}

/// Required keys per spec.
const REQUIRED_KEYS: &[&str] = &[
    "stage",
    "model_profile",
    "temperature",
    "max_tokens",
    "system_prompt",
    "user_prompt_template",
    "input_bindings",
    "output_contract",
];

/// Loaded prompt pack with all stages and pack-level hash.
#[derive(Debug, Clone)]
pub struct PromptPack {
    pub version: String,
    pub protocol: String,
    pub stages: HashMap<String, StagePrompt>,
    pub pack_hash: String,
}

/// Load manifest and all stage files from a directory.
pub fn load_prompt_pack(dir: &Path) -> Result<PromptPack, String> {
    let manifest_path = dir.join("manifest.json");
    let manifest_text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| format!("cannot read manifest.json: {e}"))?;
    let manifest: PromptManifest = serde_json::from_str(&manifest_text)
        .map_err(|e| format!("invalid manifest.json: {e}"))?;

    let mut stages = HashMap::new();
    let mut hasher = Sha256::new();

    for (stage_name, filename) in &manifest.stages {
        let stage_path = dir.join(filename);
        let text = std::fs::read_to_string(&stage_path)
            .map_err(|e| format!("cannot read {filename}: {e}"))?;
        validate_stage_keys(&text, filename)?;
        let prompt: StagePrompt = serde_json::from_str(&text)
            .map_err(|e| format!("invalid {filename}: {e}"))?;
        if prompt.stage != *stage_name {
            return Err(format!(
                "{filename}: stage field '{}' != manifest key '{stage_name}'",
                prompt.stage
            ));
        }
        stages.insert(stage_name.clone(), prompt);
    }

    // Compute pack hash from included files in order.
    for filename in &manifest.hash_policy.include_files {
        let text = std::fs::read_to_string(dir.join(filename))
            .map_err(|e| format!("hash file {filename}: {e}"))?;
        hasher.update(text.as_bytes());
    }
    let pack_hash = hex::encode(hasher.finalize());

    Ok(PromptPack {
        version: manifest.pack_version,
        protocol: manifest.protocol,
        stages,
        pack_hash,
    })
}

/// Validate that a stage JSON has all required keys.
fn validate_stage_keys(text: &str, filename: &str) -> Result<(), String> {
    let obj: serde_json::Value = serde_json::from_str(text)
        .map_err(|e| format!("{filename}: invalid JSON: {e}"))?;
    let map = obj
        .as_object()
        .ok_or_else(|| format!("{filename}: root must be object"))?;
    for key in REQUIRED_KEYS {
        if !map.contains_key(*key) {
            return Err(format!("{filename}: missing required key '{key}'"));
        }
    }
    Ok(())
}

/// Render a user prompt template with bindings.
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
