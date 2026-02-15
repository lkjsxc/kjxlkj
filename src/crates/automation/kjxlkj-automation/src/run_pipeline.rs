//! Librarian run pipeline per /docs/spec/technical/librarian-agent.md.
//! Orchestrates: ingest -> plan -> propose -> validate -> apply -> index.

use crate::pipeline_retry::{build_bindings, call_stage, parse_with_retry};
use crate::prompt_loader::{PromptPack, render_template};
use crate::provider::ProviderConfig;
use crate::safety_policy::{SafetyPolicy, SafetyRejection, evaluate_operations};
use crate::xml_parser::LibrarianResponse;
use sha2::{Digest, Sha256};

/// Pipeline result returned to caller.
#[derive(Debug)]
pub struct PipelineResult {
    pub prompt_hash: String,
    pub raw_response: String,
    pub parsed: Option<LibrarianResponse>,
    pub accepted_count: usize,
    pub rejected_count: usize,
    pub rejections: Vec<SafetyRejection>,
    pub warnings: Vec<String>,
    pub error: Option<String>,
}

/// Execute the librarian pipeline for a given rule/run.
pub async fn execute_pipeline(
    pack: &PromptPack,
    provider_config: &ProviderConfig,
    source_bundle: &str,
    goal: &str,
    scope: &str,
    taxonomy: &str,
    constraints: &str,
    safety: &SafetyPolicy,
) -> PipelineResult {
    // Stage 1: Ingest
    let ingest_prompt = match pack.stages.get("ingest") {
        Some(p) => p,
        None => return error_result("missing ingest stage prompt"),
    };
    let ingest_bindings = build_bindings(&[
        ("source_bundle", source_bundle),
        ("scope", scope),
        ("constraints", constraints),
    ]);
    let ingest_text = render_template(
        &ingest_prompt.user_prompt_template,
        &ingest_bindings,
    );
    let normalized = match call_stage(
        provider_config, ingest_prompt, &ingest_text,
    ).await {
        Ok(t) => t,
        Err(e) => return error_result(&format!("ingest failed: {e}")),
    };

    // Stage 2: Plan
    let plan_prompt = match pack.stages.get("plan") {
        Some(p) => p,
        None => return error_result("missing plan stage prompt"),
    };
    let plan_bindings = build_bindings(&[
        ("normalized_sources", &normalized),
        ("taxonomy", taxonomy),
        ("constraints", constraints),
        ("goal", goal),
    ]);
    let plan_text = render_template(
        &plan_prompt.user_prompt_template,
        &plan_bindings,
    );
    let plan = match call_stage(
        provider_config, plan_prompt, &plan_text,
    ).await {
        Ok(t) => t,
        Err(e) => return error_result(&format!("plan failed: {e}")),
    };

    // Stage 3: Propose (xml_attrless output)
    let propose_prompt = match pack.stages.get("propose") {
        Some(p) => p,
        None => return error_result("missing propose stage prompt"),
    };
    let propose_bindings = build_bindings(&[
        ("plan", &plan),
        ("normalized_sources", &normalized),
        ("constraints", constraints),
        ("output_contract", "xml_attrless"),
    ]);
    let propose_text = render_template(
        &propose_prompt.user_prompt_template,
        &propose_bindings,
    );
    let prompt_hash = compute_prompt_hash(&[
        &ingest_text, &plan_text, &propose_text,
    ]);
    let raw = match call_stage(
        provider_config, propose_prompt, &propose_text,
    ).await {
        Ok(t) => t,
        Err(e) => return error_result_with_hash(
            &format!("propose failed: {e}"), &prompt_hash,
        ),
    };

    // Stage 4: Validate + bounded repair retries
    let strict_mode = constraints.contains("strict_mode");
    let parsed = match parse_with_retry(
        &raw, safety.max_operations, strict_mode, pack, provider_config,
    ).await {
        Ok(p) => p,
        Err((e, raw_text)) => {
            return PipelineResult {
                prompt_hash, raw_response: raw_text, parsed: None,
                accepted_count: 0, rejected_count: 0,
                rejections: vec![], warnings: vec![],
                error: Some(e),
            };
        }
    };

    // Stage 5: Safety evaluation
    let (accepted, rejected) = evaluate_operations(&parsed.operations, safety);

    PipelineResult {
        prompt_hash,
        raw_response: raw.clone(),
        parsed: Some(parsed),
        accepted_count: accepted.len(),
        rejected_count: rejected.len(),
        rejections: rejected,
        warnings: vec![],
        error: None,
    }
}

fn compute_prompt_hash(texts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for t in texts {
        hasher.update(t.as_bytes());
    }
    hex::encode(hasher.finalize())
}

fn error_result(msg: &str) -> PipelineResult {
    PipelineResult {
        prompt_hash: String::new(), raw_response: String::new(),
        parsed: None, accepted_count: 0, rejected_count: 0,
        rejections: vec![], warnings: vec![],
        error: Some(msg.to_string()),
    }
}

fn error_result_with_hash(msg: &str, hash: &str) -> PipelineResult {
    PipelineResult {
        prompt_hash: hash.to_string(), raw_response: String::new(),
        parsed: None, accepted_count: 0, rejected_count: 0,
        rejections: vec![], warnings: vec![],
        error: Some(msg.to_string()),
    }
}
