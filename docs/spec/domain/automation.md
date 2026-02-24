# Automation Domain — kjxlkj-agent

**Back:** [Domain Root](/docs/spec/domain/README.md)

---

## Rule Model

### Automation Rule Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `rule_id` | UUID | Yes | Unique rule identifier |
| `workspace_id` | UUID | Yes | Owning workspace |
| `trigger` | String | Yes | Event source (e.g., `workspace_event.note_changed`) |
| `condition_json` | Object | Yes | Deterministic predicate configuration |
| `action_json` | Object | Yes | Action definition (includes kjxlkj_agent kind) |
| `enabled` | Boolean | Yes | Controls active evaluation |
| `created_at` | Timestamp | Yes | Creation time |
| `updated_at` | Timestamp | Yes | Last modification time |

### Trigger Types

| Trigger | Description |
|---------|-------------|
| `workspace_event.note_changed` | Any note create/update |
| `workspace_event.note_created` | Note creation only |
| `workspace_event.note_deleted` | Note deletion |
| `automation_schedule.cron` | Cron-based schedule |
| `manual` | Manual launch via API |

---

## kjxlkj-agent Contract

### Canonical Identity

| Requirement | Value |
|-------------|-------|
| **Agent name** | `kjxlkj-agent` (MUST) |
| **Configuration source** | `data/config.json` + `data/agent-prompt.json` |
| **Prompt format** | JSON-defined segments |
| **Memory model** | KV store (persisted across loops) |
| **Conversation logs** | Disabled by default |

### Action JSON Shape

```json
{
  "kind": "kjxlkj_agent",
  "prompt_path": "./data/agent-prompt.json",
  "provider": {
    "provider_kind": "lmstudio",
    "base_url": "http://127.0.0.1:1234/v1",
    "model": "nvidia/nemotron-3-nano",
    "timeout_ms": 300000,
    "max_tokens": 4096,
    "temperature": 0.1
  },
  "mode": "yolo",
  "memory": {
    "kv_store_path": "./data/agent-kv-store.json",
    "retain_full_conversation_logs": false
  },
  "loop": {
    "loop_delay_ms": 1500,
    "idle_delay_ms": 5000,
    "max_iterations": 100
  },
  "scope": {
    "workspace_id": "ws-uuid",
    "allowed_note_kinds": ["note", "template"],
    "cross_workspace_writes": false
  }
}
```

---

## Loop Model

### Iterative Loop Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    kjxlkj-agent Loop                        │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │ Planning │───>│Executing │───>│Evaluating│───┐         │
│  └──────────┘    └──────────┘    └──────────┘   │         │
│       ^                                          │         │
│       │                                          │         │
│       └──────────────────────────────────────────┘         │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  RAM (KV Store) - Persists Across Loops              │  │
│  │  - think_log: reasoning trace                        │  │
│  │  - plan: current step plan                           │  │
│  │  - steps: remaining steps                            │  │
│  │  - context: working context                          │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Records (Persistent Notes)                          │  │
│  │  - record_add: create new note                       │  │
│  │  - record_update: modify existing note               │  │
│  │  - record_search: find notes                         │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### Loop States

| State | Transition In | Transition Out |
|-------|---------------|----------------|
| `planning` | Start, after evaluating | Executing |
| `executing` | After planning | Evaluating |
| `evaluating` | After executing | Planning |
| `idle` | No pending work | Planning (on trigger) |

### Loop Cycle

1. **Load state** from RAM (KV store)
2. **Load prompt segments** based on current state
3. **Construct prompt** with context
4. **Call LLM provider** with prompt
5. **Parse XML instructions** from response
6. **Execute instructions** (state, RAM, record ops)
7. **Save state** to RAM (KV store)
8. **Wait** (loop_delay_ms or idle_delay_ms)
9. **Repeat**

---

## Memory Model

### KV Store Contract

**Critical:** Agent memory MUST use a mutable key-value store persisted across loops.

**Operations:**
- `ram_add(key, value)` — Add/update KV entry
- `ram_delete(key)` — Remove KV entry
- `ram_get(key)` — Retrieve KV entry (internal)

**Required Keys:**

| Key | Purpose |
|-----|---------|
| `think_log` | Reasoning trace for current loop |
| `plan` | Current step plan |
| `steps` | Remaining steps array |
| `context` | Working context for task |
| `state` | Current loop state (planning/executing/evaluating) |

### Memory Persistence

```json
// data/agent-kv-store.json
{
  "think_log": "Analyzing user request to create note...",
  "plan": "1. Search existing notes\n2. Create new note if needed",
  "steps": ["Search existing notes", "Create new note if needed"],
  "context": {
    "user_request": "Create meeting notes",
    "workspace_id": "ws-uuid"
  },
  "state": "planning"
}
```

### Conversation Log Policy

| Setting | Value | Behavior |
|---------|-------|----------|
| `retain_full_conversation_logs` | `false` (default) | Only KV store persists; no chat history |
| `retain_full_conversation_logs` | `true` | Full conversation history stored (not recommended) |

**Default:** Disabled to minimize storage and maximize privacy.

---

## Prompt JSON Contract

### File Location

- **Primary:** `data/agent-prompt.json`
- **Configurable via:** `action_json.prompt_path`

### Schema

```json
{
  "agent_name": "kjxlkj-agent",
  "version": "2026-02-15",
  "default_mode": "yolo",
  "protocol": "xml_attrless",
  "allowed_tags": [
    "state_add",
    "state_delete",
    "ram_add",
    "ram_delete",
    "record_add",
    "record_issue",
    "record_update",
    "record_search"
  ],
  "segments": [
    {
      "condition": "default",
      "prompt": "Output XML instructions only. No prose. No attributes."
    },
    {
      "condition": "default",
      "prompt": "You are one step in an iterative agent loop. RAM KV entries persist across loops. Records are persistent notes."
    },
    {
      "condition": "default",
      "prompt": "Use only: state_add, state_delete, ram_add, ram_delete, record_add, record_issue, record_update, record_search."
    },
    {
      "condition": "default",
      "prompt": "Maintain a think_log entry in RAM each loop via ram_add key think_log."
    },
    {
      "condition": "default",
      "prompt": "When creating notes without explicit title, set title to current datetime from system clock."
    },
    {
      "condition": "default",
      "prompt": "In yolo mode you may create and edit notes directly, but never write outside authorized workspace scope."
    },
    {
      "condition": "planning",
      "prompt": "Break large tasks into small executable steps and store plan in RAM keys plan,next or plan,steps."
    },
    {
      "condition": "planning",
      "prompt": "Search existing records before creating near-duplicates."
    },
    {
      "condition": "executing",
      "prompt": "Execute current step with concrete record mutations, then transition to evaluating."
    },
    {
      "condition": "evaluating",
      "prompt": "Store execution outcome in RAM and transition back to planning."
    },
    {
      "condition": "record_organizing",
      "prompt": "Deduplicate records, strengthen links, and maintain keyword quality for search."
    },
    {
      "condition": "paging",
      "prompt": "When RAM size exceeds threshold, archive critical context and use ram_delete to shrink RAM."
    },
    {
      "condition": "idle",
      "prompt": "Perform minimal maintenance while waiting for new user input."
    }
  ]
}
```

### Segment Rules

| Field | Requirement |
|-------|-------------|
| `condition` | MUST be `default` or a known state label |
| `prompt` | MUST be non-empty UTF-8 string |
| Order | Segments applied in array order |

### Supported Conditions

| Condition | When Applied |
|-----------|--------------|
| `default` | Always included (base instructions) |
| `planning` | When state = planning |
| `executing` | When state = executing |
| `evaluating` | When state = evaluating |
| `record_organizing` | During deduplication tasks |
| `paging` | When RAM size exceeds threshold |
| `idle` | When waiting for triggers |

---

## YOLO Mode Contract

### Direct Mutation Rules

| Rule | Description |
|------|-------------|
| **Direct writes** | YOLO mode allows direct note create/edit/delete |
| **Version checks** | Optimistic concurrency still applies |
| **Permission scope** | Workspace boundaries enforced |
| **Cross-workspace** | MUST be rejected with error |
| **Audit trail** | All operations logged with agent metadata |

### Scope Guardrails

```json
{
  "scope": {
    "workspace_id": "ws-uuid",
    "allowed_note_kinds": ["note", "template"],
    "cross_workspace_writes": false,
    "max_notes_per_loop": 10,
    "max_edits_per_loop": 20
  }
}
```

---

## Instruction Protocol — XML Attrless

### Allowed Tags

| Tag | Purpose | Required Children |
|-----|---------|-------------------|
| `state_add` | Add agent state | `<state>value</state>` |
| `state_delete` | Remove agent state | `<state>value</state>` |
| `ram_add` | Add/update KV | `<key>k</key><value>v</value>` |
| `ram_delete` | Remove KV | `<key>k</key>` |
| `record_add` | Create note | `<keywords>k</keywords><value>v</value>` |
| `record_issue` | Flag issue | `<key>k</key><value>v</value><metadata>m</metadata>` |
| `record_update` | Update note | `<key>k</key><value>v</value>` |
| `record_search` | Search notes | `<query>q</query>` or `<ids>...</ids>` |

### Example Output

```xml
<ram_add>
  <key>think_log</key>
  <value>Searching for existing meeting notes...</value>
</ram_add>
<record_search>
  <query>meeting notes</query>
</record_search>
```

### Parsing Rules

- UTF-8 text only
- No XML attributes (attrless)
- Instructions execute in document order
- Unknown tags ignored (or rejected in strict mode)
- Parse failures emit stable error codes

---

## Run Lifecycle

### Run States

| State | Meaning |
|-------|---------|
| `Queued` | Rule qualified, awaits execution |
| `Running` | Action executing |
| `Succeeded` | Action completed, effects committed |
| `Failed` | Action failed, error recorded |

### Idempotency

- Runs MUST be idempotent per `(rule_id, triggering_event_id)`
- Duplicate triggers replay existing run result
- Run audit logs keep operation summaries (not full chat history)

---

## Determinism and Safety

### Bounded Execution

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_iterations` | 100 | Max loop iterations per run |
| `loop_delay_ms` | 1500 | Delay between iterations |
| `max_notes_per_loop` | 10 | Max note creations per loop |
| `max_edits_per_loop` | 20 | Max note edits per loop |

### Error Handling

| Error | Behavior |
|-------|----------|
| LLM timeout | Retry with backoff (max 3), then fail |
| Parse failure | Emit error code, transition to failed |
| Schema invalid | Hard-fail at startup |
| Scope violation | Reject operation, log audit entry |

### Audit Metadata

Every run MUST log:

- `prompt_hash` — SHA-256 of prompt used
- `parser_version` — XML parser version
- `operation_count` — Number of instructions executed
- `loop_count` — Number of iterations
- `error_code` — If failed

---

## Related

- [Notes](notes.md) — note lifecycle
- [Agent technical contract](/docs/spec/technical/librarian-agent.md) — loop implementation
- [XML protocol](/docs/spec/api/librarian-xml.md) — instruction format
- [Testing](/docs/spec/technical/testing.md) — acceptance IDs
