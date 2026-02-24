# kjxlkj-agent Technical Contract

**Back:** [Technical Root](/docs/spec/technical/README.md)

---

## Canonical Identity

| Requirement | Value |
|-------------|-------|
| **Agent name** | `kjxlkj-agent` (MUST) |
| **Configuration** | Loaded from JSON (`data/config.json` + `data/agent-prompt.json`) |
| **Prompt text** | Fully defined in JSON files |
| **Memory model** | KV store persisted across loops |
| **Conversation logs** | Disabled by default |

---

## Source Reference Policy

Agent behavior requirements are derived from:
1. Agent sections in prior implementation notes
2. Normalized into this repository's canonical docs

---

## Loop Model

### Iterative Loop Architecture

```rust
pub struct AgentLoop {
    state: AgentState,
    ram: KvStore,           // Persisted across loops
    prompt_loader: PromptLoader,
    parser: XmlInstructionParser,
    provider: Box<dyn LlmProvider>,
    config: AgentConfig,
}

impl AgentLoop {
    pub async fn run_once(&mut self) -> Result<(), AgentError> {
        // 1. Load state from RAM
        let state = self.ram.get("state").unwrap_or("planning");
        
        // 2. Load prompt segments for current state
        let segments = self.prompt_loader.load_segments(&state);
        
        // 3. Construct full prompt with context
        let prompt = self.construct_prompt(&segments);
        
        // 4. Call LLM provider
        let response = self.provider.complete(&prompt).await?;
        
        // 5. Parse XML instructions
        let instructions = self.parser.parse(&response)?;
        
        // 6. Execute instructions
        for instr in instructions {
            self.execute(instr).await?;
        }
        
        // 7. Save state to RAM
        self.ram.set("think_log", &response.reasoning);
        
        // 8. Wait (loop_delay_ms or idle_delay_ms)
        self.wait().await;
        
        Ok(())
    }
}
```

### Loop States

| State | Description | Transition |
|-------|-------------|------------|
| `planning` | Breaking down tasks into steps | → executing |
| `executing` | Performing record mutations | → evaluating |
| `evaluating` | Assessing outcomes | → planning |
| `idle` | Waiting for triggers | → planning (on trigger) |

---

## Memory Model

### RAM (KV Store) Contract

**Critical:** Agent memory MUST rely on a mutable key-value store carried across loops.

```rust
pub struct KvStore {
    path: PathBuf,
    data: HashMap<String, JsonValue>,
}

impl KvStore {
    pub fn load(path: &str) -> Result<Self> {
        // Load from JSON file
    }
    
    pub fn save(&self) -> Result<()> {
        // Persist to JSON file
    }
    
    pub fn add(&mut self, key: &str, value: JsonValue) {
        self.data.insert(key.to_string(), value);
        self.save()?;
    }
    
    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.save()?;
    }
    
    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.data.get(key)
    }
}
```

### Required RAM Keys

| Key | Type | Purpose |
|-----|------|---------|
| `think_log` | String | Reasoning trace for current loop |
| `plan` | String | Current step plan |
| `steps` | Array | Remaining steps |
| `context` | Object | Working context |
| `state` | String | Current loop state |

### Memory Operations

| Operation | XML Tag | Description |
|-----------|---------|-------------|
| Add/Update | `<ram_add>` | `ram_add(key, value)` |
| Delete | `<ram_delete>` | `ram_delete(key)` |

---

## Prompt JSON Contract

### Schema Validation

```rust
pub struct AgentPrompt {
    pub agent_name: String,        // MUST equal "kjxlkj-agent"
    pub version: String,           // Date or semver
    pub default_mode: String,      // "reviewed" or "yolo"
    pub protocol: String,          // "xml_attrless"
    pub allowed_tags: Vec<String>, // Allowed XML tags
    pub segments: Vec<PromptSegment>,
}

pub struct PromptSegment {
    pub condition: String,  // "default" or state name
    pub prompt: String,     // Prompt text
}
```

### Validation Rules

| Rule | Behavior |
|------|----------|
| Invalid JSON | Hard-fail startup with `PROMPT_JSON_INVALID` |
| Missing `agent_name` | Fail with `PROMPT_SCHEMA_INVALID` |
| Missing `segments` | Fail with `PROMPT_SCHEMA_INVALID` |
| Empty segment prompt | Fail with `PROMPT_SEGMENT_EMPTY` |
| Unknown keys | Log warning, ignore |

### Loading

```rust
pub fn load_prompt(path: &str) -> Result<AgentPrompt> {
    let content = fs::read_to_string(path)?;
    let prompt: AgentPrompt = serde_json::from_str(&content)?;
    
    // Validate agent_name
    if prompt.agent_name != "kjxlkj-agent" {
        return Err(AgentError::PromptInvalid(
            "agent_name must be 'kjxlkj-agent'".into()
        ));
    }
    
    // Validate segments non-empty
    if prompt.segments.is_empty() {
        return Err(AgentError::PromptInvalid(
            "segments must not be empty".into()
        ));
    }
    
    Ok(prompt)
}
```

---

## YOLO Mode Contract

### Direct Mutation Rules

| Rule | Implementation |
|------|----------------|
| Direct note mutations | Agent can call `record_add`, `record_update` directly |
| Permission scope | Workspace ID checked on every operation |
| Optimistic version | Version checks applied to updates |
| Cross-workspace | Rejected with `CROSS_WORKSPACE_REJECTED` |

### Scope Guardrails

```rust
pub struct AgentScope {
    pub workspace_id: Uuid,
    pub allowed_note_kinds: Vec<String>,
    pub cross_workspace_writes: bool,
    pub max_notes_per_loop: usize,
    pub max_edits_per_loop: usize,
}

impl AgentScope {
    pub fn validate_note_create(&self, note: &Note) -> Result<()> {
        if note.workspace_id != self.workspace_id {
            return Err(AgentError::ScopeViolation(
                "Cross-workspace write rejected".into()
            ));
        }
        if !self.allowed_note_kinds.contains(&note.note_kind) {
            return Err(AgentError::ScopeViolation(
                "Note kind not allowed".into()
            ));
        }
        Ok(())
    }
}
```

---

## Instruction Protocol

### XML Parser

```rust
pub struct XmlInstructionParser;

impl XmlInstructionParser {
    pub fn parse(&self, xml: &str) -> Result<Vec<Instruction>> {
        // Parse XML without attributes
        // Return list of instructions
    }
    
    fn parse_state_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <state>child</state>
    }
    
    fn parse_ram_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <key> and <value> children
    }
    
    fn parse_record_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <keywords> and <value> children
    }
}
```

### Allowed Instructions

| Tag | Struct | Fields |
|-----|--------|--------|
| `state_add` | `StateAdd` | `state: String` |
| `state_delete` | `StateDelete` | `state: String` |
| `ram_add` | `RamAdd` | `key: String`, `value: String` |
| `ram_delete` | `RamDelete` | `key: String` |
| `record_add` | `RecordAdd` | `keywords: String`, `value: String` |
| `record_issue` | `RecordIssue` | `key: String`, `value: String`, `metadata: Object` |
| `record_update` | `RecordUpdate` | `key: String`, `value: String` |
| `record_search` | `RecordSearch` | `query: String` or `ids: Vec<Uuid>` |

---

## Determinism and Safety

### Bounded Retries

```rust
pub struct RetryConfig {
    pub max_retries: u32,      // Default: 3
    pub base_delay_ms: u64,    // Default: 100
    pub max_delay_ms: u64,     // Default: 5000
}

impl RetryConfig {
    pub fn delay(&self, attempt: u32) -> Duration {
        let delay = self.base_delay_ms * 2u64.pow(attempt);
        Duration::from_millis(delay.min(self.max_delay_ms))
    }
}
```

### Error Codes

| Code | Description |
|------|-------------|
| `PROMPT_JSON_INVALID` | JSON parse failure |
| `PROMPT_SCHEMA_INVALID` | Schema validation failure |
| `XML_PARSE_ERROR` | XML instruction parse failure |
| `LLM_TIMEOUT` | Provider timeout |
| `SCOPE_VIOLATION` | Cross-workspace or kind violation |
| `VERSION_CONFLICT` | Optimistic concurrency failure |

### Audit Metadata

```rust
pub struct AgentRunAudit {
    pub prompt_hash: String,      // SHA-256 of prompt
    pub parser_version: String,   // Parser version
    pub loop_count: u32,          // Iterations executed
    pub operation_count: u32,     // Instructions executed
    pub error_code: Option<String>, // If failed
    pub started_at: Timestamp,
    pub completed_at: Timestamp,
}
```

---

## Provider Adapters

### LLM Provider Trait

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<LlmResponse>;
    fn provider_name(&self) -> &str;
    fn is_available(&self) -> bool;
}
```

### Supported Providers

| Provider | Kind | Base URL |
|----------|------|----------|
| LMStudio | `lmstudio` | `http://127.0.0.1:1234/v1` |
| OpenRouter | `openrouter` | `https://openrouter.ai/api/v1` |

### LMStudio Adapter

```rust
pub struct LmStudioProvider {
    base_url: String,
    model: String,
    timeout_ms: u64,
    max_tokens: u32,
    temperature: f32,
}

#[async_trait]
impl LlmProvider for LmStudioProvider {
    async fn complete(&self, prompt: &str) -> Result<LlmResponse> {
        // POST /v1/chat/completions
        // OpenAI-compatible API
    }
    
    fn provider_name(&self) -> &str {
        "lmstudio"
    }
    
    fn is_available(&self) -> bool {
        // Health check
    }
}
```

---

## Related

- [Domain automation](automation.md) — rule model
- [XML protocol](/docs/spec/api/librarian-xml.md) — instruction format
- [Agent prompt JSON](agent-prompt-json.md) — schema definition
- [Testing](testing.md) — acceptance IDs
