# kjxlkj-agent — Autonomous Note Automation

**Back:** [Technical Root](/docs/spec/technical/README.md)

---

## Canonical Identity

| Requirement | Value |
|-------------|-------|
| **Agent name** | `kjxlkj-agent` (MUST) |
| **Configuration** | Loaded from JSON (`data/config.json` + `data/agent-prompt.json`) |
| **Prompt text** | Fully defined in JSON files (no hardcoded prompts) |
| **Memory model** | KV store persisted across loops (no conversation logs by default) |
| **Operation mode** | YOLO mode enabled (direct note mutations within scope) |

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     kjxlkj-agent Loop                            │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │  Planning   │───>│  Executing  │───>│  Evaluating │───┐    │
│  └─────────────┘    └─────────────┘    └─────────────┘   │    │
│       ^                                                  │    │
│       │                                                  │    │
│       └──────────────────────────────────────────────────┘    │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              RAM (KV Store) — Persists Across Loops      │  │
│  │  - think_log: reasoning trace for current loop           │  │
│  │  - plan: current step plan                               │  │
│  │  - steps: remaining steps array                          │  │
│  │  - context: working context for task                     │  │
│  │  - state: current loop state (planning/executing/...)    │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │           Records (Persistent Notes)                     │  │
│  │  - record_add: create new note                           │  │
│  │  - record_update: modify existing note                   │  │
│  │  - record_search: find notes by query                    │  │
│  │  - record_issue: flag issues with metadata               │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │              LLM Provider (Pluggable)                    │  │
│  │  - LMStudio (local)                                      │  │
│  │  - Ollama (local)                                        │  │
│  │  - OpenRouter (cloud)                                    │  │
│  │  - OpenAI (cloud)                                        │  │
│  └──────────────────────────────────────────────────────────┘  │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

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
    scope: AgentScope,
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

        // 7. Save state to RAM (think_log, etc.)
        self.ram.set("think_log", &response.reasoning);

        // 8. Wait (loop_delay_ms or idle_delay_ms)
        self.wait().await;

        Ok(())
    }
}
```

### Loop States

| State | Description | Entry Condition | Exit Transition |
|-------|-------------|-----------------|-----------------|
| `planning` | Breaking down tasks into steps | Start, after evaluating | → executing |
| `executing` | Performing record mutations | After planning | → evaluating |
| `evaluating` | Assessing outcomes | After executing | → planning |
| `idle` | Waiting for triggers | No pending work | → planning (on trigger) |
| `record_organizing` | Deduplication, link maintenance | Manual or scheduled | → planning |
| `paging` | RAM size management | RAM exceeds threshold | → planning |

### Loop Cycle Details

```
┌─────────────────────────────────────────────────────────────────┐
│                     Single Loop Iteration                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Step 1: Load State                                             │
│  └─> Read current state from RAM                                │
│  └─> Read context, plan, steps from RAM                         │
│                                                                  │
│  Step 2: Load Prompts                                           │
│  └─> Load default segments (always included)                    │
│  └─> Load state-specific segments                               │
│  └─> Concatenate in order                                       │
│                                                                  │
│  Step 3: Construct Prompt                                       │
│  └─> Add system instructions                                    │
│  └─> Add current context (RAM entries)                          │
│  └─> Add recent history (optional, if enabled)                  │
│  └─> Add user request (if any)                                  │
│                                                                  │
│  Step 4: Call LLM                                               │
│  └─> Send prompt to provider                                    │
│  └─> Wait for response (timeout: 30s default)                   │
│  └─> Handle errors (retry with backoff, max 3)                  │
│                                                                  │
│  Step 5: Parse XML                                              │
│  └─> Extract instructions from response                         │
│  └─> Validate instruction schema                                │
│  └─> Handle parse errors (emit error code)                      │
│                                                                  │
│  Step 6: Execute Instructions                                   │
│  └─> Execute in document order                                  │
│  └─> Handle scope violations (reject, log audit)                │
│  └─> Handle version conflicts (retry or fail)                   │
│                                                                  │
│  Step 7: Save State                                             │
│  └─> Save think_log to RAM                                      │
│  └─> Update steps array (remove completed)                      │
│  └─> Persist RAM to disk                                        │
│                                                                  │
│  Step 8: Wait                                                   │
│  └─> Sleep for loop_delay_ms (1500ms default)                   │
│  └─> Or idle_delay_ms (5000ms) if idle state                    │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Memory Model

### KV Store Contract

**Critical:** Agent memory MUST use a mutable key-value store persisted across loops.

**Data Structure:**
```rust
pub struct KvStore {
    path: PathBuf,
    data: HashMap<String, JsonValue>,
    dirty: bool,
}

impl KvStore {
    pub fn load(path: &str) -> Result<Self> {
        // Load from JSON file
        // Create empty if not exists
    }

    pub fn save(&self) -> Result<()> {
        // Persist to JSON file (atomic write)
    }

    pub fn add(&mut self, key: &str, value: JsonValue) {
        self.data.insert(key.to_string(), value);
        self.dirty = true;
        self.save()?;
    }

    pub fn delete(&mut self, key: &str) {
        self.data.remove(key);
        self.dirty = true;
        self.save()?;
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        self.data.get(key)
    }
}
```

### Required RAM Keys

| Key | Type | Purpose | Lifecycle |
|-----|------|---------|-----------|
| `think_log` | String | Reasoning trace for current loop | Overwritten each loop |
| `plan` | String | Current step plan | Updated during planning |
| `steps` | Array | Remaining steps array | Popped during execution |
| `context` | Object | Working context for task | Accumulated during loop |
| `state` | String | Current loop state | Updated on transition |
| `user_request` | String | Original user request | Set at start, cleared at end |
| `run_id` | UUID | Current run identifier | Set at start |

### Memory Operations (XML Instructions)

| Operation | XML Tag | Example |
|-----------|---------|---------|
| Add/Update | `<ram_add>` | `<ram_add><key>think_log</key><value>Analyzing...</value></ram_add>` |
| Delete | `<ram_delete>` | `<ram_delete><key>temp_data</key></ram_delete>` |

---

## Prompt JSON Contract

### File Location

- **Primary:** `data/agent-prompt.json`
- **Configurable via:** `action_json.prompt_path` in automation rule

### Schema Definition

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "type": "object",
  "required": ["agent_name", "version", "segments"],
  "properties": {
    "agent_name": {
      "type": "string",
      "const": "kjxlkj-agent"
    },
    "version": {
      "type": "string",
      "description": "Date or semver version"
    },
    "default_mode": {
      "type": "string",
      "enum": ["reviewed", "yolo"]
    },
    "protocol": {
      "type": "string",
      "const": "xml_attrless"
    },
    "allowed_tags": {
      "type": "array",
      "items": {"type": "string"}
    },
    "segments": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "required": ["condition", "prompt"],
        "properties": {
          "condition": {
            "type": "string",
            "description": "'default' or state name"
          },
          "prompt": {
            "type": "string",
            "minLength": 1
          }
        }
      }
    }
  }
}
```

### Validation Rules

| Rule | Behavior on Violation |
|------|----------------------|
| Invalid JSON | Hard-fail startup with `PROMPT_JSON_INVALID` |
| Missing `agent_name` | Fail with `PROMPT_SCHEMA_INVALID` |
| `agent_name` != "kjxlkj-agent" | Fail with `PROMPT_SCHEMA_INVALID` |
| Missing `segments` | Fail with `PROMPT_SCHEMA_INVALID` |
| Empty segments array | Fail with `PROMPT_SCHEMA_INVALID` |
| Empty segment prompt | Fail with `PROMPT_SEGMENT_EMPTY` |
| Unknown keys | Log warning once, ignore |

### Loading Implementation

```rust
pub fn load_prompt(path: &str) -> Result<AgentPrompt> {
    let content = fs::read_to_string(path)
        .map_err(|e| AgentError::PromptLoadFailed(e.to_string()))?;

    let prompt: AgentPrompt = serde_json::from_str(&content)
        .map_err(|e| AgentError::PromptJsonInvalid(e.to_string()))?;

    // Validate agent_name
    if prompt.agent_name != "kjxlkj-agent" {
        return Err(AgentError::PromptSchemaInvalid(
            "agent_name must be 'kjxlkj-agent'".into()
        ));
    }

    // Validate segments non-empty
    if prompt.segments.is_empty() {
        return Err(AgentError::PromptSchemaInvalid(
            "segments must not be empty".into()
        ));
    }

    // Validate each segment
    for (i, segment) in prompt.segments.iter().enumerate() {
        if segment.prompt.trim().is_empty() {
            return Err(AgentError::PromptSegmentEmpty(i));
        }
    }

    Ok(prompt)
}
```

---

## YOLO Mode Contract

### Direct Mutation Rules

| Rule | Implementation |
|------|----------------|
| **Direct note mutations** | Agent can call `record_add`, `record_update` directly without review |
| **Version checks** | Agent MUST obey optimistic concurrency (base_version check) |
| **Permission scope** | Agent MUST respect workspace boundaries |
| **Cross-workspace** | Cross-workspace writes MUST be rejected with `CROSS_WORKSPACE_REJECTED` |
| **Audit trail** | Agent writes auditable as `actor_type=agent` with `agent_run_id` |

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
                format!("Note kind '{}' not allowed", note.note_kind)
            ));
        }
        Ok(())
    }

    pub fn validate_note_update(&self, note: &Note) -> Result<()> {
        // Same validation as create
        self.validate_note_create(note)
    }
}
```

### YOLO Mode Configuration

```json
{
  "mode": "yolo",
  "scope": {
    "workspace_id": "ws-uuid",
    "allowed_note_kinds": ["note", "template", "summary"],
    "cross_workspace_writes": false,
    "max_notes_per_loop": 10,
    "max_edits_per_loop": 20
  }
}
```

---

## Instruction Protocol — XML Attrless

### Allowed Instructions

| Tag | Struct | Required Children | Description |
|-----|--------|-------------------|-------------|
| `state_add` | `StateAdd` | `<state>value</state>` | Add agent state |
| `state_delete` | `StateDelete` | `<state>value</state>` | Remove agent state |
| `ram_add` | `RamAdd` | `<key>k</key><value>v</value>` | Add/update KV entry |
| `ram_delete` | `RamDelete` | `<key>k</key>` | Remove KV entry |
| `record_add` | `RecordAdd` | `<keywords>k</keywords><value>v</value>` | Create note |
| `record_issue` | `RecordIssue` | `<key>k</key><value>v</value><metadata>m</metadata>` | Flag issue |
| `record_update` | `RecordUpdate` | `<key>k</key><value>v</value>` | Update note |
| `record_search` | `RecordSearch` | `<query>q</query>` or `<ids>...</ids>` | Search notes |

### XML Parser

```rust
pub struct XmlInstructionParser;

impl XmlInstructionParser {
    pub fn parse(&self, xml: &str) -> Result<Vec<Instruction>> {
        // Parse XML without attributes (attrless)
        // Return list of instructions
    }

    fn parse_state_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <state>child</state>
        let state = node.child_text("state")?;
        Ok(Instruction::StateAdd(StateAdd { state }))
    }

    fn parse_ram_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <key> and <value> children
        let key = node.child_text("key")?;
        let value = node.child_text("value")?;
        Ok(Instruction::RamAdd(RamAdd { key, value }))
    }

    fn parse_record_add(&self, node: XmlNode) -> Result<Instruction> {
        // Extract <keywords> and <value> children
        let keywords = node.child_text("keywords")?;
        let value = node.child_text("value")?;
        Ok(Instruction::RecordAdd(RecordAdd { keywords, value }))
    }
}
```

### Example Output

```xml
<!-- Planning state: break down task -->
<ram_add>
  <key>think_log</key>
  <value>User wants meeting notes. Searching existing notes first...</value>
</ram_add>
<ram_add>
  <key>plan</key>
  <value>1. Search existing meeting notes
2. Create new note if needed
3. Summarize findings</value>
</ram_add>
<ram_add>
  <key>steps</key>
  <value>["Search existing meeting notes", "Create new note if needed", "Summarize findings"]</value>
</ram_add>
<state_add>
  <state>executing</state>
</state_add>

<!-- Executing state: perform search -->
<record_search>
  <query>meeting notes</query>
</record_search>

<!-- Executing state: create note -->
<record_add>
  <keywords>meeting,notes,summary</keywords>
  <value>{"title": "Meeting Notes 2026-02-24", "markdown": "# Meeting Notes..."}</value>
</record_add>
```

### Parsing Rules

- UTF-8 text only
- No XML attributes allowed (attrless protocol)
- Instructions execute in document order
- Unknown tags ignored (or rejected in strict mode)
- Parse failures emit stable error codes

---

## Provider Adapters

### LLM Provider Trait

```rust
#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn complete(&self, prompt: &str) -> Result<LlmResponse>;
    fn provider_name(&self) -> &str;
    fn is_available(&self) -> bool;
    fn health_check(&self) -> HealthStatus;
}
```

### Supported Providers

| Provider | Kind | Base URL | Auth | Use Case |
|----------|------|----------|------|----------|
| **LMStudio** | `lmstudio` | `http://127.0.0.1:1234/v1` | None | Local models |
| **Ollama** | `ollama` | `http://127.0.0.1:11434/api` | None | Local models |
| **OpenRouter** | `openrouter` | `https://openrouter.ai/api/v1` | Bearer | Cloud models |
| **OpenAI** | `openai` | `https://api.openai.com/v1` | Bearer | Cloud models |
| **Stub** | `stub` | N/A | N/A | Testing (deterministic) |

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
        let client = reqwest::Client::new();
        let request = LmStudioRequest {
            model: &self.model,
            messages: vec![
                Message { role: "user", content: prompt }
            ],
            max_tokens: self.max_tokens,
            temperature: self.temperature,
        };

        let response = client
            .post(format!("{}/v1/chat/completions", self.base_url))
            .json(&request)
            .timeout(Duration::from_millis(self.timeout_ms))
            .send()
            .await?;

        let result: LmStudioResponse = response.json().await?;
        Ok(LlmResponse {
            content: result.choices[0].message.content.clone(),
            reasoning: result.choices[0].message.reasoning.clone(),
        })
    }

    fn provider_name(&self) -> &str {
        "lmstudio"
    }

    fn is_available(&self) -> bool {
        // Health check via GET /health
    }
}
```

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

| Code | Description | Recovery |
|------|-------------|----------|
| `PROMPT_JSON_INVALID` | JSON parse failure | Fix JSON, restart |
| `PROMPT_SCHEMA_INVALID` | Schema validation failure | Fix schema, restart |
| `XML_PARSE_ERROR` | XML instruction parse failure | Retry with backoff |
| `LLM_TIMEOUT` | Provider timeout | Retry (max 3) |
| `LLM_UNAVAILABLE` | Provider connection failed | Retry with backoff |
| `SCOPE_VIOLATION` | Cross-workspace or kind violation | Log audit, skip |
| `VERSION_CONFLICT` | Optimistic concurrency failure | Refresh, retry |
| `RATE_LIMITED` | Provider rate limit | Backoff (exponential) |

### Audit Metadata

```rust
pub struct AgentRunAudit {
    pub run_id: Uuid,
    pub rule_id: Uuid,
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

## Performance Targets

| Metric | Target |
|--------|--------|
| Loop iteration time | < 5s (P95) |
| LLM response time | < 3s (P95) |
| XML parse time | < 100ms |
| Instruction execution | < 500ms per instruction |
| RAM persistence | < 50ms |
| Max iterations per run | 100 (configurable) |

---

## Related

- [Domain automation](/docs/spec/domain/automation.md) — rule model
- [XML protocol](/docs/spec/api/librarian-xml.md) — instruction format
- [Agent prompt JSON](agent-prompt-json.md) — schema definition
- [Testing](testing.md) — acceptance IDs
- [Configuration](/docs/spec/architecture/configuration.md) — config loading
