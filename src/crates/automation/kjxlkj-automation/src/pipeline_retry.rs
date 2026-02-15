//! Pipeline retry logic per /docs/spec/api/librarian-xml.md.
//! Split from run_pipeline.rs per 200-line policy.

use crate::prompt_loader::{PromptPack, StagePrompt, render_template};
use crate::provider::{ProviderConfig, ProviderFailure, chat_with_fallback};
use crate::xml_parser::{LibrarianResponse, parse_response};
use std::collections::HashMap;

/// Maximum repair retries per spec.
pub const MAX_REPAIR_RETRIES: usize = 2;

/// Call a single pipeline stage via provider.
pub async fn call_stage(
    config: &ProviderConfig,
    prompt: &StagePrompt,
    user_text: &str,
) -> Result<String, ProviderFailure> {
    chat_with_fallback(config, &prompt.system_prompt, user_text).await
}

/// Parse response with bounded repair retries.
pub async fn parse_with_retry(
    raw: &str,
    max_ops: usize,
    strict: bool,
    pack: &PromptPack,
    config: &ProviderConfig,
) -> Result<LibrarianResponse, (String, String)> {
    match parse_response(raw, max_ops, strict) {
        Ok(parsed) => return Ok(parsed),
        Err(first_err) => {
            tracing::warn!(error = %first_err, "parse failed, attempting repair");
            let repair_prompt = match pack.stages.get("validate_repair") {
                Some(p) => p,
                None => return Err((first_err.to_string(), raw.to_string())),
            };

            let mut last_err = first_err.to_string();
            let mut last_raw = raw.to_string();

            for attempt in 0..MAX_REPAIR_RETRIES {
                let bindings = build_bindings(&[
                    ("original_response", &last_raw),
                    ("parse_diagnostics", &last_err),
                    ("constraints", ""),
                ]);
                let repair_text = render_template(
                    &repair_prompt.user_prompt_template,
                    &bindings,
                );
                match call_stage(config, repair_prompt, &repair_text).await {
                    Ok(repaired) => {
                        match parse_response(&repaired, max_ops, strict) {
                            Ok(parsed) => {
                                tracing::info!(attempt, "repair succeeded");
                                return Ok(parsed);
                            }
                            Err(e) => {
                                last_err = e.to_string();
                                last_raw = repaired;
                            }
                        }
                    }
                    Err(e) => {
                        last_err = format!("repair call failed: {e}");
                    }
                }
            }
            Err((last_err, last_raw))
        }
    }
}

/// Build template bindings from key-value pairs.
pub fn build_bindings(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}
