//! kjxlkj-agent loop implementation

use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::sync::RwLock;
use uuid::Uuid;

use kjxlkj_domain::{AgentMode, AutomationRule, AgentRun, RunStatus};

/// Agent state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentState {
    Planning,
    Executing,
    Evaluating,
    Idle,
    RecordOrganizing,
    Paging,
}

impl Default for AgentState {
    fn default() -> Self {
        Self::Idle
    }
}

/// KV store for agent memory (persists across loops)
#[derive(Debug, Clone, Default)]
pub struct KvStore {
    data: Arc<RwLock<HashMap<String, Value>>>,
    path: Option<String>,
}

impl KvStore {
    pub fn new(path: Option<String>) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            path,
        }
    }

    pub async fn add(&self, key: &str, value: Value) {
        let mut data = self.data.write().await;
        data.insert(key.to_string(), value);
        self.save().await;
    }

    pub async fn delete(&self, key: &str) {
        let mut data = self.data.write().await;
        data.remove(key);
        self.save().await;
    }

    pub async fn get(&self, key: &str) -> Option<Value> {
        let data = self.data.read().await;
        data.get(key).cloned()
    }

    pub async fn get_string(&self, key: &str) -> Option<String> {
        self.get(key).await.and_then(|v| v.as_str().map(String::from))
    }

    async fn save(&self) {
        if let Some(ref path) = self.path {
            // In production, persist to JSON file
            let data = self.data.read().await;
            let json = serde_json::to_string_pretty(&*data).unwrap_or_default();
            let _ = tokio::fs::write(path, json).await;
        }
    }

    pub async fn load(&self) {
        if let Some(ref path) = self.path {
            if let Ok(content) = tokio::fs::read_to_string(path).await {
                if let Ok(data) = serde_json::from_str::<HashMap<String, Value>>(&content) {
                    let mut store = self.data.write().await;
                    *store = data;
                }
            }
        }
    }
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    pub mode: AgentMode,
    pub prompt_path: String,
    pub memory_store_path: String,
    pub retain_full_conversation_logs: bool,
    pub loop_delay_ms: u64,
    pub idle_delay_ms: u64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            name: "kjxlkj-agent".to_string(),
            mode: AgentMode::Yolo,
            prompt_path: "./data/agent-prompt.json".to_string(),
            memory_store_path: "./data/agent-kv-store.json".to_string(),
            retain_full_conversation_logs: false,
            loop_delay_ms: 1500,
            idle_delay_ms: 5000,
        }
    }
}

/// Agent loop executor
#[derive(Debug, Clone)]
pub struct AgentLoop {
    pub state: AgentState,
    pub ram: KvStore,
    pub config: AgentConfig,
    pub run: Option<AgentRun>,
}

impl AgentLoop {
    pub fn new(config: AgentConfig) -> Self {
        let ram = KvStore::new(Some(config.memory_store_path.clone()));
        Self {
            state: AgentState::Idle,
            ram,
            config,
            run: None,
        }
    }

    pub async fn initialize(&mut self) {
        self.ram.load().await;
        
        // Initialize default state if not present
        if self.ram.get("state").await.is_none() {
            self.ram.add("state", json!("idle")).await;
        }
    }

    pub async fn run_once(&mut self) -> Result<(), AgentError> {
        // Load state
        let state_str = self.ram.get_string("state").await.unwrap_or_else(|| "idle".to_string());
        self.state = match state_str.as_str() {
            "planning" => AgentState::Planning,
            "executing" => AgentState::Executing,
            "evaluating" => AgentState::Evaluating,
            "record_organizing" => AgentState::RecordOrganizing,
            "paging" => AgentState::Paging,
            _ => AgentState::Idle,
        };

        // Load prompt segments based on state
        let segments = self.load_prompt_segments().await?;

        // Construct prompt
        let prompt = self.construct_prompt(&segments).await;

        // Call LLM (stub for now)
        let response = self.call_llm(&prompt).await?;

        // Parse XML instructions
        let instructions = self.parse_xml(&response).await?;

        // Execute instructions
        for instr in instructions {
            self.execute_instruction(instr).await?;
        }

        // Save state
        self.ram.add("think_log", json!(response)).await;

        // Wait
        let delay = if self.state == AgentState::Idle {
            self.config.idle_delay_ms
        } else {
            self.config.loop_delay_ms
        };
        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;

        Ok(())
    }

    async fn load_prompt_segments(&self) -> Result<Vec<String>, AgentError> {
        // Load from agent-prompt.json
        let content = tokio::fs::read_to_string(&self.config.prompt_path)
            .await
            .map_err(|e| AgentError::PromptLoadFailed(e.to_string()))?;

        let prompt: Value = serde_json::from_str(&content)
            .map_err(|e| AgentError::PromptJsonInvalid(e.to_string()))?;

        let segments = prompt
            .get("segments")
            .and_then(|s| s.as_array())
            .ok_or_else(|| AgentError::PromptSchemaInvalid("Missing segments".into()))?;

        let state_str = format!("{:?}", self.state).to_lowercase();
        
        let filtered: Vec<String> = segments
            .iter()
            .filter_map(|seg| {
                let condition = seg.get("condition")?.as_str()?;
                let prompt_text = seg.get("prompt")?.as_str()?;
                
                // Include if condition matches state or is "default"
                if condition == "default" || condition == &state_str {
                    Some(prompt_text.to_string())
                } else {
                    None
                }
            })
            .collect();

        Ok(filtered)
    }

    async fn construct_prompt(&self, segments: &[String]) -> String {
        let mut prompt = String::new();
        
        // Add context from RAM
        if let Some(user_request) = self.ram.get_string("user_request").await {
            prompt.push_str(&format!("User request: {}\n\n", user_request));
        }

        // Add segments
        for segment in segments {
            prompt.push_str(segment);
            prompt.push_str("\n\n");
        }

        // Add current RAM state
        prompt.push_str("Current RAM state:\n");
        let data = self.ram.data.read().await;
        for (key, value) in data.iter() {
            prompt.push_str(&format!("- {}: {}\n", key, value));
        }

        prompt
    }

    async fn call_llm(&self, _prompt: &str) -> Result<String, AgentError> {
        // Stub implementation - in production, call LLM provider
        Ok("<ram_add><key>think_log</key><value>Stub response</value></ram_add>".to_string())
    }

    async fn parse_xml(&self, xml: &str) -> Result<Vec<Instruction>, AgentError> {
        // Simplified XML parser (attrless)
        // In production, use proper XML parsing library
        let mut instructions = Vec::new();
        
        // Parse ram_add instructions
        for cap in regex::Regex::new(r"<ram_add>(.*?)</ram_add>")
            .unwrap()
            .captures_iter(xml)
        {
            if let Some(content) = cap.get(1) {
                let inner = content.as_str();
                if let (Some(key), Some(value)) = (
                    extract_xml_tag(inner, "key"),
                    extract_xml_tag(inner, "value"),
                ) {
                    instructions.push(Instruction::RamAdd(RamAdd { key, value }));
                }
            }
        }

        Ok(instructions)
    }

    async fn execute_instruction(&mut self, instr: Instruction) -> Result<(), AgentError> {
        match instr {
            Instruction::RamAdd(add) => {
                self.ram.add(&add.key, json!(add.value)).await;
            }
            Instruction::RamDelete(delete) => {
                self.ram.delete(&delete.key).await;
            }
            Instruction::StateAdd(add) => {
                self.ram.add("state", json!(add.state)).await;
            }
            Instruction::StateDelete(_) => {
                // Handle state delete
            }
            Instruction::RecordAdd(_) => {
                // Handle record add
            }
            Instruction::RecordUpdate(_) => {
                // Handle record update
            }
            Instruction::RecordSearch(_) => {
                // Handle record search
            }
            Instruction::RecordIssue(_) => {
                // Handle record issue
            }
        }
        Ok(())
    }
}

fn extract_xml_tag(content: &str, tag: &str) -> Option<String> {
    let pattern = format!("<{}>(.*?)</{}>", tag, tag);
    regex::Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(content))
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}

/// XML instruction types
#[derive(Debug, Clone)]
pub enum Instruction {
    StateAdd(StateAdd),
    StateDelete(StateDelete),
    RamAdd(RamAdd),
    RamDelete(RamDelete),
    RecordAdd(RecordAdd),
    RecordUpdate(RecordUpdate),
    RecordSearch(RecordSearch),
    RecordIssue(RecordIssue),
}

#[derive(Debug, Clone)]
pub struct StateAdd { pub state: String }
#[derive(Debug, Clone)]
pub struct StateDelete { pub state: String }
#[derive(Debug, Clone)]
pub struct RamAdd { pub key: String, pub value: String }
#[derive(Debug, Clone)]
pub struct RamDelete { pub key: String }
#[derive(Debug, Clone)]
pub struct RecordAdd { pub keywords: String, pub value: String }
#[derive(Debug, Clone)]
pub struct RecordUpdate { pub key: String, pub value: String }
#[derive(Debug, Clone)]
pub struct RecordSearch { pub query: String }
#[derive(Debug, Clone)]
pub struct RecordIssue { pub key: String, pub value: String, pub metadata: String }

/// Agent error types
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Prompt load failed: {0}")]
    PromptLoadFailed(String),

    #[error("Prompt JSON invalid: {0}")]
    PromptJsonInvalid(String),

    #[error("Prompt schema invalid: {0}")]
    PromptSchemaInvalid(String),

    #[error("Prompt segment empty at index {0}")]
    PromptSegmentEmpty(usize),

    #[error("LLM error: {0}")]
    LlmError(String),

    #[error("XML parse error: {0}")]
    XmlParseError(String),

    #[error("Scope violation: {0}")]
    ScopeViolation(String),

    #[error("Version conflict")]
    VersionConflict,

    #[error("Rate limited")]
    RateLimited,
}
