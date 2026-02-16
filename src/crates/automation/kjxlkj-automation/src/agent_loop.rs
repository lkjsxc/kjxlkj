use crate::prompt::AgentPrompt;
use crate::xml_parser;
use crate::kv_memory::KvMemory;
use kjxlkj_db::repo_automation;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::info;

/// Agent loop configuration.
pub struct AgentConfig {
    pub prompt: AgentPrompt,
    pub prompt_hash: String,
    pub provider_base_url: String,
    pub model: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub timeout_ms: u64,
    pub mode: String,
    pub loop_delay_ms: u64,
    pub idle_delay_ms: u64,
}

/// Run one iteration of the agent loop.
pub async fn run_iteration(
    pool: &PgPool,
    config: &AgentConfig,
    workspace_id: Uuid,
    run_id: Uuid,
    _rule_id: Uuid,
) -> Result<(), AgentError> {
    let kv = KvMemory::new(&config.prompt.agent_name, workspace_id);

    // 1. Build prompt from segments and current state
    let ram_entries = kv.list_all(pool).await.map_err(AgentError::Db)?;
    let prompt_text = build_prompt(&config.prompt, &ram_entries);

    // 2. Call LLM
    let response = call_llm(
        &config.provider_base_url,
        &config.model,
        &prompt_text,
        config.max_tokens,
        config.temperature,
        config.timeout_ms,
    )
    .await?;

    // 3. Parse instructions
    let instructions = xml_parser::parse_instructions(&response)
        .map_err(|e| AgentError::Parse(format!("{e}")))?;

    info!(
        "agent iteration: run={run_id}, instructions={}",
        instructions.len()
    );

    // 4. Execute instructions
    for instr in &instructions {
        match instr {
            xml_parser::Instruction::RamAdd { key, value } => {
                kv.set(pool, key, &serde_json::Value::String(value.clone()))
                    .await
                    .map_err(AgentError::Db)?;
            }
            xml_parser::Instruction::RamDelete { key } => {
                kv.delete(pool, key).await.map_err(AgentError::Db)?;
            }
            xml_parser::Instruction::StateAdd { state } => {
                kv.set(pool, "current_state", &serde_json::Value::String(state.clone()))
                    .await
                    .map_err(AgentError::Db)?;
            }
            xml_parser::Instruction::StateDelete { .. } => {
                kv.delete(pool, "current_state")
                    .await
                    .map_err(AgentError::Db)?;
            }
            _ => {
                // Record operations handled by caller (YOLO or reviewed)
                info!("record instruction: {instr:?}");
            }
        }
    }

    // 5. Update run status
    let result = serde_json::json!({
        "instruction_count": instructions.len(),
        "prompt_hash": config.prompt_hash,
    });
    repo_automation::update_run_status(pool, run_id, "succeeded", Some(&result))
        .await
        .map_err(AgentError::Db)?;

    Ok(())
}

fn build_prompt(
    config: &AgentPrompt,
    ram: &[(String, serde_json::Value)],
) -> String {
    let mut parts = Vec::new();

    // Default segments always included
    for seg in &config.segments {
        if seg.condition == "default" {
            parts.push(seg.prompt.clone());
        }
    }

    // Current state segments
    let current_state = ram.iter()
        .find(|(k, _)| k == "current_state")
        .and_then(|(_, v)| v.as_str().map(String::from));

    if let Some(ref state) = current_state {
        for seg in &config.segments {
            if seg.condition == *state {
                parts.push(seg.prompt.clone());
            }
        }
    }

    // Append RAM context
    if !ram.is_empty() {
        parts.push("Current RAM:".into());
        for (k, v) in ram {
            parts.push(format!("  {k}: {v}"));
        }
    }

    parts.join("\n")
}

async fn call_llm(
    base_url: &str,
    model: &str,
    prompt: &str,
    max_tokens: u32,
    temperature: f32,
    timeout_ms: u64,
) -> Result<String, AgentError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_millis(timeout_ms))
        .build()
        .map_err(|e| AgentError::Provider(format!("client build: {e}")))?;

    let url = format!("{base_url}/chat/completions");
    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": prompt}],
        "max_tokens": max_tokens,
        "temperature": temperature,
    });

    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| AgentError::Provider(format!("LLM request: {e}")))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(AgentError::Provider(format!("LLM {status}: {text}")));
    }

    let json: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| AgentError::Provider(format!("LLM parse: {e}")))?;

    let content = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(content)
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("db: {0}")]
    Db(sqlx::Error),
    #[error("parse: {0}")]
    Parse(String),
    #[error("provider: {0}")]
    Provider(String),
}
